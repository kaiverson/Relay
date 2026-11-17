use crate::{
    domains::{
        events::{
            domain::{repository::EventRepository, service::EventServiceTrait},
            dto::event_dto::{CreateEventDto, EventDto,},
            infra::impl_repository::EventRepo,
        },
    },
    errors::AppError,
};

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;


#[derive(Clone)]
pub struct EventService {
    pub pool: PgPool, 
    pub repo: Arc<dyn EventRepository + Send + Sync>,
}

#[async_trait]
impl EventServiceTrait for EventService {
    fn create_service(pool: PgPool) -> Arc<dyn EventServiceTrait>
    where
        Self: Sized,
    {
        Arc::new(Self { 
            pool, 
            repo: Arc::new(EventRepo {})
        })
    }

    async fn get_events(&self) -> Result<Vec<EventDto>, AppError> {
        match self.repo.find_all(self.pool.clone()).await {
            Ok(events) => {
                let event_dtos: Vec<EventDto> = events.into_iter().map(Into::into).collect();
                Ok(event_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching users: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn delete_all_events(&self) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;
        let rows_affected = match self.repo.delete_all(&mut tx).await {
            Ok(rows_affected) => rows_affected, 
            Err(err) => {
                tracing::error!("Error deleting all events: {err}");
                return Err(AppError::DatabaseError(err));
            }
        };

        tx.commit().await?;

        Ok(format!("{rows_affected} events deleted"))
    }

    async fn get_event_by_uuid(&self, uuid: Uuid) -> Result<EventDto, AppError> {
        match self.repo.find_by_uuid(self.pool.clone(), uuid).await {
            Ok(Some(event)) => Ok(EventDto::from(event)),
            Ok(None) => Err(AppError::NotFound("Event not found".into())),
            Err(err) => {
                tracing::error!("Error retrieving event: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn create_event(
        &self,
        create_event: CreateEventDto,
    ) -> Result<EventDto, AppError> {
        let mut tx = self.pool.begin().await?;

        let event_uuid = match self.repo.create(&mut tx, create_event).await {
            Ok(event_uuid) => event_uuid, 
            Err(err) => {
                tracing::error!("Error creating event: {err}");
                tx.rollback().await?;
                return Err(AppError::DatabaseError(err));
            }
        };

        tx.commit().await?;

        match self.repo.find_by_uuid(self.pool.clone(), event_uuid).await {
            Ok(Some(event)) => Ok(EventDto::from(event)),
            Ok(None) => Err(AppError::NotFound("Event not found".into())),
            Err(err) => {
                tracing::error!("Error retrieving event: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    async fn delete_event(&self, uuid: Uuid) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;
        let event_deleted = match self.repo.delete(&mut tx, uuid).await {
            Ok(event_deleted) => event_deleted,
            Err(err) => {
                tracing::error!("Error deleting event: {err}");
                tx.rollback().await?;
                return Err(AppError::DatabaseError(err));
            },
        };

        tx.commit().await?;

        Ok(
            (
                if event_deleted { 
                    "Event Deleted" 
                } 
                else { 
                    "No Changes Made" 
                }
            )
            .into()
        )
    }
}