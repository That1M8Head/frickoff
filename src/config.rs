use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use toml::de::Error;
use serde::Serialize;
use serde::Deserialize;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub frickoff: AppConfig,
    pub pathdefs: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        let default_config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config_default.toml");
        let default_config_content = fs::read_to_string(default_config_path).unwrap();
        let default_config: Config = toml::from_str(&default_config_content).unwrap();

        let frickoff = default_config.frickoff;
        let pathdefs = default_config.pathdefs;

        Config {
            frickoff,
            pathdefs,
        }
    }

}

fn expand_path(path: &str) -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_dir = dirs::config_dir().expect("Failed to get config directory");

    let path = path
        .replace("HOME", home_dir.to_str().unwrap())
        .replace("CONFIG", config_dir.to_str().unwrap())
        .replace("\\", "/");

    path
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub serious: bool,
    pub paranoid: bool,
}

pub fn read_config(file_path: &PathBuf) -> Result<Config, Error> {
    let toml_content = fs::read_to_string(file_path).unwrap_or(String::new());
    let mut config: Config = toml::from_str(&toml_content)?;
    for (_, path) in config.pathdefs.iter_mut() {
        *path = expand_path(path);
    }
    Ok(config)
}

pub fn config_exists(file_path: &PathBuf) -> bool {
    Path::new(&file_path).exists()
}

pub fn create_config(file_path: &PathBuf) {
    let config = Config::new();
    let toml_content = toml::to_string(&config).unwrap();
    if let Some(parent) = Path::new(&file_path).parent() {
        fs::create_dir_all(parent).unwrap();
    } 
    fs::write(&file_path, toml_content).unwrap();
}
