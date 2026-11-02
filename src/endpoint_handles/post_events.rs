use crate::shared_state::*;
use axum::{Json, extract::{Path, State}, http::header::HeaderMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostEventsRequest {

}

#[derive(Debug, Serialize)]
pub struct PostEventsResponse {
    status: String,
}

pub async fn post_events_handle(
    State(state): State<SharedState>, 
    Json(request): Json<PostEventsRequest>
) -> Json<PostEventsResponse> {
    Json (PostEventsResponse {
        status: "Endpoint not implimented!".to_string(),
    })
} 