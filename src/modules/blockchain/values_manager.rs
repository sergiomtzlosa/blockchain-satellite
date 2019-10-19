extern crate mongodb;
extern crate iron;

use std::collections::HashMap;
use iron::status;
use crate::modules::blockchain::blockchain_types::Value;
use crate::modules::blockchain::blockchain_types;
use crate::modules::blockchain::blockchain;
// use crate::modules::blockchain::encryption;
use crate::connection_data::*;
//use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use mongodb::{Bson, bson, doc};

pub use crate::macros;

#[allow(unused_variables)]
pub fn insert_new_document_bulk(object_json: &Vec<HashMap<String, String>>, token: &String)  -> (HashMap<String, String>, status::Status) {

    // let temp: HashMap<String, String> = deserialized_vec.into_iter().nth(0).unwrap();
    // let value_temp: String = temp.get("key1").unwrap().to_string();
    // println!("{}", value_temp);

    return (HashMap::new(), status::Ok);
}

pub fn insert_new_document(object_json: &HashMap<String, String>)  -> (HashMap<String, String>, status::Status) {

    let port_mongodb: u16 = to_u16!(&**MONGODB_PORT);

    let client: Client = Client::connect(&**MONGODB_HOST, port_mongodb).ok().expect("Failed to connect mongodb");

    let db = client.db(&**MONGODB_DATABASE);
    db.auth(&**MONGODB_USER, &**MONGODB_PASSWORD).ok().expect("Failed to authorize user");

    let collection: Collection = db.collection(&**MONGODB_COLLECTION);

    let pre_hash: String = blockchain::get_pre_hash(&collection);
    let mut object_chain: HashMap<String, Value> = blockchain::new_block(&pre_hash, &object_json);

    object_chain.remove("pre_hash");

    let data: &Value = object_chain.get("data").unwrap();
    let datetime: &Value = object_chain.get("datetime").unwrap();
    let high: &Value = object_chain.get("high").unwrap();
    let nonce: &Value = object_chain.get("nonce").unwrap();
    let hash: &Value = object_chain.get("hash").unwrap();
    let merkle_root: &Value = object_chain.get("merkle_root").unwrap();
    let timestamp: &Value = object_chain.get("timestamp").unwrap();

    let data_value: String = blockchain_types::get_string(data);
    let datetime_value: String = blockchain_types::get_string(datetime);
    let high_value: String = blockchain_types::get_string(high);
    let nonce_value: String = blockchain_types::get_string(nonce);
    let hash_value: String = blockchain_types::get_string(hash);
    let merkle_root_value: String = blockchain_types::get_string(merkle_root);
    let timestamp_value: Bson = blockchain_types::get_bson(timestamp);

    let doc = doc!{ "data" => data_value,
                     "datetime" => datetime_value,
                     "high" => high_value,
                     "nonce" => nonce_value,
                     "hash" => hash_value,
                     "merkle_root" => merkle_root_value,
                     "timestamp" => timestamp_value };

    let result_insert = collection.insert_one(doc.clone(), None);

    if result_insert.is_ok() {

        println!("Insert result OK");
    } else {
        println!("Insert result NOK");
    }

    return (HashMap::new(), status::Ok);
}

#[allow(unused_variables)]
pub fn find_documents(row: &String, date_from: &String, date_to: &String, token: &String) -> (HashMap<String, String>, status::Status){

    return (HashMap::new(), status::Ok);
}

#[allow(unused_variables)]
pub fn search_block_with_id(block_id: &String, encryption: bool) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}

pub fn drop_blockchain(collection_name: &String) -> bool {

    let port_mongodb: u16 = to_u16!(&**MONGODB_PORT);

    let client = Client::connect(&**MONGODB_HOST, port_mongodb).ok().expect("Failed to connect mongodb");

    let db = client.db(&**MONGODB_DATABASE);
    db.auth(&**MONGODB_USER, &**MONGODB_PASSWORD).ok().expect("Failed to authorize user");

    let collection = db.collection(collection_name);

    let result: bool = match collection.drop() {

        Ok(_) => {

            ((true))
        },
        Err(_) => {

            ((false))
        }
    };

    return result;
}
