use dotenv::dotenv;
use std::env::var;

//Return database URL
pub fn db_url() -> String {
    dotenv().ok();
    var("DATABASE_URL").unwrap()}
//Return domain
pub fn domain() -> String {
    dotenv().ok();
    var("DOMAIN").unwrap_or("0.0.0.0".to_string())
}
//return PORT
pub fn port() -> u16 {
    dotenv().ok();
    var("PORT")
        .unwrap_or("9000".to_string())
        .parse::<u16>()
        .unwrap()
}

//Return Secret key for JWT
pub fn jwt_sc() -> String {
    dotenv().ok();
    var("JWT_SECRET")
        .unwrap()
}

pub fn get_pass_sc() -> String {
    dotenv().ok();
    var("PASS_SECRET")
        .unwrap()
}

