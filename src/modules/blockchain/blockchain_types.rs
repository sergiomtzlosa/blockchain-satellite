use mongodb::Bson;
use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable)]
pub struct BlockChainBlock  {

    pub data: String,
    pub datetime: String,
    pub high: String,
    pub pre_hash: String,
    pub nonce: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ComplexDocumentFind  {

    pub data: HashMap<String, String>,
    pub datetime: String,
    pub high: String,
    pub nonce: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DocumentFind  {

    pub data: HashMap<String, String>,
    pub block_id: String,
    pub verified: String
}

pub enum Value {
    Bson(Bson),
    String(String)
}

pub fn get_string(data: &Value) -> String {

    let mut value_str: String = String::new();
    if let Value::String(i) = data {

        value_str = i.to_string();
    }

    return value_str;
}

pub fn get_bson(data: &Value) -> Bson {

    let mut value_bson: Bson = Bson::from(to_string!(""));
    if let Value::Bson(i) = data {

        value_bson = i.to_owned();
    }

    return value_bson;
}
