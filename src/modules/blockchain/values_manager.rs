extern crate mongodb;
extern crate iron;

use std::collections::HashMap;
use iron::status;
use rustc_serialize::json;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use mongodb::{Bson, bson, doc};

use crate::modules::blockchain::blockchain_types::Value;
use crate::modules::blockchain::blockchain_types::DocumentFind;
use crate::modules::blockchain::blockchain_types;
use crate::modules::blockchain::blockchain;
use crate::modules::blockchain::encryption;
use crate::connection_data::*;

pub use crate::macros;
pub use crate::messages;
pub use crate::http_codes;

pub fn insert_new_document_bulk(object_vec: &Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {

    let port_mongodb: u16 = to_u16!(&**MONGODB_PORT);

    let client: Client = Client::connect(&**MONGODB_HOST, port_mongodb).ok().expect("Failed to connect mongodb");

    let db = client.db(&**MONGODB_DATABASE);
    db.auth(&**MONGODB_USER, &**MONGODB_PASSWORD).ok().expect("Failed to authorize user");

    let mut array_result: Vec<HashMap<String, String>> = Vec::new();

    for map in object_vec {

        let mut result: Vec<HashMap<String, String>> = insert_new_document(&map);

        if result.len() > 0 {

            array_result.append(&mut result);
        }
    }

    return array_result;
}

pub fn insert_new_document(object_json: &HashMap<String, String>) -> Vec<HashMap<String, String>> {

    let port_mongodb: u16 = to_u16!(&**MONGODB_PORT);

    let client: Client = Client::connect(&**MONGODB_HOST, port_mongodb).ok().expect("Failed to connect mongodb");

    let db = client.db(&**MONGODB_DATABASE);
    db.auth(&**MONGODB_USER, &**MONGODB_PASSWORD).ok().expect("Failed to authorize user");

    let collection: Collection = db.collection(&**MONGODB_COLLECTION);

    let pre_hash: String = blockchain::get_pre_hash(&collection);
    let mut object_chain: HashMap<String, Value> = blockchain::new_block(&pre_hash, &object_json, &collection);

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
                    "hash" => hash_value.clone(),
                    "merkle_root" => merkle_root_value,
                    "timestamp" => timestamp_value };

    let doc_clone = doc.clone();

    let result_insert = collection.insert_one(doc_clone, None);

    let mut array_id_hash = Vec::new();

    if result_insert.is_ok() {

        // println!("Insert result OK");

        let id_block: String = blockchain::get_id_from_hash(&hash_value, &collection);
        let verified: bool = blockchain::verify_block(&id_block, &collection);

        let mut block_verify: HashMap<String, String> = HashMap::new();

        block_verify.insert(to_string!("block_id"), id_block);

        let mut verified_status = "false";

        if verified {
            verified_status = "true";
        }

        block_verify.insert(to_string!("verified"), to_string!(verified_status));

        array_id_hash.push(block_verify);

        return array_id_hash;
    }

    return Vec::new();
}

#[allow(unused_variables)]
pub fn find_documents(rows: &String, date_from: &String, date_to: &String) -> (HashMap<String, String>, status::Status){

    return (HashMap::new(), status::Ok);
}

pub fn search_block_with_id(block_id: &String, encryption: bool) -> (String, status::Status) {

    let port_mongodb: u16 = to_u16!(&**MONGODB_PORT);

    let client = Client::connect(&**MONGODB_HOST, port_mongodb).ok().expect("Failed to connect mongodb");

    let db = client.db(&**MONGODB_DATABASE);
    db.auth(&**MONGODB_USER, &**MONGODB_PASSWORD).ok().expect("Failed to authorize user");

    let collection = db.collection(&**MONGODB_COLLECTION);

    let block: Vec<_> = blockchain::get_block_from_id(block_id, &collection);

    if block.len() > 0 {

        let final_response: String;

        let mut object_result: HashMap<String, String> = HashMap::new();

        let verified: bool = blockchain::verify_block(block_id, &collection);

        let mut is_verified: String = to_string!("false");

        if verified {

            is_verified = to_string!("true");
        }

        object_result.insert(to_string!("verified"), is_verified.clone());
        object_result.insert(to_string!("block_id"), block_id.to_string());

        let single_doc = block[0].clone();
        let data_bson: &Bson = single_doc.get("data").unwrap();
        let data_str = data_bson.as_str().unwrap().to_string();

        if encryption == false {

            let decrypt_data: HashMap<String, String> = encryption::decrypt_operation_object(&data_str);

            let object_doc = DocumentFind {

                data: decrypt_data,
                block_id: block_id.to_string(),
                verified: is_verified.clone()
            };

            final_response = json::encode(&object_doc).expect("Error encoding response");

        } else {

            object_result.insert(to_string!("data"), data_str);

            final_response = json::encode(&object_result).expect("Error encoding response");
        }

        return (final_response, status::Ok);
    }

    let mut error_object: HashMap<String, String> = HashMap::new();

    error_object.insert(to_string!("code"), http_codes::HTTP_GENERIC_ERROR.to_string());
    error_object.insert(to_string!("message"), messages::DATA_NOT_FOUND.to_string());

    let str_error: String = json::encode(&error_object).expect("Error encoding response");

    return (str_error, status::InternalServerError);
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
