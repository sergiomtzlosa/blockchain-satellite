extern crate iron;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use std::collections::HashMap;
use crate::modules::users::users_manager;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;

//md5: https://docs.rs/md5/0.6.1/md5/
pub fn manage_users(request: &mut Request) -> Response {

    let str_response = utils::get_json_body(request);

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

    if http_method.to_lowercase() == "get" {

        let (result_map, code) = perform_users_get(token, &request);
        status_code = code;

        out = json::encode(&result_map).expect("Error encoding response")

    } else {

        out = match json::decode(&str_response) {

            Ok(incoming)  => {

                let map: HashMap<String, String> = incoming;

                if http_method.to_lowercase() == "post" || http_method.to_lowercase() == "put" {

                    let (result_map, code) = perform_users_post(token, http_method.to_lowercase(), &map);
                    status_code = code;

                    json::encode(&result_map).expect("Error encoding response")

                } else if http_method.to_lowercase() == "delete" {

                    let (result_map, code) = perform_users_delete(token, &map);
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

    utils::create_response(status_code, out)
}

fn perform_users_post(token: String, operation: String, map: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    let username: String = map.get("username").unwrap().to_string();
    let password: String = map.get("password").unwrap().to_string();
    let name: String = map.get("name").unwrap().to_string();
    let surname: String = map.get("surname").unwrap().to_string();
    let description: String = map.get("description").unwrap().to_string();

    if operation.to_lowercase() == "put" {

        let user_id: String = map.get("user_id").unwrap().to_string();

         users_manager::update_user(&username, &password, &name, &surname, &description, &user_id);
    }

    if  operation.to_lowercase() == "post" {

        let is_admin: String = map.get("admin").unwrap().to_string();

        return users_manager::insert_new_user(&username, &password, &name, &surname, &description, &is_admin, &token);
    }

    return (HashMap::new(), status::Ok);
}

fn perform_users_delete(token: String, map: &HashMap<String, String>) -> (HashMap<String, String>, status::Status) {

    let user_id: String = map.get("user_id").unwrap().to_string();

    return users_manager::delete_user(&user_id, &token);
}

fn perform_users_get(token: String, req: &Request) -> (HashMap<String, String>, status::Status) {

    let url_str = req.url.as_ref().as_str();
    let user_id = utils::get_param_url_with_name("user_id", url_str);

    println!("user_id value: {}", user_id);

    return users_manager::select_user(&user_id, &token);
}
