use std::path::PathBuf;

pub(crate) type Result<T> = std::result::Result<T, ErrorType>;

#[derive(Debug)]
pub(crate) enum ErrorType {
    FileError { path: PathBuf },
}
