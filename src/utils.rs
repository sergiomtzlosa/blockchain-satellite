use std::env;

pub fn unwrap_key(key: &str) -> String {

    let value = env::var(key).unwrap();
    return value;
}
