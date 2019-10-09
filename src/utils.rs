extern crate iron;

use std::env;
use iron::status;
use iron::prelude::*;
use iron::mime::Mime;
use std::io::Read;
use std::collections::HashMap;
use rustc_serialize::json;

pub fn unwrap_key(key: &str) -> String {

    let value: String = String::from(env::var(key).unwrap());

    return value;
}

pub fn create_response(ok_response: bool, status_value: status::Status, str_response: String) -> Response {

    let content_type = "application/json".parse::<Mime>().unwrap();

    if ok_response {

        Response::with((content_type, status_value, str_response))

    } else {

        Response::with((content_type, status_value, str_response))
    }
}

pub fn get_json_body(req: &mut Request) -> String {

    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("Cannot parse JSON");

    return payload;
}

pub fn create_output_payload(code: &str, message: &str) -> String {

    let mut value_object = HashMap::new();

    value_object.insert("code", code);
    value_object.insert("message", message);

    let output_payload = json::encode(&value_object).unwrap();

    return output_payload;
}

pub fn create_json_output_payload(code: &str, message: &str) -> String {

    let mut value_object = HashMap::new();

    value_object.insert("code", code);
    value_object.insert("message", message);

    let output_payload = json::encode(&value_object).expect("Error encoding response");

    return output_payload;
}
