use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::UpdateVotesRequest;

#[derive(serde::Serialize)]
pub struct UpdateVotesResponse {
    pub participant_id: String,
}

pub async fn update_votes(
    State(pool): State<PgPool>,
    Path((event_id, participant_id)): Path<(String, String)>,
    Json(payload): Json<UpdateVotesRequest>,
) -> Result<Json<UpdateVotesResponse>, StatusCode> {
    if payload.votes.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let participant_exists: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM participants WHERE id = $1 AND event_id = $2")
            .bind(&participant_id)
            .bind(&event_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!(
            error = ?e,
            event_id = %event_id,
            participant_id = %participant_id,
            "Failed to check participant existence"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if participant_exists == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    for vote in &payload.votes {
        let slot_valid: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM time_slots WHERE id = $1 AND event_id = $2")
                .bind(&vote.time_slot_id)
                .bind(&event_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!(
                error = ?e,
                event_id = %event_id,
                time_slot_id = %vote.time_slot_id,
                "Failed to validate time slot"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if slot_valid == 0 {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| {
            tracing::error!(
                error = ?e,
                event_id = %event_id,
                participant_id = %participant_id,
                "Failed to begin transaction"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    sqlx::query(
        r#"
        DELETE FROM votes
        WHERE participant_id = $1
          AND time_slot_id IN (SELECT id FROM time_slots WHERE event_id = $2)
        "#,
    )
    .bind(&participant_id)
    .bind(&event_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(
            error = ?e,
            event_id = %event_id,
            participant_id = %participant_id,
            "Failed to delete existing votes"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let time_slot_ids: Vec<String> = payload
        .votes
        .iter()
        .map(|vote| vote.time_slot_id.clone())
        .collect();
    let availabilities: Vec<i32> = payload
        .votes
        .iter()
        .map(|vote| if vote.available { 1 } else { 0 })
        .collect();

    sqlx::query(
        r#"
        INSERT INTO votes (participant_id, time_slot_id, available)
        SELECT $1, v.time_slot_id, v.available
        FROM UNNEST($2::text[], $3::int[]) AS v(time_slot_id, available)
        "#,
    )
    .bind(&participant_id)
    .bind(&time_slot_ids)
    .bind(&availabilities)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(
            error = ?e,
            event_id = %event_id,
            participant_id = %participant_id,
            vote_count = time_slot_ids.len(),
            "Failed to insert votes batch"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit()
        .await
        .map_err(|e| {
            tracing::error!(
                error = ?e,
                event_id = %event_id,
                participant_id = %participant_id,
                "Failed to commit vote update transaction"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(UpdateVotesResponse { participant_id }))
}
