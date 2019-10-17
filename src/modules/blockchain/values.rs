extern crate iron;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use crate::modules::users::users_manager;
// use crate::modules::blockchain::values_manager;

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

                let (result_map, code) = perform_blockchain_post(token, &map);
                status_code = code;

                json::encode(&result_map).expect("Error encoding response")

            } else if http_method.to_lowercase() == "put" {

                let (result_map, code) = perform_blockchain_put(token, &map);
                status_code = code;

                json::encode(&result_map).expect("Error encoding response")

            } else if http_method.to_lowercase() == "get" {

                let (result_map, code) = perform_blockchain_get(token, &map);
                status_code = code;

                json::encode(&result_map).expect("Error encoding response")

            } else if http_method.to_lowercase() == "delete" {

                let (result_map, code) = perform_blockchain_delete(&map);
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

fn perform_blockchain_post(token: String, data: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}

fn perform_blockchain_put(token: String, data: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}

fn perform_blockchain_get(token: String, data: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}

fn perform_blockchain_delete(data: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    return (HashMap::new(), status::Ok);
}
