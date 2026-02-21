use axum::{extract::State, http::HeaderMap, http::StatusCode, Json};
use sqlx::PgPool;

use crate::auth::require_admin;
use crate::models::EventSummaryResponse;

pub async fn list_events(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<Json<Vec<EventSummaryResponse>>, StatusCode> {
    let admin = require_admin(&pool, &headers).await?;

    let events = sqlx::query_as::<_, EventSummaryResponse>(
        "SELECT id, title, description, created_at FROM events WHERE admin_id = $1 ORDER BY created_at DESC",
    )
    .bind(&admin.admin_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(events))
}
