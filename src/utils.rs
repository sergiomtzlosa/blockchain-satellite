extern crate crypto;

use std::env;
use iron::status;
use iron::prelude::*;
use iron::mime::Mime;
use iron::Headers;
use std::io::Read;
use std::collections::HashMap;
use rustc_serialize::json;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use rustc_serialize::hex::ToHex;

pub fn unwrap_key(key: &str) -> String {

    let value: String = to_string!(env::var(key).unwrap());

    return value;
}

pub fn create_response(status_value: status::Status, str_response: String) -> Response {

    let content_type = "application/json".parse::<Mime>().unwrap();

    Response::with((content_type, status_value, str_response))
}

pub fn get_json_body(req: &mut Request) -> String {

    let mut payload = String::new();

    req.body.read_to_string(&mut payload).expect("Cannot parse JSON");

    return payload;
}

pub fn create_json_output_payload(code: &str, message: &str) -> String {

    let mut value_object = HashMap::new();

    value_object.insert("code", code);
    value_object.insert("message", message);

    let output_payload = json::encode(&value_object).expect("Error encoding response");

    return output_payload;
}

pub fn get_header_with_name(name_header: String, headers: &Headers) -> String {

    let mut value: String = to_string!("");

    for header in headers.iter() {

        if name_header.to_lowercase() == header.name().to_lowercase() {
            
            value = header.value_string();
            break;
        }
    }

    return value;
}

pub fn new_hash(message: &str) -> String {

    let salted: String = to_string!(message) + &unwrap_key("SALT_WORD").as_ref();

    let hmac_key: Vec<u8> = Vec::new();
    let mut hmac = Hmac::new(Sha256::new(), &hmac_key);
    hmac.input(salted.as_bytes());

    let result = hmac.result().code().to_hex();

    return result;
}
