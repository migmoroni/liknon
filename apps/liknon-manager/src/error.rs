use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ErrorClass {
    Usage,
    Blocked,
    Integrity,
    Internal,
}

impl ErrorClass {
    pub(crate) const fn exit_code(self) -> u8 {
        match self {
            Self::Usage => 3,
            Self::Blocked => 2,
            Self::Integrity => 4,
            Self::Internal => 5,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            Self::Blocked => "blocked",
            Self::Integrity => "integrity",
            Self::Internal => "internal",
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum ManagerError {
    #[error("{0}")]
    Usage(String),
    #[error("workspace boundary was not found from `{0}`")]
    WorkspaceNotFound(PathBuf),
    #[error("operation is blocked: {0}")]
    Blocked(String),
    #[error("resource resolution failed: {0}")]
    Resolution(String),
    #[error("resource integrity verification failed: {0}")]
    Integrity(String),
    #[error("Hub request failed: {0}")]
    Network(String),
    #[error("workspace state conflicts with the requested operation: {0}")]
    Conflict(String),
    #[error("I/O operation failed for `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("invalid JSON in `{path}`: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("invalid resource contract: {0}")]
    Contract(#[from] liknon_resource_contracts::ContractError),
    #[error("internal Manager error: {0}")]
    Internal(String),
}

impl ManagerError {
    pub(crate) const fn class(&self) -> ErrorClass {
        match self {
            Self::Usage(_) | Self::WorkspaceNotFound(_) => ErrorClass::Usage,
            Self::Blocked(_) | Self::Resolution(_) | Self::Conflict(_) => ErrorClass::Blocked,
            Self::Integrity(_) | Self::Network(_) | Self::Contract(_) => ErrorClass::Integrity,
            Self::Io { .. } | Self::Json { .. } | Self::Internal(_) => ErrorClass::Internal,
        }
    }

    pub(crate) const fn exit_code(&self) -> u8 {
        self.class().exit_code()
    }
}

pub(crate) trait IoResultExt<T> {
    fn at(self, path: impl Into<PathBuf>) -> Result<T, ManagerError>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn at(self, path: impl Into<PathBuf>) -> Result<T, ManagerError> {
        let path = path.into();
        self.map_err(|source| ManagerError::Io { path, source })
    }
}
