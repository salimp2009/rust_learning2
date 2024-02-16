use clap::Parser;
use colored_json::prelude::*;
use hyper::{
    /* body::HttpBody as _, */ header::CONTENT_TYPE, /* Body, Client, */ Method, Request,
    Uri,
};
use serde_json::json;
use yansi::Paint;

use crate::cli::{Cli, Commands};
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
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
    // let client = Client::new();
    let client: Client<_, Full<Bytes>> = Client::builder(TokioExecutor::new()).build_http();

    let mut res = client
        .request(
            Request::builder()
                .uri(url)
                .method(method)
                .header("Content-Type", "application/json")
                .body(body.map(Full::from).unwrap_or_else(|| Full::from("")))?,
        )
        .await?;

    let mut buf = Vec::new();
    while let Some(next) = res.frame().await {
        let chunk = next?;
        buf.extend_from_slice(chunk.data_ref().unwrap());
        println!("{:?}", chunk);
    }

    let s = String::from_utf8(buf)?;

    eprintln!("Status: {}", Paint::green(res.status()));

    if res.headers().contains_key(CONTENT_TYPE) {
        let content_type = res.headers()[CONTENT_TYPE].to_str()?;
        eprintln!("Content_Type: {}", Paint::green(content_type));
        if content_type.starts_with("application/json") {
            println!("{}", &s.to_colored_json_auto()?);
        } else {
            println!("{}", &s);
        }
    } else {
        println!("{}", &s);
    }

    Ok(())
}
