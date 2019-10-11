use crate::utils;

lazy_static! {

    #[derive(Copy, Clone, Debug)]
    pub static ref MYSQL_USER: String = utils::unwrap_key("MARIADB_USER");

    #[derive(Copy, Clone, Debug)]
    pub static ref MYSQL_PASSWORD: String = utils::unwrap_key("MARIADB_PASSWORD");

    #[derive(Copy, Clone, Debug)]
    pub static ref MYSQL_HOST: String = utils::unwrap_key("MARIADB_HOST");

    #[derive(Copy, Clone, Debug)]
    pub static ref MYSQL_PORT: String = utils::unwrap_key("MARIADB_PORT");

    #[derive(Copy, Clone, Debug)]
    pub static ref MYSQL_DATABASE: String = utils::unwrap_key("MARIADB_DATABASE");

    #[derive(Copy, Clone, Debug)]
    pub static ref MYSQL_TABLE: String = utils::unwrap_key("MARIADB_TABLE");
}
