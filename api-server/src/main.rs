use api_server::dbpool::init_db_pool;
use api_server::router::create_router;
use api_server::tracer::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
    let dbpool = init_db_pool().await.expect("couldn't initialize DB pool");

    let router = create_router(dbpool).await;

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    axum::serve(listener, router)
        .await
        .expect("unable to start server");

    // Original code with the older versions of axum & tower-http
    // axum::Server::bind(&bind_addr.parse().unwrap())
    //     .serve(router.into_make_service())
    //     .await
    //     .expect("unable to start server")
}
