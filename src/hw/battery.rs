use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct BatteryInfo {
    pub percentage: u8,
    pub status: String,
    pub voltage: f32,
    pub health_percent: f32,
    pub ac_connected: bool,
    pub model_name: String,
    pub cycle_count: u32,
}

pub fn get_battery_info() -> BatteryInfo {
    let bat_dir = Path::new("/sys/class/power_supply/BAT1");
    let fallback_dir = Path::new("/sys/class/power_supply/BAT0");
    let target = if bat_dir.exists() {
        bat_dir
    } else {
        fallback_dir
    };

    let percentage = fs::read_to_string(target.join("capacity"))
        .unwrap_or_else(|_| "100".to_string())
        .trim()
        .parse::<u8>()
        .unwrap_or(100);

    let status = fs::read_to_string(target.join("status"))
        .unwrap_or_else(|_| "Full".to_string())
        .trim()
        .to_string();

    let voltage_raw = fs::read_to_string(target.join("voltage_now"))
        .unwrap_or_else(|_| "17200000".to_string())
        .trim()
        .parse::<f32>()
        .unwrap_or(17200000.0);
    let voltage = voltage_raw / 1_000_000.0;

    let full = fs::read_to_string(target.join("charge_full"))
        .or_else(|_| fs::read_to_string(target.join("energy_full")))
        .unwrap_or_else(|_| "100".to_string())
        .trim()
        .parse::<f32>()
        .unwrap_or(100.0);

    let design = fs::read_to_string(target.join("charge_full_design"))
        .or_else(|_| fs::read_to_string(target.join("energy_full_design")))
        .unwrap_or_else(|_| "100".to_string())
        .trim()
        .parse::<f32>()
        .unwrap_or(100.0);

    let health_percent = if design > 0.0 {
        (full / design) * 100.0
    } else {
        100.0
    };

    let model_name = fs::read_to_string(target.join("model_name"))
        .unwrap_or_else(|_| "Acer Lithium Cell".to_string())
        .trim()
        .to_string();

    let cycle_count = fs::read_to_string(target.join("cycle_count"))
        .unwrap_or_else(|_| "0".to_string())
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    let ac_candidates = [
        "/sys/class/power_supply/ACAD/online",
        "/sys/class/power_supply/AC/online",
        "/sys/class/power_supply/ADP1/online",
    ];
    let mut ac_connected = false;
    for ac in &ac_candidates {
        if let Ok(c) = fs::read_to_string(ac) {
            if c.trim() == "1" {
                ac_connected = true;
                break;
            }
        }
    }

    BatteryInfo {
        percentage,
        status,
        voltage,
        health_percent,
        ac_connected,
        model_name,
        cycle_count,
    }
}

pub fn set_battery_80_limit(enable: bool) -> Result<()> {
    let val = if enable { "1" } else { "0" };

    // Acer WMI health mode threshold sysfs
    let candidates = [
        "/sys/bus/platform/drivers/acer-wmi/health_mode",
        "/sys/class/power_supply/BAT1/charge_control_limit_max",
        "/sys/class/power_supply/BAT0/charge_control_limit_max",
    ];

    for path in &candidates {
        if Path::new(path).exists() {
            let _ = fs::write(path, val);
            return Ok(());
        }
    }

    // ACPI fallback for health mode
    let opcode = if enable { 0x10008 } else { 0x00008 };
    let _ = crate::hw::wmi::call_gaming_method(1, opcode);
    Ok(())
}
