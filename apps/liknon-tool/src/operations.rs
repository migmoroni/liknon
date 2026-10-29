//! Read-only command orchestration from parsed intent to deterministic reports.

use std::collections::BTreeSet;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use liknon_resource_contracts::{LockedResource, PayloadMetadata, ResourceIdentity, ResourceKind};

use crate::{
    cli::{
        CatalogFilters, Cli, Command, ContentEncoding, MAX_CONTEXT_BYTES, MAX_CONTEXT_FILES,
        MAX_RESULT_LIMIT,
    },
    error::ToolError,
    output::{
        CatalogEntry, ContentItem, ContentRepresentation, ContextResource, Omission, ReportPayload,
        ResourceDetails, SearchResult, ToolReport,
    },
    store::{ensure_consumable, Store, MAX_SHOW_BYTES},
    workspace,
};

const INSTRUCTION_BOUNDARY: &str =
    "untrusted-resource-content-below-system-host-repository-human-and-policy-authority";

/// Dispatches one parsed command without mixing rendering into domain work.
pub(crate) fn execute(cli: Cli) -> Result<ToolReport, ToolError> {
    validate_limits(&cli.command)?;
    match cli.command {
        Command::Catalog { filters, workspace } => catalog(filters, workspace.workspace.as_deref()),
        Command::Search {
            query,
            filters,
            workspace,
        } => search(query, filters, workspace.workspace.as_deref()),
        Command::Show {
            resource,
            file,
            encoding,
            workspace,
        } => show(
            &resource,
            file.as_deref(),
            encoding,
            workspace.workspace.as_deref(),
        ),
        Command::Resolve {
            resources,
            workspace,
        } => resolve(resources, workspace.workspace.as_deref()),
        Command::Context {
            resources,
            include_dependencies,
            max_files,
            max_bytes,
            workspace,
        } => context(
            resources,
            include_dependencies,
            max_files,
            max_bytes,
            workspace.workspace.as_deref(),
        ),
    }
}

fn validate_limits(command: &Command) -> Result<(), ToolError> {
    let bounded = |name: &str, value: usize, maximum: usize| {
        if (1..=maximum).contains(&value) {
            Ok(())
        } else {
            Err(ToolError::Usage(format!(
                "`--{name}` must be between 1 and {maximum}"
            )))
        }
    };

    match command {
        Command::Catalog { filters, .. } | Command::Search { filters, .. } => {
            bounded("limit", filters.limit, MAX_RESULT_LIMIT)
        }
        Command::Context {
            max_files,
            max_bytes,
            ..
        } => {
            bounded("max-files", *max_files, MAX_CONTEXT_FILES)?;
            if (1..=MAX_CONTEXT_BYTES).contains(max_bytes) {
                Ok(())
            } else {
                Err(ToolError::Usage(format!(
                    "`--max-bytes` must be between 1 and {MAX_CONTEXT_BYTES}"
                )))
            }
        }
        Command::Show { .. } | Command::Resolve { .. } => Ok(()),
    }
}

fn catalog(
    filters: CatalogFilters,
    explicit_workspace: Option<&std::path::Path>,
) -> Result<ToolReport, ToolError> {
    let store = load_store(explicit_workspace)?;
    let matching = store
        .resources()
        .iter()
        .filter(|resource| matches_filters(resource, &filters))
        .collect::<Vec<_>>();
    let total = matching.len();
    let resources = matching
        .into_iter()
        .take(filters.limit)
        .map(CatalogEntry::from_locked)
        .collect();
    Ok(ToolReport::complete(
        "catalog",
        store.workspace().display(),
        ReportPayload::Catalog { total, resources },
    ))
}

fn search(
    query: String,
    filters: CatalogFilters,
    explicit_workspace: Option<&std::path::Path>,
) -> Result<ToolReport, ToolError> {
    let normalized = query.trim().to_lowercase();
    if normalized.is_empty() {
        return Err(ToolError::Usage(
            "search requires a non-empty query; use `catalog` to list all resources".to_owned(),
        ));
    }
    let store = load_store(explicit_workspace)?;
    let mut results = store
        .resources()
        .iter()
        .filter(|resource| matches_filters(resource, &filters))
        .filter_map(|resource| search_result(resource, &normalized))
        .collect::<Vec<_>>();
    results.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.resource.identity.cmp(&right.resource.identity))
    });
    let total = results.len();
    results.truncate(filters.limit);
    Ok(ToolReport::complete(
        "search",
        store.workspace().display(),
        ReportPayload::Search {
            query,
            total,
            results,
        },
    ))
}

fn show(
    resource: &str,
    file: Option<&str>,
    encoding: ContentEncoding,
    explicit_workspace: Option<&std::path::Path>,
) -> Result<ToolReport, ToolError> {
    let identity = parse_identity(resource)?;
    let store = load_store(explicit_workspace)?;
    let locked = store.resource(&identity)?;
    let selected = select_payload(locked, file)?;
    let content = selected
        .map(|payload| direct_content(&store, locked, payload, encoding))
        .transpose()?;
    Ok(ToolReport::complete(
        "show",
        store.workspace().display(),
        ReportPayload::Show {
            resource: Box::new(ResourceDetails::from_locked(locked)),
            content,
        },
    ))
}

fn resolve(
    resources: Vec<String>,
    explicit_workspace: Option<&std::path::Path>,
) -> Result<ToolReport, ToolError> {
    let roots = parse_roots(resources)?;
    let store = load_store(explicit_workspace)?;
    let resources = store
        .closure(&roots)?
        .into_iter()
        .map(ResourceDetails::from_locked)
        .collect();
    Ok(ToolReport::complete(
        "resolve",
        store.workspace().display(),
        ReportPayload::Resolution { roots, resources },
    ))
}

fn context(
    resources: Vec<String>,
    include_dependencies: bool,
    max_files: usize,
    max_bytes: u64,
    explicit_workspace: Option<&std::path::Path>,
) -> Result<ToolReport, ToolError> {
    let roots = parse_roots(resources)?;
    let store = load_store(explicit_workspace)?;
    let include_closure = include_dependencies
        || roots
            .iter()
            .any(|identity| identity.kind == ResourceKind::Collection);
    let selected = if include_closure {
        store.closure(&roots)?
    } else {
        roots
            .iter()
            .map(|identity| store.resource(identity))
            .collect::<Result<Vec<_>, _>>()?
    };

    let mut declared_files = 0usize;
    let mut declared_bytes = 0u64;
    for locked in &selected {
        declared_files = declared_files
            .checked_add(locked.resource.payloads.len())
            .ok_or_else(|| ToolError::Limit("declared file count overflowed".to_owned()))?;
        for payload in &locked.resource.payloads {
            declared_bytes = declared_bytes
                .checked_add(payload.length)
                .ok_or_else(|| ToolError::Limit("declared byte total overflowed".to_owned()))?;
        }
    }

    let mut included_files = 0usize;
    let mut included_bytes = 0u64;
    let mut omissions = Vec::new();
    let mut context_resources = Vec::with_capacity(selected.len());
    for locked in selected {
        ensure_consumable(locked)?;
        let mut payloads = Vec::new();
        for payload in &locked.resource.payloads {
            let next_bytes = included_bytes.checked_add(payload.length);
            let fits =
                included_files < max_files && next_bytes.is_some_and(|bytes| bytes <= max_bytes);
            if !fits {
                omissions.push(Omission {
                    identity: locked.resource.identity.clone(),
                    path: payload.path.clone(),
                    length: payload.length,
                    reason: if included_files >= max_files {
                        "max-files"
                    } else {
                        "max-bytes"
                    },
                });
                continue;
            }
            let bytes = store.payload(locked, payload, max_bytes)?;
            payloads.push(context_content(payload, bytes)?);
            included_files += 1;
            included_bytes = next_bytes.expect("fit check established a bounded sum");
        }
        context_resources.push(ContextResource {
            details: ResourceDetails::from_locked(locked),
            payloads,
        });
    }

    let payload = ReportPayload::Context {
        roots,
        resources: context_resources,
        omissions: omissions.clone(),
        declared_files,
        declared_bytes,
        included_files,
        included_bytes,
    };
    let workspace = store.workspace().display();
    if omissions.is_empty() {
        Ok(ToolReport::complete("context", workspace, payload))
    } else {
        Ok(ToolReport::partial("context", workspace, payload))
    }
}

fn load_store(explicit_workspace: Option<&std::path::Path>) -> Result<Store, ToolError> {
    Store::load(workspace::discover(explicit_workspace)?)
}

fn parse_identity(value: &str) -> Result<ResourceIdentity, ToolError> {
    value
        .parse::<ResourceIdentity>()
        .map_err(|error| ToolError::Usage(format!("invalid resource identity `{value}`: {error}")))
}

fn parse_roots(values: Vec<String>) -> Result<Vec<ResourceIdentity>, ToolError> {
    if values.len() > MAX_RESULT_LIMIT {
        return Err(ToolError::Limit(format!(
            "{} roots exceed the limit of {MAX_RESULT_LIMIT}",
            values.len()
        )));
    }
    let roots = values
        .iter()
        .map(|value| parse_identity(value))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if roots.len() != values.len() {
        return Err(ToolError::Usage("resource roots must be unique".to_owned()));
    }
    Ok(roots.into_iter().collect())
}

fn matches_filters(resource: &LockedResource, filters: &CatalogFilters) -> bool {
    filters
        .kind
        .is_none_or(|kind| resource.resource.identity.kind == kind)
        && filters
            .namespace
            .as_deref()
            .is_none_or(|namespace| resource.resource.identity.namespace == namespace)
        && filters
            .trust
            .is_none_or(|trust| resource.resource.trust.class == trust)
}

fn search_result(resource: &LockedResource, query: &str) -> Option<SearchResult> {
    let identity = resource.resource.identity.to_string().to_lowercase();
    let title = resource.resource.title.to_lowercase();
    let summary = resource.resource.summary.to_lowercase();
    let mut matched_fields = Vec::new();
    let mut score = 0u16;
    if identity == query {
        matched_fields.push("identity".to_owned());
        score = score.max(100);
    } else if identity.starts_with(query) {
        matched_fields.push("identity".to_owned());
        score = score.max(80);
    } else if identity.contains(query) {
        matched_fields.push("identity".to_owned());
        score = score.max(70);
    }
    if title == query {
        matched_fields.push("title".to_owned());
        score = score.max(60);
    } else if title.contains(query) {
        matched_fields.push("title".to_owned());
        score = score.max(50);
    }
    if summary.contains(query) {
        matched_fields.push("summary".to_owned());
        score = score.max(30);
    }
    if resource
        .resource
        .authors
        .iter()
        .any(|author| author.to_lowercase().contains(query))
    {
        matched_fields.push("authors".to_owned());
        score = score.max(20);
    }
    if resource
        .resource
        .capabilities
        .iter()
        .any(|capability| capability.to_lowercase().contains(query))
    {
        matched_fields.push("capabilities".to_owned());
        score = score.max(20);
    }
    (!matched_fields.is_empty()).then(|| SearchResult {
        score,
        matched_fields,
        resource: CatalogEntry::from_locked(resource),
    })
}

fn select_payload<'a>(
    locked: &'a LockedResource,
    requested: Option<&str>,
) -> Result<Option<&'a PayloadMetadata>, ToolError> {
    if let Some(requested) = requested {
        return locked
            .resource
            .payloads
            .iter()
            .find(|payload| payload.path == requested)
            .map(Some)
            .ok_or_else(|| ToolError::PayloadNotFound {
                resource: locked.resource.identity.to_string(),
                payload: requested.to_owned(),
            });
    }
    match locked.resource.payloads.as_slice() {
        [] => Ok(None),
        [payload] => Ok(Some(payload)),
        payloads => Err(ToolError::Usage(format!(
            "resource `{}` has {} payloads; select one with `--file`",
            locked.resource.identity,
            payloads.len()
        ))),
    }
}

fn direct_content(
    store: &Store,
    locked: &LockedResource,
    payload: &PayloadMetadata,
    encoding: ContentEncoding,
) -> Result<ContentItem, ToolError> {
    if encoding == ContentEncoding::Text && !is_textual(&payload.media_type) {
        return Err(ToolError::BinaryOutputRequired {
            resource: locked.resource.identity.to_string(),
            payload: payload.path.clone(),
        });
    }
    let bytes = store.payload(locked, payload, MAX_SHOW_BYTES)?;
    let (encoding, content) = match encoding {
        ContentEncoding::Text => (
            ContentRepresentation::Text,
            String::from_utf8(bytes).map_err(|_| ToolError::InvalidText {
                resource: locked.resource.identity.to_string(),
                payload: payload.path.clone(),
            })?,
        ),
        ContentEncoding::Base64 => (ContentRepresentation::Base64, STANDARD.encode(bytes)),
    };
    Ok(content_item(payload, encoding, Some(content)))
}

fn context_content(payload: &PayloadMetadata, bytes: Vec<u8>) -> Result<ContentItem, ToolError> {
    if is_textual(&payload.media_type) {
        let content = String::from_utf8(bytes).map_err(|_| {
            ToolError::Integrity(format!(
                "payload `{}` declares textual media type `{}` but is not valid UTF-8",
                payload.path, payload.media_type
            ))
        })?;
        Ok(content_item(
            payload,
            ContentRepresentation::Text,
            Some(content),
        ))
    } else {
        Ok(content_item(
            payload,
            ContentRepresentation::BinaryReference,
            None,
        ))
    }
}

fn content_item(
    payload: &PayloadMetadata,
    encoding: ContentRepresentation,
    content: Option<String>,
) -> ContentItem {
    ContentItem {
        path: payload.path.clone(),
        media_type: payload.media_type.clone(),
        length: payload.length,
        sha256: payload.sha256.clone(),
        encoding,
        content,
        instruction_boundary: INSTRUCTION_BOUNDARY,
    }
}

fn is_textual(media_type: &str) -> bool {
    let media_type = media_type
        .split_once(';')
        .map_or(media_type, |(value, _)| value)
        .trim()
        .to_ascii_lowercase();
    media_type.starts_with("text/")
        || matches!(
            media_type.as_str(),
            "application/json"
                | "application/ld+json"
                | "application/toml"
                | "application/x-toml"
                | "application/yaml"
                | "application/x-yaml"
                | "application/xml"
                | "application/javascript"
        )
        || media_type.ends_with("+json")
        || media_type.ends_with("+xml")
}

#[cfg(test)]
mod tests {
    use super::is_textual;

    #[test]
    fn classifies_explicit_textual_media_types_only() {
        assert!(is_textual("text/markdown; charset=utf-8"));
        assert!(is_textual("application/json"));
        assert!(is_textual("application/problem+json"));
        assert!(is_textual("image/svg+xml"));
        assert!(!is_textual("application/octet-stream"));
    }
}
