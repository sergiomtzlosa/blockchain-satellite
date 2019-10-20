Blockchain written in Rust for low-orbit satellites
----------------------------------------------------

This is a project to set-up a blockchain over a Low-Orbit satellite as part of a proof of concept for the paper:

[Blockchain and radio communications over suborbital spaceflights: Watchtowers and Mystics](https://arxiv.org/abs/1910.04835)

## Technology used

- PHP 7.1.16

- MariaDB Server 5.5.57

- MongoDB Server 3.4.15

## MySQL tune-up

The users management uses MySQL as main engine.

Execute the script present in **DDL/mysql/sensordb_DDL.sql** to fill the tables, structure, triggers and procedures:

```
mysql -u root -p < sensordb_DDL.sql
```

## Database and web services credentials

| Username      | Password      |
| ------------- |:-------------:|
| admin         | admin1234     |
| api_user      | api_user1234  |

## MongoDB tune-up

Use the script on **DDL/mongodb/001_users.js** to create the user and set permission over the MongoDB system:

```
mongo  --authenticationDatabase admin < 001_users.js
```

If your MongoDB has user authentication:

```
mongo -u username -p password --authenticationDatabase admin < 001_users.js
```

## Compile Rust program

```
cargo build
```

## Run
```
cargo run
```

## Users CRUD

- Create new user

```
curl -X POST \
  -L \
  http://localhost:8086/api/users \
  -H 'Cache-Control: no-cache' \
  -H 'Content-Type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec' \
  -d '{
	"username" : "user_3",
	"password" : "passwordUser3",
	"name" : "User3",
	"surname" : "User3 description",
	"description" : "API user",
	"admin" : "0"
}'
```

- Update user:

```
curl -X PUT \
  -L \
  http://localhost:8086/api/users \
  -H 'Cache-Control: no-cache' \
  -H 'Content-type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec' \
  -d '{
	"username" : "user_3",
	"password" : "passwordUser3",
	"name" : "User3",
	"surname" : "User3 description modified",
	"description" : "API user",
	"admin" : "0",
	"user_id" : "3"
}'
```

- Delete user

```
curl -X DELETE \
  -L \
  http://localhost:8086/api/users \
  -H 'Cache-Control: no-cache' \
  -H 'Content-type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec' \
  -d '{"user_id" : "3"}'
```

- Get user information

```
curl -X GET \
  -L \
  'http://localhost:8086/api/users?user_id=1' \
  -H 'Cache-Control: no-cache' \
  -H 'Content-type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec'
```

- Perform user login

```
curl -X POST \
  -L \
  http://localhost:8086/api/login \
  -H 'Cache-Control: no-cache' \
  -H 'Content-type: application/json' \
  -d '{
	"username" : "api_user",
	"password" : "api_user1234"
}'
```

- Insert new document in MongoDB database

```
curl -X POST \
  -L \
  http://localhost:8086/api/values \
  -H 'Cache-Control: no-cache' \
  -H 'Content-type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec' \
  -d '{
	"key1" : "value1",
	"key2" : "value2",
	"key3" : "value3",
	"key4" : "value4",
	"key5" : "value5",
	"key6" : "value6",
	"key7" : "value7"
}'
```

## Blockchain services

- Single document insertion

```
curl --request POST \
  --url http://localhost:8086/api/values \
  -H 'Cache-Control: no-cache' \
  -H 'Content-type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec' \
  --data '{
    "hash" : "value1",
    "key2" : "value2",
    "key3" : "value3",
    "key4" : "value4",
    "key5" : "value5",
    "key6" : "value6",
    "key7" : "value7"}'
```

- Multiple document insertion

```
curl --request POST \
  --url http://localhost:8086/api/values \
  -H 'Cache-Control: no-cache' \
  -H 'Content-type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec' \
  --data '[{
    "key1" : "value1",
    "key2" : "value2",
    "key3" : "value3",
    "key4" : "value4",
    "key5" : "value5",
    "key6" : "value6",
    "key7" : "value7"
    },
    {
      "key1" : "value1",
      "key2" : "value2",
      "key3" : "value3",
      "key4" : "value4",
      "key5" : "value5",
      "key6" : "value6",
      "key7" : "value7"
    }]'
```

- Get single block by id

```
curl --request GET \
  --url 'http://localhost:8086/api/values?block_id=5dac673a3537345b528984c7&encryption=1' \
  -H 'Content-type: application/json' \
  -H 'Cache-Control: no-cache' \
  -H 'Token: aca6038665c811e8a96100089be8caec'
```

- Full blockchain deletion

```
curl --request DELETE \
  --url 'http://localhost:8086/api/values?blockchain_name=sensors_values' \
  -H 'Content-type: application/json' \
  -H 'Cache-Control: no-cache' \
  -H'Token: aca6038665c811e8a96100089be8caec'
```

- Query by date

```
curl --request PUT \
  --url http://localhost:8086/api/values \
  -H 'Content-type: application/json' \
  -H 'Token: aca6038665c811e8a96100089be8caec' \
  --data '{"docs" : "1","date_from":"2019-05-01 00:00:00","date_to":"2020-05-30 00:00:00"}'
```

## Configuration

Set your connection parameters for MySQL/MariaDB and MongoDB on **.env** file.

ENJOY!
