extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use std::collections::HashMap;

use router::Router;

use rustc_serialize::json;

// use std::io::Read;

fn login(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn post_login(_ : &mut Request) -> IronResult<Response> {
//fn post_login(req: &mut Request) -> IronResult<Response> {

    // let mut payload = String::new();
    // req.body.read_to_string(&mut payload).expect("JSON body expected");
    //
    // let user: String = json::decode(&payload).expect("User object expected");
    //
    let mut value = HashMap::new();

    value.insert("message", "test");

    let payload = json::encode(&value).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, payload)))
}

fn main() {
    let mut router = Router::new();
    router.get("/login", login, "index");
    router.post("/login", post_login, "post_login");

    println!("Running on http://0.0.0.0:8080");
    Iron::new(router).http("0.0.0.0:8080").unwrap();
}
