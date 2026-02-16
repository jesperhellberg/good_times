use axum::{extract::Path, extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

#[derive(serde::Serialize)]
pub struct DeletePollResponse {
    pub id: String,
}

pub async fn delete_poll(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<DeletePollResponse>, StatusCode> {
    let result = sqlx::query("DELETE FROM events WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(DeletePollResponse { id }))
}
