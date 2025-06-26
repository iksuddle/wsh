use serde::Deserialize;
use std::{
    error::Error,
    fs::{OpenOptions, create_dir_all},
    io::Read,
    path::PathBuf,
};

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
    pub fn build(path: Option<PathBuf>) -> Result<Config, Box<dyn Error>> {
        let path = match path {
            Some(p) => p,
            None => {
                let mut default_path = dirs::config_local_dir().unwrap();
                default_path.push("wsh");
                default_path.push("config.toml");
                default_path
            }
        };

        if let Some(config_folder) = path.parent() {
            create_dir_all(config_folder)?;
        }

        // println!("config file is: {:?}", path);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let mut config_data = String::new();
        file.read_to_string(&mut config_data)?;

        Ok(toml::from_str(&config_data)?)
    }
}
