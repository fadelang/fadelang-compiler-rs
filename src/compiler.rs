pub(crate) mod error;
pub(crate) mod keywords;
pub(crate) mod lexer;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use self::error::{CompilerMessage, Error};

pub(crate) struct Config {
    pub(crate) input: PathBuf,
    pub(crate) output: PathBuf,
}

pub(crate) fn compile(config: &Config) -> error::Result<()> {
    let mut input_file = get_input_file(&config.input)?;
    let input = read_file(&mut input_file)?;

    let lexed = lexer::lex(&input);
    let lexed_string = format!("{lexed:#?}");

    let mut output_file = get_output_file(&config.output)?;
    output_file.write_all(&lexed_string.into_bytes()).unwrap();

    Ok(())
}

fn get_input_file(path: &PathBuf) -> error::Result<File> {
    if !path.exists() {
        return Err(CompilerMessage::Error(Error::DoesNotExist {
            _path: path.clone(),
        }));
    }

    let Ok(file) = File::open(path) else {
        return Err(CompilerMessage::Error(
            Error::Invalid { _path: path.clone() })
        );
    };

    Ok(file)
}

fn get_output_file(path: &PathBuf) -> error::Result<File> {
    let Ok(file) = File::create(path) else {
        return Err(CompilerMessage::Error(
            Error::Invalid { _path: path.clone() })
        );
    };

    Ok(file)
}

fn read_file(input_file: &mut File) -> error::Result<String> {
    let mut buffer = String::new();

    match input_file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err(CompilerMessage::Error(Error::Unreadable)),
    }

    Ok(buffer)
}
