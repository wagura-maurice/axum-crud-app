// src/models.rs
use serde::{Serialize, Deserialize};
use sqlx::types::Json;
use chrono::NaiveDate;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub telephone: Option<String>,
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub date_of_birth: NaiveDate,
    pub configuration: Option<Json<serde_json::Value>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Profile {
    pub id: String,
    pub user_id: String,
    pub telephone: Option<String>,
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub date_of_birth: NaiveDate,
    pub configuration: Option<Json<serde_json::Value>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct UserRole {
    pub user_id: String,
    pub role_slug: String,
}

#[derive(Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub date_of_birth: NaiveDate,
    pub configuration: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct DashboardStats {
    pub roles: Vec<String>,
    pub users: i32,
    pub orders: i32,
    pub invoices: i32,
}

#[derive(Deserialize)]
pub struct SignUpPayload {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub telephone: Option<String>,
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub date_of_birth: String,
    pub configuration: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct SignInPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateProfilePayload {
    pub username: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub date_of_birth: Option<String>,
    pub configuration: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct SettingsResponse {
    pub theme: String,
    pub language: String,
    pub notifications: bool,
}

#[derive(sqlx::FromRow)]
pub struct Role {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct RoleDetails {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct UserWithRoles {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub roles: Vec<RoleDetails>,
}
