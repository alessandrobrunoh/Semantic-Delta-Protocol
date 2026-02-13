use serde::{Deserialize, Serialize};
use crate::models::{SemanticSymbol, SemanticRecord, SymbolReference};

/// SRP Protocol Version
pub const SRP_VERSION: &str = "0.1.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct SrpRequest {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SrpResponse {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub result: Option<serde_json::Value>,
    pub error: Option<SrpRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SrpRpcError {
    pub code: i32,
    pub message: String,
}

/// Core methods of the Semantic Registry Protocol
pub mod methods {
    /// Analyze a file and return semantic symbols
    pub const ANALYZE: &str = "srp/analyze";
    /// Compare two sets of symbols and return records
    pub const DIFF: &str = "srp/diff";
    /// Get the semantic history of a symbol
    pub const GET_HISTORY: &str = "srp/history";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeParams {
    pub content: String, // Base64 or raw string
    pub extension: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub symbols: Vec<SemanticSymbol>,
    pub references: Vec<SymbolReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffParams {
    pub base_symbols: Vec<SemanticSymbol>,
    pub target_symbols: Vec<SemanticSymbol>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffResult {
    pub records: Vec<SemanticRecord>,
}
