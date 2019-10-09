extern crate iron;
extern crate router;
extern crate dotenv;

#[allow(unused_imports)]
use self::databases::*;
use dotenv::dotenv;

mod databases;
mod utils;
mod http_codes;

use iron::prelude::*;
use iron::status;
use router::Router;
// use iron::mime::Mime;
// use std::collections::HashMap;
// use rustc_serialize::json;
// use std::io::Read;

fn login(req: &mut Request) -> IronResult<Response> {

    let http_method: &str = req.method.as_ref();

    if http_method.to_lowercase() == "post" {

    } else {

    }

    println!("Method {}", http_method);

    Ok(Response::with((status::Ok, "OK")))
}

fn users(req: &mut Request) -> IronResult<Response> {

    let http_method: &str = req.method.as_ref();

    if http_method.to_lowercase() == "get" {

    } else if http_method.to_lowercase() == "post" {

    } else if http_method.to_lowercase() == "put" {

    } else if http_method.to_lowercase() == "delete" {

    } else {

    }

    println!("Method {}", http_method);

    Ok(Response::with((status::Ok, format!("{}{}\n", "OK ", http_method) )))
}

fn values(req: &mut Request) -> IronResult<Response> {

    let http_method: &str = req.method.as_ref();

    if http_method.to_lowercase() == "post" {

    } else if http_method.to_lowercase() == "put" {

    } else if http_method.to_lowercase() == "delete" {

    } else {

    }

    println!("Method {}", http_method);

    Ok(Response::with((status::Ok, "OK")))
}

// fn post_login(_ : &mut Request) -> IronResult<Response> {
// //fn post_login(req: &mut Request) -> IronResult<Response> {
//
//     // let mut payload = String::new();
//     // req.body.read_to_string(&mut payload).expect("JSON body expected");
//     //
//     // let user: String = json::decode(&payload).expect("User object expected");
//     //
//     let mut value = HashMap::new();
//
//     value.insert("message", "test");
//
//     let payload = json::encode(&value).unwrap();
//     let content_type = "application/json".parse::<Mime>().unwrap();
//     Ok(Response::with((content_type, status::Ok, payload)))
// }

static USER_SERVICE: &str = "/api/users";
static LOGIN_SERVICE: &str = "/api/login";
static VALUES_SERVICE: &str = "/api/values";

fn main() {

    dotenv().ok();

    let mut router = Router::new();

    router.get(USER_SERVICE, users, "users");
    router.post(USER_SERVICE, users, "users");
    router.put(USER_SERVICE, users, "users");
    router.delete(USER_SERVICE, users, "users");

    router.post(LOGIN_SERVICE, login, "login");

    router.post(VALUES_SERVICE, values, "values");
    router.put(VALUES_SERVICE, values, "values");
    router.delete(VALUES_SERVICE, values, "values");

    println!("salt key: {}", utils::unwrap_key("SALT_WORD"));

    println!("Running on http://0.0.0.0:8086");
    Iron::new(router).http("0.0.0.0:8086").unwrap();
}
