pub async fn create_router(dbpool: sqlx::Pool<sqlx::Sqlite>) -> axum::Router {
    use crate::api::ping;
    use axum::{routing::get, Router};
    use tower_http::cors::{Any, CorsLayer};
    use tower_http::trace::TraceLayer;
    todo!()
}
