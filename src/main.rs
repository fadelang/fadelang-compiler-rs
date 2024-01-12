#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

pub(crate) mod cli;
pub(crate) mod compiler;

use std::path::PathBuf;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command() {
        Some(Commands::Compile { input, output }) => {
            let input_path = match input {
                Some(input) => input.clone(),
                None => PathBuf::from("main.fl"),
            };

            let output_path = match output {
                Some(output) => output.clone(),
                None => {
                    let mut output = input_path.clone();
                    output.set_extension(".o.fl");
                    output
                }
            };

            let compiler_config = compiler::Config {
                input_path,
                output_path,
            };

            match compiler::compile(&compiler_config) {
                Ok(_) => println!("gud!"),
                Err(_) => println!("bad!"),
            }
        }
        None => todo!(),
    }
}
