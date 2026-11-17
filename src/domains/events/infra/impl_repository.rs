use crate::domains::events::{
    domain::{model::Event, repository::EventRepository},
    dto::event_dto::CreateEventDto,
};
use async_trait::async_trait;

use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub struct EventRepo;

const FIND_EVENT_QUERY: &str = r#"
    SELECT
        e.uuid,
        e.event_type,
        e.created_at, 
        e.payload 
    FROM events e 
    WHERE 1=1
"#;

const DELETE_ALL_EVENTS_QUERY: &str = r#"
    DELETE FROM events
"#;

const FIND_EVENT_INFO_QUERY: &str = r#"
    SELECT
        e.uuid,
        e.event_type,
        e.created_at, 
        e.payload
    FROM events e 
    WHERE e.uuid = $1
"#;

const CREATE_EVENT_QUERY: &str = r#"
    INSERT INTO events (uuid, event_type, created_at, payload)
    VALUES ($1, $2, $3, $4)
"#;

const DELETE_EVENT_QUERY: &str = r#"
    DELETE FROM events WHERE uuid = $1
"#;


#[async_trait]
impl EventRepository for EventRepo {

    async fn find_all(&self, pool: PgPool) -> Result<Vec<Event>, sqlx::Error> {
        let events = sqlx::query_as::<_, Event>(FIND_EVENT_QUERY) 
            .fetch_all(&pool)
            .await?;
        Ok(events)
    }

    async fn delete_all(&self, tx: &mut Transaction<'_, Postgres>) -> Result<usize, sqlx::Error> {
        let res = sqlx::query(DELETE_ALL_EVENTS_QUERY)
            .execute(&mut **tx)
            .await?;
        Ok(res.rows_affected() as usize)
    }

    async fn find_by_uuid(&self, pool: PgPool, uuid: Uuid) -> Result<Option<Event>, sqlx::Error> {
        let event = sqlx::query_as::<_, Event>(FIND_EVENT_INFO_QUERY)
            .bind(uuid)
            .fetch_optional(&pool)
            .await?;
        Ok(event)
    }

    async fn create(
        &self, 
        tx: &mut Transaction<'_, Postgres>,
        event: CreateEventDto,
    ) -> Result<Uuid, sqlx::Error> {
        let uuid = Uuid::new_v4();
        let created_at = Utc::now();

        sqlx::query(CREATE_EVENT_QUERY) 
            .bind(uuid.clone()) 
            .bind(event.event_type.clone())
            .bind(created_at.clone())
            .bind(event.payload.clone())
        .execute(&mut **tx)
        .await?;

        Ok(uuid)
    }

    async fn delete(
        &self, 
        tx: &mut Transaction<'_, Postgres>,
        uuid: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query(DELETE_EVENT_QUERY)
            .bind(uuid)
            .execute(&mut **tx)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}