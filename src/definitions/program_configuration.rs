use crate::definitions::interface_data_definition::InterfaceDataDefinition;
use crate::definitions::setup_definition::Setup;
use crate::definitions::test_definition::TestDefinition;
use crate::definitions::Visitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) name: String,
    pub(crate) description: String,
    interface_data_definition: InterfaceDataDefinition,
    setup: Setup,
    test: TestDefinition,
}

impl Config {
    pub fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        visitor.visit_config(self);
        self.interface_data_definition.accept(visitor);
        self.setup.accept(visitor);
    }
}
