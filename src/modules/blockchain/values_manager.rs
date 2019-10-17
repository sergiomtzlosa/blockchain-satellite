extern crate mongodb;
extern crate iron;

use std::collections::HashMap;
use iron::status;
use crate::modules::blockchain::blockchain;
//use crate::modules::blockchain::blockchain;
//use mongodb::{bson, doc};

pub use crate::macros;

pub fn insert_new_document(data: &HashMap<String, String>, token: &String)  -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}

pub fn find_documents(row: &String, date_from: &String, date_to: &String, token: &String) -> (HashMap<String, String>, status::Status){

    return (HashMap::new(), status::Ok);
}

pub fn drop_blockchain(collection_name: &String) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}

pub fn search_block_with_id(block_id: &String, encryption: bool) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}
