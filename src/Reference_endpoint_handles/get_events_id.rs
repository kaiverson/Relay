use crate::shared_state::*;
use crate::errors::internal_error;
use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde::Serialize;
use utoipa;

#[derive(Debug, Serialize)]
pub struct GetEventsIdResponse {
    id: String, 
    event: String, 
    status: String,
}
 
#[utoipa::path(get, path = "/events/{id}")]
pub async fn get_events_id_handle(
    State(state): State<SharedState>, 
    Path(_id): Path<String>,
) -> Result<Json<GetEventsIdResponse>, (StatusCode, String)> {
    let pool = &state.pool;
    let row: i32 =  sqlx::query_scalar("select 1 + 1")
        .fetch_one(pool)
        .await
        .map_err(internal_error)?;
        
    Ok(Json (GetEventsIdResponse {
        id: "test id 123".to_string(),
        event: format!("{row}"), 
        status: "accepted".to_string(),
    }))
} 