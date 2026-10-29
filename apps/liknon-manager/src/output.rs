//! Versioned Manager reports and their human and JSON renderers.

use std::io::{self, Write};

use liknon_cli_ui::{SemanticRole, Theme};
use liknon_resource_contracts::{
    CatalogSnapshotIdentity, MaterializationState, RequestedSource, ResourceIdentity,
    ResourceState, ResourceVersion,
};
use serde::Serialize;

use crate::{
    cli::OutputFormat,
    error::{IoResultExt as _, ManagerError},
    theme::ManagerTheme,
};

const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Complete response envelope emitted by one Manager command.
pub(crate) struct ManagerReport {
    schema_version: u32,
    command: String,
    outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace: Option<String>,
    #[serde(flatten)]
    payload: ReportPayload,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
/// Whether a successful command observed or committed its proposed state.
enum Outcome {
    /// The requested operation completed, including any approved mutation.
    Completed,
    /// A dry run produced a plan without committing it.
    Planned,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "resultType", rename_all = "kebab-case")]
/// Command-specific body carried by a Manager report.
pub(crate) enum ReportPayload {
    /// The workspace resource store was initialized.
    Initialized,
    /// Authenticated catalog resources matched a search.
    Search {
        /// Catalog snapshot against which the search ran.
        snapshot: CatalogSnapshotIdentity,
        /// Matching immutable resource versions.
        results: Vec<ResourceVersion>,
    },
    /// Administrative metadata for one resource version.
    Info {
        /// Selected resource metadata.
        resource: Box<ResourceVersion>,
        /// Whether the version came from installed state.
        installed: bool,
    },
    /// Desired, locked, installed, and materialized workspace state.
    State {
        /// State rows for requested or locked resources.
        resources: Vec<ResourceState>,
        /// State rows for explicit materializations.
        materializations: Vec<MaterializationState>,
    },
    /// Result of integrity verification.
    Verification {
        /// Number of verified locked resources.
        verified_resources: usize,
        /// Number of verified payload files.
        verified_payloads: usize,
        /// Whether current Hub metadata was also inspected.
        online: bool,
        /// Non-fatal differences from current Hub state.
        warnings: Vec<String>,
    },
    /// Complete proposed or committed mutation.
    Mutation {
        /// Auditable mutation plan.
        plan: Box<MutationPlan>,
    },
    /// Generated public JSON Schema.
    Schema {
        /// Schema document written without terminal decoration.
        schema: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Auditable description of every consequence of one workspace mutation.
pub(crate) struct MutationPlan {
    /// Operation that produced the plan.
    pub(crate) operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Authenticated catalog snapshot selected by the resolution.
    pub(crate) snapshot: Option<CatalogSnapshotIdentity>,
    /// Resource-version changes.
    pub(crate) changes: Vec<PlannedChange>,
    /// Resulting desired roots.
    pub(crate) requests: Vec<PlannedRequest>,
    /// Resulting exact dependency edges.
    pub(crate) dependencies: Vec<PlannedDependency>,
    /// Managed and materialized filesystem changes.
    pub(crate) files: Vec<PlannedFile>,
    /// Declared capabilities and effects requiring review.
    pub(crate) effects: Vec<PlannedEffect>,
    /// Explicit decisions needed to approve the complete plan.
    pub(crate) decisions: Vec<String>,
    /// Conflicts that prevent application.
    pub(crate) conflicts: Vec<String>,
    /// Number of Hub payloads requiring download.
    pub(crate) downloads: usize,
    /// Total bytes requiring download.
    pub(crate) download_bytes: u64,
    /// Total payload bytes covered by resulting integrity checks.
    pub(crate) verified_bytes: u64,
    /// Exact resource set after application.
    pub(crate) resulting_resources: Vec<ExactResource>,
    /// Human-readable rollback guarantee.
    pub(crate) rollback: String,
    /// Whether the plan was durably committed.
    pub(crate) committed: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// One desired root retained by the resulting manifest.
pub(crate) struct PlannedRequest {
    /// Requested resource identity.
    pub(crate) identity: ResourceIdentity,
    /// Accepted semantic-version requirement.
    pub(crate) requirement: String,
    /// Selected acquisition source.
    pub(crate) source: RequestedSource,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// One exact edge retained by the resulting lock.
pub(crate) struct PlannedDependency {
    /// Resource declaring the edge.
    pub(crate) from: ResourceIdentity,
    /// Required resource identity.
    pub(crate) to: ResourceIdentity,
    /// Exact selected target version.
    pub(crate) version: String,
    /// Whether collection membership introduced the edge.
    pub(crate) collection_member: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Planned state transition for one managed filesystem path.
pub(crate) struct PlannedFile {
    /// Filesystem action.
    pub(crate) action: FileAction,
    /// Consumer-workspace-relative path.
    pub(crate) path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Digest before the mutation when known.
    pub(crate) from_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Digest after the mutation when known.
    pub(crate) to_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Resulting or prior byte length when known.
    pub(crate) bytes: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
/// Filesystem transition represented in a mutation plan.
pub(crate) enum FileAction {
    /// A new managed path is created.
    Add,
    /// Existing managed bytes are replaced.
    Replace,
    /// A previously managed path is removed.
    Remove,
    /// Existing bytes remain unchanged.
    Retain,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Resource-declared capability or side effect surfaced for review.
pub(crate) struct PlannedEffect {
    /// Resource declaring the effect.
    pub(crate) identity: ResourceIdentity,
    /// Stable effect description.
    pub(crate) effect: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Exact resource identity and version selected by resolution.
pub(crate) struct ExactResource {
    /// Stable resource identity.
    pub(crate) identity: ResourceIdentity,
    /// Exact semantic version.
    pub(crate) version: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Version transition for one resource identity.
pub(crate) struct PlannedChange {
    /// Kind of resource transition.
    pub(crate) action: ChangeAction,
    /// Affected resource identity.
    pub(crate) identity: ResourceIdentity,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Previously locked version, if any.
    pub(crate) from_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Resulting locked version, if any.
    pub(crate) to_version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
/// Resource-level transition represented in a mutation plan.
pub(crate) enum ChangeAction {
    /// A resource enters the lock.
    Add,
    /// A resource changes exact version.
    Update,
    /// A resource leaves the lock.
    Remove,
    /// A resource remains at the same version.
    Retain,
    /// A payload gains an explicit workspace projection.
    Materialize,
}

impl ManagerReport {
    /// Builds a report for an operation that completed its requested effects.
    pub(crate) fn completed(
        command: impl Into<String>,
        workspace: Option<String>,
        payload: ReportPayload,
    ) -> Self {
        Self {
            schema_version: REPORT_SCHEMA_VERSION,
            command: command.into(),
            outcome: Outcome::Completed,
            workspace,
            payload,
        }
    }

    /// Builds a report for a dry-run plan that was not committed.
    pub(crate) fn planned(
        command: impl Into<String>,
        workspace: Option<String>,
        payload: ReportPayload,
    ) -> Self {
        Self {
            schema_version: REPORT_SCHEMA_VERSION,
            command: command.into(),
            outcome: Outcome::Planned,
            workspace,
            payload,
        }
    }
}

/// Writes a successful response in the selected output format.
pub(crate) fn write_report(
    report: &ManagerReport,
    format: OutputFormat,
    theme: &Theme,
) -> Result<(), ManagerError> {
    let stdout = io::stdout();
    let mut output = stdout.lock();
    match format {
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut output, report).map_err(|source| {
                ManagerError::Internal(format!("could not serialize JSON report: {source}"))
            })?;
            writeln!(output).at("stdout")?;
        }
        OutputFormat::Human => write_human(&mut output, report, &ManagerTheme::new(theme.clone()))?,
    }
    Ok(())
}

/// Writes a stable human or machine-readable error response.
pub(crate) fn write_error(error: &ManagerError, format: OutputFormat, theme: &Theme) {
    match format {
        OutputFormat::Human => {
            let theme = ManagerTheme::new(theme.clone());
            eprintln!(
                "{}: {}",
                theme.paint(SemanticRole::Failure, "error"),
                theme.paint(SemanticRole::DiagnosticContent, error)
            );
        }
        OutputFormat::Json => {
            let value = serde_json::json!({
                "schemaVersion": REPORT_SCHEMA_VERSION,
                "outcome": "error",
                "error": {
                    "class": error.class().as_str(),
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
    report: &ManagerReport,
    theme: &ManagerTheme,
) -> Result<(), ManagerError> {
    if let ReportPayload::Schema { schema } = &report.payload {
        serde_json::to_writer_pretty(&mut *output, schema).map_err(|source| {
            ManagerError::Internal(format!("could not serialize JSON Schema: {source}"))
        })?;
        writeln!(output).at("stdout")?;
        return Ok(());
    }

    let outcome_role = match report.outcome {
        Outcome::Completed => SemanticRole::Complete,
        Outcome::Planned => SemanticRole::Active,
    };
    writeln!(
        output,
        "{}: {}",
        theme.paint(SemanticRole::AccentPrimary, &report.command),
        theme.paint(outcome_role, format!("{:?}", report.outcome))
    )
    .at("stdout")?;
    if let Some(workspace) = &report.workspace {
        writeln!(
            output,
            "{}: {}",
            theme.paint(SemanticRole::Metadata, "Workspace"),
            theme.paint(SemanticRole::Path, workspace)
        )
        .at("stdout")?;
    }
    write_gap(output, theme.section_gap())?;
    match &report.payload {
        ReportPayload::Initialized => {
            writeln!(output, "Resource manifest and lock are ready.").at("stdout")?;
        }
        ReportPayload::Search { snapshot, results } => {
            writeln!(
                output,
                "{}: {} ({})",
                theme.paint(SemanticRole::Metadata, "Catalog"),
                theme.paint(SemanticRole::Reference, &snapshot.id),
                snapshot.sequence
            )
            .at("stdout")?;
            if results.is_empty() {
                writeln!(output, "No matching resources.").at("stdout")?;
            }
            for (index, resource) in results.iter().enumerate() {
                if index > 0 {
                    write_gap(output, theme.item_gap())?;
                }
                writeln!(
                    output,
                    "{}@{}  [{:?}/{:?}]\n  {}",
                    theme.paint(SemanticRole::AccentPrimary, &resource.identity),
                    resource.version,
                    resource.trust.class,
                    resource.trust.review,
                    resource.summary
                )
                .at("stdout")?;
            }
        }
        ReportPayload::Info {
            resource,
            installed,
        } => {
            writeln!(
                output,
                "{}@{}",
                theme.paint(SemanticRole::AccentPrimary, &resource.identity),
                resource.version
            )
            .at("stdout")?;
            writeln!(output, "Title: {}", resource.title).at("stdout")?;
            writeln!(output, "Trust: {:?}", resource.trust.class).at("stdout")?;
            writeln!(output, "Review: {:?}", resource.trust.review).at("stdout")?;
            writeln!(output, "Installed: {installed}").at("stdout")?;
            writeln!(output, "Yanked: {}", resource.yanked).at("stdout")?;
            writeln!(output, "Revoked: {}", resource.revoked).at("stdout")?;
            writeln!(output, "{}", resource.summary).at("stdout")?;
        }
        ReportPayload::State {
            resources,
            materializations,
        } => {
            if resources.is_empty() {
                writeln!(output, "No resources are requested or locked.").at("stdout")?;
            }
            for (index, resource) in resources.iter().enumerate() {
                if index > 0 {
                    write_gap(output, theme.item_gap())?;
                }
                writeln!(
                    output,
                    "{:?}  {}  desired={} locked={} trust={} activatable={}",
                    theme.paint(SemanticRole::Metadata, format!("{:?}", resource.state)),
                    resource.identity,
                    resource.desired.as_deref().unwrap_or("-"),
                    resource.locked.as_deref().unwrap_or("-"),
                    resource
                        .trust
                        .map(|trust| format!("{trust:?}"))
                        .unwrap_or_else(|| "-".to_owned()),
                    resource.activatable
                )
                .at("stdout")?;
            }
            for (index, materialization) in materializations.iter().enumerate() {
                if index > 0 || !resources.is_empty() {
                    write_gap(output, theme.item_gap())?;
                }
                writeln!(
                    output,
                    "{:?}  {}@{}:{} -> {}",
                    theme.paint(
                        SemanticRole::Metadata,
                        format!("{:?}", materialization.state)
                    ),
                    materialization.identity,
                    materialization.version,
                    materialization.payload_path,
                    materialization.destination
                )
                .at("stdout")?;
            }
        }
        ReportPayload::Verification {
            verified_resources,
            verified_payloads,
            online,
            warnings,
        } => {
            writeln!(
                output,
                "Verified {verified_resources} resources and {verified_payloads} payloads (online: {online})."
            )
            .at("stdout")?;
            for warning in warnings {
                writeln!(
                    output,
                    "{}: {warning}",
                    theme.paint(SemanticRole::Warning, "Warning")
                )
                .at("stdout")?;
            }
        }
        ReportPayload::Mutation { plan } => write_plan(output, plan, theme)?,
        ReportPayload::Schema { .. } => unreachable!(),
    }
    Ok(())
}

fn write_plan(
    output: &mut impl Write,
    plan: &MutationPlan,
    theme: &ManagerTheme,
) -> Result<(), ManagerError> {
    writeln!(
        output,
        "{}: {}",
        theme.paint(SemanticRole::Metadata, "Operation"),
        theme.paint(SemanticRole::AccentPrimary, &plan.operation)
    )
    .at("stdout")?;
    if let Some(snapshot) = &plan.snapshot {
        writeln!(output, "Catalog: {} ({})", snapshot.id, snapshot.sequence).at("stdout")?;
    }
    for (index, change) in plan.changes.iter().enumerate() {
        if index > 0 {
            write_gap(output, theme.item_gap())?;
        }
        let role = match change.action {
            ChangeAction::Add => SemanticRole::Success,
            ChangeAction::Update | ChangeAction::Materialize => SemanticRole::AccentPrimary,
            ChangeAction::Remove => SemanticRole::Failure,
            ChangeAction::Retain => SemanticRole::Complete,
        };
        writeln!(
            output,
            "{:?}  {}  {} -> {}",
            theme.paint(role, format!("{:?}", change.action)),
            change.identity,
            change.from_version.as_deref().unwrap_or("-"),
            change.to_version.as_deref().unwrap_or("-")
        )
        .at("stdout")?;
    }
    for request in &plan.requests {
        writeln!(
            output,
            "Request  {}  {}  {:?}",
            request.identity, request.requirement, request.source
        )
        .at("stdout")?;
    }
    for dependency in &plan.dependencies {
        writeln!(
            output,
            "Dependency  {} -> {}@{}{}",
            dependency.from,
            dependency.to,
            dependency.version,
            if dependency.collection_member {
                " (collection member)"
            } else {
                ""
            }
        )
        .at("stdout")?;
    }
    for file in &plan.files {
        writeln!(output, "File {:?}  {}", file.action, file.path).at("stdout")?;
    }
    for effect in &plan.effects {
        writeln!(output, "Effect  {}  {}", effect.identity, effect.effect).at("stdout")?;
    }
    for decision in &plan.decisions {
        writeln!(output, "Decision  {decision}").at("stdout")?;
    }
    for conflict in &plan.conflicts {
        writeln!(
            output,
            "{}  {conflict}",
            theme.paint(SemanticRole::Blocked, "Conflict")
        )
        .at("stdout")?;
    }
    writeln!(
        output,
        "Downloads: {} files, {} bytes",
        plan.downloads, plan.download_bytes
    )
    .at("stdout")?;
    writeln!(output, "Verified result: {} bytes", plan.verified_bytes).at("stdout")?;
    writeln!(
        output,
        "Result: {} resources",
        plan.resulting_resources.len()
    )
    .at("stdout")?;
    writeln!(output, "Rollback: {}", plan.rollback).at("stdout")?;
    writeln!(output, "Committed: {}", plan.committed).at("stdout")?;
    Ok(())
}

fn write_gap(output: &mut impl Write, lines: usize) -> Result<(), ManagerError> {
    for _ in 0..lines {
        writeln!(output).at("stdout")?;
    }
    Ok(())
}
