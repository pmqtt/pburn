use crate::definitions::{write, Visitor};
use serde::{Deserialize, Serialize};
use std::fs::File;
use bollard::service::Config;

#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionType {
    #[allow(non_camel_case_types)]
    mqtt,
    #[allow(non_camel_case_types)]
    rest,
    #[allow(non_camel_case_types)]
    tcp,
}
impl ConnectionType{
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_connection_type_def(self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MqttMessage {
    pub(crate) name: String,
    pub(crate) parameter: Vec<serde_yaml::Value>,
    pub(crate) topic: String,
    pub(crate) payload: String,
}
impl MqttMessage {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_mqtt_message_def(self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProtocolDataDescription {
    #[allow(non_camel_case_types)]
    mqtt_message(MqttMessage),
}

impl ProtocolDataDescription {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        match self {
            ProtocolDataDescription::mqtt_message(value) => {
                value.accept(visitor);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
    pub(crate) connection_type: ConnectionType,
    pub(crate) protocol_data_description: Vec<ProtocolDataDescription>,
}

impl Definition {
    pub fn generate_connection_type(self: &Self, file: &mut File) {
        match self.connection_type {
            ConnectionType::mqtt => {
                write(
                    file,
                    "The used connection type is MQTT \n\n  \n".to_string(),
                );
            }
            ConnectionType::rest => {
                write(file, "The used connection type is REST \n".to_string());
            }
            ConnectionType::tcp => {
                write(file, "The uses connection type is TCP \n".to_string());
            }
        }
    }

    pub fn generate_protocol_definition(self: &Self, file: &mut File) {
        for x in &self.protocol_data_description {
            match x {
                ProtocolDataDescription::mqtt_message(value) => {
                    write(file, format!("### MQTT-Message {}\n\n", value.name));
                    write(file, "**Parameters:**\n\n".to_string());
                    for parameter_items in &value.parameter {
                        if let Some(parameters) = parameter_items.as_mapping() {
                            for (key, value) in parameters {
                                let Some(key_str) = key.as_str() else { todo!() };
                                let Some(value_str) = value.as_str() else {
                                    todo!()
                                };
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

    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_defination(self);
        self.connection_type.accept(visitor);
    }

}


#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceDataDefinition {
    pub(crate) data_def: Vec::<Definition>
}
impl InterfaceDataDefinition {
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_interface_data_def(self);
        for def in &mut self.data_def {
            def.accept(visitor);
        }
    }
}
