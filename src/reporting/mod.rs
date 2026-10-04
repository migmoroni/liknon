//! Presentation boundaries for validation execution and completed results.
//!
//! Active runs publish transient progress through an internal terminal
//! renderer. Immutable reports are rendered only after completion through
//! [`result`]. Shared text formatting remains private to this module tree,
//! while visual semantics are supplied by [`crate::theme`].

pub(crate) mod execution;
/// Rendering of an immutable completed validation report.
pub mod result;

mod format;
mod tree;
