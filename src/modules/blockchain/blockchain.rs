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
use crate::modules::blockchain::blockchain_types::BlockChainBlock;
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

    // println!("docs {}", docs.len());

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

pub fn new_block(pre_hash: &String, doc: &HashMap<String, String>, collection: &Collection) -> HashMap<String, Value> {

    let datetime: DateTime<Utc> = Utc::now();
    let result: String = format!("{:02}-{:02}-{:02} {:02}:{:02}:{:02}",  datetime.year(),  datetime.month(),  datetime.day(), datetime.hour(), datetime.minute(), datetime.second());

    let date: Bson = Bson::from(datetime);

    let mut block: HashMap<String, Value> = HashMap::new();

    let str_doc: String = json::encode(&doc).expect("Error encoding response");

    block.insert(to_string!("data"), Value::String(encryption::encrypt_operation(&str_doc)));
    block.insert(to_string!("datetime"), Value::String(result));
    block.insert(to_string!("high"), Value::String(get_chain_high(collection)));
    block.insert(to_string!("pre_hash"), Value::String(to_string!(pre_hash)));
    block.insert(to_string!("nonce"), Value::String(new_nonce()));

    let hash_new: String = new_hash(&block);
    block.insert(to_string!("hash"), Value::String(to_string!(&hash_new)));

    let merkle_hash: &String = &get_merkle_hash(collection);

    let mut final_hash: &String = merkle_hash;

    if final_hash == "" {

        final_hash = &hash_new;
    }

    block.insert(to_string!("merkle_root"), Value::String(to_string!(final_hash)));
    block.insert(to_string!("timestamp"), Value::Bson(date));

    // https://stackoverflow.com/questions/44930257/how-can-i-handle-a-hashmap-with-potentially-multiple-types-in-its-values-being

    return block;
}

pub fn get_id_from_hash(hash: &String, collection: &Collection) -> String {

    let mut options: FindOptions = FindOptions::new();

    options.limit = Some(1);
    options.projection = Some(doc! { "_id" : true });

    let query = doc! { "hash" : hash };

    let cursor = collection.find(Some(query), Some(options)).ok().expect("Failed to execute find.");

    let docs_ids : Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    if docs_ids.len() == 0 {

        return to_string!("");
    }

    let doc: OrderedDocument = docs_ids[0].clone();

    let id_doc = doc.get_object_id("_id").unwrap();
    let mongo_id_hex = id_doc.to_hex();

    return mongo_id_hex;
}

pub fn get_block_from_id(id_block: &String, collection: &Collection) -> Vec<OrderedDocument> {

    let mut options: FindOptions = FindOptions::new();

    options.limit = Some(1);

    let obj_id = mongodb::oid::ObjectId::with_string(id_block).unwrap();
    let query = doc! { "_id" :  obj_id };

    let cursor = collection.find(Some(query), Some(options)).ok().expect("Failed to execute find.");

    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    if docs.len() > 0 {

        return docs;
    }

    return Vec::new();
}

pub fn verify_block(id_block: &String, collection: &Collection)  -> bool {

    let mut options: FindOptions = FindOptions::new();

    options.limit = Some(2);
    options.sort = Some(doc! { "_id" : -1 });

    let obj_id = mongodb::oid::ObjectId::with_string(id_block).unwrap();
    let query = doc! { "_id" : { "$lte" : obj_id } };

    let cursor = collection.find(Some(query), Some(options)).ok().expect("Failed to execute find.");

    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    // println!("recover objects verify_block: {}", docs.len());

    if docs.len() > 0 {

        let mut records_verify = docs[0].clone();

        let nonce: String = records_verify.get("nonce").unwrap().to_string().replace("\"", "").replace(" ", "").clone();
        let mut hash_now: String = records_verify.get("hash").unwrap().to_string().clone();
        hash_now = hash_now.replace("\"", "").replace(" ", "");

        records_verify.remove("pre_hash");

        if docs.len() == 2 {

            let other_doc = docs[1].clone();
            records_verify.insert(to_string!("pre_hash"), other_doc.get("hash").unwrap().to_string().clone());

        } else {

            records_verify.insert(to_string!("pre_hash"), to_string!(""));
        }

        records_verify.remove("nonce");
        records_verify.insert(to_string!("nonce"), nonce);

        let mut map_final: HashMap<String, Value> = HashMap::new();

        let data_str: &Bson = records_verify.get("data").unwrap();
        let datetime_str: &Bson = records_verify.get("datetime").unwrap();
        let high_str: &Bson = records_verify.get("high").unwrap();
        let pre_hash_str: &Bson = records_verify.get("pre_hash").unwrap();
        let nonce_str: &Bson = records_verify.get("nonce").unwrap();

        map_final.insert(to_string!("data"), Value::String(data_str.as_str().unwrap().to_string()));
        map_final.insert(to_string!("datetime"), Value::String(datetime_str.as_str().unwrap().to_string()));
        map_final.insert(to_string!("high"), Value::String(high_str.as_str().unwrap().to_string()));
        map_final.insert(to_string!("pre_hash"), Value::String(pre_hash_str.as_str().unwrap().to_string()));
        map_final.insert(to_string!("nonce"), Value::String(nonce_str.as_str().unwrap().to_string()));

        let hash_value = new_hash(&map_final);

        // println!("hash_value: {}", hash_value);
        // println!("hash_now: {}", hash_now);

        if hash_value == hash_now {

            return true;
        }
    }

    return false;
}

pub fn next_block_id(block_id: &String, collection: &Collection) -> String {

    let mut options: FindOptions = FindOptions::new();

    options.limit = Some(1);
    options.projection = Some(doc! { "_id" : true });

    let obj_id = mongodb::oid::ObjectId::with_string(block_id).unwrap();
    let query = doc! { "_id" : { "$gt" : obj_id } };

    let cursor = collection.find(Some(query), Some(options)).ok().expect("Failed to execute find.");

    let docs : Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    if docs.len() > 0 {

        let id_doc = docs[0].get_object_id("_id").unwrap();
        let mongo_id_hex = id_doc.to_hex();

        return mongo_id_hex;
    }

    return to_string!("");
}

fn new_hash(block: &HashMap<String, Value>) -> String {

    let data: &Value = block.get("data").unwrap();
    let datetime: &Value = block.get("datetime").unwrap();
    let high: &Value = block.get("high").unwrap();
    let pre_hash: &Value = block.get("pre_hash").unwrap();
    let nonce: &Value = block.get("nonce").unwrap();

    let data_value: String = blockchain_types::get_string(data).replace("\"", "");
    let datetime_value: String = blockchain_types::get_string(datetime).replace("\"", "");
    let high_value: String = blockchain_types::get_string(high).replace("\"", "");
    let pre_hash_value: String = blockchain_types::get_string(pre_hash).replace("\"", "");
    let nonce_value: String = blockchain_types::get_string(nonce).replace("\"", "");

    let object = BlockChainBlock {

       data: data_value,
       datetime: datetime_value,
       high: high_value,
       pre_hash: pre_hash_value,
       nonce: nonce_value
    };

    let json_data: String = json::encode(&object).unwrap();

    // println!("{}", json_data);

    let mut hash: String = utils::sha256_encode(&json_data);
    hash = hash.replace("\"", "").replace(" ", "");

    return hash;
}

fn new_nonce() -> String {

    let bytes = rand::thread_rng().gen::<[u8; 32]>();

    let nonce = hex::encode(&bytes);

    return nonce;
}

fn get_merkle_hash(collection: &Collection) -> String {

    let values = get_all_ids(collection);

    let tree = MerkleTree::from_vec(digest, values);

    let root_hash = tree.root_hash();
    let final_root_hash = hex::encode(&root_hash);

    return final_root_hash;
}

fn get_all_ids(collection: &Collection) -> Vec<String> {

    let mut options: FindOptions = FindOptions::new();

    options.limit = Some(6);
    options.sort = Some(doc! { "_id" : -1 });
    options.projection = Some(doc! { "_id" : true });

    let cursor = collection.find(None, Some(options)).ok().expect("Failed to execute find.");

    let docs_ids : Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    // println!("recover objects: {}", docs_ids.len());

    let mut vec_ids: Vec<String> = Vec::new();

    let mut i = 0;
    for doc in docs_ids {

        let id_doc = doc.get_object_id("_id").unwrap();
        let mongo_id_hex = id_doc.to_hex();

        // println!("objec_id: {}", mongo_id_hex);

        vec_ids.insert(i, mongo_id_hex);

        i = i + 1;
    }

    return vec_ids;
}

fn get_chain_high(collection: &Collection) -> String {

    let mut options: FindOptions = FindOptions::new();

    options.sort = Some(doc! { "_id" : -1 });

    let cursor = collection.find(None, Some(options)).ok().expect("Failed to execute find.");

    let docs : Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    return docs.len().to_string();
}
