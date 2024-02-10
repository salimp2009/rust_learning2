use clap::{Parser, Subcommand};
use colored_json::prelude::*;
use hyper::{body::HttpBody as _, header::CONTENT_TYPE, Body, Client, Method, Request, Uri};
use serde_json::json;
use yansi::Paint;

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

    match cli.command {
        Commands::List => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Delete { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::DELETE,
                None,
            )
            .await
        }
        Commands::Read { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Create { body } => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
                Method::POST,
                Some(json!({"body":body}).to_string()),
            )
            .await
        }
        Commands::Update {
            id,
            body,
            completed,
        } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::PUT,
                Some(json!({"body":body,"completed":completed}).to_string()),
            )
            .await
        }
    }

    // Ok(())
}

async fn request(
    url: hyper::Uri,
    method: Method,
    body: Option<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}
