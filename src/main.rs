extern crate iron;
extern crate router;
extern crate dotenv;
extern crate rustc_serialize;
extern crate crypto;

use blockchain_rust::modules::login::login;
use blockchain_rust::modules::users::users;
use blockchain_rust::modules::blockchain::blockchain;
use blockchain_rust::modules::blockchain::encryption;
use blockchain_rust::connection_data::*;
use dotenv::dotenv;
use iron::prelude::*;
use router::Router;
use std::str;
use std::fs;

#[macro_use]
mod macros;

pub mod utils;

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

    let response = blockchain::manage_blockchain(req);

    Ok(response)
}

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
    router.get(VALUES_SERVICE, values, "values");
    router.delete(VALUES_SERVICE, values, "values");

    let server = to_string!("0.0.0.0:") + &**WEBSERVER_PORT;

    encryption_test();

    println!("");
    println!(" - Starting webserver with Rust...");
    println!(" - Webserver running on http://{}", server);

    Iron::new(router).http(server).unwrap();
}

fn encryption_test() {

    let message = "hello world!";

    let encrypted_data: Vec<u8> = encryption::encrypt_operation(&message);

    println!("");
    println!("Encryption test");
    println!("--------------");
    println!("");

    println!("{:?}", &encrypted_data);

    // let temp_str: String = String::from_utf8_lossy(&encrypted_data).to_string();

    fs::write("/tmp/foo", encrypted_data).expect("Unable to write file");

    let data = fs::read("/tmp/foo").expect("Unable to read file");

    println!("{:?}", data);

    let decrypted_data: Vec<u8> = encryption::decrypt_operation(data);

    println!("decoded message: {}", String::from_utf8_lossy(&decrypted_data));
}
