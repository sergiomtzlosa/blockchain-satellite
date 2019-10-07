extern crate iron;
extern crate router;

mod config;
mod http_codes;

use iron::prelude::*;
// use iron::mime::Mime;
use iron::status;
// use std::collections::HashMap;

use router::Router;

// use rustc_serialize::json;

// use std::io::Read;

fn login(req: &mut Request) -> IronResult<Response> {

    let http_method: &str = req.method.as_ref();

    println!("Method {}", http_method);

    Ok(Response::with((status::Ok, "OK")))
}

fn users(req: &mut Request) -> IronResult<Response> {

    let http_method: &str = req.method.as_ref();

    println!("Method {}", http_method);

    Ok(Response::with((status::Ok, "OK")))
}

fn values(req: &mut Request) -> IronResult<Response> {

    let http_method: &str = req.method.as_ref();

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

fn main() {

    let mut router = Router::new();

    router.get("/users", users, "users");
    router.post("/users", users, "users");
    router.put("/users", users, "users");
    router.delete("/users", users, "users");

    router.post("/login", login, "login");

    router.post("/values", values, "values");
    router.put("/values", values, "values");
    router.delete("/values", values, "values");

    println!("salt {}", config::SALT_WORD);

    println!("Running on http://0.0.0.0:8080");
    Iron::new(router).http("0.0.0.0:8080").unwrap();
}
