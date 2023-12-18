use std::fs;
use std::io::Write;
use crate::definitions::interface_data_definition::{ConnectionType, InterfaceDataDefinition, MqttMessage, ProtocolDataDescription};
use crate::definitions::program_configuration::{Config, IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition, TestDefinition};
use crate::definitions::setup_definition::{ConnectionCmd, CreateDockerMongoDbCmd, DataEntry, InitMongoDbCmd, Setup, SetupCommand};

pub mod setup_definition;
pub mod interface_data_definition;
pub mod program_configuration;

pub fn write(file: &mut fs::File, s: String){
    match file.write(s.as_bytes()){
        Ok(_)=>{ return;}
        Err(e)=>{ println!("write not work");}
    }
}

trait Visitor{
    fn visit_config(&self:Self,config: Config);
    fn visit_test_def(&self:Self, def: TestDefinition);
    fn visit_run_def(&self:Self,def: RunDefinition);
    fn visit_send_mqtt_def(&self:Self,def: SendMqttDefinition);
    fn visit_recv_mqtt_dev(&self:Self,def: RecvMqttDefinition);
    fn visit_is_equal_def(&self:Self,def: IsEqualDefintion);
    fn visit_regex_def(&self:Self,def: RegexDefinition);
    fn visit_create_docker_mongo(&self:Self,def: CreateDockerMongoDbCmd);
    fn visit_data_entry(&self:Self,def: DataEntry);
    fn visit_init_mongo(&self:Self, def: InitMongoDbCmd);
    fn visit_connection_def(&self:Self,def: ConnectionCmd);
    fn visit_setup_command_def(&self:Self,def: SetupCommand);
    fn visit_setup_def(&self:Self,def: Setup);
    fn visit_interface_data_def(&self:Self,def: InterfaceDataDefinition);
    fn visit_protocol_data_description_def(&self:Self,def: ProtocolDataDescription);
    fn visit_mqtt_message_def(&self:Self,def: MqttMessage);
    fn visit_connection_type_def(&self:Self,def: ConnectionType);
}

struct GenerateMarkDownVisitor;
