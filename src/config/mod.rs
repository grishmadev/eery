pub mod tools;
use std::process;

pub struct Config {
    keywords_path: String,
    tools_path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    InvalidCommand(String),
    NoValue(String),
}

impl Config {
    pub fn default() -> Self {
        Self {
            keywords_path: "".into(),
            tools_path: "".into(),
        }
    }

    pub fn get_args() -> Result<Config, ConfigError> {
        let mut args = std::env::args().skip(1);
        let mut config: Config = Self::default();
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
            } else {
                return Err(ConfigError::InvalidCommand(arg));
            }
        }
        Ok(config)
    }
}

pub fn show_help() {
    println!("This is Help!");
}
