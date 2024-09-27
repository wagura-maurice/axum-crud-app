// src/utils.rs
use actix_web::{web, HttpResponse, ResponseError};
use actix_web::http::header::{AUTHORIZATION, BEARER};
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorBadRequest;
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Duration, Utc};
use sqlx::{PgPool};
use uuid::Uuid;
use lazy_static::lazy_static;
use dotenv::dotenv;
use std::env;

// Load environment variables
lazy_static! {
    static ref SECRET_KEY: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    static ref JWT_EXPIRY: i64 = env::var("JWT_EXPIRY")
        .unwrap_or_else(|_| "3600".to_string()) // Default to 1 hour if not set
        .parse()
        .expect("JWT_EXPIRY must be a valid integer");
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

impl Claims {
    fn new(user_id: &str) -> Self {
        Claims {
            sub: user_id.to_string(),
            exp: Utc::now().timestamp() + *JWT_EXPIRY,
        }
    }
}

pub type Bearer = String;

pub fn extract_bearer(req: ServiceRequest) -> impl Future<Item=Bearer, Error=ErrorBadRequest> {
    let auth_header = req.headers().get(AUTHORIZATION);
    match auth_header {
        Some(auth_header) => {
            let auth_header = auth_header.to_str().unwrap();
            if auth_header.starts_with(BEARER) {
                let bearer_token = auth_header.trim_start_matches(BEARER).trim();
                ok(bearer_token.to_owned())
            } else {
                err(ErrorBadRequest("Invalid Authorization header"))
            }
        },
        None => err(ErrorBadRequest("Authorization header missing")),
    }
}

pub fn generate_jwt(user_id: &str) -> String {
    let claims = Claims::new(user_id);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY.as_ref())).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::ErrorKind> {
    let secret_key = EncodingKey::from_secret(SECKET_KEY.as_ref());
    jsonwebtoken::decode::<Claims>(token, &secret_key, &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256))
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub async fn fetch_user_by_id(pool: &PgPool, user_id: &str) -> Result<models::User, Box<dyn std::error::Error>> {
    let user = sqlx::query_as!(
        models::User,
        r#"SELECT * FROM users WHERE id = $1"#,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub fn success_response<T>(data: Option<T>, message: &str, status_code: actix_web::http::StatusCode) -> HttpResponse
where
    T: Serialize,
{
    HttpResponse::build(status_code)
        .json(json!({
            "status": "success",
            "message": message,
            "data": data
        }))
}

pub fn error_response(message: &str, status_code: actix_web::http::StatusCode) -> HttpResponse {
    HttpResponse::build(status_code)
        .json(json!({
            "status": "error",
            "message": message,
            "data": null
        }))
}

// Custom error types and implementations
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Jwt(jsonwebtoken::errors::ErrorKind),
    Validation(String),
    InternalServerError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}

impl From<jsonwebtoken::errors::ErrorKind> for AppError {
    fn from(e: jsonwebtoken::errors::ErrorKind) -> Self {
        AppError::Jwt(e)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Jwt(_) => StatusCode::UNAUTHORIZED,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({
                "status": "error",
                "message": format!("{:?}", self),
                "data": null
            }))
    }
}
