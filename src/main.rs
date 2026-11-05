use crate::shared_state::{AppState, ConnectionPool, SharedState};
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
pub mod errors;
pub mod endpoint_handles;
 
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(info(title = "Relay API", version = "0.1.0", description = "A durable webhook delivery service."))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_init();

    let pool = connection_pool_init().await;

    let shared_state: SharedState = Arc::new(AppState::new(pool));
    
    let (mut app, doc) = app_init(shared_state).split_for_parts();
    app = app.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", doc));
    
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await 
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

}  

fn tracing_init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn connection_pool_init() -> ConnectionPool {
    let db_connection_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL should be set");

    tracing::info!("connecting to database host: {}", db_connection_url.split('@').last().unwrap_or("?"));
     
    PgPoolOptions::new()
        .max_connections(16)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_url)
        .await
        .expect("Can't connect to database")
}

fn app_init(shared_state: SharedState) -> OpenApiRouter {
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