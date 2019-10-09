use std::env;

pub fn unwrap_key(key: &str) -> String {

    let value: String = String::from(env::var(key).unwrap());

    return value;
}
