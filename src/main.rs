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

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use controller::{home};
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
        App::new()
            .wrap(auth::Auth)
            .wrap(actix_web::middleware::Logger::default())
            .route("/login", web::post().to(home::login))
            .route("/view_holidays", web::get().to(home::fetch_holidays))
            .route("/register_user", web::post().to(home::register_user))
            
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
