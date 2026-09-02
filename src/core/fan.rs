use crate::core::config::{load_config, save_config};
use crate::hw::wmi;
use anyhow::Result;

pub fn set_mode(mode: &str) -> Result<()> {
    let mode_lower = mode.to_lowercase();
    let mut cfg = load_config();

    match mode_lower.as_str() {
        "max" | "turbo" => {
            wmi::set_fans_max()?;
            cfg.mode = "max".to_string();
            cfg.coolboost = true;
        }
        "auto" => {
            wmi::set_fans_auto()?;
            cfg.mode = "auto".to_string();
            cfg.coolboost = false;
        }
        "custom" => {
            wmi::set_fans_custom(cfg.cpu_fan_target, cfg.gpu_fan_target)?;
            cfg.mode = "custom".to_string();
        }
        "silent" => {
            wmi::set_fans_auto()?;
            wmi::set_power_profile("quiet")?;
            cfg.mode = "auto".to_string();
            cfg.profile = "quiet".to_string();
        }
        _ => anyhow::bail!("Invalid fan mode: '{}'. Valid: auto, max, custom, silent", mode),
    }

    save_config(&cfg);
    Ok(())
}

pub fn toggle_mode() -> Result<String> {
    let cfg = load_config();
    if cfg.mode == "max" || cfg.mode == "turbo" {
        set_mode("auto")?;
        Ok("auto".to_string())
    } else {
        set_mode("max")?;
        Ok("max".to_string())
    }
}

pub fn set_custom_speeds(cpu: u8, gpu: u8) -> Result<()> {
    wmi::set_fans_custom(cpu, gpu)?;
    let mut cfg = load_config();
    cfg.mode = "custom".to_string();
    cfg.cpu_fan_target = cpu;
    cfg.gpu_fan_target = gpu;
    save_config(&cfg);
    Ok(())
}

pub fn set_coolboost_state(enable: bool) -> Result<()> {
    wmi::set_coolboost(enable)?;
    let mut cfg = load_config();
    cfg.coolboost = enable;
    save_config(&cfg);
    Ok(())
}
