#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub(crate) mod cli;
pub(crate) mod compiler;

use clap::Parser;
use cli::Cli;

fn main() {
    match Cli::parse() {
        Cli::Compile { input, output } => {
            let output = if let Some(output) = output {
                output
            } else {
                let mut output = input.clone();
                output.set_extension("o.lu");
                output
            };

            let config = compiler::Config { input, output };
            match compiler::compile(&config) {
                Ok(_) => println!("Compilation successful!"),
                Err(error) => println!("Compilation failed! {error:?}"),
            }
        }
    }
}
