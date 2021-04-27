use crate::model::{dbmethods, structs};
use actix_web::{web, HttpResponse};

//METHOD=POST, PATH="/login"
pub async fn login(jsondata: web::Json<structs::Logindata>) -> HttpResponse {
    //Declare  new Response Object
    let mut rbody = structs::Response::new();
    if jsondata.username.to_string().is_empty() {
        rbody.message.code = "UA101".to_string();
        rbody.message.description = "empty username".to_string();
        return HttpResponse::NotAcceptable().json(&rbody);
    }

    //Query database for username
    let results = dbmethods::fetch_user(&jsondata.username);

    //Check if Query results are empty,if not validate password
    //Create token, modify response object and return Response object
    if results.is_empty() {
        rbody.message.code = "UA101".to_string();
        rbody.message.description = "Invalid User/Password Combination".to_string();
        return HttpResponse::NotFound().json(&rbody);
    } else if dbmethods::varify_pass(&jsondata.password, &results[0].password) {
        let token = dbmethods::create_jwt(&results[0].username.to_string());

        rbody.success = true;
        rbody.message.code = "200".to_string();
        rbody.message.description = "Success".to_string();
        rbody.data.token = token;
        return HttpResponse::Ok().json(&rbody);
    } else {
        rbody.message.code = "UA101".to_string();
        rbody.message.description = "Invalid User/Password Combination".to_string();
        HttpResponse::NotFound().json(&rbody)
    }
}

pub async fn fetch_holidays(jsondata: web::Json<structs::Year>) -> HttpResponse {
    if jsondata.year.is_empty() {
        let mut rbody = structs::Response::new();
        rbody.success = false;
        rbody.message.description = "Year is empty".to_string();
        return HttpResponse::Ok().json(&rbody);
    }

    //Query database for holidays
    let results = dbmethods::fetch_holidays(&jsondata.year);

    //check if results are empty
    if results.is_empty() {
        let mut rbody = structs::Response::new();
        rbody.success = false;
        rbody.message.description = "No holidays for this year found".to_string();
        return HttpResponse::Ok().json(&rbody);
    }

    //Respond with Results
    HttpResponse::Ok().json(&results)
}

pub async fn register_user(jsondata: web::Json<structs::NewUser>) -> HttpResponse {
    //Declare Body of Response
    let mut rbody = structs::Response::new();

    //Convert JSON Request type from "web::Json<structs::Year>" to "Year"
    let new_user = jsondata.into_inner();

    //fetch user details from database
    let temp_user = dbmethods::fetch_user(&new_user.username);

    //check if results are empty(user already exists in database), if yes then set response as user already exists
    if !temp_user.is_empty() {
        rbody.message.description = "User already exists".to_string();
        return HttpResponse::Ok().json(&rbody);
    }

    //if not create new user
    let result = dbmethods::create_user(new_user);

    //check whether user created or not and respond
    if result.is_ok() {
        rbody.success = false;
        rbody.message.description = "User created successfully".to_string();
        return HttpResponse::Ok().json(&rbody);
    } else {
        rbody.message.description = "User creation failed".to_string();
        return HttpResponse::Ok().json(&rbody);
    }
}
