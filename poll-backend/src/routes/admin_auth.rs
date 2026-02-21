use axum::{extract::State, http::HeaderMap, http::StatusCode, Json};
use sqlx::PgPool;

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use rand_core::OsRng;
use uuid::Uuid;

use crate::auth::require_admin;
use crate::models::{AdminAuthRequest, AdminAuthResponse, LogoutResponse};

#[derive(sqlx::FromRow)]
struct AdminRow {
    id: String,
    name: String,
    password_hash: String,
}

pub async fn signup_admin(
    State(pool): State<PgPool>,
    Json(payload): Json<AdminAuthRequest>,
) -> Result<Json<AdminAuthResponse>, StatusCode> {
    let name = payload.name.trim();
    if name.is_empty() || payload.password.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let admin_id = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let admin_insert = sqlx::query(
        "INSERT INTO admins (id, name, password_hash, created_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(&admin_id)
    .bind(name)
    .bind(&password_hash)
    .bind(&now)
    .execute(&mut *tx)
    .await;

    if let Err(err) = admin_insert {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.is_unique_violation() {
                return Err(StatusCode::CONFLICT);
            }
        }
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    sqlx::query("INSERT INTO admin_sessions (id, admin_id, created_at) VALUES ($1, $2, $3)")
        .bind(&token)
        .bind(&admin_id)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AdminAuthResponse {
        token,
        admin_id,
        name: name.to_string(),
    }))
}

pub async fn login_admin(
    State(pool): State<PgPool>,
    Json(payload): Json<AdminAuthRequest>,
) -> Result<Json<AdminAuthResponse>, StatusCode> {
    let name = payload.name.trim();
    if name.is_empty() || payload.password.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let admin = sqlx::query_as::<_, AdminRow>(
        "SELECT id, name, password_hash FROM admins WHERE name = $1",
    )
    .bind(name)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let parsed_hash = PasswordHash::new(&admin.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query("INSERT INTO admin_sessions (id, admin_id, created_at) VALUES ($1, $2, $3)")
        .bind(&token)
        .bind(&admin.id)
        .bind(&now)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AdminAuthResponse {
        token,
        admin_id: admin.id,
        name: admin.name,
    }))
}

pub async fn logout_admin(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<Json<LogoutResponse>, StatusCode> {
    let admin = require_admin(&pool, &headers).await?;

    let result = sqlx::query("DELETE FROM admin_sessions WHERE id = $1")
        .bind(&admin.token)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(LogoutResponse { ok: true }))
}
