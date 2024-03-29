use crate::schema::{config_holidays, user_login};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use std::default::Default;

//Object structure for  Querying and inserting users
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user_login"]
pub struct Users {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub facebookconnect: Option<String>,
    pub googleconnect: Option<String>,
    pub password: String,
    pub ip_address: String,
    pub isactive: Option<bool>,
    pub sort_order: Option<i64>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct NUsers {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Text"]
    pub firstname: String,
    #[sql_type = "Text"]
    pub lastname: String,
    #[sql_type = "Text"]
    pub username: String,
    #[sql_type = "Text"]
    pub email: String,
    #[sql_type = "Text"]
    pub mobile: String,
    #[sql_type = "Nullable<Text>"]
    pub facebookconnect: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub googleconnect: Option<String>,
    #[sql_type = "Text"]
    #[serde(skip_serializing)]
    pub password: String,
    #[sql_type = "Text"]
    pub ip_address: String,
    #[sql_type = "Nullable<Bool>"]
    pub isactive: Option<bool>,
    #[sql_type = "Nullable<BigInt>"]
    pub sort_order: Option<i64>,
    #[sql_type = "Nullable<Timestamp>"]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[sql_type = "Nullable<Text>"]
    pub created_by: Option<String>,
    #[sql_type = "Nullable<Timestamp>"]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[sql_type = "Nullable<Text>"]
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Default)]
#[table_name = "user_login"]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub facebookconnect: Option<String>,
    pub googleconnect: Option<String>,
    pub password: String,
    pub ip_address: String,
    pub isactive: Option<bool>,
    pub sort_order: Option<i64>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logindata {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, QueryableByName, Clone)]
#[table_name = "config_holidays"]
pub struct Holidays {
    pub id: i64,
    pub holiday_date: String,
    pub holiday_desc: String,
    pub createdat: Option<chrono::NaiveDateTime>,
    pub updatedat: Option<chrono::NaiveDateTime>,
}

//Structure for Login Response
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: Msg,
    pub data: Tokendata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub code: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokendata {
    pub name: String,
    pub token: String,
}

//Methods for Response
impl Response {
    pub fn new() -> Response {
        Response {
            success: false,
            message: Msg {
                code: "".to_string(),
                description: "".to_string(),
            },
            data: Tokendata {
                name: "".to_string(),
                token: "".to_string(),
            },
        }
    }
}

//Object structure for JWT claims
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub username: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Year {
    pub year: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lambdastruct {
    pub success: bool,
    pub result: Vec<Option<String>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Dynamouserid {
    pub id: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Messagedynamo {
    #[serde(rename = "ID")]
    pub id: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
#[table_name = "config_holidays"]

pub struct Medium {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Text"]
    pub firstname: String,
    #[sql_type = "Text"]
    pub lastname: String,
    #[sql_type = "Nullable<Text>"]
    pub email: Option<String>,
    #[sql_type = "Text"]
    pub mobile: String,
    #[sql_type = "Text"]
    #[serde(skip_serializing)]
    pub password: String,
    #[sql_type = "Nullable<Bool>"]
    pub isactive: Option<bool>,
    #[sql_type = "Nullable<BigInt>"]
    pub sort_order: Option<i64>,
    #[sql_type = "Nullable<Timestamp>"]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[sql_type = "Nullable<Text>"]
    pub created_by: Option<String>,
}
