//! Stable Tool error taxonomy and path-aware I/O adaptation.

use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Stable process-level category assigned to an operation failure.
pub(crate) enum ErrorClass {
    /// The invocation or workspace selection is invalid.
    Usage,
    /// A valid read cannot proceed in the selected local state.
    Blocked,
    /// Persisted metadata or payload bytes violate their contract.
    Integrity,
    /// The Tool itself could not complete the operation.
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
/// Failure produced while discovering, verifying, or returning local resources.
pub(crate) enum ToolError {
    /// Invalid command arguments or an unsupported invocation.
    #[error("{0}")]
    Usage(String),
    /// No initialized resource store could be discovered.
    #[error("resource workspace was not found from `{0}`")]
    WorkspaceNotFound(PathBuf),
    /// A partial Liknon workspace marker prevents unambiguous discovery.
    #[error("workspace `{0}` contains `.liknon` but no resource store")]
    PartialWorkspace(PathBuf),
    /// The requested stable identity is not present in the lock.
    #[error("installed resource `{0}` was not found")]
    ResourceNotFound(String),
    /// The requested payload path is not declared by the resource.
    #[error("resource `{resource}` has no payload `{payload}`")]
    PayloadNotFound {
        /// Requested resource identity.
        resource: String,
        /// Requested payload path.
        payload: String,
    },
    /// The selected resource cannot be consumed by this Tool version.
    #[error("resource `{resource}` is incompatible with this Tool: {reason}")]
    Incompatible {
        /// Selected resource identity.
        resource: String,
        /// Failed compatibility requirement.
        reason: String,
    },
    /// Security revocation blocks payload retrieval.
    #[error("resource `{0}` is security-revoked")]
    Revoked(String),
    /// Text output was requested for a non-textual payload.
    #[error("payload `{payload}` for `{resource}` requires explicit `--encoding=base64`")]
    BinaryOutputRequired {
        /// Selected resource identity.
        resource: String,
        /// Selected payload path.
        payload: String,
    },
    /// A payload declared textual content that was not valid UTF-8.
    #[error("text payload `{payload}` for `{resource}` is not valid UTF-8")]
    InvalidText {
        /// Selected resource identity.
        resource: String,
        /// Selected payload path.
        payload: String,
    },
    /// A valid operation exceeded a deterministic safety bound.
    #[error("operation limit exceeded: {0}")]
    Limit(String),
    /// Authenticated metadata or installed bytes violate their contract.
    #[error("resource integrity verification failed: {0}")]
    Integrity(String),
    /// Filesystem access failed at a known path.
    #[error("I/O operation failed for `{path}`: {source}")]
    Io {
        /// Affected filesystem path.
        path: PathBuf,
        /// Original I/O failure.
        #[source]
        source: io::Error,
    },
    /// A consumer-owned JSON document could not be decoded.
    #[error("invalid JSON in `{path}`: {source}")]
    Json {
        /// Affected JSON path.
        path: PathBuf,
        /// Original JSON failure.
        #[source]
        source: serde_json::Error,
    },
    /// A versioned resource-domain contract is invalid.
    #[error("invalid resource contract: {0}")]
    Contract(#[from] liknon_resource_contracts::ContractError),
    /// An invariant internal to the Tool failed.
    #[error("internal Tool error: {0}")]
    Internal(String),
}

impl ToolError {
    /// Classifies the error without inspecting its rendered message.
    pub(crate) const fn class(&self) -> ErrorClass {
        match self {
            Self::Usage(_) | Self::WorkspaceNotFound(_) | Self::PartialWorkspace(_) => {
                ErrorClass::Usage
            }
            Self::ResourceNotFound(_)
            | Self::PayloadNotFound { .. }
            | Self::Incompatible { .. }
            | Self::Revoked(_)
            | Self::BinaryOutputRequired { .. }
            | Self::Limit(_) => ErrorClass::Blocked,
            Self::InvalidText { .. }
            | Self::Integrity(_)
            | Self::Json { .. }
            | Self::Contract(_) => ErrorClass::Integrity,
            Self::Io { .. } | Self::Internal(_) => ErrorClass::Internal,
        }
    }

    /// Returns the stable process exit code for this error.
    pub(crate) const fn exit_code(&self) -> u8 {
        self.class().exit_code()
    }

    /// Returns the stable machine-readable error code.
    pub(crate) const fn code(&self) -> &'static str {
        match self {
            Self::Usage(_) => "invalid-invocation",
            Self::WorkspaceNotFound(_) => "workspace-not-found",
            Self::PartialWorkspace(_) => "partial-workspace",
            Self::ResourceNotFound(_) => "resource-not-found",
            Self::PayloadNotFound { .. } => "payload-not-found",
            Self::Incompatible { .. } => "incompatible-resource",
            Self::Revoked(_) => "revoked-resource",
            Self::BinaryOutputRequired { .. } => "binary-output-required",
            Self::InvalidText { .. } => "invalid-text-payload",
            Self::Limit(_) => "limit-exceeded",
            Self::Integrity(_) => "integrity-failure",
            Self::Io { .. } => "io-failure",
            Self::Json { .. } => "invalid-json",
            Self::Contract(_) => "invalid-contract",
            Self::Internal(_) => "internal-failure",
        }
    }
}

/// Adds the affected path when converting an I/O result into a Tool error.
pub(crate) trait IoResultExt<T> {
    /// Maps an I/O error while retaining the path being accessed.
    fn at(self, path: impl Into<PathBuf>) -> Result<T, ToolError>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn at(self, path: impl Into<PathBuf>) -> Result<T, ToolError> {
        let path = path.into();
        self.map_err(|source| ToolError::Io { path, source })
    }
}
