extern crate iron;
extern crate router;
extern crate dotenv;
extern crate rustc_serialize;
extern crate crypto;

use blockchain_rust::modules::login::login;
use blockchain_rust::modules::users::users;
use blockchain_rust::modules::blockchain::values;
// use blockchain_rust::modules::blockchain::encryption;
use blockchain_rust::connection_data::*;
use dotenv::dotenv;
use iron::prelude::*;
use router::Router;
use std::str;
use blockchain_rust::utils;
use std::env;

#[macro_use]
mod macros;

#[macro_use]
mod typeinfo;

static USER_SERVICE: &str = "/api/users";
static LOGIN_SERVICE: &str = "/api/login";
static VALUES_SERVICE: &str = "/api/values";

fn login(req: &mut Request) -> IronResult<Response> {

    let response = login::perform_login(req);

    Ok(response)

}

fn users(req: &mut Request) -> IronResult<Response> {

    let response = users::manage_users(req);

    Ok(response)
}

fn values(req: &mut Request) -> IronResult<Response> {

    let response = values::manage_values(req);

    Ok(response)
}

fn main() {

    println!("");
    println!("\tRust blockchain v1.0");
    println!("\t---------------------");
    println!("");
    println!("");

    let use_docker: bool = utils::uses_docker(env::args());

    if use_docker {

        dotenv::from_filename(".env-docker").ok();

        println!("\tLoading Docker configuration...");

    } else {

        dotenv().ok();

        println!("\tLoading default configuration...");
    }

    println!("");

    let mut router = Router::new();

    router.get(USER_SERVICE, users, "users");
    router.post(USER_SERVICE, users, "users");
    router.put(USER_SERVICE, users, "users");
    router.delete(USER_SERVICE, users, "users");

    router.post(LOGIN_SERVICE, login, "login");

    router.post(VALUES_SERVICE, values, "values");
    router.put(VALUES_SERVICE, values, "values");
    router.get(VALUES_SERVICE, values, "values");
    router.delete(VALUES_SERVICE, values, "values");

    let server = to_string!("0.0.0.0:") + &**WEBSERVER_PORT;

    // encryption_test();

    println!("");
    println!(" - Starting webserver with Rust...");
    println!(" - Webserver running on http://{}", server);
    println!("");

    Iron::new(router).http(server).unwrap();
}

// Encryption test
// fn encryption_test() {
//
//     let message = "hello world!";
//
//     let encrypted_data: String = encryption::encrypt_operation(&message);
//
//     println!("");
//     println!("Encryption test");
//     println!("--------------");
//     println!("");
//
//     println!("{:?}", &encrypted_data);
//
//     let decrypted_data: Vec<u8> = encryption::decrypt_operation(&encrypted_data);
//
//     println!("decoded message: {}", String::from_utf8_lossy(&decrypted_data));
// }
