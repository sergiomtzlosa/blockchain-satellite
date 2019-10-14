extern crate iron;

use iron::prelude::*;
use iron::status;
//use crate::modules::blockchain::blockchain_manager;

pub use crate::utils;

pub fn manage_blockchain(request: &mut Request) -> Response {

    let http_method: &str = request.method.as_ref();

    if http_method.to_lowercase() == "post" {

    } else if http_method.to_lowercase() == "put" {

    } else if http_method.to_lowercase() == "get" {

    } else if http_method.to_lowercase() == "delete" {

    } else {

    }

    println!("Method {}", http_method);

    utils::create_response(status::Ok, to_string!(""))
}
