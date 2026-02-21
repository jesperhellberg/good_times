use axum::{extract::State, http::HeaderMap, http::StatusCode, Json};
use sqlx::PgPool;

use crate::auth::require_admin;
use crate::models::{CreateEventRequest, EventRow, TimeSlotRow};

#[derive(serde::Serialize)]
pub struct CreateEventResponse {
    pub id: String,
}

pub async fn create_poll(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<CreateEventResponse>), StatusCode> {
    let admin = require_admin(&pool, &headers).await?;

    if payload.title.trim().is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    if payload.time_slots.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let event = EventRow::new(payload.title, payload.description, admin.admin_id);

    // Use a transaction so event + slots are created atomically
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        "INSERT INTO events (id, title, description, created_at, admin_id) VALUES ($1, $2, $3, $4, $5)",
    )
        .bind(&event.id)
        .bind(&event.title)
        .bind(&event.description)
        .bind(&event.created_at)
        .bind(&event.admin_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for slot_input in payload.time_slots {
        let slot = TimeSlotRow::new(&event.id, slot_input.starts_at, slot_input.ends_at);

        sqlx::query(
            "INSERT INTO time_slots (id, event_id, starts_at, ends_at) VALUES ($1, $2, $3, $4)",
        )
            .bind(&slot.id)
            .bind(&slot.event_id)
            .bind(&slot.starts_at)
            .bind(&slot.ends_at)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateEventResponse { id: event.id }),
    ))
}
