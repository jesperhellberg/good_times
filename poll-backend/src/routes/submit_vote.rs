use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::{ParticipantRow, SubmitVoteRequest, SubmitVoteResponse};

pub async fn submit_vote(
    State(pool): State<PgPool>,
    Path(event_id): Path<String>,
    Json(payload): Json<SubmitVoteRequest>,
) -> Result<(StatusCode, Json<SubmitVoteResponse>), StatusCode> {
    let name = payload.participant_name.trim().to_string();

    if name.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    // Verify event exists
    let event_exists: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM events WHERE id = $1")
            .bind(&event_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, event_id = %event_id, "Failed to check event existence");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if event_exists == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Verify all submitted time_slot_ids actually belong to this event
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

    let participant = ParticipantRow::new(&event_id, name);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, event_id = %event_id, "Failed to begin transaction");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    sqlx::query("INSERT INTO participants (id, event_id, name, created_at) VALUES ($1, $2, $3, $4)")
        .bind(&participant.id)
        .bind(&participant.event_id)
        .bind(&participant.name)
        .bind(&participant.created_at)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(
            error = ?e,
            event_id = %event_id,
            participant_id = %participant.id,
            "Failed to insert participant"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !payload.votes.is_empty() {
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
            ON CONFLICT (participant_id, time_slot_id) DO UPDATE SET available = excluded.available
            "#,
        )
        .bind(&participant.id)
        .bind(&time_slot_ids)
        .bind(&availabilities)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!(
                error = ?e,
                event_id = %event_id,
                participant_id = %participant.id,
                vote_count = time_slot_ids.len(),
                "Failed to insert votes batch"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    tx.commit()
        .await
        .map_err(|e| {
            tracing::error!(
                error = ?e,
                event_id = %event_id,
                participant_id = %participant.id,
                "Failed to commit vote transaction"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SubmitVoteResponse {
            participant_id: participant.id,
        }),
    ))
}
