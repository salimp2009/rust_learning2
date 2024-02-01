use tracing_subscriber::{
    filter::LevelFilter, fmt, prelude::*, util::SubscriberInitExt, EnvFilter,
};

pub fn init_tracing() {
    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());

    // initializing the tracing with usefull info; not all the info
    // fmt::layer formats a readable format
    // Envfilter lets us to select what to trace
    // info for sqlx; debug + info for tower_http
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}
