use std::path::PathBuf;

pub(crate) type Result<T> = std::result::Result<T, CompilerError>;

pub(crate) enum CompilerError {
    FileError(FileError),
}

pub(crate) enum FileError {
    DoesNotExist { _path: PathBuf },
    Invalid { _path: PathBuf },
    Unreadable,
}
