//! Rendering and error translation for embedded validation knowledge.

use crate::knowledge;
use std::{fmt::Write as _, io::Write};

pub(super) enum KnowledgeError {
    Selection(String),
    Internal(String),
}

pub(super) fn catalog(json: bool, output: &mut impl Write) -> Result<(), KnowledgeError> {
    if json {
        return write_exact(output, knowledge::catalog_bytes()?).map_err(KnowledgeError::Internal);
    }

    let catalog = knowledge::catalog().map_err(KnowledgeError::Internal)?;
    let mut rendered = String::new();
    writeln!(
        rendered,
        "Embedded validation knowledge ({} documents)",
        catalog.documents.len()
    )
    .expect("writing to a string cannot fail");
    for document in catalog.documents {
        writeln!(
            rendered,
            "{} [{}] — {}\n  applicability: {}",
            document.id,
            document.kind,
            document.summary,
            document.applicability.join(", ")
        )
        .expect("writing to a string cannot fail");
    }
    write_exact(output, rendered.as_bytes()).map_err(KnowledgeError::Internal)
}

pub(super) fn show(document_id: &str, output: &mut impl Write) -> Result<(), KnowledgeError> {
    if !knowledge::valid_document_id(document_id) {
        return Err(KnowledgeError::Selection(format!(
            "invalid knowledge document ID {document_id:?}"
        )));
    }
    let bytes = knowledge::document(document_id)
        .map_err(KnowledgeError::Internal)?
        .ok_or_else(|| {
            KnowledgeError::Selection(format!("unknown knowledge document ID {document_id:?}"))
        })?;
    write_exact(output, bytes).map_err(KnowledgeError::Internal)
}

impl From<String> for KnowledgeError {
    fn from(value: String) -> Self {
        Self::Internal(value)
    }
}

fn write_exact(output: &mut impl Write, bytes: &[u8]) -> Result<(), String> {
    output
        .write_all(bytes)
        .and_then(|()| output.flush())
        .map_err(|error| format!("cannot write standard output: {error}"))
}
