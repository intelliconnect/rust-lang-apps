use crate::model::structs;
use crate::vars;
use chrono::prelude::*;
use diesel::PgConnection;
use diesel::{prelude::*, sql_query};
use dotenv::dotenv;
use easy_password::bcrypt;
use failure::{format_err, Error};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rusoto_core::{ByteStream, Region, RusotoError};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput};
use rusoto_s3::{PutObjectError, PutObjectOutput, S3, S3Client};
use std::collections::HashMap;

//TODO; implement r2d2 for connection pool
// Error handling
//Create new database connection and return
pub fn getdbconn() -> PgConnection {
    dotenv().ok();
    let database_url = vars::db_url();
    PgConnection::establish(&database_url).unwrap()
}

//Query database for username
pub fn fetch_user(jusername: &str) -> std::vec::Vec<structs::Users> {
    use crate::schema::user_login::dsl::*;

    //create new databse connection
    let connection = getdbconn();

    let results = user_login
        .filter(username.eq(jusername))
        .load::<structs::Users>(&connection)
        .unwrap();
    // return results
    results
}

//Create New JWT
pub fn create_jwt(uid: &str) -> String {
    //Set token to expire after 60 days
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(60))
        .unwrap()
        .timestamp();

    //Set claims for JWT
    let claims = structs::Claims {
        username: uid.to_owned(),
        exp: expiration as usize,
    };

    //Set header(algorithm) for JWT
    let header = Header::new(Algorithm::HS512);

    //Encode Token by using header,claims and secret key(from env variables) and return token
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(vars::jwt_sc().as_ref()),
    )
    .expect("JWT creation failed");
    token
}

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<structs::Claims>> {
    let sc_key = vars::jwt_sc();
    jsonwebtoken::decode::<structs::Claims>(
        &token,
        &DecodingKey::from_secret(sc_key.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
}

pub fn fetch_holidays(year: &str) -> std::vec::Vec<structs::Holidays> {
    //create new databse connection
    let connection: PgConnection = getdbconn();

    //Query database
    match sql_query(format!(
        "SELECT * FROM config_holidays WHERE holiday_date LIKE '%{}%'",
        year
    ))
    .load(&connection)
    {
        Ok(result) => return result,
        Err(err) => {
            error!("sql query failed");
            panic!("sql query failed {:?}", err);
        }
    }
}

pub fn create_user(jsondata: structs::NewUser) -> Result<bool, actix_web::error::Error> {
    use crate::schema::user_login::dsl::user_login;
    let connection = getdbconn();
    let mut jsondata = jsondata;
    let encrypted_password = hash_pass(&jsondata.password);
    jsondata.password = encrypted_password;

    let results = diesel::insert_into(user_login)
        .values(&jsondata)
        .get_results::<structs::Users>(&connection);
    return Ok(results.is_ok());
}

pub fn hash_pass(pass_string: &str) -> String {
    let pass_secretkey = vars::get_pass_sc();

    let hash = bcrypt::hash_password(pass_string, pass_secretkey.as_ref(), 12).unwrap();
    hash
}

pub fn varify_pass(login_pass: &str, hash_pass: &str) -> bool {
    let pass_secretkey = vars::get_pass_sc();
    return bcrypt::verify_password(login_pass, hash_pass, pass_secretkey.as_ref()).unwrap();
}

#[tokio::main]
pub async fn send_to_s3(bst: ByteStream, filename: String) -> Result<PutObjectOutput,RusotoError<PutObjectError>> {
    let put_request = rusoto_s3::PutObjectRequest {
        bucket: "elastic-search-bucket-test".to_owned(),
        key: filename.clone(),
        body: Some(bst),
        ..Default::default()
    };

    let client = S3Client::new(Region::ApSoutheast1);

    match client.put_object(put_request).await {
        Ok(response) => {return Ok(response)},
        Err(err) => {return Err(err);},
    };
}

#[tokio::main]
pub async fn list_data(user_id: String) -> Result<Vec<structs::Messagedynamo>, Error> {
    let client = DynamoDbClient::new(Region::UsEast2);
    let expression = "ID =:id".to_string();
    let mut values = HashMap::new();
    values.insert(":id".into(), string_to_attr(user_id.to_string()));
    let query = QueryInput {
        table_name: "rusttest".into(),
        key_condition_expression: Some(expression),
        expression_attribute_values: Some(values),
        ..Default::default()
    };
    match client.query(query).await {
        Ok(data) => {
            let msg = data
                .items
                .ok_or_else(|| format_err!("No items found in dynamodb"))?;

            let mut messages = Vec::new();

            let id1 = msg[0]
                .get("ID")
                .ok_or_else(|| format_err!("error"))
                .and_then(attr_to_string)?;
            let msg = msg[0]
                .get("message")
                .ok_or_else(|| format_err!("error"))
                .and_then(attr_to_string)?;
            let message = structs::Messagedynamo {
                id: id1,
                message: msg,
            };
            messages.push(message);

            Ok(messages)
        }
        Err(err) => return Err(format_err!("{:?}", err)),
    }
}

pub fn string_to_attr(s: String) -> AttributeValue {
    AttributeValue {
        s: Some(s),
        ..Default::default()
    }
}

pub fn attr_to_string(attr: &AttributeValue) -> Result<String, Error> {
    if let Some(value) = &attr.s {
        Ok(value.to_string())
    } else {
        Err(format_err!("no string value"))
    }
}