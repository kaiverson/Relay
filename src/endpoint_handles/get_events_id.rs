use crate::shared_state::*;
use axum::{Json, extract::{Path, State}, http::header::HeaderMap};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize)]
// struct GetEventsIdRequest {
//     event_type: String, 
//     payload: serde_json::Value,
// }

#[derive(Debug, Serialize)]
pub struct GetEventsIdResponse {
    id: String, 
    event: String, 
    status: String,
}

pub async fn get_events_id_handle(State(state): State<SharedState>, Path(id): Path<String>, header_map: HeaderMap) -> Json<GetEventsIdResponse> {
    let mut log = state.log.write().unwrap();

    log.push(String::from("get_events_id_handle"));
    for (s1, s2) in header_map.into_iter() {
        println!("{}: {:?}", s1.unwrap(), s2);
    }
    Json (GetEventsIdResponse {
        id: "event_asdf".to_string(),
        event: log.join(", "), 
        status: "accepted".to_string(),
    })
} 