use axum::{extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::models::EventSummaryResponse;

pub async fn list_events(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<EventSummaryResponse>>, StatusCode> {
    let events = sqlx::query_as::<_, EventSummaryResponse>(
        "SELECT id, title, description, created_at FROM events ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(events))
}
