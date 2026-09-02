use crate::core::config::{load_config, save_config};
use crate::hw::wmi;
use anyhow::Result;

pub fn set_profile(profile: &str) -> Result<()> {
    let p_lower = profile.to_lowercase();
    wmi::set_power_profile(&p_lower)?;
    
    let mut cfg = load_config();
    cfg.profile = p_lower;
    save_config(&cfg);
    Ok(())
}

pub fn next_profile() -> Result<String> {
    let cfg = load_config();
    let next = match cfg.profile.as_str() {
        "quiet" | "eco" | "saver" => "balanced",
        "balanced" | "balance" => "performance",
        "performance" | "perf" => "turbo",
        "turbo" => "quiet",
        _ => "balanced",
    };
    set_profile(next)?;
    Ok(next.to_string())
}
