// src/services.rs
use super::*;
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use argon2::{Argon2, PasswordHasher};

pub async fn create_user<'a, E>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    payload: &SignUpPayload,
    hashed_password: &str,
    user_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let date_of_birth = NaiveDate::parse_from_str(&payload.date_of_birth, "%Y-%m-%d")?;

    sqlx::query!(
        r#"INSERT INTO users (
            id,
            username,
            email,
            password,
            telephone,
            salutation,
            first_name,
            middle_name,
            last_name,
            gender,
            address_line_1,
            address_line_2,
            city,
            state,
            country,
            date_of_birth,
            configuration,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
        user_id,
        payload.username,
        payload.email.as_ref(),
        hashed_password,
        payload.telephone.as_ref(),
        payload.salutation.as_ref(),
        payload.first_name.as_ref(),
        payload.middle_name.as_ref(),
        payload.last_name.as_ref(),
        payload.gender.as_ref(),
        payload.address_line_1.as_ref(),
        payload.address_line_2.as_ref(),
        payload.city.as_ref(),
        payload.state.as_ref(),
        payload.country.as_ref(),
        date_of_birth,
        payload.configuration.as_ref(),
        Utc::now().naive_utc(),
        Utc::now().naive_utc(),
    )
    .execute(tx)
    .await?;

    Ok(())
}

pub async fn create_profile(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    payload: &SignUpPayload,
    user_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"INSERT INTO profiles (
            id,
            user_id,
            telephone,
            salutation,
            first_name,
            middle_name,
            last_name,
            gender,
            address_line_1,
            address_line_2,
            city,
            state,
            country,
            date_of_birth,
            configuration,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"#,
        Uuid::new_v4().to_string(),
        user_id,
        payload.telephone.as_ref(),
        payload.salutation.as_ref(),
        payload.first_name.as_ref(),
        payload.middle_name.as_ref(),
        payload.last_name.as_ref(),
        payload.gender.as_ref(),
        payload.address_line_1.as_ref(),
        payload.address_line_2.as_ref(),
        payload.city.as_ref(),
        payload.state.as_ref(),
        payload.country.as_ref(),
        NaiveDate::parse_from_str(&payload.date_of_birth, "%Y-%m-%d")?,
        payload.configuration.as_ref(),
        Utc::now().naive_utc(),
        Utc::now().naive_utc(),
    )
    .execute(tx)
    .await?;

    Ok(())
}

pub async fn assign_role(
    user_id: &str,
    role_slug: &str,
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"INSERT INTO users_roles (user_id, role_slug)
         VALUES ($1, $2)"#,
        user_id,
        role_slug,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn fetch_user_profile(
    pool: &PgPool,
    user_id: &str,
) -> Result<UserProfile, Box<dyn std::error::Error>> {
    let profile = sqlx::query_as!(
        UserProfile,
        r#"SELECT u.id, u.username, u.email, p.telephone, p.salutation, p.first_name, p.middle_name, p.last_name, p.gender, p.address_line_1, p.address_line_2, p.city, p.state, p.country, p.date_of_birth, p.configuration FROM users u
        LEFT JOIN profiles p ON u.id = p.user_id
        WHERE u.id = $1"#,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(profile)
}

pub async fn update_user_profile(
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

pub async fn fetch_user_settings(
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

pub async fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, argon2::Params::default());
    argon2.hash_password(password.as_bytes(), &argon2::Salt::generate(argon2::salt_size!()))
        .map_err(Into::into)
        .map(|hash_result| hash_result.to_string())
}

pub async fn verify_password(password: &str, hashed_password: &str) -> Result<bool, argon2::Error> {
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, argon2::Params::default());
    argon2.verify_password(password.as_bytes(), &argon2::PasswordHash::from_string(hashed_password)?)
        .map_err(Into::into)
        .map(|is_valid| is_valid)
}
