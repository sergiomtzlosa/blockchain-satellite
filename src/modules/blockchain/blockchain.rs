extern crate mongodb;

use std::collections::HashMap;
use mongodb::coll::Collection;
use mongodb::coll::options::FindOptions;
use mongodb::ordered::OrderedDocument;
use mongodb::{Bson, bson, doc};
use chrono::prelude::*;
use rustc_serialize::json;
use rand::Rng;
use ring::digest::{Algorithm, SHA256};
use crate::merkle::MerkleTree;
use crate::modules::blockchain::encryption;
use crate::modules::blockchain::blockchain_types::Value;
use crate::modules::blockchain::blockchain_types;

pub use crate::utils;

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA256;

pub fn get_pre_hash(collection: &Collection) -> String {

    let mut options: FindOptions = FindOptions::new();

    options.limit = Some(1);
    options.sort = Some(doc! { "_id" : -1 });

    let cursor = collection.find(None, Some(options)).ok().expect("Failed to execute find.");

    let docs : Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    println!("docs {}", docs.len());

    if docs.len() == 0 {

        return to_string!("");
    }

    let docs: OrderedDocument = docs[0].clone();

    let pre_hash: String = match docs.get_str("hash") {

        Ok(result) => result.to_string(),
        Err(_) => return to_string!("")
    };

    // println!("docs {}", pre_hash);

    return pre_hash;
}

pub fn new_block(pre_hash: &String, doc: &HashMap<String, String>) -> HashMap<String, Value> {

    let datetime: DateTime<Utc> = Utc::now();
    let result: String = format!("{:02}-{:02}-{:02} {:02}:{:02}:{:02}",  datetime.year(),  datetime.month(),  datetime.day(), datetime.hour(), datetime.minute(), datetime.second());

    let date: Bson = Bson::from(datetime);

    let mut block: HashMap<String, Value> = HashMap::new();

    let str_doc: String = json::encode(&doc).expect("Error encoding response");

    block.insert(to_string!("data"), Value::String(encryption::encrypt_operation(&str_doc)));
    block.insert(to_string!("datetime"), Value::String(result));
    block.insert(to_string!("high"), Value::String(get_chain_high()));
    block.insert(to_string!("pre_hash"), Value::String(to_string!(pre_hash)));
    block.insert(to_string!("nonce"), Value::String(new_nonce()));

    let hash_new: String = new_hash(&block);
    block.insert(to_string!("hash"), Value::String(to_string!(&hash_new)));

    let merkle_hash: &String = &get_merkle_hash();

    let mut final_hash: &String = merkle_hash;

    if final_hash == "" {

        final_hash = &hash_new;
    }

    block.insert(to_string!("merkle_root"), Value::String(to_string!(final_hash)));
    block.insert(to_string!("timestamp"), Value::Bson(date));

    // https://stackoverflow.com/questions/44930257/how-can-i-handle-a-hashmap-with-potentially-multiple-types-in-its-values-being

    return block;
}

fn new_hash(block: &HashMap<String, Value>) -> String {

    let data: &Value = block.get("data").unwrap();
    let datetime: &Value = block.get("datetime").unwrap();
    let high: &Value = block.get("high").unwrap();
    let pre_hash: &Value = block.get("pre_hash").unwrap();
    let nonce: &Value = block.get("nonce").unwrap();

    let data_value: String = blockchain_types::get_string(data);
    let datetime_value: String = blockchain_types::get_string(datetime);
    let high_value: String = blockchain_types::get_string(high);
    let pre_hash_value: String = blockchain_types::get_string(pre_hash);
    let nonce_value: String = blockchain_types::get_string(nonce);

    let mut new_block: HashMap<String, String> = HashMap::new();

    new_block.insert(to_string!("data"), data_value);
    new_block.insert(to_string!("datetime"), datetime_value);
    new_block.insert(to_string!("high"), high_value);
    new_block.insert(to_string!("pre_hash"), pre_hash_value);
    new_block.insert(to_string!("nonce"), nonce_value);

    let json_data: String = json::encode(&new_block).expect("Error encoding response");

    let hash: String = utils::sha256_encode(&json_data);

    return hash;
}

fn new_nonce() -> String {

    let bytes = rand::thread_rng().gen::<[u8; 32]>();

    let nonce = hex::encode(&bytes);

    let final_nonce: String = format!("{:?}", nonce);

    return final_nonce;
}

fn get_merkle_hash() -> String {

    let values = vec!["one", "two", "three", "four"];

    let tree = MerkleTree::from_vec(digest, values);

    let root_hash = tree.root_hash();
    let final_root_hash = encryption::base64_encode(&root_hash);

    return final_root_hash;
}

fn get_all_ids() -> Vec<String> {

    return Vec::new();
}

fn get_chain_high() -> String {

    return to_string!("");
}
