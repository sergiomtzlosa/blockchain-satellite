extern crate iron;

use iron::prelude::*;
use iron::status;
use crate::modules::users::users_manager;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;

//md5: https://docs.rs/md5/0.6.1/md5/
pub fn manage_users(request: &mut Request) -> Response {

    let str_response = utils::get_json_body(request);

    let token: String = utils::get_header_with_name(to_string!("Token"), &request.headers);

    println!("{}", token);
    
    if token.len() == 0 {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
    }

    let http_method: &str = request.method.as_ref();

    if users_manager::enabled_user(&token) {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::USER_DISABLED));
    }

    if users_manager::expired_token(&token) {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::TOKEN_EXPIRED));
    }

// let username = $decoded['username'];
// let password = $decoded['password'];
// let name = $decoded['name'];
// let surname = $decoded['surname'];
// let description = $decoded['description'];

    if http_method.to_lowercase() == "get" {

        perform_users_get(str_response)

    } else if http_method.to_lowercase() == "post" || http_method.to_lowercase() == "put" {

        perform_users_post(str_response)

    } else if http_method.to_lowercase() == "delete" {

        perform_users_delete(str_response)

    } else {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR))
    }
}

fn perform_users_get(json_response: String) -> Response {

    let status_code: status::Status;

    utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR))

}

fn perform_users_post(json_response: String) -> Response {

    let status_code: status::Status;

    utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR))

}

fn perform_users_delete(json_response: String) -> Response {

    let status_code: status::Status;

    utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR))

}
