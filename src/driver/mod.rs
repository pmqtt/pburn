use mongodb::{options::ClientOptions, Client, Collection};
use serde_json::Value;
use tokio::runtime::Runtime;

pub struct MongoDb {
    #[allow(dead_code)]
    client: mongodb::Client,
    database: mongodb::Database,
}
pub fn create_mongo_database_if_not_exists(
    uri: &str,
    db_name: &str,
) -> Result<MongoDb, mongodb::error::Error> {
    let client_options = ClientOptions::parse(uri);
    match client_options {
        Ok(c_opt) => {
            let client = Client::with_options(c_opt);
            match client {
                Ok(c) => {
                    let database = c.database(db_name);
                    Ok(MongoDb {
                        client: c,
                        database: database,
                    })
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn insert_json_into_collection(
    db: &MongoDb,
    collection_name: &str,
    document: &serde_json::Value,
) -> Result<(), mongodb::error::Error> {
    let collection: Collection<Value> = db.database.collection(collection_name);
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match collection.insert_one(document, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    })
}
