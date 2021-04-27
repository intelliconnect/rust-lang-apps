use crate::model::{dbmethods, structs};
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = Authmiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(Authmiddleware { service })
    }
}

pub struct Authmiddleware<S> {
    service: S,
}

impl<S, B> Service for Authmiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        //println!("{:?}",req.uri());
        let mut token_verification = false;

        //Skip middleware for this login API
        if req.uri().to_string() == *"/login" || req.uri().to_string() == *"/register_user" {
            token_verification = true
        }

        //Check if token is present
        //If yes,Decode and Validate token
        //Insert username from token into request header
        if let Some(token) = req.headers().get("AUTHORIZATION") {
            if let Ok(token_str) = token.to_str() {
                debug!("Token converted to string");
                if token_str.starts_with("bearer") || token_str.starts_with("Bearer") {
                    let token = token_str[6..token_str.len()].trim();
                    let decode_response = dbmethods::decode_token(token.to_string());
                    if decode_response.is_ok() {
                        let token_data = decode_response.unwrap();
                        let username = token_data.claims.username;
                        req.headers_mut().insert(
                            header::HeaderName::from_static("token_username"),
                            header::HeaderValue::from_str(&username).unwrap(),
                        );
                        // println!("{:?}",req.headers());
                        token_verification = true
                    }
                }
            }
        }
        // Continue if Token Verification is complete
        //If not, Stop and respond to request
        if token_verification {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            let mut rbody = structs::Response::new();
            rbody.success = false;
            rbody.message.code = "UA101".to_string();
            rbody.message.description = "Token Validation failed".to_string();

            Box::pin(async move {
                Ok(req.into_response(HttpResponse::Unauthorized().json(rbody).into_body()))
            })
        }
    }
}
