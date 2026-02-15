use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Database row types ────────────────────────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
pub struct EventRow {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TimeSlotRow {
    pub id: String,
    pub event_id: String,
    pub starts_at: String,
    pub ends_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ParticipantRow {
    pub id: String,
    pub event_id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct VoteRow {
    pub participant_id: String,
    pub time_slot_id: String,
    pub available: bool,
}

// ── API request / response types ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    /// List of time slots to create alongside the event
    pub time_slots: Vec<TimeSlotInput>,
}

#[derive(Debug, Deserialize)]
pub struct TimeSlotInput {
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TimeSlotResponse {
    pub id: String,
    pub starts_at: String,
    pub ends_at: String,
    /// Number of participants who marked this slot as available
    pub available_count: i64,  // COUNT() always returns i64 in SQLx
}

#[derive(Debug, Serialize)]
pub struct ParticipantResponse {
    pub id: String,
    pub name: String,
    /// Map of time_slot_id -> available
    pub votes: Vec<VoteResponse>,
}

#[derive(Debug, Serialize)]
pub struct VoteResponse {
    pub time_slot_id: String,
    pub available: bool,
}

#[derive(Debug, Serialize)]
pub struct PollResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
    pub time_slots: Vec<TimeSlotResponse>,
    pub participants: Vec<ParticipantResponse>,
}

#[derive(Debug, Serialize)]
pub struct EventSummaryResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitVoteRequest {
    pub participant_name: String,
    /// One entry per time slot
    pub votes: Vec<VoteInput>,
}

#[derive(Debug, Deserialize)]
pub struct VoteInput {
    pub time_slot_id: String,
    pub available: bool,
}

#[derive(Debug, Serialize)]
pub struct SubmitVoteResponse {
    pub participant_id: String,
}

// ── Constructors ──────────────────────────────────────────────────────────────

impl EventRow {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

impl TimeSlotRow {
    pub fn new(event_id: &str, starts_at: DateTime<Utc>, ends_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_id: event_id.to_string(),
            starts_at: starts_at.to_rfc3339(),
            ends_at: ends_at.to_rfc3339(),
        }
    }
}

impl ParticipantRow {
    pub fn new(event_id: &str, name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_id: event_id.to_string(),
            name,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}
