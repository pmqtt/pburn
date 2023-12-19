use std::collections::HashMap;

use core::option::Option;
use crate::definitions::interface_data_definition::{ConnectionType, InterfaceDataDefinition, MqttMessage, ProtocolDataDescription};
use crate::definitions::program_configuration::Config;
use crate::definitions::setup_definition::{ConnectionCmd, CreateDockerMongoDbCmd, DataEntry, InitMongoDbCmd, Setup, SetupCommand};
use crate::definitions::test_definition::{IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition, TestDefinition};
use crate::definitions::Visitor;
use crate::docker::{ create_mongo_db_container};
use crate::driver::{create_mongo_database_if_not_exists, insert_json_into_collection, MongoDb};
use crate::visitors::generate_setup_environment::DbData::{mongo, none};


fn yaml_to_json(yaml_value:& serde_yaml::Value) -> Result<serde_json::Value, serde_json::Error> {
    let json_string = serde_json::to_string(&yaml_value)?;
    let json_value = serde_json::from_str(&json_string)?;
    Ok(json_value)
}

struct MongoDbData{
    container_id: String,
    data: MongoDb,
    collection: Option<String>

}

struct PostgresData{

}
enum DbData{
    mongo(MongoDbData),
    postgres(PostgresData),
    none
}
pub struct GenerateSetupEnvironmentVisitor{
    container_id_to_name: HashMap<String,String>,
    current_db_data: DbData
}
impl GenerateSetupEnvironmentVisitor{
    pub fn new()->GenerateSetupEnvironmentVisitor{
        GenerateSetupEnvironmentVisitor{
            container_id_to_name: HashMap::new(),
            current_db_data: DbData::none
        }
    }
}
impl Visitor for GenerateSetupEnvironmentVisitor{
    fn visit_create_docker_mongo(&mut self, def: &mut CreateDockerMongoDbCmd) {
        let ret_id = create_mongo_db_container(def.name.as_str(),"mongo",def.user.as_str(),def.password.as_str(),def.port.to_string().as_str());
        match(ret_id){
            Ok(value) => {
                match(&def.docker_id){
                    None => {
                        def.docker_id = Some(value);
                        self.container_id_to_name.insert(def.name.clone(),def.docker_id.as_ref().unwrap().clone());
                        let connection_str = format!("mongodb://{}:{}@{}:{}",def.user,def.password,def.host,def.port);
                        println!("Connection String:{}",connection_str);

                        let mongo_db = create_mongo_database_if_not_exists(connection_str.as_str(),
                                                                           def.database.as_str());
                        match mongo_db {
                            Ok(db) => {
                                self.current_db_data = DbData::mongo(MongoDbData{
                                    container_id: def.docker_id.clone().unwrap(),
                                    data: db,
                                    collection: None
                                });
                            }
                            Err(e) => {
                                println!("{:?}",e);
                            }
                        }
                    }
                    Some(id) => {
                        println!("Docker exists");
                    }
                }
            }
            Err(_) => {}
        }
    }

    fn visit_init_mongo(&mut self, def: &mut InitMongoDbCmd) {
        println!("init mongo");
        match self.container_id_to_name.get(&def.name) {
            None => {
                println!("NONE");
            }
            Some(id) => {
                match &mut self.current_db_data {
                    DbData::mongo(ref mut db_data) => {
                        db_data.collection = Some(def.collection.clone());
                    }
                    DbData::postgres(_) => {}
                    DbData::none => {}
                }
            }
        }
    }

    fn visit_data_entry(&mut self, def: &mut DataEntry) {
        println!("data entry");
        match &self.current_db_data {
            DbData::mongo(data) => {
                let json = yaml_to_json(&def.data_entry);
                match json{
                    Ok(value) => {
                        let insertedStr = format!("[{}]", value);
                        insert_json_into_collection(&data.data,&data.collection.as_ref().unwrap().as_str(),&value);
                    }
                    Err(_) => {}
                }
            }
            DbData::postgres(_) => {

            }
            DbData::none => {
                println!("none");
            }
        }
    }


    fn visit_config(&mut self, config: &mut Config) {

    }

    fn visit_test_def(&mut self, def: &mut TestDefinition) {
    }

    fn visit_run_def(&mut self, def: &mut RunDefinition) {
    }

    fn visit_send_mqtt_def(&mut self, def: &mut SendMqttDefinition) {
    }

    fn visit_recv_mqtt_dev(&mut self, def: &mut RecvMqttDefinition) {
    }

    fn visit_is_equal_def(&mut self, def: &mut IsEqualDefintion) {
    }

    fn visit_regex_def(&mut self, def: &mut RegexDefinition) {
    }

     fn visit_connection_def(&mut self, def: &mut ConnectionCmd) {
    }

    fn visit_setup_command_def(&mut self, def: &mut SetupCommand) {
    }

    fn visit_setup_def(&mut self, def: &mut Setup) {
    }

    fn visit_interface_data_def(&mut self, def: &mut InterfaceDataDefinition) {
    }

    fn visit_protocol_data_description_def(&mut self, def: &mut ProtocolDataDescription) {
    }

    fn visit_mqtt_message_def(&mut self, def: &mut MqttMessage) {
    }

    fn visit_connection_type_def(&mut self, def: &mut ConnectionType) {
    }
}