use std::path::PathBuf;

pub(crate) type Result<T> = std::result::Result<T, CompilerMessage>;

#[derive(Debug)]
pub(crate) enum CompilerMessage {
    Error(Error),
    //    Warning(),
}

#[derive(Debug)]
pub(crate) enum Error {
    DoesNotExist { _path: PathBuf },
    Invalid { _path: PathBuf },
    Unreadable,
}
