use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, SqlitePool};

use crate::error::Error;

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Todo {
    pub async fn list(dbpool: SqlitePool) -> Result<Vec<Todo>, Error> {
        query_as("select * from todos")
            .fetch_all(&dbpool)
            .await
            .map_err(Into::into)
    }

    pub async fn read(dbpool: SqlitePool, id: i64) -> Result<Todo, Error> {
        query_as("select * from todos where id =?")
            .bind(id)
            .fetch_one(&dbpool)
            .await
            .map_err(Into::into)
    }

    // add code here
}
