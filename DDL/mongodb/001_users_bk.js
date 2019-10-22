// mongo  --authenticationDatabase admin < 001_users.js
// With user authentication: mongo -u username -p password --authenticationDatabase admin < 001_users.js

use sensors;

db.createUser({
      user: "data_api",
      pwd: "data_api",
      roles: [
         { role: "dbOwner", db: "sensors" }
      ]
    }
);

