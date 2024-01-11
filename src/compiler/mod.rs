pub(crate) mod error;
pub(crate) mod lexer;

use std::{fs::File, io::Read, path::PathBuf};

pub(crate) struct Config {
    pub(crate) input_path: PathBuf,
    pub(crate) output_path: PathBuf,
}

pub(crate) fn compile(config: Config) -> error::Result<()> {
    let mut input_file = get_input_file(&config.input_path)?;
    let input = read_input_file(&mut input_file)?;

    let _lexed = lexer::lex(&input);

    // todo: create output_file if not exists
    // todo: write output to output_file

    Ok(())
}

fn get_input_file(path: &PathBuf) -> error::Result<File> {
    if !path.exists() {
        return Err(error::CompilerError::InputFileDoesNotExist {
            path: path.to_path_buf(),
        });
    }

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Err(error::CompilerError::InputFileInvalid {
                path: path.to_path_buf(),
            })
        }
    };

    Ok(file)
}

fn read_input_file(input_file: &mut File) -> error::Result<String> {
    let mut buffer = String::new();

    match input_file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err(error::CompilerError::InputFileUnreadable),
    }

    Ok(buffer)
}
