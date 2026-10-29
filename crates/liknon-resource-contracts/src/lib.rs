//! Versioned contracts shared by the Liknon resource ecosystem.
//!
//! This crate is deliberately transport- and application-neutral. It defines
//! exact resource identities, authenticated Hub metadata, desired and locked
//! workspace state, and validation helpers. It never performs network access,
//! mutates a workspace, or executes resource content.

mod catalog;
mod error;
mod hash;
mod identity;
mod path;
mod signature;
mod state;
mod validation;

pub use catalog::*;
pub use error::ContractError;
pub use hash::{canonical_json_sha256, sha256_bytes, validate_sha256};
pub use identity::{ResourceIdentity, ResourceKind, ResourceSpec};
pub use path::{validate_relative_path, RESOURCE_ROOT_RELATIVE};
#[cfg(feature = "signing")]
pub use signature::{public_key_base64, sign_detached};
pub use signature::{
    verify_detached, DetachedSignature, SignatureAlgorithm, TrustKey, TrustKeyStatus, TrustRoot,
};
pub use state::*;
pub use validation::{
    validate_catalog, validate_resource, validate_store_state, validate_trust_root,
    ValidationLimits,
};

/// Resource contract schema version supported by this crate.
pub const RESOURCE_SCHEMA_VERSION: u32 = 1;

/// Hub catalog schema version supported by this crate.
pub const CATALOG_SCHEMA_VERSION: u32 = 1;

/// Mutable catalog-pointer schema version supported by this crate.
pub const CATALOG_POINTER_SCHEMA_VERSION: u32 = 1;

/// Consumer desired-state manifest schema version supported by this crate.
pub const RESOURCE_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Consumer exact lock schema version supported by this crate.
pub const RESOURCE_LOCK_SCHEMA_VERSION: u32 = 1;

/// Detached signature schema version supported by this crate.
pub const SIGNATURE_SCHEMA_VERSION: u32 = 1;

/// Trust-root schema version supported by this crate.
pub const TRUST_ROOT_SCHEMA_VERSION: u32 = 1;

/// Canonical retained catalog evidence path below `.liknon/resources/`.
pub const RETAINED_CATALOG_PATH: &str = ".metadata/catalog.json";

/// Canonical retained catalog-signature path below `.liknon/resources/`.
pub const RETAINED_CATALOG_SIGNATURE_PATH: &str = ".metadata/catalog.json.sig.json";

/// Canonical retained trust-root path below `.liknon/resources/`.
pub const RETAINED_TRUST_ROOT_PATH: &str = ".metadata/trust-root.json";
