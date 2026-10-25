use std::{collections::HashMap, error::Error, process::Command};

use crate::{FunctionCall, NeedleData};

pub fn parse_command(cmd: &str) -> Result<NeedleData, Box<dyn Error>> {
    let out = Command::new("engine/linux-x86_64/needle")
        .args([
            "--model",
            "engine/linux-x86_64/needle3.cact",
            "--prompt",
            cmd,
            "--tools",
            "./tools.engine.json",
        ])
        .output()?;
    let output = String::from_utf8_lossy(&out.stdout).to_lowercase();
    println!("parsed: {output}");
    Ok(serde_json::from_str(&output)?)
}

pub fn parse_tools(calls: Vec<FunctionCall>) -> Result<(), Box<dyn Error>> {
    for call in calls {
        let name = call.name.as_str();
        match name {
            "play_music" => {
                Command::new("limusic-app").output()?;
            }
            "set_volume" => {
                let volume = call.arguments.get("level").unwrap().as_u64().unwrap();
                let output = Command::new("amixer")
                    .args(["-q", "sset", "Master", &format!("{}%", volume)])
                    .output()?;
                println!("volume: {}", String::from_utf8_lossy(&output.stdout));
            }
            _ => {}
        }
    }
    Ok(())
}

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

    let call = FunctionCall {
        name: best_match.0.clone(),
        arguments: serde_json::Map::new(),
    };
    Ok(())
}
