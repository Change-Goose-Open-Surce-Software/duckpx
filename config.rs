vuse serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub background_color: String,
    pub foreground_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            background_color: "#FFFFFF".to_string(),
            foreground_color: "#FFA500".to_string(),
        }
    }
}

pub fn load_config() -> Config {
    let config_path = Path::new("config.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap_or_else(|_| Config::default())
    } else {
        let config = Config::default();
        fs::write(config_path, toml::to_string(&config).unwrap()).unwrap();
        config
    }
}
