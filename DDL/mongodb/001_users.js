// mongo  --authenticationDatabase admin < 001_users.js
// With user authentication: mongo -u username -p password --authenticationDatabase admin < 001_users.js
//

use admin

var existsRoot = db.system.users.find({user:'root'}).count()

if (!existsRoot) {

  db.createUser(
    {
      user: "root",
      pwd: "root",
      roles: [ { role: "root", db: "admin" } ]
    }
  );

  print("Admin created!!!")

} else {

  print("Admin already exists!!!")
}

var authResult = db.auth('root','root')

if (authResult) {

  print("Authenticated as root!!!")

  use sensors;

  var user = db.getUser("data_api")

  if (user != null) {

     db.dropUser("data_api")

     print("User data_api dropped!!!")
  }

  db.createUser(
    {
      user: "data_api",
      pwd: "data_api",
      roles: [
         { role: "dbOwner", db: "sensors" }
      ]
    }
  );

  print("User data_api created!!!")

} else {

    print("Fail authentication as root!!!")
}
