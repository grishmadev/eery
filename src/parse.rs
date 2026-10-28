use std::{collections::HashMap, error::Error, process::Command};

use crate::{NeedleData, config::Config};

/// converts speech to tool calls
pub fn parse_speech(config: &Config, cmd: &str) -> Result<NeedleData, Box<dyn Error>> {
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
    let data = serde_json::from_str(&output)?;
    Ok(data)
}

// NOTE: still in dev so ignore this
pub fn parse_tools_manual(output: String) -> Result<(), Box<dyn Error>> {
    let negation = ["never", "don't", "do not", "not", "no"];
    let mut tool_names: HashMap<String, Vec<String>> = HashMap::new();
    let captured = output.split_whitespace().collect::<Vec<_>>();
    tool_names.insert(
        "play_music".into(),
        ["song", "songs", "music", "play", "tunes"]
            .iter_mut()
            .map(|g| g.to_string())
            .collect(),
    );
    for n in negation {
        if captured.contains(&n) {
            return Ok(());
        }
    }

    let mut candidates = vec![];
    for c in captured {
        for (name, keywords) in &tool_names {
            let mut points = 0.0;
            for k in keywords {
                let match_value = strsim::jaro_winkler(c, k);
                println!("matching {} / {}: {}", c, k, match_value);
                points += match_value;
            }
            println!("points for {}: {}", name, points);
            if points >= 0.5 {
                println!("potential candidates: {}", name);
                candidates.push((name.clone(), points));
            }
        }
    }

    let mut candidates = candidates.iter();
    let Some(mut best_match) = candidates.next() else {
        return Ok(());
    };
    for c in candidates {
        if c.1 > best_match.1 {
            best_match = c;
        }
    }
    println!("best match: {}", best_match.0);

    Ok(())
}
