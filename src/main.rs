extern crate iron;
extern crate router;
extern crate dotenv;
extern crate rustc_serialize;

pub use blockchain_rust::modules::login::login;
use dotenv::dotenv;
use iron::prelude::*;
use iron::status;
use router::Router;

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

    let server = to_string!("0.0.0.0:") + &utils::unwrap_key("WEBSERVER_PORT");

    println!("Running on http://{}", server);

    Iron::new(router).http(server).unwrap();
}
