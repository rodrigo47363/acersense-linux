use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcerConfig {
    pub mode: String,
    pub profile: String,
    pub coolboost: bool,
    pub cpu_fan_target: u8,
    pub gpu_fan_target: u8,
    #[serde(default)]
    pub sync_fans: bool,
    pub battery_health_80: bool,
    pub winkey_locked: bool,
    pub touchpad_locked: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_theme() -> String {
    "human".to_string()
}

impl Default for AcerConfig {
    fn default() -> Self {
        Self {
            mode: "auto".to_string(),
            profile: "balanced".to_string(),
            coolboost: false,
            cpu_fan_target: 50,
            gpu_fan_target: 50,
            sync_fans: false,
            battery_health_80: false,
            winkey_locked: false,
            touchpad_locked: false,
            theme: "human".to_string(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    // 1. If running under sudo / pkexec, resolve the invoking real user's config
    if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        if !sudo_user.is_empty() && sudo_user != "root" {
            let p = PathBuf::from(format!("/home/{}/.config/acersense/config.json", sudo_user));
            return p;
        }
    }
    // 2. Global system config fallback (used by systemd service if user config not found)
    let sys_path = PathBuf::from("/etc/acersense/config.json");
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let user_path = PathBuf::from(home).join(".config/acersense/config.json");

    if !user_path.exists() && sys_path.exists() {
        return sys_path;
    }
    user_path
}

pub fn load_config() -> AcerConfig {
    let path = get_config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<AcerConfig>(&content) {
            return cfg;
        }
    }
    // Fallback to /etc/acersense/config.json if user path didn't exist
    if let Ok(content) = fs::read_to_string("/etc/acersense/config.json") {
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
        let _ = fs::write(&path, &json);
        // Also sync to /etc/acersense/config.json if root permissions are available
        let sys_path = PathBuf::from("/etc/acersense/config.json");
        if let Some(parent) = sys_path.parent() {
            if fs::create_dir_all(parent).is_ok() {
                let _ = fs::write(sys_path, &json);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = AcerConfig::default();
        assert_eq!(cfg.mode, "auto");
        assert_eq!(cfg.profile, "balanced");
        assert!(!cfg.coolboost);
        assert_eq!(cfg.cpu_fan_target, 50);
        assert_eq!(cfg.gpu_fan_target, 50);
        assert!(!cfg.sync_fans);
        assert!(!cfg.battery_health_80);
        assert_eq!(cfg.theme, "human");
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let cfg = AcerConfig {
            mode: "custom".into(),
            cpu_fan_target: 85,
            gpu_fan_target: 90,
            theme: "nordic".into(),
            ..Default::default()
        };

        let json = serde_json::to_string(&cfg).expect("Failed to serialize");
        let deserialized: AcerConfig = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(deserialized.mode, "custom");
        assert_eq!(deserialized.cpu_fan_target, 85);
        assert_eq!(deserialized.gpu_fan_target, 90);
        assert_eq!(deserialized.theme, "nordic");
    }

    #[test]
    fn test_config_backward_compatibility() {
        // Legacy JSON omitting `theme` and `sync_fans`
        let legacy_json = r#"{
            "mode": "max",
            "profile": "turbo",
            "coolboost": true,
            "cpu_fan_target": 100,
            "gpu_fan_target": 100,
            "battery_health_80": true,
            "winkey_locked": true,
            "touchpad_locked": false
        }"#;

        let cfg: AcerConfig = serde_json::from_str(legacy_json)
            .expect("Backward compatibility deserialization failed");
        assert_eq!(cfg.mode, "max");
        assert_eq!(cfg.theme, "human"); // Default fallback
        assert!(!cfg.sync_fans); // Default fallback
    }
}
