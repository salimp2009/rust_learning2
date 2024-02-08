use api_server::dbpool::init_db_pool;
use api_server::router::create_router;
use api_server::tracer::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
    let dbpool = init_db_pool().await.expect("couldn't initialize DB pool");

    let router = create_router(dbpool).await;

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    axum::Server::bind(&bind_addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .expect("unable to start server")
}
