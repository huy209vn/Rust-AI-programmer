//! Shared types & utilities.
#![deny(warnings)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub doc_id: String,
    pub start: usize,
    pub end: usize,
    pub sha256: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub class: String, // borrow | trait | type | vis | syntax | move | ...
    pub file: String,
    pub line: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchBundle {
    pub id: String,
    pub files_changed: u32,
    pub lines_changed: u32,
    pub diff: String,
    pub anchors: Vec<Anchor>,
    pub risk: String,
    pub confidence: f32,
}
