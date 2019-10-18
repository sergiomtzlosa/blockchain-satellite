extern crate mongodb;
extern crate iron;

use std::collections::HashMap;
use iron::status;
//use crate::modules::blockchain::blockchain;
// use blockchain_rust::typeinfo::TypeInfo;
use crate::connection_data::*;
//use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub use crate::macros;

#[allow(unused_variables)]
pub fn insert_new_document(data: &HashMap<String, String>, token: &String)  -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}

#[allow(unused_variables)]
pub fn find_documents(row: &String, date_from: &String, date_to: &String, token: &String) -> (HashMap<String, String>, status::Status){

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

#[allow(unused_variables)]
pub fn search_block_with_id(block_id: &String, encryption: bool) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}
