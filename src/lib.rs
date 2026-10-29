pub mod config;
pub mod parse;
pub mod voice;
use std::{env, error::Error, fs, path::Path, process::Command};

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
    pub suppressed_calls: Vec<FunctionCall>,
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

pub fn setup() -> Result<(), Box<dyn Error>> {
    match Command::new("needle-engine").output() {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("needle-engine not found. Run install.sh first.");
            std::process::exit(1);
        }
        Err(e) => return Err(e.into()),
    }

    let home = env::home_dir().unwrap();
    let config_path = Path::new(&home).join(".config/eery");
    fs::create_dir_all(&config_path)?;
    let tools_path = config_path.join("tools.json");
    if !tools_path.exists() {
        fs::write(&tools_path, "[]")?;
    }
    let keywords_path = config_path.join("keywords.txt");
    if !keywords_path.exists() {
        fs::write(&keywords_path, "")?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct ParseResult {
    pub confidence: f32,
    pub calls: Vec<FunctionCall>,
}
