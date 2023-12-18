use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use crate::definitions::interface_data_definition::ConnectionType;
use crate::definitions::write;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDockerMongoDbCmd {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
    name: String,
}

impl CreateDockerMongoDbCmd{
    pub fn generate_mark_down(self: &Self,file: &mut fs::File) {
        write(file, "### Create Mongo Database \n\n --- \n".to_string());
        let tpl = format!("- `host`: {}\n- `port`: {}\n- `user`: {}\n- `password`: {}\n- `database`: {}\n",self.host,self.port,self.user,self.password,self.database);
        write(file,tpl);
    }
}


//serde_yaml::Value
#[derive(Debug, Serialize, Deserialize)]
pub struct DataEntry {
    pub(crate) data_entry: serde_yaml::Value
}

impl DataEntry{
    pub fn generate_mark_down(self: &Self,file: &mut fs::File) {
        if let Some(entry) = self.data_entry.as_mapping(){
            for (key,value) in entry{

                let Some(key_str) = key.as_str() else { todo!() };
                let mut value_str:String="".to_string();
                match(value){
                    Value::Null => {}
                    Value::Bool(_) => {}
                    Value::Number(x) => {
                        if let Some(a) = x.as_f64() {
                            value_str = a.to_string();
                        }
                    }
                    Value::String(x) => {
                        value_str = x.to_string();
                    }
                    Value::Sequence(_) => {}
                    Value::Mapping(_) => {}
                }
                write(file,format!("- `{}`: {}\n",key_str,value_str))

            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitMongoDbCmd {
    name: String,
    collection: String,
    pub(crate) data: Vec<DataEntry>
}

impl InitMongoDbCmd{
    pub fn generate_mark_down(self: &Self,file: &mut fs::File) {
        write(file,format!("### Create MONGO Collection {} \n\n ---\n",self.collection));
        for entry in &self.data{
            write(file,"\n**Data**\n\n".to_string());
            entry.generate_mark_down(file);
        }

    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionCmd {
    name: String,
    connection_type: ConnectionType,
    host: String,
    port: String
}
impl ConnectionCmd{
    pub fn generate_mark_down(self: &Self,file: &mut fs::File) {
        write(file, "### Create Connection Server  \n\n --- \n".to_string());
        write(file,format!("- `name`: {}\n",self.name));
        write(file,format!("- `connection type`: {}\n","mqtt"));
        write(file,format!("- `host`: {}\n",self.host));
        write(file,format!("- `port`: {}\n",self.port));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetupCommand{
    create_docker_mongodb(CreateDockerMongoDbCmd),
    init_mongodb(InitMongoDbCmd),
    connection(ConnectionCmd)
}

impl SetupCommand{
    pub fn generate_mark_down(self: &Self,file: &mut fs::File) {
        match self{
            SetupCommand::create_docker_mongodb(value) => {
                value.generate_mark_down(file);
            }
            SetupCommand::init_mongodb(value) => {
                value.generate_mark_down(file);
            }
            SetupCommand::connection(value) => {
                value.generate_mark_down(file);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup {
    pub(crate) entries: Vec<SetupCommand>
}

impl Setup{
    pub fn generate_mark_down(self: &Self,file: &mut fs::File) {
        write(file, "## Setup Environment Description \n\n --- \n".to_string());
        for entry in &self.entries{
            entry.generate_mark_down(file);
        }
    }
}