#[allow(dead_code)]
struct MongoDBConnector {

   host: &'static str,
   user: &'static str,
   password: &'static str,
   port: &'static str,
   database: &'static str
}

#[allow(dead_code)]
impl MongoDBConnector {

   pub fn new(host: &'static str, user: &'static str, password: &'static str, port:&'static str) -> MongoDBConnector {

       MongoDBConnector{host: host, user: user, password: password, port: port, database: ""}
   }

   pub fn new_init(host: &'static str, user: &'static str, password: &'static str, port: &'static str, database: &'static str) -> MongoDBConnector {

       MongoDBConnector{host: host, user: user, password: password, port: port, database: database}
   }

   pub fn set_database(&mut self, database: &'static str) {

     self.database = database;
   }
}
