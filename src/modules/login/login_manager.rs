extern crate mysql;

pub use crate::macros;
pub use super::super::databases::connector_mysql::MySQLConnector;
use crate::utils;
use std::collections::HashMap;
use mysql as my;
//use chrono::{NaiveDate, NaiveDateTime};

#[derive(Clone, Copy)]
struct User {
    enabled: bool
}

struct UserLogin {

    token: String,
    user_id: i32,
}

/*
struct bbdd_time {
    date: NaiveDateTime,
}
*/

pub fn fill_values(host: &'static str, user: &'static str, password: &'static str, port:&'static str, database: &'static str) -> MySQLConnector{

    let data_values = MySQLConnector::new_init(host, user, password, port, database);

    return data_values;
}

pub fn connection_values() -> HashMap<String, String> {

    let user = utils::unwrap_key("MARIADB_USER");
    let password = utils::unwrap_key("MARIADB_PASSWORD");
    let host = utils::unwrap_key("MARIADB_HOST");
    let port = utils::unwrap_key("MARIADB_PORT");
    let database = utils::unwrap_key("MARIADB_DATABASE");
    let table = utils::unwrap_key("MARIADB_TABLE");

    let mut map_values: HashMap<String, String> = HashMap::new();

    map_values.insert(to_string!("user"), user);
    map_values.insert(to_string!("password"), password);
    map_values.insert(to_string!("host"), host);
    map_values.insert(to_string!("port"), port);
    map_values.insert(to_string!("database"), database);
    map_values.insert(to_string!("table"), table);

    return map_values;
}

pub fn enabled_user(username: String) -> bool {

    let values: HashMap<String, String> = connection_values();

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", values.get("user").unwrap(), values.get("password").unwrap(), values.get("host").unwrap(), values.get("port").unwrap(), values.get("database").unwrap());
    let pool = my::Pool::new(conn_string).unwrap();

    let query: String = format!("SELECT enabled FROM sensors_users WHERE username = '{}' LIMIT 1", username);

    let selected_user: Vec<User> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let enabled = my::from_row(row);

            User {
                enabled: enabled
            }
        }).collect()
    }).unwrap();

    let rows = selected_user.len();

    let mut user_valid: bool = false;

    if rows > 0 {

        let user_enabled: bool = selected_user[0].enabled;

        if user_enabled {

            user_valid = true;
        }
    }

    return user_valid;
}

pub fn login_user(username: String, password: String) -> (String, String) {

    let values: HashMap<String, String> = connection_values();

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", values.get("user").unwrap(), values.get("password").unwrap(), values.get("host").unwrap(), values.get("port").unwrap(), values.get("database").unwrap());
    let pool = my::Pool::new(conn_string).unwrap();

    let hash_password = to_string!(utils::new_hash(password.as_ref()));

    let query: String = format!("CALL login_user_actions('{}', '{}');", username, hash_password);

    let logged_user: Vec<UserLogin> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let (token, user_id) = my::from_row(row);

            UserLogin {
                token: token,
                user_id: user_id
            }
        }).collect()
    }).unwrap();

    let rows = logged_user.len();

    if rows > 0 {

        let token = logged_user[0].token.to_string();
        let user_id = logged_user[0].user_id.to_string();

	return (token, user_id);
    }

    return (to_string!(""), to_string!(""));
}
