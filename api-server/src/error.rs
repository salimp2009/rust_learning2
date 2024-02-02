use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Sqlx(StatusCode, String),
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Sqlx(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        }
    }
}
