use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::models::{
    ParticipantResponse, PollResponse, TimeSlotResponse, VoteResponse,
};

pub async fn get_poll(
    State(pool): State<SqlitePool>,
    Path(event_id): Path<String>,
) -> Result<Json<PollResponse>, StatusCode> {
    // Fetch the event
    let event = sqlx::query!(
        "SELECT id as \"id!\", title, description, created_at FROM events WHERE id = ?",
        event_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Fetch time slots with vote counts
    let slots = sqlx::query!(
        r#"
        SELECT
            ts.id as "id!",
            ts.starts_at,
            ts.ends_at,
            COUNT(CASE WHEN v.available = 1 THEN 1 END) AS "available_count!"
        FROM time_slots ts
        LEFT JOIN votes v ON v.time_slot_id = ts.id
        WHERE ts.event_id = ?
        GROUP BY ts.id
        ORDER BY ts.starts_at ASC
        "#,
        event_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch participants
    let participants = sqlx::query!(
        "SELECT id as \"id!\", name FROM participants WHERE event_id = ? ORDER BY created_at ASC",
        event_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch all votes for this event in one query
    let votes = sqlx::query!(
        r#"
        SELECT
            v.participant_id as "participant_id!",
            v.time_slot_id as "time_slot_id!",
            v.available as "available!"
        FROM votes v
        JOIN time_slots ts ON ts.id = v.time_slot_id
        WHERE ts.event_id = ?
        "#,
        event_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let time_slots = slots
        .into_iter()
        .map(|s| TimeSlotResponse {
            id: s.id,
            starts_at: s.starts_at,
            ends_at: s.ends_at,
            available_count: s.available_count,
        })
        .collect();

    let participants = participants
        .into_iter()
        .map(|p| {
            let participant_votes = votes
                .iter()
                .filter(|v| v.participant_id == p.id)
                .map(|v| VoteResponse {
                    time_slot_id: v.time_slot_id.clone(),
                    available: v.available != 0,
                })
                .collect();

            ParticipantResponse {
                id: p.id,
                name: p.name,
                votes: participant_votes,
            }
        })
        .collect();

    Ok(Json(PollResponse {
        id: event.id,
        title: event.title,
        description: event.description,
        created_at: event.created_at,
        time_slots,
        participants,
    }))
}
