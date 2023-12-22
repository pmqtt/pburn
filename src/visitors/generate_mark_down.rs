use crate::definitions::interface_data_definition::{ConnectionType, Definition, InterfaceDataDefinition, MqttMessage, ProtocolDataDescription};
use crate::definitions::program_configuration::Config;
use crate::definitions::setup_definition::{
    ConnectionCmd, CreateDockerMongoDbCmd, DataEntry, InitMongoDbCmd, Setup, SetupCommand,
};
use crate::definitions::write;
use serde_yaml::Value;
use std::fs;

pub struct GenerateMarkDownVisitor {
    pub file: fs::File,
}
impl GenerateMarkDownVisitor {
    pub fn from_filename(filename: String) -> GenerateMarkDownVisitor {
        GenerateMarkDownVisitor {
            file: fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)
                .unwrap(),
        }
    }
}

impl crate::definitions::Visitor for GenerateMarkDownVisitor {
    fn visit_config(&mut self, config: &mut Config) {
        let title: &String = &config.name;
        let desc: &String = &config.description;
        write(&mut self.file, format!("# {}\n", title));
        write(&mut self.file, format!("{}\n\n---\n\n", desc));
    }
    fn visit_create_docker_mongo(&mut self, def: &mut CreateDockerMongoDbCmd) {
        write(
            &mut self.file,
            "### Create Mongo Database \n\n --- \n".to_string(),
        );
        let tpl = format!(
            "- `host`: {}\n- `port`: {}\n- `user`: {}\n- `password`: {}\n- `database`: {}\n",
            def.host, def.port, def.user, def.password, def.database
        );
        write(&mut self.file, tpl);
    }

    fn visit_data_entry(&mut self, def: &mut DataEntry) {
        write(&mut self.file, "\n**Data**\n\n".to_string());
        if let Some(entry) = def.data_entry.as_mapping() {
            for (key, value) in entry {
                let Some(key_str) = key.as_str() else { todo!() };
                let mut value_str: String = "".to_string();
                match value {
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
                write(&mut self.file, format!("- `{}`: {}\n", key_str, value_str))
            }
        }
    }

    fn visit_init_mongo(&mut self, def: &mut InitMongoDbCmd) {
        write(
            &mut self.file,
            format!("### Create MONGO Collection {} \n\n ---\n", def.collection),
        );
    }

    fn visit_connection_def(&mut self, def: &mut ConnectionCmd) {
        write(
            &mut self.file,
            "### Create Connection Server  \n\n --- \n".to_string(),
        );
        write(&mut self.file, format!("- `name`: {}\n", def.name));
        write(&mut self.file, format!("- `connection type`: {}\n", "mqtt"));
        write(&mut self.file, format!("- `host`: {}\n", def.host));
        write(&mut self.file, format!("- `port`: {}\n", def.port));
    }

    #[allow(unused_variables)]
    fn visit_setup_command_def(&mut self, def: &mut SetupCommand) {}
    #[allow(unused_variables)]
    fn visit_setup_def(&mut self, def: &mut Setup) {
        write(
            &mut self.file,
            "## Setup Environment Description \n\n --- \n".to_string(),
        );
    }


    fn visit_defination(&mut self, def: &mut Definition) {
        write(
            &mut self.file,
            "## IDD - Interface Data Definition \n\n --- \n".to_string(),
        );
        def.generate_connection_type(&mut self.file);
        def.generate_protocol_definition(&mut self.file);
        write(&mut self.file, "--- \n".to_string());
    }

    #[allow(unused_variables)]
    fn visit_interface_data_def(&mut self, def: &mut InterfaceDataDefinition) {
        //TODO if needed
    }


}
