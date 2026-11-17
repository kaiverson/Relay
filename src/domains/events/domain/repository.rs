use crate::domains::events::{dto::event_dto::CreateEventDto};

use super::model::Event;

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;


#[async_trait]
pub trait EventRepository: Send + Sync {

    async fn find_all(&self, pool: PgPool) 
        -> Result<Vec<Event>, sqlx::Error>;

    async fn delete_all(&self, tx: &mut Transaction<'_, Postgres>)
        -> Result<usize, sqlx::Error>;

    async fn find_by_uuid(&self, pool: PgPool, uuid: Uuid) 
        -> Result<Option<Event>, sqlx::Error>;

    async fn create(
        &self, 
        tx: &mut Transaction<'_, Postgres>,
        event: CreateEventDto,
    ) -> Result<Uuid, sqlx::Error>;

    async fn delete(
        &self, 
        tx: &mut Transaction<'_, Postgres>,
        uuid: Uuid,
    ) -> Result<bool, sqlx::Error>;
}