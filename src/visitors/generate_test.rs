use std::rc::Rc;
use crate::definitions::interface_data_definition::MqttMessage;
use crate::definitions::test_definition::{IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition, TestDefinition};
use crate::definitions::Visitor;
use crate::visitors::generate_communication_protocol::{GenerateCommunicationProtocol, MqttProtocol, Parameter, ParameterType, ParameterValue, ProtocolType};
use crate::visitors::generate_setup_environment::GenerateSetupEnvironmentVisitor;

use crate::network::mqtt_broker::{MqttBroker, MqttMessageFromBroker};

#[derive(Clone)]
pub enum VerifierKind{
    is_equal_mqtt_message( MqttProtocol,Vec<Parameter>),
}


#[derive(Clone)]
pub enum Verifier{
    IsEqual(String, String,bool)
}

pub trait PlaybookItem{
    fn execute(&mut self)->Result<bool,String>{
        Ok(true)
    }
    fn verify(&mut self)->Result<bool,String>{
        Ok(true)
    }


    fn set_verifier(&mut self, verifier: Verifier){

    }

}

pub struct BrokerConnection{
    pub(crate) host: String,
    pub(crate) port: String
}

pub struct MqttSendPlaybookItem{
    pub(crate) item: MqttProtocol,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) broker_connection: BrokerConnection,
    pub(crate) mqtt_broker: MqttBroker
}
impl PlaybookItem for MqttSendPlaybookItem{
    fn execute(&mut self)->Result<bool,String>{
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
    fn execute(&mut self) -> Result<bool, String> {
        //println!("{:?}",self.mqtt_broker.get_message().payload);
        Ok(true)
    }
    // Introduce Data Map
    fn verify(&mut self) -> Result<bool, String> {
       let payload = self.mqtt_broker.get_message().payload;
       match &self.verifier{
           None => {}
           Some(v) => {
               match v{
                   Verifier::IsEqual(left, right, allow) => {
                       // Todo: allow on failure
                       if payload.as_str() == right{
                           return Ok(true);
                       }
                       return Ok(false);
                   }
               }
           }
       }
       Ok(true)
    }

    fn set_verifier(&mut self, verifier: Verifier){
        self.verifier = Some(verifier);
    }
}


pub struct GenerateTest{
    pub(crate) playbook: Vec<Box<dyn PlaybookItem>>,
    pub(crate) idd: Rc<GenerateCommunicationProtocol>,
    pub(crate) setup: Rc<GenerateSetupEnvironmentVisitor>
}
impl GenerateTest{
    pub fn new(setup: Rc<GenerateSetupEnvironmentVisitor>,idd: Rc<GenerateCommunicationProtocol>) -> GenerateTest{
        GenerateTest{
            playbook: vec![],
            idd: idd,
            setup: setup
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
        let option = self.idd.protocol_type.get(&def.message);
        match option{
            None => {}
            Some(value) => {
                match value {
                    ProtocolType::Mqtt(p) => {
                        let mut arguments:Vec<Parameter> = Vec::new();
                        for i in 0..def.parameters.len() {
                            arguments.push(Parameter{
                                key:  p.parameters[i].key.clone(),
                                typ: ParameterType::Str,
                                value: ParameterValue::Str(def.parameters[i].clone()),
                            });
                        }
                        let host = self.setup.get_connection_property_by_id(&def.used_connection).unwrap().get_host();
                        let port = self.setup.get_connection_property_by_id(&def.used_connection).unwrap().get_port();

                        let playbook_item = MqttSendPlaybookItem {
                            item: p.clone(),
                            parameters: arguments,
                            broker_connection: BrokerConnection {
                                host: host.clone(),
                                port: port.clone(),
                            },
                            mqtt_broker: MqttBroker::new(&host,port.clone().parse::<u16>().unwrap()),
                        };
                        self.playbook.push(Box::new(playbook_item) as Box<dyn PlaybookItem>);

                    }
                    ProtocolType::None => {

                    }
                }
            }
        }
    }



    #[allow(unused_variables)]
    fn visit_recv_mqtt_dev(&mut self, def: &mut RecvMqttDefinition) {
        let option = self.idd.protocol_type.get(&def.message);
        match option {
            None => {}
            Some(value) => {
                match value{
                    ProtocolType::Mqtt(p) => {
                        let mut arguments:Vec<Parameter> = Vec::new();
                        for i in 0..def.parameters.len() {
                            arguments.push(Parameter{
                                key:  p.parameters[i].key.clone(),
                                typ: ParameterType::Str,
                                value: ParameterValue::Str(def.parameters[i].clone()),
                            });
                        }
                        let host = self.setup.get_connection_property_by_id(&def.used_connection).unwrap().get_host();
                        let port = self.setup.get_connection_property_by_id(&def.used_connection).unwrap().get_port();
                        let mut broker = MqttBroker::new(&host,port.clone().parse::<u16>().unwrap());
                        let topic: String = p.create_topic(&arguments).unwrap().clone();
                        broker.subscribe(&topic);
                        let playbook_item = MqttRecvPlaybookItem {
                            item: p.clone(),
                            parameters: arguments.clone(),
                            broker_connection: BrokerConnection {
                                host: host.clone(),
                                port: port.clone(),
                            },
                            mqtt_broker:broker,
                            verifier: None
                        };
                        self.playbook.push(Box::new(playbook_item) as Box<dyn PlaybookItem>);
                    }
                    ProtocolType::None => {}
                }
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