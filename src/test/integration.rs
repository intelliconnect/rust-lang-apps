use crate::controller::home;
use crate::model::structs;
use actix_web::{test, web, App};

#[actix_rt::test]
async fn test_index_post() {
    let data = structs::Logindata {
        username: "kaustubhbabar".to_string(),
        password: "1313n218u41".to_string(),
    };

    let mut app = test::init_service(App::new().route("/login", web::post().to(home::login))).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&data)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_register_user() {
    let data = structs::NewUser {
        firstname: "Kaustubh".to_string(),
        lastname: "Babar".to_string(),
        username: "kaustubhbabar".to_string(),
        email: "kaustubhbabar@gmail.com".to_string(),
        mobile: "123456789".to_string(),
        password: "1313n218u41".to_string(),
        ip_address: "124.245.55.124".to_string(),
        ..Default::default()
    };

    let mut app =
        test::init_service(App::new().route("/register_user", web::post().to(home::register_user)))
            .await;

    let req = test::TestRequest::post()
        .uri("/register_user")
        .set_json(&data)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}
