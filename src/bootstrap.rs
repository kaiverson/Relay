use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::Config;


pub async fn setup_database(config: &Config) -> Result<PgPool, sqlx::Error> {
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
