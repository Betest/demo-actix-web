use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, Value};

use crate::models::userModel::User;

use super::generateToken::generate_token;

pub(crate) async fn login(user: web::Json<User>) -> impl Responder {
    // validar las credenciales del usuario en una base de datos o servicio externo
    let valid = user.username == "usuario" && user.password == "contraseña";

    if valid {
        let token = generate_token(&user.username);

        HttpResponse::Ok().json(json!({
            "message": "Login successful",
            "token": token,
        }))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "Invalid username or password",
        }))
    }
}