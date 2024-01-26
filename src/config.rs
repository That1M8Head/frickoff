use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::de::Error;
use serde::Serialize;
use serde::Deserialize;

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
        let pathdefs = HashMap::new();

        Config {
            frickoff,
            pathdefs,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub serious: bool,
    pub paranoid: bool,
}

pub fn read_config(file_path: &str) -> Result<Config, Error> {
    let toml_content = fs::read_to_string(file_path).unwrap_or(String::new());
    let config: Config = toml::from_str(&toml_content)?;
    Ok(config)
}

pub fn config_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

pub fn create_config(file_path: &str) {
    let config = Config::new();
    let toml_content = toml::to_string(&config).unwrap();
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent).unwrap();
    } 
    fs::write(file_path, toml_content).unwrap();
}
