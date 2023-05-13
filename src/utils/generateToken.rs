use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use jsonwebtoken::EncodingKey;

pub(crate) fn generate_token(username: &str) -> String {
    let claims = json!({
        "sub": username,
        "iat": 1600000000,
        "exp": 1900000000,
    });
    let key = EncodingKey::from_secret("secret".as_ref());
    encode(&Header::default(), &claims, &key).unwrap()
}

pub(crate) async fn protected() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Protected resource",
    }))
}