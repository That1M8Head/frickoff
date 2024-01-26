use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use toml::de::Error;
use serde::Serialize;
use serde::Deserialize;
use dirs;

/// The `Config` struct represents the configuration file.
/// 
/// # Fields
/// * `frickoff` - The `AppConfig` struct containing the Frick Off configuration.
/// * `pathdefs` - A hashmap containing the paths to text editors or other
/// utilities that Frick Off accesses.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub frickoff: AppConfig,
    pub pathdefs: HashMap<String, String>,
}

/// The implementation of the `Config` struct.
/// 
/// # Methods
/// * `new()` - Creates a new `Config` struct.
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

/// Expands the `HOME` and `CONFIG` variables in the path and replaces
/// Windows-style backslashes with forward slashes.
/// 
/// # Arguments
/// * `path` - The path to expand.
/// 
/// # Returns
/// * `String` - The expanded path.
fn expand_path(path: &str) -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_dir = dirs::config_dir().expect("Failed to get config directory");
    let config_local_dir = dirs::config_local_dir().expect("Failed to get local config directory");

    let path = path
        .replace("HOME", home_dir.to_str().unwrap())
        .replace("CONFIG", config_dir.to_str().unwrap())
        .replace("LOCAL", config_local_dir.to_str().unwrap())
        .replace("\\", "/");

    path
}

/// The `AppConfig` struct represents the configuration file for Frick Off,
/// not the paths to text editors or other utilities.
/// 
/// # Fields
/// * `serious` - Whether or not Frick Off should be serious.
/// * `paranoid` - Whether or not Frick Off should be paranoid.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub serious: bool,
    pub paranoid: bool,
}

/// Reads the configuration file and returns a `Config` struct.
/// 
/// # Arguments
/// * `file_path` - The path to the configuration file.
/// 
/// # Returns
/// * `Result<Config, Error>` - The configuration file, or an error.
pub fn read_config(file_path: &PathBuf) -> Result<Config, Error> {
    let toml_content = fs::read_to_string(file_path).unwrap_or(String::new());
    let mut config: Config = toml::from_str(&toml_content)?;
    for (_, path) in config.pathdefs.iter_mut() {
        *path = expand_path(path);
    }
    Ok(config)
}

/// Checks if the configuration file exists.
/// 
/// # Arguments
/// * `file_path` - The path to the configuration file.
/// 
/// # Returns
/// * `bool` - Whether or not the configuration file exists.
pub fn config_exists(file_path: &PathBuf) -> bool {
    Path::new(&file_path).exists()
}

/// Creates the configuration file if it doesn't exist.
/// 
/// # Arguments
/// * `file_path` - The path to the configuration file.
/// 
/// # Returns
/// * `()` - Nothing.
pub fn create_config(file_path: &PathBuf) {
    let config = Config::new();
    let toml_content = toml::to_string(&config).unwrap();
    if let Some(parent) = Path::new(&file_path).parent() {
        fs::create_dir_all(parent).unwrap();
    } 
    fs::write(&file_path, toml_content).unwrap();
}
