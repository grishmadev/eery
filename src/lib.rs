pub mod config;
pub mod parse;
pub mod voice;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WhistleData {
    pub text: String,
    pub language: String,
    pub ttft_ms: f32,
    pub decode_tps: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedleData {
    // pub type: String,
    pub success: bool,
    pub error: Option<String>,
    pub error_code: Option<String>,
    pub reason: Option<String>,
    pub function_calls: Vec<FunctionCall>,
    pub suppressed_calls: Vec<String>,
    pub reasoning: String,
    pub confidence: f32,
    pub prefill_tps: f32,
    pub decode_tps: f32,
    pub peak_ram_mb: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: serde_json::Map<String, serde_json::Value>,
}
