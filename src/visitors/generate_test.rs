use crate::definitions::test_definition::{IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition, TestDefinition};
use crate::definitions::Visitor;

struct GenerateTestVisitor{}

impl Visitor for GenerateTestVisitor{

    #[allow(unused_variables)]
    fn visit_test_def(&mut self, def: &mut TestDefinition) {
        
    }
    #[allow(unused_variables)]
    fn visit_run_def(&mut self, def: &mut RunDefinition) {
        
    }

    fn visit_send_mqtt_def(&mut self, def: &mut SendMqttDefinition) {
        
    }
    #[allow(unused_variables)]
    fn visit_recv_mqtt_dev(&mut self, def: &mut RecvMqttDefinition) {
        
    }
    #[allow(unused_variables)]
    fn visit_is_equal_def(&mut self, def: &mut IsEqualDefintion) {
        
    }
    #[allow(unused_variables)]
    fn visit_regex_def(&mut self, def: &mut RegexDefinition) {
        
    }

}