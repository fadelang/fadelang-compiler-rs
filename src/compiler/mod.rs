pub(crate) mod error;
pub(crate) mod lexer;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use self::error::{CompilerError, FileError};

pub(crate) struct Config {
    pub(crate) input_path: PathBuf,
    pub(crate) output_path: PathBuf,
}

pub(crate) fn compile(config: &Config) -> error::Result<()> {
    let mut input_file = get_input_file(&config.input_path)?;
    let input = read_file(&mut input_file)?;

    let _lexed = lexer::lex(&input);

    let mut output_file = get_output_file(&config.output_path)?;
    output_file
        .write_all(&String::from("test").into_bytes())
        .unwrap();

    Ok(())
}

fn get_input_file(path: &PathBuf) -> error::Result<File> {
    if !path.exists() {
        return Err(CompilerError::FileError(FileError::DoesNotExist {
            _path: path.clone(),
        }));
    }

    let Ok(file) = File::open(path) else {
        return Err(CompilerError::FileError(
            FileError::Invalid { _path: path.clone() })
        );
    };

    Ok(file)
}

fn get_output_file(path: &PathBuf) -> error::Result<File> {
    let Ok(file) = File::create(path) else {
        return Err(CompilerError::FileError(
            FileError::Invalid { _path: path.clone() })
        );
    };

    Ok(file)
}

fn read_file(input_file: &mut File) -> error::Result<String> {
    let mut buffer = String::new();

    match input_file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err(CompilerError::FileError(FileError::Unreadable)),
    }

    Ok(buffer)
}
