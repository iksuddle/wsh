use serde::Deserialize;
use std::{
    io::{self},
    path::PathBuf,
};

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Toml(toml::de::Error),
}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::Toml(value)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub prompt: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prompt: "> ".to_owned(),
        }
    }
}
impl Config {
    pub fn build(path: Option<PathBuf>) -> Result<Config, ConfigError> {
        let config_path = match path {
            Some(path) => path,
            None => {
                let xdg_dirs = xdg::BaseDirectories::with_prefix("wsh");
                xdg_dirs.place_config_file("config.toml")?
            }
        };

        let config_data = std::fs::read_to_string(config_path)?;

        Ok(toml::from_str(&config_data)?)
    }
}
