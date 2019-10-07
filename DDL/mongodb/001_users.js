db.createUser(
    {
      user: "data_api",
      pwd: "data_api",
      roles: [
         { role: "dbOwner", db: "sensors" },
         { role: "dbOwner", db: "sensors_arduino" }
      ]
    }
);
