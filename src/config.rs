use std::env;
use dotenvy;


#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String, 
    pub database_max_connections: u32, 
    pub database_min_connections: u32, 

    pub service_host: String, 
    pub service_port: String, 
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,

            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(5))
                .unwrap_or(5), 
            database_min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(1))
                .unwrap_or(1),

            service_host: env::var("SERVICE_HOST")?, 
            service_port: env::var("SERVICE_PORT")?,
        })
    }
}