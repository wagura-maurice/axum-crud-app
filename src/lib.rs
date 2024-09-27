// src/lib.rs
use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use sqlx::sqlite::SqlitePool;  // Use SqlitePool instead of PgPool
use dotenv::dotenv;
use std::env;

mod controllers;
mod models;
mod services;
mod utils;

// Define the environment interface for D1 binding
pub interface Env {
    DB: D1Database;  // This refers to the D1Database binding set in wrangler.toml
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // For local development, use a DATABASE_URL from the environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://cloudflare-d1-database".to_string());

    // Connect to SQLite (D1 database)
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create SQLite pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::default())
            .configure(controllers::config)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
