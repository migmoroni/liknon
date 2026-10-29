//! Versioned Tool reports and their human and JSON renderers.

use std::io::{self, Write};

use liknon_cli_ui::{SemanticRole, Theme};
use liknon_resource_contracts::{
    LockedCompatibility, LockedEdge, LockedResource, LockedResourceOrigin, PayloadMetadata,
    ResourceIdentity, ResourceVersion, SourceMetadata, TrustMetadata, WorkflowMetadata,
};
use serde::Serialize;

use crate::{
    cli::OutputFormat,
    error::{IoResultExt as _, ToolError},
    theme::ToolTheme,
};

const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Complete response envelope emitted by one Tool command.
pub(crate) struct ToolReport {
    schema_version: u32,
    command: String,
    outcome: Outcome,
    workspace: String,
    #[serde(flatten)]
    payload: ReportPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
/// Completeness of one successful Tool response.
pub(crate) enum Outcome {
    /// Every selected result fit within the declared bounds.
    Complete,
    /// Security metadata is complete, but explicit content limits omitted payloads.
    Partial,
}

#[derive(Debug, Clone, Serialize)]
#[serde(
    tag = "resultType",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
/// Command-specific body carried by a Tool report.
pub(crate) enum ReportPayload {
    /// Bounded metadata for installed resources.
    Catalog {
        /// Number of matching resources before the result limit.
        total: usize,
        /// Deterministically ordered returned resources.
        resources: Vec<CatalogEntry>,
    },
    /// Deterministic metadata-search results.
    Search {
        /// Caller-supplied query.
        query: String,
        /// Number of matches before the result limit.
        total: usize,
        /// Stable ranked matches.
        results: Vec<SearchResult>,
    },
    /// One exact resource and optional verified payload.
    Show {
        /// Complete locked metadata for the selected resource.
        resource: Box<ResourceDetails>,
        /// Selected payload content, when one was requested or unambiguous.
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<ContentItem>,
    },
    /// Exact locked dependency closure.
    Resolution {
        /// Explicit caller-selected roots.
        roots: Vec<ResourceIdentity>,
        /// Dependencies-first closure.
        resources: Vec<ResourceDetails>,
    },
    /// Bounded content handoff for humans or external hosts.
    Context {
        /// Explicit caller-selected roots.
        roots: Vec<ResourceIdentity>,
        /// Selected resources with verified payload representations.
        resources: Vec<ContextResource>,
        /// Payloads omitted only because of explicit context limits.
        omissions: Vec<Omission>,
        /// Number of payload files represented or omitted.
        declared_files: usize,
        /// Aggregate declared bytes represented or omitted.
        declared_bytes: u64,
        /// Number of payload files included in the envelope.
        included_files: usize,
        /// Aggregate payload bytes verified for the envelope.
        included_bytes: u64,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Bounded routing metadata for one installed resource.
pub(crate) struct CatalogEntry {
    /// Stable resource identity.
    pub(crate) identity: ResourceIdentity,
    /// Exact locked version.
    pub(crate) version: String,
    /// Human-facing title.
    pub(crate) title: String,
    /// Concise routing summary.
    pub(crate) summary: String,
    /// Authorship retained by immutable metadata.
    pub(crate) authors: Vec<String>,
    /// SPDX license expression.
    pub(crate) license: String,
    /// Authored-source provenance.
    pub(crate) source: SourceMetadata,
    /// Trust and review facts without instruction authority.
    pub(crate) trust: TrustMetadata,
    /// Exact acquisition origin.
    pub(crate) origin: LockedResourceOrigin,
    /// Locked compatibility and activation facts.
    pub(crate) compatibility: LockedCompatibility,
    /// Declared generic capabilities.
    pub(crate) capabilities: Vec<String>,
    /// Whether content may represent executable instructions or bytes.
    pub(crate) contains_executable_content: bool,
    /// Whether newer resolution should avoid this exact version.
    pub(crate) yanked: bool,
    /// Whether a replacement is recommended.
    pub(crate) deprecated: bool,
    /// Whether security revocation blocks content retrieval.
    pub(crate) revoked: bool,
    /// Number of exact locked dependency edges.
    pub(crate) dependency_count: usize,
    /// Number of declared payload files.
    pub(crate) payload_count: usize,
    /// Declared payload routing metadata.
    pub(crate) payloads: Vec<PayloadMetadata>,
    /// Bounded workflow graph summary when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) workflow: Option<WorkflowSummary>,
    /// Integrity established without reading payload content.
    pub(crate) metadata_integrity: IntegrityState,
    /// Payload bytes are not inspected by discovery operations.
    pub(crate) payload_integrity: IntegrityState,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Bounded workflow routing facts exposed by catalog operations.
pub(crate) struct WorkflowSummary {
    /// Declared entry node IDs.
    pub(crate) entry_nodes: Vec<String>,
    /// Declared terminal node IDs.
    pub(crate) terminal_nodes: Vec<String>,
    /// Number of declared nodes.
    pub(crate) node_count: usize,
    /// Number of declared transitions.
    pub(crate) transition_count: usize,
    /// Distinct declared effects.
    pub(crate) effects: Vec<String>,
    /// Referenced agent-profile identities.
    pub(crate) agent_profiles: Vec<ResourceIdentity>,
    /// Referenced external tool-operation IDs.
    pub(crate) tool_operations: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Stable metadata match returned by local search.
pub(crate) struct SearchResult {
    /// Relative ranking score used only for deterministic local ordering.
    pub(crate) score: u16,
    /// Metadata fields containing the normalized query.
    pub(crate) matched_fields: Vec<String>,
    /// Bounded resource metadata.
    pub(crate) resource: CatalogEntry,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Complete locked metadata for an explicitly selected resource.
pub(crate) struct ResourceDetails {
    /// Immutable resource metadata.
    pub(crate) resource: ResourceVersion,
    /// Digest authenticating canonical resource metadata.
    pub(crate) manifest_sha256: String,
    /// Exact acquisition origin.
    pub(crate) origin: LockedResourceOrigin,
    /// Canonical installation path below the resource store.
    pub(crate) install_path: String,
    /// Exact locked dependency edges.
    pub(crate) dependencies: Vec<LockedEdge>,
    /// Locked compatibility and activation facts.
    pub(crate) compatibility: LockedCompatibility,
    /// Integrity established for the locked metadata.
    pub(crate) metadata_integrity: IntegrityState,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
/// Verification depth established for one part of a response.
pub(crate) enum IntegrityState {
    /// Exact bytes and digest were verified.
    Verified,
    /// Payload bytes were deliberately not loaded by this operation.
    NotInspected,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// One exact verified payload representation.
pub(crate) struct ContentItem {
    /// Declared path within the resource.
    pub(crate) path: String,
    /// Declared media type.
    pub(crate) media_type: String,
    /// Exact byte length.
    pub(crate) length: u64,
    /// Exact SHA-256 digest.
    pub(crate) sha256: String,
    /// Returned representation.
    pub(crate) encoding: ContentRepresentation,
    /// Exact UTF-8 text or standard padded Base64, absent for binary references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) content: Option<String>,
    /// Resource content never gains host or policy authority through retrieval.
    pub(crate) instruction_boundary: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
/// Representation selected for one verified payload.
pub(crate) enum ContentRepresentation {
    /// Exact UTF-8 text.
    Text,
    /// Standard padded Base64.
    Base64,
    /// Verified binary payload described without embedding its bytes.
    BinaryReference,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// One resource represented in a bounded context envelope.
pub(crate) struct ContextResource {
    /// Complete locked metadata.
    pub(crate) details: ResourceDetails,
    /// Verified payload representations that fit declared bounds.
    pub(crate) payloads: Vec<ContentItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Explicit payload omission caused by a caller-visible context limit.
pub(crate) struct Omission {
    /// Resource owning the omitted payload.
    pub(crate) identity: ResourceIdentity,
    /// Omitted payload path.
    pub(crate) path: String,
    /// Declared byte length.
    pub(crate) length: u64,
    /// Stable omission reason.
    pub(crate) reason: &'static str,
}

impl ToolReport {
    /// Builds one complete response.
    pub(crate) fn complete(
        command: impl Into<String>,
        workspace: impl Into<String>,
        payload: ReportPayload,
    ) -> Self {
        Self::new(command, workspace, Outcome::Complete, payload)
    }

    /// Builds one response whose explicit limits omitted payload content.
    pub(crate) fn partial(
        command: impl Into<String>,
        workspace: impl Into<String>,
        payload: ReportPayload,
    ) -> Self {
        Self::new(command, workspace, Outcome::Partial, payload)
    }

    fn new(
        command: impl Into<String>,
        workspace: impl Into<String>,
        outcome: Outcome,
        payload: ReportPayload,
    ) -> Self {
        Self {
            schema_version: REPORT_SCHEMA_VERSION,
            command: command.into(),
            outcome,
            workspace: workspace.into(),
            payload,
        }
    }
}

impl CatalogEntry {
    /// Projects bounded routing metadata without loading payload bytes.
    pub(crate) fn from_locked(locked: &LockedResource) -> Self {
        Self {
            identity: locked.resource.identity.clone(),
            version: locked.resource.version.clone(),
            title: locked.resource.title.clone(),
            summary: locked.resource.summary.clone(),
            authors: locked.resource.authors.clone(),
            license: locked.resource.license.clone(),
            source: locked.resource.source.clone(),
            trust: locked.resource.trust.clone(),
            origin: locked.origin.clone(),
            compatibility: locked.compatibility.clone(),
            capabilities: locked.resource.capabilities.clone(),
            contains_executable_content: locked.resource.contains_executable_content,
            yanked: locked.resource.yanked,
            deprecated: locked.resource.deprecated,
            revoked: locked.resource.revoked,
            dependency_count: locked.dependencies.len(),
            payload_count: locked.resource.payloads.len(),
            payloads: locked.resource.payloads.clone(),
            workflow: locked.resource.workflow.as_ref().map(WorkflowSummary::from),
            metadata_integrity: IntegrityState::Verified,
            payload_integrity: IntegrityState::NotInspected,
        }
    }
}

impl From<&WorkflowMetadata> for WorkflowSummary {
    fn from(workflow: &WorkflowMetadata) -> Self {
        let mut effects = workflow
            .nodes
            .iter()
            .flat_map(|node| node.effects.iter().cloned())
            .collect::<Vec<_>>();
        effects.sort();
        effects.dedup();
        let mut agent_profiles = workflow
            .nodes
            .iter()
            .flat_map(|node| node.agent_profiles.iter().cloned())
            .collect::<Vec<_>>();
        agent_profiles.sort();
        agent_profiles.dedup();
        let mut tool_operations = workflow
            .nodes
            .iter()
            .flat_map(|node| node.tool_operations.iter().cloned())
            .collect::<Vec<_>>();
        tool_operations.sort();
        tool_operations.dedup();
        Self {
            entry_nodes: workflow.entry_nodes.clone(),
            terminal_nodes: workflow.terminal_nodes.clone(),
            node_count: workflow.nodes.len(),
            transition_count: workflow.transitions.len(),
            effects,
            agent_profiles,
            tool_operations,
        }
    }
}

impl ResourceDetails {
    /// Projects complete locked metadata for an explicit selection.
    pub(crate) fn from_locked(locked: &LockedResource) -> Self {
        Self {
            resource: locked.resource.clone(),
            manifest_sha256: locked.manifest_sha256.clone(),
            origin: locked.origin.clone(),
            install_path: locked.install_path.clone(),
            dependencies: locked.dependencies.clone(),
            compatibility: locked.compatibility.clone(),
            metadata_integrity: IntegrityState::Verified,
        }
    }
}

/// Writes a successful response in the selected output format.
pub(crate) fn write_report(
    report: &ToolReport,
    format: OutputFormat,
    theme: &Theme,
) -> Result<(), ToolError> {
    let stdout = io::stdout();
    let mut output = stdout.lock();
    match format {
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut output, report).map_err(|source| {
                ToolError::Internal(format!("could not serialize JSON report: {source}"))
            })?;
            writeln!(output).at("stdout")?;
        }
        OutputFormat::Human => write_human(&mut output, report, &ToolTheme::new(theme.clone()))?,
    }
    Ok(())
}

/// Writes a stable human or machine-readable error response.
pub(crate) fn write_error(error: &ToolError, format: OutputFormat, theme: &Theme) {
    match format {
        OutputFormat::Human => {
            let theme = ToolTheme::new(theme.clone());
            let message = safe_inline(&error.to_string());
            eprintln!(
                "{}: {}",
                theme.paint(SemanticRole::Failure, "error"),
                theme.paint(SemanticRole::DiagnosticContent, message)
            );
        }
        OutputFormat::Json => {
            let value = serde_json::json!({
                "schemaVersion": REPORT_SCHEMA_VERSION,
                "outcome": "error",
                "error": {
                    "class": error.class().as_str(),
                    "code": error.code(),
                    "message": error.to_string(),
                    "exitCode": error.exit_code()
                }
            });
            if serde_json::to_writer_pretty(io::stdout().lock(), &value).is_ok() {
                println!();
            }
        }
    }
}

fn write_human(
    output: &mut impl Write,
    report: &ToolReport,
    theme: &ToolTheme,
) -> Result<(), ToolError> {
    let outcome_role = match report.outcome {
        Outcome::Complete => SemanticRole::Complete,
        Outcome::Partial => SemanticRole::Blocked,
    };
    writeln!(
        output,
        "{}: {}",
        theme.paint(SemanticRole::AccentPrimary, &report.command),
        theme.paint(outcome_role, format!("{:?}", report.outcome))
    )
    .at("stdout")?;
    writeln!(
        output,
        "{}: {}",
        theme.paint(SemanticRole::Metadata, "Workspace"),
        theme.paint(SemanticRole::Path, safe_inline(&report.workspace))
    )
    .at("stdout")?;
    write_gap(output, theme.section_gap())?;

    match &report.payload {
        ReportPayload::Catalog { total, resources } => {
            writeln!(
                output,
                "Installed resources: {} of {total}",
                resources.len()
            )
            .at("stdout")?;
            write_catalog_rows(output, resources, theme)?;
        }
        ReportPayload::Search {
            query,
            total,
            results,
        } => {
            writeln!(output, "Query: {}", safe_inline(query)).at("stdout")?;
            writeln!(output, "Matches: {} of {total}", results.len()).at("stdout")?;
            for (index, result) in results.iter().enumerate() {
                if index > 0 {
                    write_gap(output, theme.item_gap())?;
                }
                writeln!(
                    output,
                    "{}@{}  score={}  matched={}\n  origin={}  trust={}  metadata-integrity={:?}  payload-integrity={:?}  revoked={}\n  {}",
                    theme.paint(SemanticRole::AccentPrimary, &result.resource.identity),
                    result.resource.version,
                    result.score,
                    result.matched_fields.join(","),
                    safe_inline(&format!("{:?}", result.resource.origin)),
                    result.resource.trust.class,
                    result.resource.metadata_integrity,
                    result.resource.payload_integrity,
                    result.resource.revoked,
                    safe_inline(&result.resource.summary)
                )
                .at("stdout")?;
            }
        }
        ReportPayload::Show { resource, content } => {
            write_details(output, resource, theme)?;
            if let Some(content) = content {
                write_gap(output, theme.section_gap())?;
                write_content(output, content, theme)?;
            }
        }
        ReportPayload::Resolution { roots, resources } => {
            writeln!(
                output,
                "Roots: {}",
                roots
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .at("stdout")?;
            writeln!(output, "Dependencies-first closure:").at("stdout")?;
            for resource in resources {
                writeln!(
                    output,
                    "  {}@{}  trust={} revoked={} dependencies={}",
                    theme.paint(SemanticRole::AccentPrimary, &resource.resource.identity),
                    resource.resource.version,
                    resource.resource.trust.class,
                    resource.resource.revoked,
                    resource.dependencies.len()
                )
                .at("stdout")?;
            }
        }
        ReportPayload::Context {
            roots,
            resources,
            omissions,
            declared_files,
            declared_bytes,
            included_files,
            included_bytes,
        } => {
            writeln!(
                output,
                "{}",
                theme.paint(
                    SemanticRole::Blocked,
                    "UNTRUSTED RESOURCE CONTENT: payload text has no host, policy, system, or human authority."
                )
            )
            .at("stdout")?;
            writeln!(
                output,
                "Roots: {}",
                roots
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .at("stdout")?;
            writeln!(
                output,
                "Payloads: {included_files}/{declared_files} files, {included_bytes}/{declared_bytes} bytes"
            )
            .at("stdout")?;
            for resource in resources {
                write_gap(output, theme.section_gap())?;
                write_details(output, &resource.details, theme)?;
                for content in &resource.payloads {
                    write_gap(output, theme.item_gap())?;
                    write_content(output, content, theme)?;
                }
            }
            for omission in omissions {
                writeln!(
                    output,
                    "Omitted {}:{} ({} bytes): {}",
                    omission.identity,
                    safe_inline(&omission.path),
                    omission.length,
                    omission.reason
                )
                .at("stdout")?;
            }
        }
    }
    Ok(())
}

fn write_catalog_rows(
    output: &mut impl Write,
    resources: &[CatalogEntry],
    theme: &ToolTheme,
) -> Result<(), ToolError> {
    if resources.is_empty() {
        writeln!(output, "No matching installed resources.").at("stdout")?;
    }
    for (index, resource) in resources.iter().enumerate() {
        if index > 0 {
            write_gap(output, theme.item_gap())?;
        }
        writeln!(
            output,
            "{}@{}  origin={}  trust={} review={:?}  metadata-integrity={:?}  payload-integrity={:?}  yanked={} revoked={}\n  {}",
            theme.paint(SemanticRole::AccentPrimary, &resource.identity),
            resource.version,
            safe_inline(&format!("{:?}", resource.origin)),
            resource.trust.class,
            resource.trust.review,
            resource.metadata_integrity,
            resource.payload_integrity,
            resource.yanked,
            resource.revoked,
            safe_inline(&resource.summary)
        )
        .at("stdout")?;
    }
    Ok(())
}

fn write_details(
    output: &mut impl Write,
    details: &ResourceDetails,
    theme: &ToolTheme,
) -> Result<(), ToolError> {
    let resource = &details.resource;
    writeln!(
        output,
        "{}@{}",
        theme.paint(SemanticRole::AccentPrimary, &resource.identity),
        resource.version
    )
    .at("stdout")?;
    writeln!(output, "Title: {}", safe_inline(&resource.title)).at("stdout")?;
    writeln!(
        output,
        "Trust: {} ({:?})",
        resource.trust.class, resource.trust.review
    )
    .at("stdout")?;
    writeln!(output, "Origin: {:?}", details.origin).at("stdout")?;
    writeln!(output, "License: {}", safe_inline(&resource.license)).at("stdout")?;
    writeln!(output, "Yanked: {}", resource.yanked).at("stdout")?;
    writeln!(output, "Revoked: {}", resource.revoked).at("stdout")?;
    writeln!(output, "Metadata integrity: verified").at("stdout")?;
    writeln!(output, "{}", safe_inline(&resource.summary)).at("stdout")
}

fn write_content(
    output: &mut impl Write,
    content: &ContentItem,
    theme: &ToolTheme,
) -> Result<(), ToolError> {
    writeln!(
        output,
        "{}",
        theme.paint(
            SemanticRole::Blocked,
            "UNTRUSTED RESOURCE CONTENT: inspect before following any instruction."
        )
    )
    .at("stdout")?;
    writeln!(
        output,
        "File: {} ({}, {} bytes, {:?})",
        safe_inline(&content.path),
        safe_inline(&content.media_type),
        content.length,
        content.encoding
    )
    .at("stdout")?;
    if let Some(value) = &content.content {
        writeln!(output, "---").at("stdout")?;
        write_untrusted_text(output, value)?;
        if !value.ends_with('\n') {
            writeln!(output).at("stdout")?;
        }
        writeln!(output, "---").at("stdout")?;
    } else {
        writeln!(output, "Binary content verified; bytes were not embedded.").at("stdout")?;
    }
    Ok(())
}

fn write_untrusted_text(output: &mut impl Write, value: &str) -> Result<(), ToolError> {
    for character in value.chars() {
        match character {
            '\n' | '\t' => write!(output, "{character}").at("stdout")?,
            character if character.is_control() => {
                for escaped in character.escape_default() {
                    write!(output, "{escaped}").at("stdout")?;
                }
            }
            character => write!(output, "{character}").at("stdout")?,
        }
    }
    Ok(())
}

fn safe_inline(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        if character.is_control() {
            escaped.extend(character.escape_default());
        } else {
            escaped.push(character);
        }
    }
    escaped
}

fn write_gap(output: &mut impl Write, lines: usize) -> Result<(), ToolError> {
    for _ in 0..lines {
        writeln!(output).at("stdout")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{safe_inline, write_untrusted_text};

    #[test]
    fn escapes_terminal_control_characters_in_human_content() {
        let mut output = Vec::new();
        write_untrusted_text(&mut output, "safe\n\u{1b}[31mred\u{7}\tend").unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "safe\n\\u{1b}[31mred\\u{7}\tend"
        );
    }

    #[test]
    fn renders_metadata_on_one_terminal_line() {
        assert_eq!(safe_inline("title\n\u{1b}[31m"), "title\\n\\u{1b}[31m");
    }
}
