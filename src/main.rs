use crate::shared_state::{AppState, SharedState};
use crate::endpoint_handles::{
    get_events_id_deliveries::get_events_id_deliveries_handle,
    get_events_id::get_events_id_handle,
    get_healthz::get_healthz_handle,
    get_metrics::get_metrics_handle,
    get_readyz::get_readyz_handle,
    post_deliveries_id_replay::post_deliveries_id_replay_handle,
    post_events::post_events_handle, 
};
pub mod shared_state;
pub mod endpoint_handles;
 
use axum::{ 
    Router, routing::{get, post},
};
use std::sync::{
    {Arc, RwLock}
};

#[tokio::main]
async fn main() {
    let shared_state: SharedState = Arc::new(AppState {
        log: RwLock::new(vec![]),
    });

    let app = create_app(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await 
        .unwrap();
    axum::serve(listener, app).await.unwrap();

}  

fn create_app(shared_state: SharedState) -> Router {
    let router = Router::new()
        .route("/events", get(post_events_handle))
        .route("/events/{id}", get(get_events_id_handle))
        .route("/events/{id}/deliveries", get(get_events_id_deliveries_handle))
        .route("/deliveries/{id}/replay", post(post_deliveries_id_replay_handle))
        .route("/healthz", get(get_healthz_handle))
        .route("/readyz", get(get_readyz_handle))
        .route("/metrics", get(get_metrics_handle));
    
    Router::new()
        .nest("/v1", router)
        .with_state(shared_state)
}