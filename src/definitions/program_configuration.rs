use std::fs;
use serde::{Deserialize, Serialize};
use crate::definitions::interface_data_definition::InterfaceDataDefinition;
use crate::definitions::setup_definition::{Setup, SetupCommand};
use crate::definitions::write;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegexDefinition {
    left: String,
    right: String,
    allow_failure: bool
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IsEqualDefintion {
    left: String,
    right: serde_yaml::Value,
    allow_failure: bool
}
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationDefinition {
    is_eq(IsEqualDefintion),
    regex(RegexDefinition)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecvMqttDefinition {
    message: String,
    verify: Vec<VerificationDefinition>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMqttDefinition {
    message: String,
    parameters: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RunDefinition {
    send_mqtt(SendMqttDefinition),
    recv_mqtt(RecvMqttDefinition)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestDefinition {
    run: Vec<RunDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    description: String,
    interface_data_definition: InterfaceDataDefinition,
    setup: Setup,
    test: TestDefinition,
}

impl Config{
    pub fn generate_mark_down(self: &Config, filename: &String){
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename).unwrap();

        let title: &String = &self.name;
        let desc: &String = &self.description;
        write(&mut file,format!("# {}\n",title));
        write(&mut file,format!("{}\n\n---\n\n",desc));
        self.interface_data_definition.generate_mark_down(&mut file);
        self.setup.generate_mark_down(&mut file);
    }

}

