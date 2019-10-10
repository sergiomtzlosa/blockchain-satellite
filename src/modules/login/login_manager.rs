extern crate mysql;

pub use super::super::databases::connector_mysql::MySQLConnector;
use crate::utils;
use std::collections::HashMap;
use mysql as my;

#[allow(dead_code)]
struct User {
    user_id: i32,
    username: Option<String>,
    password: Option<String>,
    name: Option<String>,
    surname: Option<String>,
    description: Option<String>,
    enabled: bool,
    is_admin: bool,
}

#[allow(dead_code)]
struct UserToken {
    token: Option<String>,
    expired: bool,
}

pub fn fill_values(host: &'static str, user: &'static str, password: &'static str, port:&'static str, database: &'static str) -> MySQLConnector{

    let data_values = MySQLConnector::new_init(host, user, password, port, database);

    return data_values;
}

pub fn connexion_values() -> HashMap<String, String> {

    let user = utils::unwrap_key("MARIADB_USER");
    let password = utils::unwrap_key("MARIADB_PASSWORD");
    let host = utils::unwrap_key("MARIADB_HOST");
    let port = utils::unwrap_key("MARIADB_PORT");
    let database = utils::unwrap_key("MARIADB_DATABASE");
    let table = utils::unwrap_key("MARIADB_TABLE");

    let mut map_values: HashMap<String, String> = HashMap::new();

    map_values.insert(String::from("user"), user);
    map_values.insert(String::from("password"), password);
    map_values.insert(String::from("host"), host);
    map_values.insert(String::from("port"), port);
    map_values.insert(String::from("database"), database);
    map_values.insert(String::from("table"), table);

    return map_values;

}

pub fn enabled_user(token: String) -> bool {

    let values: HashMap<String, String>  = connexion_values();

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", values.get("user").unwrap(), values.get("password").unwrap(), values.get("host").unwrap(), values.get("port").unwrap(), values.get("database").unwrap());
    let pool = my::Pool::new(conn_string).unwrap();

    let query: String = format!("SELECT enabled FROM sensors_users WHERE username = '{}' LIMIT 1", token);

    let selected_user: Vec<User> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let (user_id, username, password, name, surname, description, enabled, is_admin) = my::from_row(row);

            User {
                user_id: user_id,
                username: username,
                password: password,
                name: name,
                surname: surname,
                description: description,
                enabled: enabled,
                is_admin: is_admin
            }
        }).collect()
    }).unwrap();

    let rows = selected_user.len();

    let mut expired: bool = false;

    if rows > 0 {

        let user_enabled: bool = selected_user[0].enabled;

        if user_enabled {

            expired = true;
        }

    }

    return expired;
}
