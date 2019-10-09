extern crate iron;
extern crate router;
extern crate dotenv;
extern crate rustc_serialize;

#[allow(unused_imports)]
use self::databases::*;
use dotenv::dotenv;
use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::collections::HashMap;

mod databases;
mod utils;
mod http_codes;
mod messages;

fn login(req: &mut Request) -> IronResult<Response> {

    let http_method: &str = req.method.as_ref();

    if http_method.to_lowercase() == "post" {

        let str_response = utils::get_json_body(req);

        //println!("{}", str_response);

        let out = match json::decode(&str_response) {

            Ok(incoming)  => {

                let map: HashMap<String, String> =  incoming;

                let user_name = map.get("username").unwrap();
                let password = map.get("password").unwrap();

                println!("{} - {}", user_name, password);

                json::encode(&"nice").expect("Error encoding response")
            },
            Err(_) => {

                let response = messages::JSON_CONTENT_NOT_VALID;

                utils::create_json_output_payload(http_codes::HTTP_GENERIC_ERROR, response)
            }
        };

        Ok(utils::create_response(true,status::InternalServerError,out))

    } else {

        Ok(utils::create_response(true,
            status::InternalServerError,
            utils::create_output_payload(http_codes::HTTP_GENERIC_ERROR, messages::INTERNAL_ERROR)))
    }

    //println!("Method {}", http_method);

    //Ok(Response::with((status::Ok, "OK")))
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
