extern crate iron;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use crate::modules::users::users_manager;
use crate::modules::blockchain::values_manager;
use crate::modules::blockchain::container_objects::ContainerObjects;
use rustc_serialize;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;
pub use crate::typeinfo;

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

        out = json::encode(&result_map).expect("Error encoding response");

    } else {

        out = match json::decode(&str_response) {

            Ok(incoming)  => {

                let object: HashMap<String, String> = incoming;

                if http_method.to_lowercase() == "post" {

                    // test
                    let final_data: ContainerObjects = ContainerObjects{

                        array: Vec::new(),
                        map: HashMap::new()
                    };

                    let (result_map, code) = perform_blockchain_post(&token, &final_data);
                    status_code = code;

                    json::encode(&result_map).expect("Error encoding response")

                } else if http_method.to_lowercase() == "put" {

                    let (result_map, code) = perform_blockchain_put(&token, &object);
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

    println!("{}", out);

    utils::create_response(status_code, out)
}

fn perform_blockchain_post(token: &String, data: &ContainerObjects) -> (HashMap<String, String>, status::Status) {

    return values_manager::insert_new_document(&data, &token);
}

fn perform_blockchain_put(token: &String, data: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    let row: String =  data.get("docs").unwrap().to_string();

    let mut date_from: String =  data.get("date_from").unwrap().to_string();

    if date_from.len() == 0 {

        date_from = to_string!("");
    }

    let mut date_to: String =  data.get("date_to").unwrap().to_string();

    if date_to.len() == 0 {

        date_to = to_string!("");
    }

    return values_manager::find_documents(&row, &date_from, &date_to, &token);
}

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

fn perform_blockchain_get(block_id: &String, encryption: bool) -> (HashMap<String, String>, status::Status) {

    return values_manager::search_block_with_id(block_id, encryption);
}
