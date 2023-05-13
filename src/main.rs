use actix_web::{
    dev, get, http::{header, StatusCode}, middleware::{Logger, ErrorHandlers, ErrorHandlerResponse}, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder, Result
};

use actix_cors::Cors;
use dotenv::dotenv;
use serde_json::json;
use std::sync::Mutex;
// use chrono::{Local};

mod api;
mod models {
    pub mod entry_model;
}
use api::services;
use models::entry_model::TodolistEntry;

struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>,
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

async fn error_handler(_req: HttpRequest) -> impl Responder {
    let error_message = "Oops! Página no encontrada.";
    let error_response = json!({ "error": error_message });
    HttpResponse::NotFound().json(error_response)
}

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_message = "Oops! Página no encontrada.";
    let error_response = json!({ "error": error_message });
    // Destructures ServiceResponse into request and response components
    let (req, res) = res.into_parts();
    // Create a new response with the modified body
    let res = res.set_body(error_message).map_into_boxed_body();
    // Create a new ServiceResponse with the modified response
    let res = dev::ServiceResponse::new(req, res).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

fn handle_bad_request<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut()
    .headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Bad Request Error"),
    );
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

fn add_error_body<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_message = "Oops! Página no encontrada.";
    let error_response = json!({ "error": error_message });
    // Destructures ServiceResponse into request and response components
    let (req, res) = res.into_parts();
    // Create a new response with the modified body
    let res = res.set_body(error_message).map_into_boxed_body();
    // Create a new ServiceResponse with the modified response
    let res = dev::ServiceResponse::new(req, res).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
 }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "actix_web=info");
    // }

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_any_origin()
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
            .wrap(cors)
            .wrap(Logger::default())
            // .default_service(web::route().to(error_handler))
            .wrap(
                ErrorHandlers::new()
                .default_handler_client(add_error_body) // or .default_handler_server
                // .handler(StatusCode::NOT_FOUND, handle_bad_request)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
