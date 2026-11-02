use crate::shared_state::{AppState, SharedState};
use crate::endpoint_handles::{
    get_events_id_deliveries,
    get_events_id,
    get_healthz,
    get_metrics,
    get_readyz,
    post_deliveries_id_replay,
    post_events, 
};
pub mod shared_state;
pub mod endpoint_handles;
 
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Arc;

#[derive(OpenApi)]
#[openapi(info(title = "Relay API", version = "0.1", description = "A durable webhook delivery service."))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let shared_state: SharedState = Arc::new(AppState::new());

    let (mut app, doc) = create_app(shared_state).split_for_parts();
    app = app.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", doc));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await 
        .unwrap();

    axum::serve(listener, app).await.unwrap();

}  

fn create_app(shared_state: SharedState) -> OpenApiRouter {
    let router = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(post_events::post_events_handle))
        .routes(routes!(get_events_id::get_events_id_handle))
        .routes(routes!(get_events_id_deliveries::get_events_id_deliveries_handle))
        .routes(routes!(post_deliveries_id_replay::post_deliveries_id_replay_handle))
        .routes(routes!(get_healthz::get_healthz_handle))
        .routes(routes!(get_readyz::get_readyz_handle))
        .routes(routes!(get_metrics::get_metrics_handle));
    
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/v1", router)
        .with_state(shared_state)
}