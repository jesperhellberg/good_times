use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::models::UpdateVotesRequest;

#[derive(serde::Serialize)]
pub struct UpdateVotesResponse {
    pub participant_id: String,
}

pub async fn update_votes(
    State(pool): State<SqlitePool>,
    Path((event_id, participant_id)): Path<(String, String)>,
    Json(payload): Json<UpdateVotesRequest>,
) -> Result<Json<UpdateVotesResponse>, StatusCode> {
    if payload.votes.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let participant_exists: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM participants WHERE id = ? AND event_id = ?")
            .bind(&participant_id)
            .bind(&event_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if participant_exists == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    for vote in &payload.votes {
        let slot_valid: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM time_slots WHERE id = ? AND event_id = ?")
                .bind(&vote.time_slot_id)
                .bind(&event_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if slot_valid == 0 {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"
        DELETE FROM votes
        WHERE participant_id = ?
          AND time_slot_id IN (SELECT id FROM time_slots WHERE event_id = ?)
        "#,
    )
    .bind(&participant_id)
    .bind(&event_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for vote in payload.votes {
        let available = vote.available;
        sqlx::query(
            r#"
            INSERT INTO votes (participant_id, time_slot_id, available)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&participant_id)
        .bind(&vote.time_slot_id)
        .bind(available)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UpdateVotesResponse { participant_id }))
}
