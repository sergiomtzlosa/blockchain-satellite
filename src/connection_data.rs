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

    #[derive(Copy, Clone, Debug)]
    pub static ref WEBSERVER_PORT: String = utils::unwrap_key("WEBSERVER_PORT");

    #[derive(Copy, Clone, Debug)]
    pub static ref MONGODB_HOST: String = utils::unwrap_key("MONGODB_HOST");

    #[derive(Copy, Clone, Debug)]
    pub static ref MONGODB_PORT: String = utils::unwrap_key("WEBSERVER_PORT");

    #[derive(Copy, Clone, Debug)]
    pub static ref MONGODB_USER: String = utils::unwrap_key("MONGODB_USER");

    #[derive(Copy, Clone, Debug)]
    pub static ref MONGODB_PASSWORD: String = utils::unwrap_key("MONGODB_PASSWORD");

    #[derive(Copy, Clone, Debug)]
    pub static ref MONGODB_DATABASE: String = utils::unwrap_key("MONGODB_DATABASE");

    #[derive(Copy, Clone, Debug)]
    pub static ref MONGODB_COLLECTION: String = utils::unwrap_key("MONGODB_COLLECTION");

}
