use std::sync::Arc;

use sqlx::PgPool;

use crate::domains::events::{EventService, EventServiceTrait};

use super::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config, 
    pub event_service: Arc<dyn EventServiceTrait> 
}

impl AppState {
    pub fn new(
        config: Config, 
        event_service: Arc<dyn EventServiceTrait>,
    ) -> Self {
        Self {
            config,
            event_service,
        }
    }
}


pub fn build_app_state(pool: PgPool, config: Config) -> AppState {
    let event_service: Arc<dyn EventServiceTrait> = EventService::create_service(pool.clone());
    
    AppState::new(
        config, 
        event_service,
    )
}