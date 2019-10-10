#[allow(dead_code)]
pub struct MySQLConnector {

   host: &'static str,
   user: &'static str,
   password: &'static str,
   port: &'static str,
   database: &'static str
}

impl MySQLConnector {

   pub fn new(host: &'static str, user: &'static str, password: &'static str, port:&'static str) -> MySQLConnector {

       MySQLConnector{host: host, user: user, password: password, port: port, database: ""}
   }

   pub fn new_init(host: &'static str, user: &'static str, password: &'static str, port: &'static str, database: &'static str) -> MySQLConnector {

       MySQLConnector{host: host, user: user, password: password, port: port, database: database}
   }

   pub fn set_database(&mut self, database: &'static str) {

       self.database = database;
   }
}
