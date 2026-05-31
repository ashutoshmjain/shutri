//! Configuration management for #SMS paths

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub mdbook_repo: PathBuf,
    pub social_repo: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/amj"));
        Self {
            mdbook_repo: home.join("github/deepDive"),
            social_repo: home.join("github/social-media"),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("sms");
    
    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        fs::create_dir_all(&config_dir)?;
        let default_config = Config::default();
        let toml = str::replace(&toml::to_string(&default_config)?, "\"", "\"");
        fs::write(&config_path, toml)?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
