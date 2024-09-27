// src/controllers.rs
use super::*;
use sqlx::{PgPool};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize)]
pub struct DashboardStats {
    roles: Vec<String>,
    users: i32,
    orders: i32,
    invoices: i32,
}

#[derive(Serialize)]
pub struct UserProfileDetails {
    username: String,
    email: String,
    telephone: Option<String>,
    salutation: Option<String>,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    gender: Option<String>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    date_of_birth: String,
    configuration: Option<serde_json::Value>,
    roles: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateProfilePayload {
    username: Option<String>,
    email: Option<String>,
    telephone: Option<String>,
    salutation: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    gender: Option<String>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    date_of_birth: Option<String>,
    configuration: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct SettingsResponse {
    theme: String,
    language: String,
    notifications: bool,
}

pub async fn dashboard(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let stats = match fetch_dashboard_stats(&state.pool).await {
        Ok(stats) => stats,
        Err(_) => return error_response("Failed to fetch dashboard stats", StatusCode::INTERNAL_SERVER_ERROR),
    };

    success_response(stats, "Dashboard stats retrieved successfully", StatusCode::OK)
}

pub async fn user_profile(
    Extension(state): Extension<Arc<AppState>>,
    TypedHeader(token): TypedHeader<Bearer>,
) -> impl IntoResponse {
    let decoded_token = match decode::<Claims>(
        &token.token(),
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(decoded) => decoded.claims.sub,
        Err(_) => return error_response("Invalid token", StatusCode::UNAUTHORIZED).into_response(),
    };

    let profile = match services::fetch_user_profile(&state.pool, &decoded_token).await {
        Ok(profile) => profile,
        Err(_) => return error_response("Failed to fetch user profile", StatusCode::INTERNAL_SERVER_ERROR),
    };

    success_response(profile, "User profile retrieved successfully", StatusCode::OK)
}

pub async fn update_profile(
    Extension(state): Extension<Arc<AppState>>,
    TypedHeader(token): TypedHeader<Bearer>,
    Json(payload): Json<UpdateProfilePayload>,
) -> impl IntoResponse {
    let decoded_token = match decode::<Claims>(
        &token.token(),
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(decoded) => decoded.claims.sub,
        Err(_) => return error_response("Invalid token", StatusCode::UNAUTHORIZED).into_response(),
    };

    if let Err(e) = update_user_profile(&state.pool, &decoded_token, &payload).await {
        return error_response(&format!("Failed to update profile: {}", e), StatusCode::INTERNAL_SERVER_ERROR);
    }

    success_response(None, "User profile updated successfully", StatusCode::OK)
}

pub async fn settings(
    Extension(state): Extension<Arc<AppState>>,
    TypedHeader(token): TypedHeader<Bearer>,
) -> impl IntoResponse {
    let decoded_token = match decode::<Claims>(
        &token.token(),
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(decoded) => decoded.claims.sub,
        Err(_) => return error_response("Invalid token", StatusCode::UNAUTHORIZED).into_response(),
    };

    let settings = match fetch_user_settings(&state.pool, &decoded_token).await {
        Ok(settings) => settings,
        Err(_) => return error_response("Failed to fetch user settings", StatusCode::INTERNAL_SERVER_ERROR),
    };

    success_response(settings, "User settings retrieved successfully", StatusCode::OK)
}

async fn fetch_dashboard_stats(pool: &PgPool) -> Result<DashboardStats, Box<dyn std::error::Error>> {
    let roles = sqlx::query!(
        r#"SELECT DISTINCT slug FROM roles"#,
        pool
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|role| role.slug)
    .collect();

    let users_count = sqlx::query!(
        r#"SELECT COUNT(*) FROM users"#,
        pool
    )
    .fetch_one(pool)
    .await?.count.unwrap_or(0);

    let orders_count = sqlx::query!(
        r#"SELECT COUNT(*) FROM orders"#,
        pool
    )
    .fetch_one(pool)
    .await?.count.unwrap_or(0);

    let invoices_count = sqlx::query!(
        r#"SELECT COUNT(*) FROM invoices"#,
        pool
    )
    .fetch_one(pool)
    .await?.count.unwrap_or(0);

    Ok(DashboardStats {
        roles,
        users: users_count,
        orders: orders_count,
        invoices: invoices_count,
    })
}

async fn update_user_profile(
    pool: &PgPool,
    user_id: &str,
    payload: &UpdateProfilePayload,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut query = String::from("UPDATE users SET ");
    let mut values = Vec::new();
    let mut params = Vec::new();

    if let Some(username) = &payload.username {
        query.push_str("username = $1, ");
        values.push(username);
        params.push(sqlx::types::Json(&username));
    }

    if let Some(email) = &payload.email {
        query.push_str("email = $2, ");
        values.push(email);
        params.push(sqlx::types::Json(&email));
    }

    // Add more fields as needed...

    query.pop(); // Remove trailing comma and space
    query.push_str(" WHERE id = $3");

    values.push(user_id);
    params.push(sqlx::types::Json(&user_id));

    sqlx::query_as!(models::User, &query, &params)
        .execute(pool)
        .await?;

    Ok(())
}

async fn fetch_user_settings(
    pool: &PgPool,
    user_id: &str,
) -> Result<SettingsResponse, Box<dyn std::error::Error>> {
    let settings = sqlx::query_as!(
        SettingsResponse,
        r#"SELECT theme, language, notifications FROM user_settings WHERE user_id = $1"#,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(settings)
}
