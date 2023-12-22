use crate::definitions::interface_data_definition::{ConnectionType, Definition, InterfaceDataDefinition, MqttMessage, ProtocolDataDescription};
use crate::definitions::program_configuration::Config;
use crate::definitions::setup_definition::{
    ConnectionCmd, CreateDockerMongoDbCmd, DataEntry, InitMongoDbCmd, Setup, SetupCommand,
};
use crate::definitions::test_definition::{
    IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition,
    TestDefinition,
};
use std::fs;
use std::io::Write;
use std::panic::PanicInfo;

pub mod interface_data_definition;
pub mod program_configuration;
pub mod setup_definition;
pub(crate) mod test_definition;

pub fn write(file: &mut fs::File, s: String) {
    match file.write(s.as_bytes()) {
        Ok(_) => {
            return;
        }
        Err(e) => {
            println!("write not work:{:?} ", e);
        }
    }
}

pub(crate) trait Visitor {
    fn visit_config(&mut self, config: &mut Config){
        
    }
    fn visit_test_def(&mut self, def: &mut TestDefinition){
        
    }
    fn visit_run_def(&mut self, def: &mut RunDefinition){
        
    }
    fn visit_send_mqtt_def(&mut self, def: &mut SendMqttDefinition){
        
    }
    fn visit_recv_mqtt_dev(&mut self, def: &mut RecvMqttDefinition){
        
    }
    fn visit_is_equal_def(&mut self, def: &mut IsEqualDefintion){
        
    }
    fn visit_regex_def(&mut self, def: &mut RegexDefinition){
        
    }
    fn visit_create_docker_mongo(&mut self, def: &mut CreateDockerMongoDbCmd){
        
    }
    fn visit_data_entry(&mut self, def: &mut DataEntry){
        
    }
    fn visit_init_mongo(&mut self, def: &mut InitMongoDbCmd){
        
    }
    fn visit_connection_def(&mut self, def: &mut ConnectionCmd){
        
    }
    fn visit_setup_command_def(&mut self, def: &mut SetupCommand){
        
    }
    fn visit_setup_def(&mut self, def: &mut Setup){
        
    }
    fn visit_definition(&mut self, def: &mut Definition){
        
    }
    fn visit_interface_data_def(&mut self, def: &mut InterfaceDataDefinition){
        
    }
    fn visit_protocol_data_description_def(&mut self, def: &mut ProtocolDataDescription){
        
    }
    fn visit_mqtt_message_def(&mut self, def: &mut MqttMessage){
        
    }
    fn visit_connection_type_def(&mut self, def: &mut ConnectionType){
        
    }
}
