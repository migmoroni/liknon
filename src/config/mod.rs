//! Loading and semantic validation for liknon configuration.

mod directories;
mod loader;
pub(crate) mod parameters;
mod validation;

pub use validation::ValidatedConfig;

use crate::error::ValidatorError;
use std::path::Path;

/// Loads and fully validates a configuration before any subprocess starts.
///
/// # Errors
///
/// Returns [`ValidatorError`] when discovery, file access, JSON deserialization,
/// or semantic validation fails. No declared tool or check is executed before
/// this function returns successfully.
pub fn load(explicit: Option<&Path>, current: &Path) -> Result<ValidatedConfig, ValidatorError> {
    validation::validate(loader::load(explicit, current)?)
}

/// Parses configuration bytes before provisioning creates any destination.
pub(crate) fn parse_at(
    contents: &[u8],
    canonical_path: &Path,
) -> Result<loader::ParsedConfig, ValidatorError> {
    let config: crate::contracts::config::Config = serde_json::from_slice(contents)
        .map_err(|error| ValidatorError::invalid(canonical_path, error.to_string()))?;
    Ok(loader::ParsedConfig {
        config,
        path: canonical_path.to_path_buf(),
    })
}

/// Applies the ordinary semantic and filesystem-aware validation path.
pub(crate) fn validate_parsed(
    parsed: loader::ParsedConfig,
) -> Result<ValidatedConfig, ValidatorError> {
    validation::validate(parsed)
}
