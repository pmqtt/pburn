use std::rc::Rc;
use std::thread;
use std::time::Duration;
use crate::definitions::test_definition::{IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition, TestDefinition};
use crate::definitions::Visitor;
use crate::visitors::generate_communication_protocol::{GenerateCommunicationProtocol, MqttProtocol, Parameter, ParameterType, ParameterValue, ProtocolType};
use crate::visitors::generate_setup_environment::GenerateSetupEnvironmentVisitor;
use rumqttc::{MqttOptions, Client, QoS, ClientError, ConnectionError, Event, Incoming, Outgoing, Connection};
use crate::network::mqtt_broker::MqttBroker;
//use crate::network::{await_connection_to_broker, await_publish_is_send, connect_send_to_broker};


pub trait PlaybookItem{
    fn execute(&mut self)->Result<bool,String>{
        Ok(true)
    }

    fn verify(&mut self)->Result<bool,String>{
        Ok(true)
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
    pub(crate) mqtt_broker: MqttBroker
}
impl PlaybookItem for MqttRecvPlaybookItem {
    fn execute(&mut self) -> Result<bool, String> {
        println!("{:?}",self.mqtt_broker.get_message().payload);
        Ok(true)
    }

    fn verify(&mut self) -> Result<bool, String> {
       Ok(true)
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
                        broker.subscribe(&p.topic);
                        let playbook_item = MqttRecvPlaybookItem {
                            item: p.clone(),
                            parameters: arguments,
                            broker_connection: BrokerConnection {
                                host: host.clone(),
                                port: port.clone(),
                            },
                            mqtt_broker:broker,
                        };
                        self.playbook.push(Box::new(playbook_item) as Box<dyn PlaybookItem>);
                    }
                    ProtocolType::None => {}
                }
            }
        }
    }
    /*
    #[allow(unused_variables)]
    fn visit_is_equal_def(&mut self, def: &mut IsEqualDefintion) {
        
    }
    #[allow(unused_variables)]
    fn visit_regex_def(&mut self, def: &mut RegexDefinition) {
    }
    */
}