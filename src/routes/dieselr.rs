use actix_web::web::{self, ServiceConfig};

use crate::controller::home;

pub fn configure(app: &mut ServiceConfig) {
    app.route("/login", web::post().to(home::login))
        .route("/view_holidays", web::get().to(home::fetch_holidays))
        .route("/register_user", web::post().to(home::register_user))
        .route("/list_users", web::get().to(home::list_users));
}
