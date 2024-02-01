use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

pub async fn init_db_pool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.sqlite".to_string());

    let dbpool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::from_str(&db_connection_str)?.create_if_missing(true))
        .await
        .expect("cant connect to database");

    sqlx::migrate!()
        .run(&dbpool)
        .await
        .expect("cant migrate database");

    Ok(dbpool)
}
