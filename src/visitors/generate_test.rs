use std::rc::Rc;
use crate::definitions::test_definition::{IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition, TestDefinition};
use crate::definitions::Visitor;
use crate::visitors::generate_communication_protocol::{GenerateCommunicationProtocol, MqttProtocol, Parameter, ParameterType, ParameterValue, ProtocolType};
use crate::visitors::generate_setup_environment::GenerateSetupEnvironmentVisitor;

pub trait PlaybookItem{
    fn execute(&self)->Result<bool,String>{
        Ok(true)
    }

    fn verify(&self)->Result<bool,String>{
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
    pub(crate) broker_connection: BrokerConnection
}
impl PlaybookItem for MqttSendPlaybookItem{
    fn execute(&self)->Result<bool,String>{
        println!("{:?}",&self.item.create_payload(&self.parameters));
        Ok(true)
    }
}


pub struct MqttRecvPlaybookItem{
    pub(crate) item: MqttProtocol,
    pub(crate) parameters: Vec<Parameter>
}
impl PlaybookItem for MqttRecvPlaybookItem {

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

                        let playbook_item = MqttSendPlaybookItem {
                            item: p.clone(),
                            parameters: arguments,
                            broker_connection: BrokerConnection {
                                host: self.setup.,
                                port: "".to_string(),
                            },
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