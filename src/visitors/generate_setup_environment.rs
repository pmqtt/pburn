use std::collections::HashMap;

use crate::definitions::interface_data_definition::{ConnectionType, Definition, InterfaceDataDefinition, MqttMessage, ProtocolDataDescription};
use crate::definitions::program_configuration::Config;
use crate::definitions::setup_definition::{
    ConnectionCmd, CreateDockerMongoDbCmd, DataEntry, InitMongoDbCmd, Setup, SetupCommand,
};
use crate::definitions::test_definition::{
    IsEqualDefintion, RecvMqttDefinition, RegexDefinition, RunDefinition, SendMqttDefinition,
    TestDefinition,
};
use crate::definitions::Visitor;
use crate::docker::{create_mongo_db_container, create_mosquitto_docker, docker_rm_container};
use crate::driver::{create_mongo_database_if_not_exists, insert_json_into_collection, MongoDb};
use core::option::Option;

fn yaml_to_json(yaml_value: &serde_yaml::Value) -> Result<serde_json::Value, serde_json::Error> {
    let json_string = serde_json::to_string(&yaml_value)?;
    let json_value = serde_json::from_str(&json_string)?;
    Ok(json_value)
}

struct MongoDbData {
    #[allow(dead_code)]
    container_id: String,
    data: MongoDb,
    collection: Option<String>,
}

struct PostgresData {}
enum DbData {
    #[allow(non_camel_case_types)]
    mongo(MongoDbData),
    #[allow(non_camel_case_types, dead_code)]
    postgres(PostgresData),
    #[allow(non_camel_case_types)]
    none,
}
pub struct GenerateSetupEnvironmentVisitor {
   pub(crate) container_id_to_name: HashMap<String, String>,
   pub(crate) current_db_data: DbData,
}
impl GenerateSetupEnvironmentVisitor {
    pub fn new() -> GenerateSetupEnvironmentVisitor {
        GenerateSetupEnvironmentVisitor {
            container_id_to_name: HashMap::new(),
            current_db_data: DbData::none,
        }
    }
}

impl Drop for GenerateSetupEnvironmentVisitor{
    fn drop(&mut self) {
        for (name,id) in &self.container_id_to_name{
            print!("Remove container {} with id: {}",name,id);
            let res = docker_rm_container(id.as_str());
            match res {
                Ok(_) => {
                    print!("... OK!\n");
                }
                Err(e) => {
                    print!("... Failed:{:?}\n",e);
                }
            }
        }
    }
}

impl Visitor for GenerateSetupEnvironmentVisitor {
    fn visit_create_docker_mongo(&mut self, def: &mut CreateDockerMongoDbCmd) {
        let ret_id = create_mongo_db_container(
            def.name.as_str(),
            "mongo",
            def.user.as_str(),
            def.password.as_str(),
            def.port.to_string().as_str(),
        );
        match ret_id {
            Ok(value) => match &def.docker_id {
                None => {
                    def.docker_id = Some(value);
                    self.container_id_to_name
                        .insert(def.name.clone(), def.docker_id.as_ref().unwrap().clone());
                    let connection_str = format!(
                        "mongodb://{}:{}@{}:{}",
                        def.user, def.password, def.host, def.port
                    );
                    println!("Connection String:{}", connection_str);

                    let mongo_db = create_mongo_database_if_not_exists(
                        connection_str.as_str(),
                        def.database.as_str(),
                    );
                    match mongo_db {
                        Ok(db) => {
                            self.current_db_data = DbData::mongo(MongoDbData {
                                container_id: def.docker_id.clone().unwrap(),
                                data: db,
                                collection: None,
                            });
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                }
                Some(id) => {
                    println!("Docker exists with id: {}", id);
                }
            },
            Err(_) => {}
        }
    }

    fn visit_init_mongo(&mut self, def: &mut InitMongoDbCmd) {
        println!("init mongo");
        match self.container_id_to_name.get(&def.name) {
            None => {
                println!("NONE");
            }
            Some(_) => match &mut self.current_db_data {
                DbData::mongo(ref mut db_data) => {
                    db_data.collection = Some(def.collection.clone());
                }
                DbData::postgres(_) => {}
                DbData::none => {}
            },
        }
    }

    fn visit_data_entry(&mut self, def: &mut DataEntry) {
        println!("data entry");
        match &self.current_db_data {
            DbData::mongo(data) => {
                let json = yaml_to_json(&def.data_entry);
                match json {
                    Ok(value) => {
                        match insert_json_into_collection(
                            &data.data,
                            &data.collection.as_ref().unwrap().as_str(),
                            &value,
                        ) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            DbData::postgres(_) => {}
            DbData::none => {
                println!("none");
            }
        }
    }
    fn visit_connection_def(&mut self, def: &mut ConnectionCmd) {
        let res = create_mosquitto_docker(def.name.as_str(),"eclipse-mosquitto",def.port.as_str());
        match res{
            Ok(id) => {
                self.container_id_to_name.insert(def.name.clone(),id);
            }
            Err(e) => { println!("{:?}",e);}
        }
    }

}
