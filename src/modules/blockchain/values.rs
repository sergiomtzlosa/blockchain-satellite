extern crate iron;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use crate::modules::users::users_manager;
use crate::modules::blockchain::values_manager;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;

pub fn manage_blockchain(request: &mut Request) -> Response {

    let str_response = utils::get_json_body(request);

    if str_response.len() == 0 {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
    }

    let token: String = utils::get_header_with_name(to_string!("Token"), &request.headers);

    if token.len() == 0 {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
    }

    if users_manager::enabled_user(&token) {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::USER_DISABLED));
    }

    if users_manager::expired_token(&token) {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::TOKEN_EXPIRED));
    }

    let http_method: &str = request.method.as_ref();

    let status_code;
    let out;

    out = match json::decode(&str_response) {

        Ok(incoming)  => {

            let map: HashMap<String, String> = incoming;

            if http_method.to_lowercase() == "post" {

                let (result_map, code) = perform_blockchain_post(&token, &map);
                status_code = code;

                json::encode(&result_map).expect("Error encoding response")

            } else if http_method.to_lowercase() == "put" {

                let (result_map, code) = perform_blockchain_put(&token, &map);
                status_code = code;

                json::encode(&result_map).expect("Error encoding response")

            } else if http_method.to_lowercase() == "delete" {

                let url_str = request.url.as_ref().as_str();
                let blockchain_name = utils::get_param_url_with_name("blockchain_name", url_str);

                let (result_map, code) = perform_blockchain_delete(&blockchain_name);
                status_code = code;

                json::encode(&result_map).expect("Error encoding response")

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

    utils::create_response(status_code, out)
}

fn perform_blockchain_post(token: &String, data: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

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

    return values_manager::drop_blockchain(collection);
}

fn perform_blockchain_get(block_id: &String, encryption: bool) -> (HashMap<String, String>, status::Status) {

    return values_manager::search_block_with_id(block_id, encryption);
}
