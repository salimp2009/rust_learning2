use std::i64;

use clap::{Parser, Subcommand};
use hyper::Uri;

#[derive(Parser)]
pub(crate) struct Cli {
    pub url: hyper::Uri,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    List,
    // create a new todo
    Create {
        body: String,
    },
    // read a todo
    Read {
        id: i64,
    },
    Update {
        id: i64,
        body: String,
        #[arg(short, long)]
        completed: bool,
    },
    Delete {
        id: i64,
    },
}
