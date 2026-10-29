use std::io::{self, Write};

use liknon_resource_contracts::{
    CatalogSnapshotIdentity, MaterializationState, RequestedSource, ResourceIdentity,
    ResourceState, ResourceVersion,
};
use serde::Serialize;

use crate::{
    cli::OutputFormat,
    error::{IoResultExt as _, ManagerError},
};

const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
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
enum Outcome {
    Completed,
    Planned,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "resultType", rename_all = "kebab-case")]
pub(crate) enum ReportPayload {
    Initialized,
    Search {
        snapshot: CatalogSnapshotIdentity,
        results: Vec<ResourceVersion>,
    },
    Info {
        resource: Box<ResourceVersion>,
        installed: bool,
    },
    State {
        resources: Vec<ResourceState>,
        materializations: Vec<MaterializationState>,
    },
    Verification {
        verified_resources: usize,
        verified_payloads: usize,
        online: bool,
        warnings: Vec<String>,
    },
    Mutation {
        plan: Box<MutationPlan>,
    },
    Schema {
        schema: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MutationPlan {
    pub(crate) operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) snapshot: Option<CatalogSnapshotIdentity>,
    pub(crate) changes: Vec<PlannedChange>,
    pub(crate) requests: Vec<PlannedRequest>,
    pub(crate) dependencies: Vec<PlannedDependency>,
    pub(crate) files: Vec<PlannedFile>,
    pub(crate) effects: Vec<PlannedEffect>,
    pub(crate) decisions: Vec<String>,
    pub(crate) conflicts: Vec<String>,
    pub(crate) downloads: usize,
    pub(crate) download_bytes: u64,
    pub(crate) verified_bytes: u64,
    pub(crate) resulting_resources: Vec<ExactResource>,
    pub(crate) rollback: String,
    pub(crate) committed: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlannedRequest {
    pub(crate) identity: ResourceIdentity,
    pub(crate) requirement: String,
    pub(crate) source: RequestedSource,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlannedDependency {
    pub(crate) from: ResourceIdentity,
    pub(crate) to: ResourceIdentity,
    pub(crate) version: String,
    pub(crate) collection_member: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlannedFile {
    pub(crate) action: FileAction,
    pub(crate) path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) from_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bytes: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum FileAction {
    Add,
    Replace,
    Remove,
    Retain,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlannedEffect {
    pub(crate) identity: ResourceIdentity,
    pub(crate) effect: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExactResource {
    pub(crate) identity: ResourceIdentity,
    pub(crate) version: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlannedChange {
    pub(crate) action: ChangeAction,
    pub(crate) identity: ResourceIdentity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) from_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to_version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ChangeAction {
    Add,
    Update,
    Remove,
    Retain,
    Materialize,
}

impl ManagerReport {
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

pub(crate) fn write_report(
    report: &ManagerReport,
    format: OutputFormat,
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
        OutputFormat::Human => write_human(&mut output, report)?,
    }
    Ok(())
}

pub(crate) fn write_error(error: &ManagerError, format: OutputFormat) {
    match format {
        OutputFormat::Human => eprintln!("error: {error}"),
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

fn write_human(output: &mut impl Write, report: &ManagerReport) -> Result<(), ManagerError> {
    if let ReportPayload::Schema { schema } = &report.payload {
        serde_json::to_writer_pretty(&mut *output, schema).map_err(|source| {
            ManagerError::Internal(format!("could not serialize JSON Schema: {source}"))
        })?;
        writeln!(output).at("stdout")?;
        return Ok(());
    }

    writeln!(output, "{}: {:?}", report.command, report.outcome).at("stdout")?;
    if let Some(workspace) = &report.workspace {
        writeln!(output, "Workspace: {workspace}").at("stdout")?;
    }
    match &report.payload {
        ReportPayload::Initialized => {
            writeln!(output, "Resource manifest and lock are ready.").at("stdout")?;
        }
        ReportPayload::Search { snapshot, results } => {
            writeln!(output, "Catalog: {} ({})", snapshot.id, snapshot.sequence).at("stdout")?;
            if results.is_empty() {
                writeln!(output, "No matching resources.").at("stdout")?;
            }
            for resource in results {
                writeln!(
                    output,
                    "{}@{}  [{:?}/{:?}]\n  {}",
                    resource.identity,
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
            writeln!(output, "{}@{}", resource.identity, resource.version).at("stdout")?;
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
            for resource in resources {
                writeln!(
                    output,
                    "{:?}  {}  desired={} locked={} trust={} activatable={}",
                    resource.state,
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
            for materialization in materializations {
                writeln!(
                    output,
                    "{:?}  {}@{}:{} -> {}",
                    materialization.state,
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
                writeln!(output, "Warning: {warning}").at("stdout")?;
            }
        }
        ReportPayload::Mutation { plan } => write_plan(output, plan)?,
        ReportPayload::Schema { .. } => unreachable!(),
    }
    Ok(())
}

fn write_plan(output: &mut impl Write, plan: &MutationPlan) -> Result<(), ManagerError> {
    writeln!(output, "Operation: {}", plan.operation).at("stdout")?;
    if let Some(snapshot) = &plan.snapshot {
        writeln!(output, "Catalog: {} ({})", snapshot.id, snapshot.sequence).at("stdout")?;
    }
    for change in &plan.changes {
        writeln!(
            output,
            "{:?}  {}  {} -> {}",
            change.action,
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
        writeln!(output, "Conflict  {conflict}").at("stdout")?;
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
