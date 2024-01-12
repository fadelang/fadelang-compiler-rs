use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn command(&self) -> &Option<Commands> {
        &self.command
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[clap(alias = "c")]
    Compile {
        #[arg(short, long, default_value = "main.fl")]
        input: Option<PathBuf>,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
