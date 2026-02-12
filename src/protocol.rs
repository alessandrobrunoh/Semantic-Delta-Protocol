use serde::{Deserialize, Serialize};
use crate::models::{SemanticSymbol, SemanticDelta, SymbolReference};

/// SDP Protocol Version
pub const SDP_VERSION: &str = "0.1.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct SdpRequest {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SdpResponse {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub result: Option<serde_json::Value>,
    pub error: Option<SdpRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SdpRpcError {
    pub code: i32,
    pub message: String,
}

/// Core methods of the Semantic Delta Protocol
pub mod methods {
    /// Analyze a file and return semantic symbols
    pub const ANALYZE: &str = "sdp/analyze";
    /// Compare two sets of symbols and return deltas
    pub const DIFF: &str = "sdp/diff";
    /// Get the semantic history of a symbol
    pub const GET_HISTORY: &str = "sdp/history";
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
    pub deltas: Vec<SemanticDelta>,
}
