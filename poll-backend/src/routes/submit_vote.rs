use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::models::{ParticipantRow, SubmitVoteRequest, SubmitVoteResponse};

pub async fn submit_vote(
    State(pool): State<SqlitePool>,
    Path(event_id): Path<String>,
    Json(payload): Json<SubmitVoteRequest>,
) -> Result<(StatusCode, Json<SubmitVoteResponse>), StatusCode> {
    let name = payload.participant_name.trim().to_string();

    if name.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    // Verify event exists
    let event_exists = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM events WHERE id = ?",
        event_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if event_exists == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Verify all submitted time_slot_ids actually belong to this event
    for vote in &payload.votes {
        let slot_valid = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM time_slots WHERE id = ? AND event_id = ?",
            vote.time_slot_id,
            event_id
        )
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if slot_valid == 0 {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
    }

    let participant = ParticipantRow::new(&event_id, name);

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        "INSERT INTO participants (id, event_id, name, created_at) VALUES (?, ?, ?, ?)",
        participant.id,
        participant.event_id,
        participant.name,
        participant.created_at,
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for vote in payload.votes {
        let available = vote.available;
        sqlx::query!(
            r#"
            INSERT INTO votes (participant_id, time_slot_id, available)
            VALUES (?, ?, ?)
            ON CONFLICT (participant_id, time_slot_id) DO UPDATE SET available = excluded.available
            "#,
            participant.id,
            vote.time_slot_id,
            available,
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(SubmitVoteResponse {
            participant_id: participant.id,
        }),
    ))
}
