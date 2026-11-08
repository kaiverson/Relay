use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait EventServiceTrait: Send + Sync {
    fn create_service(
        pool: PgPool, 
    ) -> Arc<dyn EventServiceTrait>
    where 
        Self: Sized;
}

#[derive(Clone)]
pub struct EventService {
    pub pool: PgPool, 
}

#[async_trait]
impl EventServiceTrait for EventService {
    fn create_service(
        pool: PgPool
    ) -> Arc<dyn EventServiceTrait> {
        Arc::new(Self {
            pool
        })
    }
}