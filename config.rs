use serde::{Deserialize, Serialize};
use std::fs;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    pub background: String,
    pub foreground: String,
    pub square: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UI {
    pub toolbar_position: String,  // "top" oder "bottom"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub colors: Colors,
    pub ui: UI,
}

impl Config {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir()
            .expect("Konnte Konfigurationsverzeichnis nicht finden")
            .join("duckpx");

        let config_path = config_dir.join("config.toml");

        // Erstelle Verzeichnis, falls nicht vorhanden
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).unwrap();
        }

        // Standardkonfiguration, falls keine existiert
        if !config_path.exists() {
            let default_config = Config {
                colors: Colors {
                    background: "#FFFFFF".to_string(),
                    foreground: "#000000".to_string(),
                    square: "#FFA500".to_string(),
                },
                ui: UI {
                    toolbar_position: "top".to_string(),
                },
            };
            let toml_string = toml::to_string(&default_config).unwrap();
            fs::write(&config_path, toml_string).unwrap();
            return default_config;
        }

        // Lade bestehende Konfiguration
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
