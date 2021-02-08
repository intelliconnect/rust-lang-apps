# RUST Actix-Web Microservice


### Description:
-   ##### Register with user details
-   ##### Login with username and password.Get verified and receive JWT token
-   ##### Bcrypt for password hashing
-   ##### JWT token Based Authontication
-   ##### Postgres for user database

-   ##### API Endpoints:
    ###### Registering new user
    ###### Login
    ###### Fetching data from Postgres database

## Dependencies:

Here's what does what:
Crate | Description
:---------|:--------
[actix-web](https://github.com/actix/actix-web)|Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
[Diesel](https://diesel.rs)|Diesel is a Safe, Extensible ORM and Query Builder for Rust
[Serde](https://crates.io/crates/serde)|Serde is a framework for serializing and deserializing Rust data structures
[dotenv](https://crates.io/crates/dotenv)|Required for loading environment variables from .env file
[env_logger](https://crates.io/crates/env_logger)|Implements a logger that can be configured via environment variables.
[jsonwebtoken](https://crates.io/crates/jsonwebtoken)|To Create and parses JWT (JSON Web Tokens)
[http](https://crates.io/crates/http)|A general purpose library of common HTTP types



## Run locally

> Before you get started, make sure that you have [PostgreSQL](https://postgresql.org), [Rust](https://rust-lang.org), [Cargo](https://doc.rust-lang.org/cargo/), and the [Diesel](https://diesel.rs) CLI installed and that you have Postgres running somewhere.

```bash
# Fetch the repo
git clone https://github.com/intelliconnect/rust-lang-apps.git


# Add environment variables to .env file.
nano .env

diesel setup
diesel migration run

cargo check
cargo run # could take a while!
```


# API Endpoints
## 1)Register
```
curl -i --request POST \
  --url http://0.0.0.0:9000/register_user \
  --header 'content-type: application/json' \
  --data '{
        "firstname":"abc",
        "lastname":"bbq",
        "username":"admin",
        "email":"admin@gmail.com",
        "mobile":"123456789",
        "password":"1313n218u41",
        "ip_address":"124.245.55.124",
        "isactive":true
}'
```

## 2)Login
```
curl -i --request POST \
  --url http://0.0.0.0:9000/login \
  --header 'content-type: application/json' \
  --data '{ "username":"admin","password":"1313n218u41"}'
```

## 3)View Holidays
```
curl -i --request GET \
  --url http://0.0.0.0:9000/view_holidays \
  --header 'content-type: application/json' \
  --header 'Authorization: Bearer <token>' \
  --data '{ "year": "2020" }'
```

## Future TODOs 
- Error handling
- Connection Pooling for Rust/Diesel






