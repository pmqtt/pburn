use std::fs;
use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::definitions::program_configuration::Config;
use crate::definitions::write;

#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionType{
    mqtt,
    rest,
    tcp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MqttMessage{
    name: String,
    parameter: Vec<serde_yaml::Value>,
    topic: String,
    payload: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProtocolDataDescription{
    mqtt_message(MqttMessage)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceDataDefinition{
    pub(crate) connection_type: ConnectionType,
    protocol_data_description: Vec<ProtocolDataDescription>
}

impl InterfaceDataDefinition{
    pub fn generate_mark_down(self: &Self,file: &mut fs::File){
        write(file, "## IDD - Interface Data Definition \n\n --- \n".to_string());
        self.generate_connection_type(file);
        self.generate_protocol_definition(file);
        write(file, "--- \n".to_string());
    }

    fn generate_connection_type(self: &Self, file: &mut File) {
        match (self.connection_type) {
            ConnectionType::mqtt => {
                write(file, "The used connection type is MQTT \n\n  \n".to_string());
            }
            ConnectionType::rest => {
                write(file, "The used connection type is REST \n".to_string());
            }
            ConnectionType::tcp => {
                write(file, "The uses connection type is TCP \n".to_string());
            }
        }
    }

    fn generate_protocol_definition(self: &Self, file: &mut File) {
        for x in &self.protocol_data_description {
            match (x) {
                ProtocolDataDescription::mqtt_message(value) => {
                    write(file, format!("### MQTT-Message {}\n\n", value.name));
                    write(file, "**Parameters:**\n\n".to_string());
                    for parameter_items in &value.parameter {
                        if let Some(parameters) = parameter_items.as_mapping() {
                            for (key, value) in parameters {
                                let Some(key_str) = key.as_str() else { todo!() };
                                let Some(value_str) = value.as_str() else { todo!() };
                                write(file, format!("- `{}`: {}\n", key_str, value_str));
                            }
                        }
                    }

                    write(file, format!("\n\n**Topic:**\n - `{}`\n\n", value.topic));
                    write(file, format!("**Payload:**\n - `{}`\n\n", value.payload));
                }
            }
        }
    }
}