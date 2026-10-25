use std::{collections::HashMap, error::Error, fs, process::Command};

use serde::Deserialize;

pub struct Tool {
    pub name: String,
    pub script: String,
    pub params: HashMap<String, String>,
}

impl Tool {
    pub fn parse(path: Option<String>) -> Result<Vec<Self>, Box<dyn Error>> {
        let path = if let Some(path) = path {
            path
        } else {
            "tools.json".into()
        };
        let content = fs::read_to_string(&path)?;
        let parsed: Vec<ToolDef> = serde_json::from_str(&content)?;
        let tools: HashMap<String, ToolDef> =
            parsed.into_iter().map(|f| (f.name.clone(), f)).collect();

        let mut raw: Vec<serde_json::Value> = serde_json::from_str(&content)?;
        for t in &mut raw {
            if let Some(o) = t.as_object_mut() {
                o.remove("run");
                o.remove("confirm");
            }
        }
        std::fs::write("tools.engine.json", serde_json::to_string(&raw)?)?;

        Ok(vec![])
    }

    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        let mut script = self.script.split_whitespace();
        let Some(command) = script.next() else {
            return Err("".into());
        };
        let mut output = Command::new(command);
        for c in script {
            output.arg(c);
        }
        let output = output.output()?;
        println!("Out: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}

#[derive(Deserialize)]
struct ToolDef {
    name: String,
    #[serde(default)]
    run: Option<String>,
    #[serde(default)]
    confirm: bool,
}

#[derive(Deserialize)]
struct Call {
    name: String,
    #[serde(default)]
    arguments: serde_json::Map<String, serde_json::Value>,
}
