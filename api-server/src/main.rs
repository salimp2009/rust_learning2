use api_server::tracer::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
    println!("Hello, world!");
}
