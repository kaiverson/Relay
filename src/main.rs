use sqlx::{PgPool, postgres::PgPoolOptions};

use Relay::{
    app::create_router,
    app_state::build_app_state,
    config::Config,
};
use tracing::info;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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



async fn setup_database(config: &Config) -> Result<PgPool, sqlx::Error> {
    let mut attempts = 0;
    let pool = loop {
        attempts += 1;
        match PgPoolOptions::new()
            .max_connections(config.database_max_connections)
            .min_connections(config.database_min_connections)
            .connect(&config.database_url)
            .await 
        {
            Ok(pool) => break pool, 
            Err(err) => {
                if attempts >= 3 {
                    return Err(err);
                }
                eprintln!(
                    "Postgres not ready yet ({:?}), retrying in 1s... (attempt {}/{})",
                    err, attempts, 3
                );
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    };

    Ok(pool)
}


pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await 
        .expect("Failed to install CTRL+C signal handler");
}


// async fn connection_pool_init() -> ConnectionPool {
//     let db_connection_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL should be set");

//     tracing::info!("connecting to database host: {}", db_connection_url.split('@').last().unwrap_or("?"));
     
//     PgPoolOptions::new()
//         .max_connections(16)
//         .acquire_timeout(Duration::from_secs(5))
//         .connect(&db_connection_url)
//         .await
//         .expect("Can't connect to database")
// }

