use api_server::dbpool::init_db_pool;
use api_server::tracer::init_tracing;
#[tokio::main]
async fn main() {
    init_tracing();
    let dbpool = init_db_pool().await.expect("couldn't initialize DB pool");

    println!("Hello, world!");
}
