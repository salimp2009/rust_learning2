use axum::extract::{Path, State};
use axum::Json;
use sqlx::{Acquire, SqlitePool};

use crate::error::Error;
use crate::todo::{CreateTodo, Todo, UpdateTodo};

pub(crate) async fn ping(State(dbpool): State<SqlitePool>) -> Result<String, Error> {
    use sqlx::Connection;

    let mut conn = dbpool.acquire().await?;
    conn.ping()
        .await
        .map(|_| "ok".to_string())
        .map_err(Into::into)
}
