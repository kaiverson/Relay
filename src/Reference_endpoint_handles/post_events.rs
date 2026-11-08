use crate::shared_state::*;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use utoipa;

#[derive(Debug, Deserialize)]
pub struct PostEventsRequest {

}

#[derive(Debug, Serialize)]
pub struct PostEventsResponse {
    status: String,
}

#[utoipa::path(
    post, 
    path = "/events", 
)]
pub async fn post_events_handle(
    State(state): State<SharedState>, 
    Json(request): Json<PostEventsRequest>
) -> Json<PostEventsResponse> {
    Json (PostEventsResponse {
        status: "Endpoint not implimented!".to_string(),
    })
} 