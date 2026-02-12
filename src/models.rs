use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SemanticSymbol {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub scope: Option<String>,
    pub snapshot_id: i64,
    pub chunk_hash: String,
    pub structural_hash: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_byte: usize,
    pub end_byte: usize,
    pub parent_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SymbolReference {
    pub symbol_name: String,
    pub snapshot_id: i64,
    pub start_line: usize,
    pub start_byte: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeltaKind {
    Added,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SemanticDelta {
    pub id: i64,
    pub project_id: Option<String>,
    pub from_snapshot_id: Option<i64>,
    pub to_snapshot_id: i64,
    pub symbol_name: String,
    pub new_name: Option<String>,
    pub kind: DeltaKind,
    pub structural_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub hash: String,
    pub content: Vec<u8>,
    pub kind: String,
}
