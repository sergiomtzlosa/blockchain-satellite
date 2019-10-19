extern crate crypto;

use std::env;
use iron::status;
use iron::prelude::*;
use iron::mime::Mime;
use iron::Headers;
use url::Url;
use std::io::Read;
use std::collections::HashMap;
use rustc_serialize::json;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use rustc_serialize::hex::ToHex;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fs;

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

pub fn create_json_output_payload_object(object: HashMap<String, String>) -> String {

    let output_payload = json::encode(&object).expect("Error encoding response");

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

pub fn get_param_url_with_name(name_param: &str, url: &str) -> String {

    let parsed_url = Url::parse(url).unwrap();
    let query_params: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    let value = query_params.get(name_param).unwrap();

    return to_string!(value);
}

pub fn sha256_encode(str_item: &str) -> String {

    let str_final: String = to_string!(str_item);

    let hmac_key: Vec<u8> = Vec::new();
    let mut hmac = Hmac::new(Sha256::new(), &hmac_key);
    hmac.input(str_final.as_bytes());

    let result = hmac.result().code().to_hex();

    return result;
}

pub fn new_hash(message: &str) -> String {

    let salted: String = to_string!(message) + &unwrap_key("SALT_WORD").as_ref();

    let hmac_key: Vec<u8> = Vec::new();
    let mut hmac = Hmac::new(Sha256::new(), &hmac_key);
    hmac.input(salted.as_bytes());

    let result = hmac.result().code().to_hex();

    return result;
}

pub fn is_numeric(str_value: &str) -> bool {

    let mut check_number = false;
    let check_number_value = str_value.parse::<i32>();

    match check_number_value {
        Ok(_) => {
            check_number = true
        },
        Err(_) => {}
    }

    return check_number;
}

pub fn md5_file(file_path: &str) -> String {

    let file_contents_str: String = fs::read_to_string(file_path).unwrap();

    let mut md5_object = Md5::new();
    md5_object.input_str(&file_contents_str);

    let result: String = md5_object.result_str();

    return result;
}

pub fn sha256_file(file_path: &str) -> String {

    let file_contents_str: String = fs::read_to_string(file_path).unwrap();

    let mut sha = Sha256::new();
    sha.input_str(&file_contents_str);

    let result: String = sha.result_str();

    return result;
}

pub fn md5_string(str_value: &str) -> String {

    let mut md5_object = Md5::new();
    md5_object.input_str(&str_value);

    let result: String = md5_object.result_str();

    return result;
}

pub fn sha256_string(str_value: &str) -> String {

    let mut sha = Sha256::new();
    sha.input_str(&str_value);

    let result: String = sha.result_str();

    return result;
}
