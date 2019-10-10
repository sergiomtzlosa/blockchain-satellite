extern crate iron;

use rustc_serialize::json;
use iron::prelude::*;
use iron::status;
use std::collections::HashMap;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;
pub use super::login_manager;
pub use super::super::databases::connector_mysql;

pub fn perform_login(request: &mut Request) -> Response {

    //connector_mysql::MySQLConnector::empty_connector();

    let http_method = request.method.as_ref();

    if http_method.to_lowercase() == "post" {

        let str_response = utils::get_json_body(request);

        let status_code;

        let out = match json::decode(&str_response) {

            Ok(incoming)  => {

                let map: HashMap<String, String> = incoming;

                let username = map.get("username").unwrap();
                let password = map.get("password").unwrap();

                //println!("{} - {}", username, password);

                if username.len() == 0 || password.len() == 0 {

                    status_code = status::InternalServerError;
                    utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR,  messages::JSON_CONTENT_NOT_VALID)

                } else {

                    // Perform login against database and check the output

                    status_code = status::Ok;
                    json::encode(&"nice").expect("Error encoding response")
                }
            },
            Err(_) => {

                status_code = status::InternalServerError;
                utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR,  messages::JSON_CONTENT_NOT_VALID)
            }
        };

        utils::create_response(status_code, out)

    } else {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR))
    }
}
