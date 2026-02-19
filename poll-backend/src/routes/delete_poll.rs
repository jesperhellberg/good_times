use axum::{extract::Path, extract::State, http::HeaderMap, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::auth::require_admin;

#[derive(serde::Serialize)]
pub struct DeletePollResponse {
    pub id: String,
}

pub async fn delete_poll(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<DeletePollResponse>, StatusCode> {
    let admin = require_admin(&pool, &headers).await?;

    let event_admin_id: Option<String> =
        sqlx::query_scalar("SELECT admin_id FROM events WHERE id = ?")
            .bind(&id)
            .fetch_optional(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(event_admin_id) = event_admin_id else {
        return Err(StatusCode::NOT_FOUND);
    };

    if event_admin_id != admin.admin_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query("DELETE FROM events WHERE id = ? AND admin_id = ?")
        .bind(&id)
        .bind(&admin.admin_id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(DeletePollResponse { id }))
}
