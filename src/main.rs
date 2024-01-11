#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

pub(crate) mod lexer;

use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let buf = match open_file("main.fl") {
        Ok(buf) => buf,
        Err(err) => handle_error(&err),
    };
    let _lexed = lexer::lex(&buf);
}

fn open_file(file_name: &str) -> io::Result<String> {
    let mut buf = String::new();

    let mut file = File::open(file_name)?;
    let bytes_read = file.read_to_string(&mut buf)?;
    assert!(bytes_read != 0, "0 bytes read thats prob bad??");

    Ok(buf)
}

pub(crate) fn handle_error(err: &io::Error) -> ! {
    panic!("Unhandled Error: {err}")
}
