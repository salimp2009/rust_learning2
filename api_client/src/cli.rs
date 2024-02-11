use clap::{Parser, Subcommand};
// use hyper;

#[derive(Parser)]
pub(crate) struct Cli {
    /// Base URL of API service
    pub url: hyper::Uri,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// List all todos
    List,
    /// Create a new todo
    Create { body: String },
    /// Read a todo
    Read { id: i64 },
    /// Update a todo
    Update {
        /// The todo ID
        id: i64,
        /// The todo body
        body: String,
        /// Mark todo as completed
        #[arg(short, long)]
        completed: bool,
    },
    /// The todo ID
    Delete { id: i64 },
}
