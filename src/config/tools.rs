use std::{collections::HashMap, error::Error, fs, process::Command};

use serde::Deserialize;
use serde_json::Value;

use crate::FunctionCall;

pub struct Tool {
    pub name: String,
    pub script: String,
    pub params: HashMap<String, ToolDef>,
}

pub fn parse_tools(path: Option<String>) -> Result<HashMap<String, ToolDef>, Box<dyn Error>> {
    let path = if let Some(path) = path {
        path
    } else {
        "tools.json".into()
    };
    let content = fs::read_to_string(&path)?;
    let parsed: Vec<ToolDef> = serde_json::from_str(&content)?;

    let tools: HashMap<String, ToolDef> = parsed.into_iter().map(|f| (f.name.clone(), f)).collect();

    let mut raw: Vec<serde_json::Value> = serde_json::from_str(&content)?;
    for t in &mut raw {
        if let Some(o) = t.as_object_mut() {
            o.remove("run");
        }
    }
    let content = serde_json::to_string(&raw)?;
    std::fs::write("/tmp/tools.engine.json", content)?;

    Ok(tools)
}

/// runs tools provided in the map and call
pub fn run_tools(
    tools: &HashMap<String, ToolDef>,
    calls: &[FunctionCall],
) -> Result<(), Box<dyn Error>> {
    for f in calls {
        let tool = tools.get(&f.name).ok_or("unknown tool")?;
        let mut script = tool
            .run
            .as_deref()
            .ok_or("tool has no run script")?
            .to_string();

        for (k, v) in &f.arguments {
            let val = match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            script = script.replace(&format!("{{{k}}}"), &val);
        }
        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(script);
        let status = cmd.status()?;
        if !status.success() {
            return Err(format!("{} exited with {status}", f.name).into());
        }
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct ToolDef {
    pub name: String,
    #[serde(default)]
    pub run: Option<String>,
    #[serde(default)]
    pub confirm: bool,
}
