use serde::{Deserialize, Serialize};
use std::fs;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    pub square: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UI {
    pub toolbar_position: String,  // "top", "bottom", "left", "right"
    pub manual_sidebar_position: String,  // "top", "bottom", "left", "right"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub colors: Colors,
    pub ui: UI,
    pub language: String,  // "de", "en", "fr", "ru", "zh"
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
                    square: "#FFA500".to_string(),
                },
                ui: UI {
                    toolbar_position: "top".to_string(),
                    manual_sidebar_position: "left".to_string(),
                },
                language: Self::detect_language(),
            };
            let toml_string = toml::to_string(&default_config).unwrap();
            fs::write(&config_path, toml_string).unwrap();
            return default_config;
        }

        // Lade bestehende Konfiguration
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    }

    fn detect_language() -> String {
        // Versuche Systemsprache zu erkennen
        if let Ok(lang) = std::env::var("LANG") {
            let lang_lower = lang.to_lowercase();
            if lang_lower.starts_with("de") {
                return "de".to_string();
            } else if lang_lower.starts_with("en") {
                return "en".to_string();
            } else if lang_lower.starts_with("fr") {
                return "fr".to_string();
            } else if lang_lower.starts_with("ru") {
                return "ru".to_string();
            } else if lang_lower.starts_with("zh") {
                return "zh".to_string();
            }
        }
        
        // Fallback auf Englisch
        "en".to_string()
    }
}
