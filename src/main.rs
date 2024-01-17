#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

pub(crate) mod cli;
pub(crate) mod compiler;

use std::path::PathBuf;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Compile { input, output } => {
            let input = if let Some(input) = input {
                input
            } else {
                PathBuf::from("main.lu")
            };

            let output = if let Some(output) = output {
                output
            } else {
                let mut output = input.clone();
                output.set_extension("o.lu");
                output
            };

            let compiler_config = compiler::Config { input, output };

            match compiler::compile(&compiler_config) {
                Ok(_) => println!("Compilation successful!"),
                Err(error) => println!("Compilation failed! {error:?}"),
            }
        }
    }
}
