use crate::api::{ping, todo_create, todo_delete, todo_list, todo_read, todo_update};
use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

// database is passed to Router; which takes the ownership
pub async fn create_router(dbpool: sqlx::Pool<sqlx::Sqlite>) -> axum::Router {
    Router::new()
        // returns 200
        .route("/alive", get(|| async { "ok" }))
        .route("/ready", get(ping))
        // Api routes are nested under v1
        .nest(
            "v1", //
            Router::new()
                // path /v1/todos; two methods are allowed; GET & POST
                .route("/todos", get(todo_list).post(todo_create))
                // path /v1/todos/:id;
                .route(
                    "/todos/:id",
                    get(todo_read).put(todo_update).delete(todo_delete),
                ),
        )
        // handing the database connection to Router to be passed to handlers
        .with_state(dbpool)
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
