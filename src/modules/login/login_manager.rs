extern crate mysql;

pub use crate::macros;
use crate::connection_data::*;
use crate::utils;
use mysql as my;

#[derive(Debug, Clone, Copy)]
pub struct User {
    pub enabled: bool
}

#[derive(Debug)]
struct UserLogin {

    token: String,
    user_id: i32,
}

pub fn enabled_user(username: String) -> bool {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
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

    let mut user_enabled: bool = false;

    if rows > 0 {

        user_enabled = selected_user[0].enabled;
    }

    return user_enabled;
}

pub fn login_user(username: String, password: String) -> (String, String) {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
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

        let object: &UserLogin = &logged_user[0];
        let token = object.token.to_string();
        let user_id = object.user_id.to_string();

	return (token, user_id);
    }

    return (to_string!(""), to_string!(""));
}
