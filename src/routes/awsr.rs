use actix_web::web::{self, ServiceConfig};

use crate::controller::awsc;

pub fn configure(app: &mut ServiceConfig) {
    app.route(
        "/lambda_example",
        web::get().to(awsc::lambda_example_synchronus),
    )
    .route("/upload_file", web::post().to(awsc::upload_file))
    .route("/dynamodb_example", web::get().to(awsc::dynamodb_example));
}
