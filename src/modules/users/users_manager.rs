extern crate mysql;
extern crate iron;

use crate::connection_data::*;
use crate::modules::login::login_manager::User;
use crate::utils;
use mysql as my;
use mysql::params;
use iron::status;
use std::collections::HashMap;
pub use crate::messages;
pub use crate::http_codes;

#[derive(Debug, Clone, Copy)]
struct UserExpired {
    expired: bool
}

#[derive(Debug, Clone, Copy)]
struct TokenAdmin {
    is_admin: bool
}

#[derive(Debug, Clone)]
struct UserExists {
    username: String,
    password: String
}

#[derive(Debug, Clone)]
struct UserInsert {
    username: String,
    hash_password: String,
    name: String,
    surname: String,
    description: String,
    admin: bool
}

#[derive(Debug, Clone)]
struct UserUpdate {
    username: String,
    hash_password: String,
    name: String,
    surname: String,
    description: String,
    user_id: i32
}

#[derive(Debug, Clone)]
struct LastRow {
    user_id: i32,
    token: String,
}

#[derive(Debug, Clone, Copy)]
struct UserDelete {
    user_id: i32
}

#[derive(Debug, Clone, Copy)]
struct UserId {
    user_id: i32
}

#[derive(Debug, Clone)]
struct UserFullData {
    user_id: i32,
    username: String,
    password: String,
    name: String,
    surname: String,
    description: String,
    token: String,
    enabled: bool,
    is_admin: bool
}

fn check_userid_exists(user_id: &String) -> bool {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let query: String = format!("SELECT user_id FROM sensors_users WHERE user_id = {} LIMIT 1", user_id);

    let selected_user: Vec<UserId> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let user_id = my::from_row(row);

            UserId {
                user_id: user_id
            }
        }).collect()
    }).unwrap();

    let rows = selected_user.len();

    let userid_exists: bool = if rows > 0 { true } else { false };

    return userid_exists;
}

fn username_password_exists(username: &String, password: &String) -> bool {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let hash_password: String = utils::new_hash(password);

    let query: String = format!("SELECT username, password FROM sensors_users WHERE username LIKE '%{}%' OR password LIKE '%{}%' LIMIT 1", username, hash_password);

    let selected_user: Vec<UserExists> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let (username, password) = my::from_row(row);

            UserExists {
                username: username,
                password: password
            }
        }).collect()
    }).unwrap();

    let rows = selected_user.len();

    let is_admin_token: bool = if rows > 0 { true } else { false };

    return is_admin_token;
}

fn is_token_admin(token: &String) -> bool {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let query: String = format!("SELECT is_admin FROM sensors_users AS s1 INNER JOIN sensors_tokens AS s2 ON s1.user_id = s2.token_user_id WHERE s2.token = '{}' LIMIT 1", token);

    let selected_user: Vec<TokenAdmin> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let is_admin = my::from_row(row);

            TokenAdmin {
                is_admin: is_admin
            }
        }).collect()
    }).unwrap();

    let rows = selected_user.len();

    let mut is_admin_token: bool = false;

    if rows > 0 {

        is_admin_token = selected_user[0].is_admin;
    }

    return is_admin_token;
}

fn get_last_inserted_row() -> (String, String) {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let query: String = to_string!("SELECT s1.user_id, s2.token FROM sensors_users s1 INNER JOIN sensors_tokens s2 ON s1.user_id = s2.token_user_id ORDER BY s1.user_id DESC LIMIT 1");

    let items: Vec<LastRow> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let (user_id, token) = my::from_row(row);

            LastRow {
                user_id: user_id,
                token: token
            }
        }).collect()
    }).unwrap();

    let rows = items.len();

    if rows > 0 {

        let object: &LastRow = &items[0];
        let user_id: &i32 = &object.user_id;
        let token: &String = &object.token;

        return (user_id.to_string(), to_string!(token));
    }

    return (to_string!(""), to_string!(""));
}

pub fn enabled_user(token: &String) -> bool {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let query: String = format!("SELECT enabled FROM sensors_users AS s1 INNER JOIN sensors_tokens AS s2 ON s1.user_id = s2.token_user_id WHERE s2.token = '{}' LIMIT 1", token);

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

pub fn expired_token(token: &String) -> bool {

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let query: String = format!("SELECT expired FROM sensors_tokens WHERE token = '{}' LIMIT 1", token);

    let selected_user: Vec<UserExpired> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let expired = my::from_row(row);

            UserExpired {
                expired: expired
            }
        }).collect()
    }).unwrap();

    let rows = selected_user.len();

    let mut user_expired: bool = false;

    if rows > 0 {

        user_expired = selected_user[0].expired;
    }

    return user_expired;
}

pub fn insert_new_user(username: &String, password: &String, name: &String, surname: &String, description: &String, is_admin: &String, token: &String) -> (HashMap<String, String>, status::Status) {

    if token.len() == 0 {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::USERNAME_PASSWORD_EXISTS));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    // Check if token is admin, only admin can insert
    if !is_token_admin(token) {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::USERNAME_PASSWORD_EXISTS));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    // Check if username or password EXISTS
    if username_password_exists(username, password) {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::USERNAME_PASSWORD_EXISTS));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    let mut admin = false;

    if is_token_admin(token) {

      if is_admin.len() > 0 {

        if is_admin == "1" {

          admin = true;
        }
      }
    }

    let hash_password = utils::new_hash(password);

    let query_head = r"INSERT INTO sensors_users (username, password, name, surname, description, is_admin) ";
    let query_tail = "VALUES (:username, :hash_password, :name, :surname, :description, :admin)";

    let query_final = to_string!(query_head) + &query_tail;

    let insert_user = vec![
        UserInsert {
            username: to_string!(username),
            hash_password: to_string!(hash_password),
            name: to_string!(name),
            surname: to_string!(surname),
            description: to_string!(description),
            admin: admin
        }
    ];

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    for mut stmt in pool.prepare(query_final).into_iter() {

        let user: &UserInsert = &insert_user[0];

        stmt.execute(params!{
            "username" => &user.username,
            "hash_password" => &user.hash_password,
            "name" => &user.name,
            "surname" => &user.surname,
            "description" => &user.description,
            "admin" => &user.admin,
        }).unwrap();
    }

    let (user_id, token) = get_last_inserted_row();

    if user_id.len() > 0 && token.len() > 0 {

        let mut map: HashMap<String, String> = HashMap::new();

        map.insert(to_string!("user_id"), user_id);
        map.insert(to_string!("token"), token);

        return (map, status::Created);
    }

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(to_string!("message"), to_string!(messages::CANNOT_INSERT));
    map.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

    return (map, status::InternalServerError);
}

pub fn update_user(username: &String, password: &String, name: &String, surname: &String, description: &String, user_id: &String) -> (HashMap<String, String>, status::Status) {

    if username.len() == 0 || password.len() == 0 || name.len() == 0 || surname.len() == 0 || description.len() == 0 || user_id.len() == 0 {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::ONE_OR_MORE_PARAMETERS_EMPTY));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    if !utils::is_numeric(user_id) {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::USER_ID_NOT_NUMBER));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);

    }

    let hash_password = utils::new_hash(password);

    let query = format!("CALL update_user_data('{}', '{}', '{}', '{}', '{}', {})", username, hash_password, name, surname, description, to_int!(user_id));

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    pool.prep_exec(query, ()).unwrap();

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(to_string!("message"), to_string!(messages::USER_UPDATED));
    map.insert(to_string!("code"), to_string!(http_codes::HTTP_OK));

    return (map, status::Ok);
}

pub fn delete_user(user_id: &String, token: &String) -> (HashMap<String, String>, status::Status) {

    if user_id.len() == 0 || token.len() == 0 {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::USERNAME_PASSWORD_EXISTS));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    if !utils::is_numeric(&user_id) {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::CANNOT_DELETE));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    if check_userid_exists(&user_id) == false {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::CANNOT_DELETE));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    let query = r"DELETE FROM sensors_users WHERE user_id = :username";

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let delete_user = vec![
        UserDelete {
            user_id: to_int!(user_id)
        }
    ];

    for mut stmt in pool.prepare(query).into_iter() {

        let user: &UserDelete = &delete_user[0];

        stmt.execute(params!{
            "username" => &user.user_id
        }).unwrap();
    }

    let mut result: HashMap<String, String> = HashMap::new();

    result.insert(to_string!("message"), to_string!(messages::USER_DELETED));
    result.insert(to_string!("code"), to_string!(http_codes::HTTP_DELETED));

    return (result, status::Ok);
}

pub fn select_user(user_id: &String, token: &String)  -> (HashMap<String, String>, status::Status) {

    if user_id.len() == 0 || token.len() == 0 {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::USERNAME_PASSWORD_EXISTS));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

     if check_userid_exists(&user_id) == false {

        let mut result: HashMap<String, String> = HashMap::new();

        result.insert(to_string!("message"), to_string!(messages::USERNAME_OR_PASSWORD_NOT_EXISTS));
        result.insert(to_string!("code"), to_string!(http_codes::HTTP_GENERIC_ERROR));

        return (result, status::InternalServerError);
    }

    let mut query = to_string!("SELECT user_id, username, password, name, surname, description, token, enabled, is_admin  ");
    query = query + &format!("FROM sensors_users AS s1 INNER JOIN sensors_tokens AS s2 ON s1.user_id = s2.token_user_id WHERE s1.user_id = {} LIMIT 1", user_id);

    let conn_string: String = format!("mysql://{}:{}@{}:{}/{}", &**MYSQL_USER, &**MYSQL_PASSWORD, &**MYSQL_HOST, &**MYSQL_PORT, &**MYSQL_DATABASE);
    let pool = my::Pool::new(conn_string).unwrap();

    let selected_user: Vec<UserFullData> = pool.prep_exec(query, ()).map(|result| {

        result.map(|x| x.unwrap()).map(|row| {

            let (user_id, username, password, name, surname, description, token, enabled, is_admin) = my::from_row(row);

            UserFullData {
                user_id: user_id,
                username: username,
                password: password,
                name: name,
                surname: surname,
                description: description,
                token: token,
                enabled: enabled,
                is_admin: is_admin
            }
        }).collect()
    }).unwrap();

    let rows = selected_user.len();

    if rows > 0 {

       let user: &UserFullData = &selected_user[0];

       let mut result: HashMap<String, String> = HashMap::new();

       result.insert(to_string!("username"), to_string!(&user.username));
       result.insert(to_string!("password"),  to_string!(&user.password));
       result.insert(to_string!("name"),  to_string!(&user.name));
       result.insert(to_string!("surname"),  to_string!(&user.surname));
       result.insert(to_string!("description"),  to_string!(&user.description));
       result.insert(to_string!("token"),  to_string!(&user.token));

       if is_token_admin(token) {

          result.insert(to_string!("user_id"), user.user_id.to_string());

          let enabled_str: String = if user.enabled {  to_string!("true") } else { to_string!("false") };
          result.insert(to_string!("enabled"), enabled_str);

          let is_admin_str: String = if user.is_admin {  to_string!("true") } else { to_string!("false") };
          result.insert(to_string!("is_admin"), is_admin_str);
       }

       return (result, status::Ok);
    }

    let mut result: HashMap<String, String> = HashMap::new();

    result.insert(to_string!("message"), to_string!(messages::DATA_NOT_FOUND));
    result.insert(to_string!("code"), to_string!(http_codes::HTTP_OK));

    return (result, status::Ok);
}
