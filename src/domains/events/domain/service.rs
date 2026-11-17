use crate::{
    errors::AppError,
    domains::events::dto::event_dto::{EventDto, CreateEventDto},
};

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait EventServiceTrait: Send + Sync {

    fn create_service(
        pool: PgPool, 
    ) -> Arc<dyn EventServiceTrait>
    where 
        Self: Sized;

    async fn get_events(&self) -> Result<Vec<EventDto>, AppError>;

    async fn delete_all_events(&self) -> Result<String, AppError>;

    async fn get_event_by_uuid(&self, uuid: Uuid) -> Result<EventDto, AppError>;

    async fn create_event(
        &self, 
        create_event: CreateEventDto,
    ) -> Result<EventDto, AppError>;
 
    async fn delete_event(&self, uuid: Uuid) -> Result<String, AppError>;
}