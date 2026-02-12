use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSymbol {
    pub id: Option<i64>,
    pub snapshot_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub kind: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_byte: usize,
    pub end_byte: usize,
    pub structural_hash: String,
    pub chunk_hash: String,
}
