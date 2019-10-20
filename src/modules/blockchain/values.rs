extern crate iron;
extern crate serde_json;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use crate::modules::users::users_manager;
use crate::modules::blockchain::values_manager;
use rustc_serialize;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;

#[derive(RustcDecodable, RustcEncodable)]
pub struct InsertionResult  {

    pub docs_inserted: String,
    pub blocks: Vec<HashMap<String, String>>
}

pub fn manage_values(request: &mut Request) -> Response {

    let str_response = utils::get_json_body(request);

    let http_method: &str = request.method.as_ref();

    if str_response.len() == 0 && (http_method.to_lowercase() != "delete" || http_method.to_lowercase() != "get") {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
    }

    let token: String = utils::get_header_with_name(to_string!("Token"), &request.headers);

    if token.len() == 0 {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
    }

    if !users_manager::enabled_user(&token) {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::USER_DISABLED));
    }

    if users_manager::expired_token(&token) {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::TOKEN_EXPIRED));
    }

    let status_code;
    let out;

    if http_method.to_lowercase() == "delete" {

        let url_str = request.url.as_ref().as_str();
        let blockchain_name = utils::get_param_url_with_name("blockchain_name", url_str);

        if blockchain_name.len() == 0 {

            utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
        }

        let (result_map, code) = perform_blockchain_delete(&blockchain_name);
        status_code = code;

        out = json::encode(&result_map).expect("Error encoding response");

    } else if http_method.to_lowercase() == "get" {

        let url_str = request.url.as_ref().as_str();
        let block_id = utils::get_param_url_with_name("block_id", url_str);
        let encryption = utils::get_param_url_with_name("encryption", url_str);

        if block_id.len() == 0 || encryption.len() == 0 {

            utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
        }

        if !utils::is_numeric(&encryption) {

            utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
        }

        let value_encryption: bool = if to_int!(&encryption) == 0 { false } else { true };

        let (result_map, code) = perform_blockchain_get(&block_id, value_encryption);
        status_code = code;

        out = result_map;

    } else if http_method.to_lowercase() == "post" {

        let object = json::decode(&str_response);

        if object.is_ok() {

            let map_object: HashMap<String, String> = object.unwrap();

            let (result_map, code) = perform_blockchain_post(&map_object);
            status_code = code;

            out = result_map;

        } else {

            let raw_object = serde_json::from_str(&str_response);

            if raw_object.is_err() {

                status_code = status::InternalServerError;
                out = utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR);

            } else {

                let deserialized_vec: Vec<HashMap<String, String>> = raw_object.unwrap();

                let (result_map, code) = perform_blockchain_post_bulk(&deserialized_vec);
                status_code = code;

                out = result_map;
            }
        }

    } else {

        out = match json::decode(&str_response) {

            Ok(incoming)  => {

                let object: HashMap<String, String> = incoming;

                if http_method.to_lowercase() == "put" {

                    let (result_map, code) = perform_blockchain_put(&object);
                    status_code = code;

                    json::encode(&result_map).expect("Error encoding response")

                } else {

                    status_code = status::InternalServerError;
                    utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR)
                }

            }, Err(_) => {

                status_code = status::InternalServerError;
                utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR)
            }
        };
    }

    // println!("{}", out);

    utils::create_response(status_code, out)
}

fn perform_blockchain_post(data: &HashMap<String, String>) -> (String, status::Status) {

    let vec: Vec<HashMap<String, String>> = values_manager::insert_new_document(&data);

    let vec_len = vec.len();

    if vec_len > 0 {

         let object_result = InsertionResult {

             docs_inserted: vec_len.to_string(),
             blocks: vec
         };

         let encoded_object = json::encode(&object_result).expect("Error encoding response");

         return (encoded_object, status::Ok);
    }

    let mut value_error: HashMap<String, String> = HashMap::new();

    value_error.insert(to_string!("code"), http_codes::HTTP_GENERIC_ERROR.to_string());
    value_error.insert(to_string!("message"), messages::CANNOT_INSERT.to_string());

    let str_value: String = json::encode(&value_error).expect("Error encoding response");

    return (str_value, status::InternalServerError);
}

fn perform_blockchain_post_bulk(data: &Vec<HashMap<String, String>>) -> (String, status::Status) {

    let vec: Vec<HashMap<String, String>> = values_manager::insert_new_document_bulk(&data);

    let vec_len = vec.len();

    if vec_len > 0 {

         let object_result = InsertionResult {

             docs_inserted: vec_len.to_string(),
             blocks: vec
         };

         let encoded_object = json::encode(&object_result).expect("Error encoding response");

         return (encoded_object, status::Ok);
    }

    let mut value_error: HashMap<String, String> = HashMap::new();

    value_error.insert(to_string!("code"), http_codes::HTTP_GENERIC_ERROR.to_string());
    value_error.insert(to_string!("message"), messages::CANNOT_INSERT.to_string());

    let str_value: String = json::encode(&value_error).expect("Error encoding response");

    return (str_value, status::InternalServerError);
}

fn perform_blockchain_put(data: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    let rows: String = data.get("docs").unwrap().to_string();

    let mut date_from: String =  data.get("date_from").unwrap().to_string();

    if date_from.len() == 0 {

        date_from = to_string!("");
    }

    let mut date_to: String =  data.get("date_to").unwrap().to_string();

    if date_to.len() == 0 {

        date_to = to_string!("");
    }

    return values_manager::find_documents(&rows, &date_from, &date_to);
}

#[allow(unused_variables)]
fn perform_blockchain_delete(collection: &String) -> (HashMap<String, String>, status::Status) {

    let deleted: bool =  values_manager::drop_blockchain(collection);

    let mut result: HashMap<String, String> = HashMap::new();

    let status;

    if deleted == true {

        result.insert(to_string!("message"), messages::BLOCKCHAIN_DELETED.to_string());

        status = status::Ok;

    } else {

        result.insert(to_string!("message"), messages::DATA_NOT_FOUND.to_string());

        status = status::InternalServerError;
    }

    return (result, status);
}

fn perform_blockchain_get(block_id: &String, encryption: bool) -> (String, status::Status) {

    return values_manager::search_block_with_id(block_id, encryption);
}
