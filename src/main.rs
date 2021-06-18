#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate easy_password;

mod controller;
mod middleware;
mod model;
mod routes;
mod schema;
#[cfg(test)]
mod test;
mod vars;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use middleware::auth;

use crate::model::dbmethods::string_to_attr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variables
    dotenv().ok();
    //Initiate Logger
    env_logger::init();
    info!("starting server");

    let s = String::from("hello");
    let a = string_to_attr(s);
    println!("{:#?}", a);

    //Configure and Start New HTTP server
    HttpServer::new(move || {
        //Cors::permissive() should only be used development purposes, not to be used in production
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(auth::Auth)
            .wrap(actix_web::middleware::Logger::default())
            .configure(routes::dieselr::configure)
            .configure(routes::awsr::configure)
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
