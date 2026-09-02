use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcerConfig {
    pub mode: String,
    pub profile: String,
    pub coolboost: bool,
    pub cpu_fan_target: u8,
    pub gpu_fan_target: u8,
    pub battery_health_80: bool,
    pub winkey_locked: bool,
    pub touchpad_locked: bool,
}

impl Default for AcerConfig {
    fn default() -> Self {
        Self {
            mode: "auto".to_string(),
            profile: "balanced".to_string(),
            coolboost: false,
            cpu_fan_target: 50,
            gpu_fan_target: 50,
            battery_health_80: false,
            winkey_locked: false,
            touchpad_locked: false,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home).join(".config/acersense/config.json")
}

pub fn load_config() -> AcerConfig {
    let path = get_config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<AcerConfig>(&content) {
            return cfg;
        }
    }
    AcerConfig::default()
}

pub fn save_config(cfg: &AcerConfig) {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(cfg) {
        let _ = fs::write(&path, json);
    }
}
