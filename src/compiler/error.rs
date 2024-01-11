use std::path::PathBuf;

pub(crate) type Result<T> = std::result::Result<T, CompilerError>;

pub(crate) enum CompilerError {
    InputFileDoesNotExist { path: PathBuf },
    InputFileInvalid { path: PathBuf },
    InputFileUnreadable,
}
