use serde_json::{Map, Value};
use std::{collections::HashMap, error::Error, process::Command};

use crate::{
    FunctionCall, NeedleData, ParseResult,
    config::{Config, tools::ToolDef},
};

/// converts speech to tool calls
pub fn parse_speech(config: &Config, cmd: &str) -> Result<ParseResult, Box<dyn Error>> {
    let out = Command::new("needle-engine")
        .args([
            "--model",
            &config.parse_model_path,
            "--prompt",
            cmd,
            "--tools",
            "/tmp/tools.engine.json",
        ])
        .output()?;
    let output = String::from_utf8_lossy(&out.stdout).to_lowercase();
    println!("parsed speech: {output}");
    let data = serde_json::from_str::<NeedleData>(&output)?;
    let result = ParseResult {
        confidence: data.confidence,
        calls: data.function_calls,
    };
    Ok(result)
}

const NEGATIONS: [&str; 5] = ["never", "don't", "dont", "not", "no"];
const WORD_MATCH: f64 = 0.9; // closeness to the target word
const MIN_SCORE: f64 = 1.0; // roughly "at least one exact keyword"
const MARGIN: f64 = 0.5; // winner must beat the runner-up by this much

// NOTE: still in dev so ignore this
fn normalize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .replace('’', "'")
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(String::from)
        .collect()
}

fn best_similarity(words: &[String], kw: &str) -> f64 {
    words
        .iter()
        .map(|w| strsim::jaro_winkler(w, kw))
        .fold(0.0, f64::max)
}

fn _extract_args(name: &str, words: &[String]) -> Option<Map<String, Value>> {
    let mut args = Map::new();
    if name == "set_volume" {
        let n: i64 = words.iter().find_map(|w| w.parse().ok())?;
        if !(0..=100).contains(&n) {
            return None;
        }
        args.insert("level".into(), n.into());
    }
    Some(args)
}

pub fn parse_speech_manual(
    tools: &HashMap<String, ToolDef>,
    output: &str,
) -> Result<ParseResult, Box<dyn Error>> {
    let none = || ParseResult {
        confidence: 0.0,
        calls: vec![],
    };
    let words = normalize(output);

    for w in &words {
        if NEGATIONS.contains(&w.as_str()) {
            return Ok(none());
        }
    }

    // One score per tool
    let mut scored: Vec<(&String, f64)> = tools
        .iter()
        .filter_map(|(name, def)| {
            let score: f64 = def
                .keywords
                .iter()
                .map(|k| best_similarity(&words, &k.to_lowercase()))
                .filter(|&s| s >= WORD_MATCH)
                .sum();
            if score >= MIN_SCORE {
                Some((name, score))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by(|a, b| b.1.total_cmp(&a.1));

    let Some(&(name, score)) = scored.first() else {
        return Ok(none());
    };
    if let Some(&(_, second)) = scored.get(1)
        && score - second < MARGIN
    {
        // words too similar, ignore
        return Ok(none());
    }

    // Arguments unusable in manual mode. Sorry!
    // let Some(arguments) = extract_args(name, &words) else {
    //     return Ok(none());
    // };

    Ok(ParseResult {
        confidence: (score as f32 / 2.0).min(1.0),
        calls: vec![FunctionCall {
            name: name.clone(),
            arguments: Map::new(),
        }],
    })
}
