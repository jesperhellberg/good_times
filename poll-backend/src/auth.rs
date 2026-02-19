use axum::http::HeaderMap;
use sqlx::SqlitePool;

use axum::http::StatusCode;

#[derive(sqlx::FromRow)]
struct AdminContextRow {
    admin_id: String,
}

pub struct AdminContext {
    pub admin_id: String,
    pub token: String,
}

pub async fn require_admin(
    pool: &SqlitePool,
    headers: &HeaderMap,
) -> Result<AdminContext, StatusCode> {
    let token = headers
        .get("x-admin-token")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let admin = sqlx::query_as::<_, AdminContextRow>(
        r#"
        SELECT a.id AS admin_id
        FROM admin_sessions s
        JOIN admins a ON a.id = s.admin_id
        WHERE s.id = ?
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(AdminContext {
        admin_id: admin.admin_id,
        token: token.to_string(),
    })
}
