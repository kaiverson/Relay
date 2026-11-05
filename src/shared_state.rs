use axum::extract::{FromRef, FromRequestParts};
use axum::http::StatusCode;
use axum::http::request::Parts;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio_postgres::NoTls;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap};
use serde::{Deserialize, Serialize};

use crate::errors::internal_error;

pub type ConnectionPool = sqlx::Pool<sqlx::Postgres>;
pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub pool: ConnectionPool,
}

impl AppState {
    pub fn new(pool: ConnectionPool) -> Self {
        AppState { pool }
    }
}

// pub struct DatabaseConnection(
//     pub PooledConnection<'static, PostgresConnectionManager<NoTls>>
// );

// impl FromRequestParts<SharedState> for DatabaseConnection {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(
//         _parts: &mut Parts, 
//         state: &SharedState,
//     ) -> Result<Self, Self::Rejection> {
//         let conn = state.pool.get_owned().await.map_err(internal_error)?;
//         Ok(Self(conn))
//     }
// }
