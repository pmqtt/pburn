use crate::definitions::Visitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegexDefinition {
    left: String,
    right: String,
    allow_failure: bool,
}
impl RegexDefinition {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_regex_def(self);
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IsEqualDefintion {
    left: String,
    right: serde_yaml::Value,
    allow_failure: bool,
}
impl IsEqualDefintion {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_is_equal_def(self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationDefinition {
    #[allow(non_camel_case_types)]
    is_eq(IsEqualDefintion),
    #[allow(non_camel_case_types)]
    regex(RegexDefinition),
}
impl VerificationDefinition {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        match self {
            VerificationDefinition::is_eq(value) => {
                value.accept(visitor);
            }
            VerificationDefinition::regex(value) => {
                value.accept(visitor);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecvMqttDefinition {
    message: String,
    verify: Vec<VerificationDefinition>,
}
impl RecvMqttDefinition {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_recv_mqtt_dev(self);
        for v in &mut self.verify {
            v.accept(visitor);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMqttDefinition {
    message: String,
    parameters: Vec<String>,
}
impl SendMqttDefinition {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_send_mqtt_def(self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RunDefinition {
    #[allow(non_camel_case_types)]
    send_mqtt(SendMqttDefinition),
    #[allow(non_camel_case_types)]
    recv_mqtt(RecvMqttDefinition),
}
impl RunDefinition {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        match self {
            RunDefinition::send_mqtt(value) => {
                value.accept(visitor);
            }
            RunDefinition::recv_mqtt(value) => {
                value.accept(visitor);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestDefinition {
    run: Vec<RunDefinition>,
}
impl TestDefinition {
    #[allow(dead_code)]
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_test_def(self);
        for r in &mut self.run {
            r.accept(visitor);
        }
    }
}
