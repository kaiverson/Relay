use crate::shared_state::*;
use axum::{Json, extract::{Path, State}, http::header::HeaderMap};
use serde::Serialize;
use utoipa;

#[derive(Debug, Serialize)]
pub struct GetEventsIdResponse {
    id: String, 
    event: Event, 
    status: String,
}

#[utoipa::path(get, path = "/events/{id}")]
pub async fn get_events_id_handle(State(state): State<SharedState>, Path(id): Path<String>, header_map: HeaderMap) -> Json<GetEventsIdResponse> {
    let mut db = state.db.write().expect("Write lock should be available because server is single threaded for now.");
    
    let event = db.entry("get_events_id_accesses".to_string())
        .or_insert_with(|| Event {name: "".to_string()});
    event.name.push('a');

    Json (GetEventsIdResponse {
        id: "event_asdf".to_string(),
        event: event.clone(), 
        status: "accepted".to_string(),
    })
} 