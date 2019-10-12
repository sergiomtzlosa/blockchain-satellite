extern crate iron;

use iron::prelude::*;
use iron::status;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;

pub fn manage_users(request: &mut Request) -> Response {

    let str_response = utils::get_json_body(request);

    let http_method: &str = request.method.as_ref();

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
