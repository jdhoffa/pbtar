use actix_cors::Cors;
use actix_web::{
    http, middleware, web, App, HttpServer,
};
use dotenv::dotenv;
use log::{info, error};
use sqlx::postgres::PgPoolOptions;
use std::{env, time::Duration};

mod models;
mod routes;
mod config;
mod db;
mod errors;

async fn connect_to_db_with_retry(database_url: &str, max_retries: u32) -> Result<sqlx::PgPool, sqlx::Error> {
    let mut retries = 0;
    let retry_delay = Duration::from_secs(5);

    loop {
        match PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
        {
            Ok(pool) => {
                info!("Successfully connected to database");
                return Ok(pool);
            },
            Err(err) => {
                retries += 1;
                if retries >= max_retries {
                    error!("Failed to connect to database after {} retries: {}", max_retries, err);
                    return Err(err);
                }
                error!("Failed to connect to database (attempt {}/{}): {}. Retrying in {} seconds...", 
                       retries, max_retries, err, retry_delay.as_secs());
                tokio::time::sleep(retry_delay).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    
    // Load database configuration from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");
    
    // Create a database connection pool with retry mechanism
    let db_pool = match connect_to_db_with_retry(&database_url, 10).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create database connection pool: {}", e);
            std::process::exit(1);
        }
    };

    // Set up server with database connection pool
    info!("Starting server at http://0.0.0.0:8080");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
            
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
