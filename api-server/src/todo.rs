use crate::error::Error;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, SqlitePool};

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
        query_as("select * from todos where id = ?")
            .bind(id)
            .fetch_one(&dbpool)
            .await
            .map_err(Into::into)
    }

    pub async fn create(dbpool: SqlitePool, new_todo: CreateTodo) -> Result<Todo, Error> {
        query_as("insert into todos (body) values (?) returning *")
            .bind(new_todo.body())
            .fetch_one(&dbpool)
            .await
            .map_err(Into::into)
    }

    pub async fn update(
        dbpool: SqlitePool,
        id: i64,
        updated_todo: UpdateTodo,
    ) -> Result<Todo, Error> {
        query_as(
            "update todos   \
            set body = ( CASE WHEN $1 NOTNULL THEN ? ELSE (body) END ) , \
            completed = ?, \
            updated_at = datetime('now') where id = ? returning *",
        )
        .bind(updated_todo.body())
        .bind(updated_todo.completed())
        .bind(id)
        .fetch_one(&dbpool)
        .await
        .map_err(Into::into)
    }

    pub async fn delete(dbpool: SqlitePool, id: i64) -> Result<(), Error> {
        query("delete from todos where id = ?")
            .bind(id)
            .execute(&dbpool)
            .await?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    body: Option<String>,
    completed: bool,
}

impl UpdateTodo {
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
    // add code here
}

#[derive(Deserialize)]
pub struct CreateTodo {
    body: String,
}

impl CreateTodo {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }
    // add code here
}
