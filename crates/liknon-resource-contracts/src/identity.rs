use std::{fmt, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{validate_relative_path, ContractError};

/// Stable singular resource kinds understood by the version 1 protocol.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceKind {
    /// Explanatory or reference knowledge.
    Knowledge,
    /// Reusable AI operational guidance.
    Skill,
    /// A generic graph of externally conducted actions.
    Workflow,
    /// An AI role, posture, scope, and constraint declaration.
    AgentProfile,
    /// A descriptor for externally supplied application capabilities.
    Tool,
    /// Reusable prompt content.
    Prompt,
    /// Human- or agent-oriented procedural guidance.
    Guide,
    /// A reusable authored file template.
    Template,
    /// A dependency-only grouping of resources.
    Collection,
}

impl ResourceKind {
    /// Returns the canonical pluralized directory below the resource store.
    #[must_use]
    pub const fn directory(self) -> &'static str {
        match self {
            Self::Knowledge => "knowledge",
            Self::Skill => "skills",
            Self::Workflow => "workflows",
            Self::AgentProfile => "agent-profiles",
            Self::Tool => "tools",
            Self::Prompt => "prompts",
            Self::Guide => "guides",
            Self::Template => "templates",
            Self::Collection => "collections",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Knowledge => "knowledge",
            Self::Skill => "skill",
            Self::Workflow => "workflow",
            Self::AgentProfile => "agent-profile",
            Self::Tool => "tool",
            Self::Prompt => "prompt",
            Self::Guide => "guide",
            Self::Template => "template",
            Self::Collection => "collection",
        })
    }
}

impl FromStr for ResourceKind {
    type Err = ContractError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "knowledge" => Ok(Self::Knowledge),
            "skill" => Ok(Self::Skill),
            "workflow" => Ok(Self::Workflow),
            "agent-profile" => Ok(Self::AgentProfile),
            "tool" => Ok(Self::Tool),
            "prompt" => Ok(Self::Prompt),
            "guide" => Ok(Self::Guide),
            "template" => Ok(Self::Template),
            "collection" => Ok(Self::Collection),
            _ => Err(ContractError::InvalidIdentity(format!(
                "unknown resource kind `{value}`"
            ))),
        }
    }
}

/// Kind, publisher namespace, and stable name of one resource.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceIdentity {
    /// Singular protocol kind.
    pub kind: ResourceKind,
    /// Publisher-controlled namespace.
    pub namespace: String,
    /// Stable resource name inside the namespace and kind.
    pub name: String,
}

impl ResourceIdentity {
    /// Creates and validates one stable identity.
    pub fn new(
        kind: ResourceKind,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let identity = Self {
            kind,
            namespace: namespace.into(),
            name: name.into(),
        };
        identity.validate()?;
        Ok(identity)
    }

    /// Validates namespace and name normalization.
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_segment(&self.namespace, "namespace")?;
        validate_segment(&self.name, "name")?;
        validate_relative_path(&self.namespace)?;
        validate_relative_path(&self.name)
    }

    /// Returns the canonical path relative to `.liknon/resources/` for a version.
    pub fn version_path(&self, version: &str) -> Result<String, ContractError> {
        self.validate()?;
        semver::Version::parse(version).map_err(|error| {
            ContractError::InvalidVersion(format!("resource version `{version}`: {error}"))
        })?;
        let path = format!(
            "{}/{}/{}/{}",
            self.kind.directory(),
            self.namespace,
            self.name,
            version
        );
        validate_relative_path(&path)?;
        Ok(path)
    }
}

impl fmt::Display for ResourceIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}/{}", self.kind, self.namespace, self.name)
    }
}

impl FromStr for ResourceIdentity {
    type Err = ContractError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (kind, remainder) = value.split_once(':').ok_or_else(|| {
            ContractError::InvalidIdentity(format!("`{value}` must use <kind>:<namespace>/<name>"))
        })?;
        let (namespace, name) = remainder.split_once('/').ok_or_else(|| {
            ContractError::InvalidIdentity(format!("`{value}` must use <kind>:<namespace>/<name>"))
        })?;
        if name.contains('/') {
            return Err(ContractError::InvalidIdentity(format!(
                "`{value}` contains more than one namespace separator"
            )));
        }
        Self::new(kind.parse()?, namespace, name)
    }
}

/// A resource identity paired with a semantic version requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceSpec {
    /// Stable resource identity.
    pub identity: ResourceIdentity,
    /// Accepted semantic versions.
    pub requirement: String,
}

impl FromStr for ResourceSpec {
    type Err = ContractError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (identity, requirement) = value
            .rsplit_once('@')
            .map_or((value, "*"), |(identity, requirement)| {
                (identity, requirement)
            });
        if requirement.is_empty() {
            return Err(ContractError::InvalidVersion(format!(
                "resource spec `{value}` has an empty requirement"
            )));
        }
        semver::VersionReq::parse(requirement).map_err(|error| {
            ContractError::InvalidVersion(format!("requirement `{requirement}`: {error}"))
        })?;
        Ok(Self {
            identity: identity.parse()?,
            requirement: requirement.to_owned(),
        })
    }
}

fn validate_segment(value: &str, label: &str) -> Result<(), ContractError> {
    let valid = !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_lowercase()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_' | b'.')
        })
        && !value.contains("..")
        && !value.ends_with('-')
        && !value.ends_with('_')
        && !value.ends_with('.');
    if valid {
        Ok(())
    } else {
        Err(ContractError::InvalidIdentity(format!(
            "{label} `{value}` must be normalized lower-case ASCII"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::{ResourceIdentity, ResourceKind, ResourceSpec};

    #[test]
    fn identity_round_trips_through_canonical_text() {
        let identity: ResourceIdentity = "knowledge:official/evidence-quality".parse().unwrap();
        assert_eq!(identity.kind, ResourceKind::Knowledge);
        assert_eq!(identity.to_string(), "knowledge:official/evidence-quality");
        assert_eq!(
            identity.version_path("1.2.3").unwrap(),
            "knowledge/official/evidence-quality/1.2.3"
        );
    }

    #[test]
    fn resource_spec_defaults_to_any_version() {
        let spec: ResourceSpec = "skill:community/review".parse().unwrap();
        assert_eq!(spec.requirement, "*");
        let exact: ResourceSpec = "skill:community/review@=1.2.3".parse().unwrap();
        assert_eq!(exact.requirement, "=1.2.3");
    }

    #[test]
    fn identity_paths_are_portable_across_supported_systems() {
        assert!(ResourceIdentity::new(ResourceKind::Knowledge, "con", "guide").is_err());
        assert!(ResourceIdentity::new(ResourceKind::Knowledge, "official", "nul.txt").is_err());

        let identity = ResourceIdentity::new(ResourceKind::Knowledge, "official", "guide").unwrap();
        let oversized_version = format!("1.0.0-{}", "a".repeat(256));
        assert!(identity.version_path(&oversized_version).is_err());
    }
}
