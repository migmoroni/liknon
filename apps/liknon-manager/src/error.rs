//! Stable Manager error taxonomy and filesystem error adaptation.

use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Stable process-level category assigned to an operation failure.
pub(crate) enum ErrorClass {
    /// The invocation or workspace selection is invalid.
    Usage,
    /// A valid operation cannot proceed in the current state.
    Blocked,
    /// Authenticated or persisted data failed an integrity requirement.
    Integrity,
    /// The Manager itself could not complete an internal operation.
    Internal,
}

impl ErrorClass {
    /// Returns the stable CLI exit code for this category.
    pub(crate) const fn exit_code(self) -> u8 {
        match self {
            Self::Usage => 3,
            Self::Blocked => 2,
            Self::Integrity => 4,
            Self::Internal => 5,
        }
    }

    /// Returns the machine-readable category name.
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
/// Failure produced while selecting, resolving, verifying, or storing resources.
pub(crate) enum ManagerError {
    /// Invalid command arguments or an unsupported invocation.
    #[error("{0}")]
    Usage(String),
    /// No initialized workspace boundary could be discovered.
    #[error("workspace boundary was not found from `{0}`")]
    WorkspaceNotFound(PathBuf),
    /// A valid operation is waiting on an external precondition.
    #[error("operation is blocked: {0}")]
    Blocked(String),
    /// No resource set satisfies all requested constraints.
    #[error("resource resolution failed: {0}")]
    Resolution(String),
    /// Authenticated metadata or installed bytes violate their contract.
    #[error("resource integrity verification failed: {0}")]
    Integrity(String),
    /// Hub transport failed before authenticated data could be obtained.
    #[error("Hub request failed: {0}")]
    Network(String),
    /// Existing workspace state conflicts with the requested mutation.
    #[error("workspace state conflicts with the requested operation: {0}")]
    Conflict(String),
    /// Filesystem access failed at a known path.
    #[error("I/O operation failed for `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    /// A consumer-owned JSON document could not be decoded.
    #[error("invalid JSON in `{path}`: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    /// A versioned resource-domain contract is invalid.
    #[error("invalid resource contract: {0}")]
    Contract(#[from] liknon_resource_contracts::ContractError),
    /// An invariant internal to the Manager failed.
    #[error("internal Manager error: {0}")]
    Internal(String),
}

impl ManagerError {
    /// Classifies the error without inspecting its rendered message.
    pub(crate) const fn class(&self) -> ErrorClass {
        match self {
            Self::Usage(_) | Self::WorkspaceNotFound(_) => ErrorClass::Usage,
            Self::Blocked(_) | Self::Resolution(_) | Self::Conflict(_) => ErrorClass::Blocked,
            Self::Integrity(_) | Self::Network(_) | Self::Contract(_) => ErrorClass::Integrity,
            Self::Io { .. } | Self::Json { .. } | Self::Internal(_) => ErrorClass::Internal,
        }
    }

    /// Returns the stable process exit code for this error.
    pub(crate) const fn exit_code(&self) -> u8 {
        self.class().exit_code()
    }
}

/// Adds the affected path when converting an I/O result into a Manager error.
pub(crate) trait IoResultExt<T> {
    /// Maps an I/O error while retaining the path being accessed.
    fn at(self, path: impl Into<PathBuf>) -> Result<T, ManagerError>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn at(self, path: impl Into<PathBuf>) -> Result<T, ManagerError> {
        let path = path.into();
        self.map_err(|source| ManagerError::Io { path, source })
    }
}
