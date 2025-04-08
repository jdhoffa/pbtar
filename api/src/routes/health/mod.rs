use actix_web::{get, HttpResponse, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    name: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: "Climate Scenarios Database API".to_string(),
    };
    
    HttpResponse::Ok().json(response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}