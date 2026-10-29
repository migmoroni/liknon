use thiserror::Error;

/// Error returned when a resource contract is malformed or unverifiable.
#[derive(Debug, Error)]
pub enum ContractError {
    /// A stable identifier is malformed.
    #[error("invalid resource identity: {0}")]
    InvalidIdentity(String),
    /// A semantic version or requirement is malformed.
    #[error("invalid semantic version contract: {0}")]
    InvalidVersion(String),
    /// A resource path is unsafe or outside its declared boundary.
    #[error("unsafe resource path `{path}`: {reason}")]
    UnsafePath {
        /// Rejected contract path.
        path: String,
        /// Human-readable rejection reason.
        reason: String,
    },
    /// A hexadecimal SHA-256 value is malformed.
    #[error("invalid SHA-256 digest `{0}`")]
    InvalidDigest(String),
    /// A versioned document has an unsupported schema version.
    #[error("unsupported {document} schema version {actual}; expected {expected}")]
    UnsupportedSchema {
        /// Kind of document being checked.
        document: &'static str,
        /// Supported exact schema version.
        expected: u32,
        /// Version found in the document.
        actual: u32,
    },
    /// A contract contains duplicate or inconsistent data.
    #[error("invalid resource contract: {0}")]
    InvalidContract(String),
    /// JSON serialization needed for canonical hashing failed.
    #[error("could not serialize canonical contract bytes: {0}")]
    Serialization(#[from] serde_json::Error),
    /// Signature or trust metadata could not be decoded.
    #[error("invalid signature encoding: {0}")]
    SignatureEncoding(String),
    /// No acceptable trust key exists for a signature.
    #[error("untrusted signature key `{0}`")]
    UntrustedKey(String),
    /// A trust key cannot authenticate the supplied bytes.
    #[error("signature verification failed for key `{0}`")]
    SignatureVerification(String),
}
