use clap::Parser;
use hyper::Uri;

use crate::cli::{Cli, Commands};

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let mut uri_builder = Uri::builder();
    if let Some(scheme) = cli.url.scheme() {
        uri_builder = uri_builder.scheme(scheme.clone());
    }

    if let Some(authority) = cli.url.authority() {
        uri_builder = uri_builder.authority(authority.clone());
    }

    Ok(())
}
