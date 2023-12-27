use std::rc::Rc;
use std::time::Duration;
use clap::builder::TypedValueParser;
use tokio::time::timeout;
use crate::definitions::interface_data_definition::MqttMessage;
use crate::definitions::test_definition::{IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition, TestDefinition};
use crate::definitions::Visitor;
use crate::visitors::generate_communication_protocol::{GenerateCommunicationProtocol, MqttProtocol, Parameter, ParameterType, ParameterValue, ProtocolType};
use crate::visitors::generate_setup_environment::GenerateSetupEnvironmentVisitor;

use crate::network::mqtt_broker::{MqttBroker, MqttMessageFromBroker};
use crate::runtime::{RunTimeData, RuntimeEnvironment};

#[derive(Clone)]
pub enum VerifierKind{
    is_equal_mqtt_message( MqttProtocol,Vec<Parameter>),
}


#[derive(Clone)]
pub enum Verifier{
    IsEqual(String, String,bool)
}

pub trait PlaybookItem{
    fn execute(&mut self,env : &mut RuntimeEnvironment)->Result<bool,String>{
        Ok(true)
    }
    fn verify(&mut self,env: &mut RuntimeEnvironment)->Result<bool,String>{
        Ok(true)
    }


    fn set_verifier(&mut self, verifier: Verifier){}

}

pub struct BrokerConnection{
    pub(crate) host: String,
    pub(crate) port: String,
}

pub struct MqttSendPlaybookItem{
    pub(crate) item: MqttProtocol,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) broker_connection: BrokerConnection,
    pub(crate) mqtt_broker: MqttBroker
}
impl PlaybookItem for MqttSendPlaybookItem{
    fn execute(&mut self, env: &mut RuntimeEnvironment)->Result<bool,String>{
        let payload: String = self.item.create_payload(&self.parameters).unwrap().clone();
        let topic: String = self.item.create_topic(&self.parameters).unwrap().clone();
        self.mqtt_broker.send(&topic,&payload);
        Ok(true)
    }
}

impl MqttSendPlaybookItem {

}


pub struct MqttRecvPlaybookItem{
    pub(crate) item: MqttProtocol,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) broker_connection: BrokerConnection,
    pub(crate) mqtt_broker: MqttBroker,
    pub(crate) verifier: Option<Verifier>
}


impl PlaybookItem for MqttRecvPlaybookItem {
    fn execute(&mut self, env: &mut RuntimeEnvironment) -> Result<bool, String> {
        let res = self.mqtt_broker.get_message();
        match res{
            Ok(message) => {
                let payload = message.payload;
                env.insert("$PAYLOAD".to_string(), RunTimeData::String(payload.clone()));
                Ok(true)
            }
            Err(err) => {
                return Err(err);
            }
        }

    }

    fn verify(&mut self, env: &mut RuntimeEnvironment) -> Result<bool, String> {
        if let Some(Verifier::IsEqual(left, right, _allow)) = &self.verifier {
            let l = env.get(left);
            let r = env.get(right);
            return Ok(match (l, r) {
                (None, None) => left == right,
                (Some(RunTimeData::String(v)), None) | (None, Some(RunTimeData::String(v))) => v == right || v == left,
                (Some(RunTimeData::String(lv)), Some(RunTimeData::String(rv))) => lv == rv,
                _ => false,
            });
        }
        Ok(false)
    }

    fn set_verifier(&mut self, verifier: Verifier){
        self.verifier = Some(verifier);
    }
}


pub struct GenerateTest{
    pub(crate) playbook: Vec<Box<dyn PlaybookItem>>,
    pub(crate) idd: Rc<GenerateCommunicationProtocol>,
    pub(crate) setup: Rc<GenerateSetupEnvironmentVisitor>,
    pub(crate) environment: RuntimeEnvironment
}
impl GenerateTest{
    pub fn new(setup: Rc<GenerateSetupEnvironmentVisitor>,idd: Rc<GenerateCommunicationProtocol>, env: RuntimeEnvironment) -> GenerateTest{
        GenerateTest{
            playbook: vec![],
            idd: idd,
            setup: setup,
            environment: env
        }
    }

    fn set_last_verifier(&mut self, verifier: Verifier){
        if let Some(item) = self.playbook.last_mut() {
            item.set_verifier(verifier);
        }
    }

}

impl Visitor for GenerateTest{

    #[allow(unused_variables)]
    fn visit_test_def(&mut self, def: &mut TestDefinition) {
        
    }
    #[allow(unused_variables)]
    fn visit_run_def(&mut self, def: &mut RunDefinition) {

    }

    fn visit_send_mqtt_def(&mut self, def: &mut SendMqttDefinition) {
        if let Some(ProtocolType::Mqtt(p)) = self.idd.protocol_type.get(&def.message) {
            let arguments: Vec<Parameter> = def.parameters.iter().enumerate().map(|(i, param)| {
                Parameter {
                    key: p.parameters[i].key.clone(),
                    typ: ParameterType::Str,
                    value: ParameterValue::Str(param.clone()),
                }
            }).collect();

            if let Some(connection_prop) = self.setup.get_connection_property_by_id(&def.used_connection) {
                let host = connection_prop.get_host();
                let port = connection_prop.get_port();
                let mqtt_broker = MqttBroker::new(&host, port.parse::<u16>().unwrap(),None);

                let playbook_item = MqttSendPlaybookItem {
                    item: p.clone(),
                    parameters: arguments,
                    broker_connection: BrokerConnection { host: host.clone(), port: port.clone() },
                    mqtt_broker,
                };
                self.playbook.push(Box::new(playbook_item) as Box<dyn PlaybookItem>);
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_recv_mqtt_dev(&mut self, def: &mut RecvMqttDefinition) {
        if let Some(ProtocolType::Mqtt(p)) = self.idd.protocol_type.get(&def.message) {
            let arguments: Vec<Parameter> = def.parameters.iter().enumerate().map(|(i, param)| {
                Parameter {
                    key: p.parameters[i].key.clone(),
                    typ: ParameterType::Str,
                    value: ParameterValue::Str(param.clone()),
                }
            }).collect();

            if let Some(connection_prop) = self.setup.get_connection_property_by_id(&def.used_connection) {
                let host = connection_prop.get_host();
                let port = connection_prop.get_port();
                let mut broker = MqttBroker::new(&host, port.parse::<u16>().unwrap(),Some(Duration::from_secs(def.timeout.parse::<u64>().unwrap())));

                let topic = p.create_topic(&arguments).unwrap_or_default();
                broker.subscribe(&topic);

                let playbook_item = MqttRecvPlaybookItem {
                    item: p.clone(),
                    parameters: arguments,
                    broker_connection: BrokerConnection { host: host.clone(), port: port.clone() },
                    mqtt_broker: broker,
                    verifier: None
                };
                self.playbook.push(Box::new(playbook_item) as Box<dyn PlaybookItem>);
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_is_equal_def(&mut self, def: &mut IsEqualDefintion) {
        self.set_last_verifier(Verifier::IsEqual(def.left.clone(),def.right.clone(),def.allow_failure));
    }
    /*
    #[allow(unused_variables)]
    fn visit_regex_def(&mut self, def: &mut RegexDefinition) {
    }
    */
}