pub mod tools;
use std::{
    env,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug)]
pub struct Config {
    /// set keywords path
    pub keywords_path: String,
    /// set tools path
    pub tools_path: String,
    /// set voice model path
    pub voice_model_path: String,
    /// set parse model path
    pub parse_model_path: String,
    /// Audio file path
    pub audio: String,
    /// Parse via model or manually
    pub manual: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    InvalidCommand(String),
    NoValue(String),
}

impl Config {
    pub fn get_config_path() -> PathBuf {
        let home = env::home_dir().unwrap();
        Path::new(&home).join(".config/eery/")
    }

    pub fn get_default() -> Self {
        let config_path = Self::get_config_path();
        let keywords_path = config_path.join("keywords.txt").into_string().unwrap();
        let tools_path = config_path.join("tools.json").into_string().unwrap();
        let voice_model_path = config_path.join("whistle.cact").into_string().unwrap();
        let parse_model_path = config_path.join("needle3.cact").into_string().unwrap();
        let audio = "/tmp/eery.wav".into();
        let manual = false;
        Self {
            keywords_path,
            tools_path,
            voice_model_path,
            parse_model_path,
            audio,
            manual,
        }
    }

    pub fn get_args() -> Result<Config, ConfigError> {
        let mut args = std::env::args().skip(1);
        let mut config: Config = Self::get_default();
        let extract = |f: Option<String>, arg: String| -> Result<String, ConfigError> {
            if let Some(word) = f {
                Ok(word)
            } else {
                Err(ConfigError::NoValue(arg))
            }
        };
        while let Some(arg) = args.next() {
            if ["--help", "-h"].contains(&arg.as_str()) {
                show_help();
                process::exit(0);
            } else if arg == "--keywords" {
                config.keywords_path = extract(args.next(), arg)?;
            } else if arg == "--tools" {
                config.tools_path = extract(args.next(), arg)?;
            } else if arg == "--voice-model" {
                config.voice_model_path = extract(args.next(), arg)?;
            } else if arg == "--parse-model" {
                config.parse_model_path = extract(args.next(), arg)?;
            } else if arg == "--audio" {
                config.audio = extract(args.next(), arg)?;
            } else if arg == "--manual" {
                config.manual = true;
            } else {
                return Err(ConfigError::InvalidCommand(arg));
            }
        }
        Ok(config)
    }
}

pub fn show_help() {
    println!(
        "You can look into src/config/mod.rs for a glance on usage.\n\rIt's fairly simple for even a non-rust background."
    );
}
