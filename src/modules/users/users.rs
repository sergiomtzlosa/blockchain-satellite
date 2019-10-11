extern crate iron;

use iron::prelude::*;
use iron::status;

pub use crate::messages;
pub use crate::utils;
pub use crate::http_codes;

pub fn manage_users(request: &mut Request) -> Response {

    let http_method: &str = request.method.as_ref();

    if http_method.to_lowercase() == "get" {

        perform_users_get();

    } else if http_method.to_lowercase() == "post" {

        perform_users_post();

    } else if http_method.to_lowercase() == "put" {

        perform_users_put();

    } else if http_method.to_lowercase() == "delete" {

        perform_users_delete();

    } else {

        utils::create_response(status::InternalServerError, utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR));
    }

    //println!("Method {}", http_method);

    Response::with((status::Ok, format!("{}{}\n", "OK ", http_method)))
}

fn perform_users_get() {

}

fn perform_users_post() {

}

fn perform_users_put() {

}

fn perform_users_delete() {

}
