use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use crate::definitions::interface_data_definition::{ Definition, InterfaceDataDefinition, MqttMessage, ProtocolDataDescription};
use crate::definitions::{Visitor, write};


#[derive(Clone)]
pub enum ParameterValue{
    Int(u32),
    Real(f64),
    Str(String),
    Bool(bool),
    None
}

impl fmt::Display for ParameterValue{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParameterValue::Int(v) => {
                return write!(f,"{}",v);
            }
            ParameterValue::Real(v) => {
                return write!(f,"{}",v);
            }
            ParameterValue::Str(v) => {
                return write!(f,"{}",v);
            }
            ParameterValue::Bool(v) => {
                return write!(f,"{}",v);
            }
            ParameterValue::None => {
               return  write!(f,"None");
            }
        }
    }
}

#[derive(Clone)]
pub enum ParameterType{
    Int,
    Real,
    Str,
    Bool,
    None
}

#[derive(Clone)]
pub struct Parameter{
    pub(crate) key: String,
    pub(crate) typ: ParameterType,
    pub(crate) value: ParameterValue
}
impl Parameter{
    pub fn new(key:String,typ: ParameterType, value: ParameterValue) -> Result<Parameter,String>{
        match typ{
            ParameterType::Int => {
                match &value{
                    ParameterValue::Int(x) => {}
                    _ =>{
                        return Err("Expected type int".to_string());
                    }
                }
            }
            ParameterType::Real => {
                match  &value{
                    ParameterValue::Real(x) => {}
                    _ =>{
                        return Err("Expected type real".to_string());
                    }
                }
            }
            ParameterType::Str => {
                match  &value{
                    ParameterValue::Str(x) => {}
                    _ =>{
                        return Err("Expected type str".to_string());
                    }
                }
            }
            ParameterType::Bool => {
                match  &value{
                    ParameterValue::Bool(x) => {}
                    _ =>{
                        return Err("Expected type bool".to_string());
                    }
                }
            }
            ParameterType::None => {
                return Err("Expected type".to_string());
            }
        }
        Ok(Parameter{
            key,
            typ,
            value,
        })
    }
}

#[derive(Clone)]
pub struct MqttProtocol{
    pub(crate) topic: String,
    pub(crate) payload: String,
    pub(crate) parameters: Vec<Parameter>
}

impl MqttProtocol{
    pub fn create_topic(&self,input: &Vec<Parameter>) -> Result<String,String>{
        if input.len() == self.parameters.len(){
            let mut topic = self.topic.clone();
            for i in 0..input.len() {
                // Todo check is input[i] == parameter[i]
                topic = topic.replace(format!("${}",self.parameters[i].key).as_str(),
                                      input[i].value.to_string().as_str());
            }

            return Ok(topic);
        }
        Err("parameter list and expected list size aren't equal".to_string())
    }
    pub fn create_payload(&self,input: &Vec<Parameter>) ->  Result<String,String>{
        if input.len() == self.parameters.len(){
            let mut payload = self.topic.clone();
            for i in 0..input.len() {
                // Todo check is input[i] == parameter[i]
                payload = payload.replace(format!("${}",self.parameters[i].key).as_str(),
                                      input[i].value.to_string().as_str());
            }

            return Ok(payload);
        }
        Err("parameter list and expected list size aren't equal".to_string())
    }
}


pub enum ProtocolType{
    Mqtt(MqttProtocol),
    None
}

pub struct GenerateCommunicationProtocol{
    pub(crate) protocol_type: HashMap<String,ProtocolType>
}

impl GenerateCommunicationProtocol{
    pub fn new() -> GenerateCommunicationProtocol{
        GenerateCommunicationProtocol{
            protocol_type: Default::default(),
        }
    }
}

impl Visitor for GenerateCommunicationProtocol{

    fn visit_interface_data_def(&mut self, def: &mut InterfaceDataDefinition) {
        self.protocol_type = HashMap::new();
    }

    fn visit_protocol_data_description_def(&mut self, def: &mut ProtocolDataDescription) {

    }

    fn visit_definition(&mut self, def: &mut Definition) {
    }

    fn visit_mqtt_message_def(&mut self, def: &mut MqttMessage) {
        let mut parameter_list :Vec<Parameter> = Vec::new();
        for x in &def.parameter{
            if let Some(parameters) = x.as_mapping() {
                for (key, value) in parameters {
                    let Some(key_str) = key.as_str() else { todo!() };
                    let Some(type_str) = value.as_str() else { todo!() };
                    let mut parameter_type : ParameterType = ParameterType::None;
                    match type_str{
                        "int" => {
                            parameter_type = ParameterType::Int;
                        }
                        "real" => {
                            parameter_type = ParameterType::Real;
                        }
                        "bool" => {
                            parameter_type = ParameterType::Bool;
                        }
                        "string" =>{
                            parameter_type = ParameterType::Str;
                        }
                        _ => {
                            parameter_type = ParameterType::None;
                        }
                    }

                    parameter_list.push(Parameter{
                        key: key_str.to_string(),
                        typ: parameter_type,
                        value: ParameterValue::None,
                    })
                }
            }
        }

        self.protocol_type.insert(def.name.clone(),ProtocolType::Mqtt(MqttProtocol{
            topic: def.topic.clone(),
            payload: def.payload.clone(),
            parameters: parameter_list,
        }));
    }

}