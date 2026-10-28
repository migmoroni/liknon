use std::{io, path::PathBuf, process::ExitStatus};

use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("contract violation: {0}")]
    Contract(String),
    #[error("failed to read {path}: {source}")]
    Read { path: PathBuf, source: io::Error },
    #[error("failed to write {path}: {source}")]
    Write { path: PathBuf, source: io::Error },
    #[error("invalid JSON in {path}: {source}")]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("failed to serialize JSON: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to start {program}: {source}")]
    Spawn { program: String, source: io::Error },
    #[error("{program} failed with {status}: {stderr}")]
    Command {
        program: String,
        status: ExitStatus,
        stderr: String,
    },
}

pub(crate) fn contract(message: impl Into<String>) -> Error {
    Error::Contract(message.into())
}
