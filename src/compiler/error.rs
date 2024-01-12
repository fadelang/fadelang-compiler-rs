use std::path::PathBuf;

pub(crate) type Result<T> = std::result::Result<T, CompilerError>;

#[derive(Debug)]
pub(crate) enum CompilerError {
    FileError(FileError),
}

#[derive(Debug)]
pub(crate) enum FileError {
    DoesNotExist { _path: PathBuf },
    Invalid { _path: PathBuf },
    Unreadable,
}
