#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate easy_password;

mod controller;
mod middleware;
mod model;
mod schema;
#[cfg(test)]
mod test;
mod vars;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use controller::{awsc, home};
use middleware::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variables
    dotenv().ok();
    //Initiate Logger
    env_logger::init();
    info!("starting server");
    //Configure and Start New HTTP server
    HttpServer::new(move || {
        //Cors::permissive() should only be used development purposes, not to be used in production
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(auth::Auth)
            .wrap(actix_web::middleware::Logger::default())
            .route("/login", web::post().to(home::login))
            .route("/view_holidays", web::get().to(home::fetch_holidays))
            .route("/register_user", web::post().to(home::register_user))
            .route("/list_users", web::get().to(home::list_users))
            .route(
                "/lambda_example",
                web::get().to(awsc::lambda_example_synchronus),
            )
            .route("/upload_file", web::post().to(awsc::upload_file))
            .route("/dynamodb_example", web::get().to(awsc::dynamodb_example))
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
