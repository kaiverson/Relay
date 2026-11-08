use Relay::{
    app::create_router,
    app_state::build_app_state,
    bootstrap::{setup_database, shutdown_signal},
    config::Config,
};
use tracing::info;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    
    let config = Config::from_env()?;
    let pool = setup_database(&config).await?;
    let state = build_app_state(pool, config.clone());
    let app = create_router(state);

    let addr = format!("{}:{}", config.service_host, config.service_port);

    info!("Server running at {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}  
