use crate::utils;
use std::collections::HashMap;
pub use crate::macros;

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
