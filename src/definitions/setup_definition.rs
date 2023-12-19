use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use crate::definitions::interface_data_definition::ConnectionType;
use crate::definitions::{Visitor, write};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDockerMongoDbCmd {
    pub(crate) host: String,
    pub(crate) port: String,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) database: String,
    pub(crate) docker_host: Option<String>,
    pub(crate) docker_port: Option<String>,
    pub(crate) docker_id: Option<String>,
    pub(crate) name: String,
}

impl CreateDockerMongoDbCmd{
    pub fn accept<V: Visitor>(&mut self,visitor: &mut V){
        visitor.visit_create_docker_mongo(self);
    }
}


//serde_yaml::Value
#[derive(Debug, Serialize, Deserialize)]
pub struct DataEntry {
    pub(crate) data_entry: serde_yaml::Value
}

impl DataEntry{
    pub fn accept<V: Visitor>(&mut self,visitor: &mut V){
        visitor.visit_data_entry(self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitMongoDbCmd {
    pub(crate) name: String,
    pub(crate) database: String,
    pub(crate) collection: String,
    pub(crate) data: Vec<DataEntry>
}

impl InitMongoDbCmd{
    pub fn accept<V: Visitor>(&mut self,visitor: &mut V){
        visitor.visit_init_mongo( self);
        for entry in &mut self.data{
            entry.accept(visitor);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionCmd {
    pub(crate) name: String,
    pub(crate)connection_type: ConnectionType,
    pub(crate) host: String,
    pub(crate) port: String
}
impl ConnectionCmd{
    pub fn accept<V: Visitor>(&mut self,visitor: &mut V) {
        visitor.visit_connection_def(self);
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetupCommand{
    create_docker_mongodb(CreateDockerMongoDbCmd),
    init_mongodb(InitMongoDbCmd),
    connection(ConnectionCmd)
}

impl SetupCommand{
    pub fn accept<V: Visitor>(&mut self,visitor: &mut V){
        match self{
            SetupCommand::create_docker_mongodb(value) => {
                value.accept(visitor);
            }
            SetupCommand::init_mongodb(value) => {
                value.accept(visitor);
            }
            SetupCommand::connection(value) => {
                value.accept(visitor);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup {
    pub(crate) entries: Vec<SetupCommand>
}

impl Setup{
    pub fn accept<V: Visitor>(&mut self,visitor: &mut V){
        visitor.visit_setup_def(self);
        for entry in &mut self.entries{
            entry.accept(visitor);
        }
    }
}