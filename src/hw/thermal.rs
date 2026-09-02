use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::hw::battery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    // CPU
    pub cpu_temp: f32,
    pub cpu_clock: u32,
    pub cpu_power: f32,
    pub cpu_load: f32,

    // GPU
    pub gpu_temp: f32,
    pub gpu_clock: u32,
    pub gpu_power: f32,
    pub gpu_vram_used: f32,
    pub gpu_vram_total: f32,
    pub gpu_active: bool,

    // Memory
    pub ram_used: f32,
    pub ram_total: f32,
    pub ram_pct: f32,
    pub swap_pct: f32,

    // Storage
    pub nvme_temp: f32,
    pub nvme_health: u8,

    // Fans & Battery
    pub cpu_rpm: u32,
    pub gpu_rpm: u32,
    pub bat_pct: u8,
    pub bat_status: String,
    pub bat_voltage: f32,
    pub bat_health: f32,
}

impl Default for TelemetryData {
    fn default() -> Self {
        Self {
            cpu_temp: 50.0,
            cpu_clock: 2500,
            cpu_power: 15.0,
            cpu_load: 5.0,
            gpu_temp: 45.0,
            gpu_clock: 210,
            gpu_power: 8.0,
            gpu_vram_used: 0.1,
            gpu_vram_total: 4.0,
            gpu_active: true,
            ram_used: 6.0,
            ram_total: 16.0,
            ram_pct: 38.0,
            swap_pct: 0.0,
            nvme_temp: 42.0,
            nvme_health: 98,
            cpu_rpm: 2400,
            gpu_rpm: 2600,
            bat_pct: 100,
            bat_status: "Full".to_string(),
            bat_voltage: 17.2,
            bat_health: 85.0,
        }
    }
}

pub fn get_cpu_temp() -> f32 {
    for i in 0..12 {
        let type_path = format!("/sys/class/thermal/thermal_zone{}/type", i);
        let temp_path = format!("/sys/class/thermal/thermal_zone{}/temp", i);
        if let Ok(sensor_type) = fs::read_to_string(&type_path) {
            let t = sensor_type.trim();
            if t == "x86_pkg_temp" || t == "acpitz" || t == "B0D4" {
                if let Ok(raw_temp) = fs::read_to_string(&temp_path) {
                    if let Ok(val) = raw_temp.trim().parse::<f32>() {
                        if val > 1000.0 {
                            return val / 1000.0;
                        }
                    }
                }
            }
        }
    }

    if let Ok(raw) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(val) = raw.trim().parse::<f32>() {
            return if val > 1000.0 { val / 1000.0 } else { val };
        }
    }
    45.0
}

pub fn get_cpu_usage() -> f32 {
    if let Ok(stat) = fs::read_to_string("/proc/stat") {
        if let Some(line) = stat.lines().next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let user: u64 = parts[1].parse().unwrap_or(0);
                let nice: u64 = parts[2].parse().unwrap_or(0);
                let system: u64 = parts[3].parse().unwrap_or(0);
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let total = user + nice + system + idle;
                let active = user + nice + system;
                if total > 0 {
                    return (active as f32 / total as f32) * 100.0;
                }
            }
        }
    }
    10.0
}

pub fn get_cpu_freq_mhz() -> u32 {
    let mut total_khz = 0.0;
    let mut count = 0.0;

    if let Ok(entries) = glob::glob("/sys/devices/system/cpu/cpu*/cpufreq/scaling_cur_freq") {
        for entry in entries.flatten() {
            if let Ok(content) = fs::read_to_string(entry) {
                if let Ok(khz) = content.trim().parse::<f32>() {
                    total_khz += khz;
                    count += 1.0;
                }
            }
        }
    }

    if count > 0.0 {
        return ((total_khz / count) / 1000.0) as u32;
    }
    2500
}

pub fn get_cpu_power_w() -> f32 {
    let rapl_paths = [
        "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj",
        "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/energy_uj",
    ];

    for path in &rapl_paths {
        if let Ok(energy_1_str) = fs::read_to_string(path) {
            if let Ok(energy_1) = energy_1_str.trim().parse::<u64>() {
                thread::sleep(Duration::from_millis(50));
                if let Ok(energy_2_str) = fs::read_to_string(path) {
                    if let Ok(energy_2) = energy_2_str.trim().parse::<u64>() {
                        let delta = energy_2.saturating_sub(energy_1) as f32;
                        let watts = delta / 50_000.0;
                        if watts > 0.0 && watts < 200.0 {
                            return watts;
                        }
                    }
                }
            }
        }
    }

    // Fallback: estimate from CPU load and clock frequency
    let load = get_cpu_usage() / 100.0;
    let clock = get_cpu_freq_mhz() as f32 / 1000.0;
    let estimated = 10.0 + (load * 35.0) * (clock / 2.5);
    estimated.clamp(8.0, 65.0)
}

pub fn get_nvidia_telemetry() -> (f32, u32, f32, f32, f32, bool) {
    let d3_path = "/sys/bus/pci/devices/0000:01:00.0/power/runtime_status";
    if let Ok(status) = fs::read_to_string(d3_path) {
        if status.trim() == "suspended" {
            return (0.0, 0, 0.0, 0.0, 4.0, false);
        }
    }

    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=temperature.gpu,clocks.gr,power.draw,memory.used,memory.total")
        .arg("--format=csv,noheader,nounits")
        .output();

    if let Ok(cmd_res) = output {
        if cmd_res.status.success() {
            let stdout = String::from_utf8_lossy(&cmd_res.stdout);
            let parts: Vec<&str> = stdout.trim().split(',').collect();
            if parts.len() >= 5 {
                let temp = parts[0].trim().parse::<f32>().unwrap_or(45.0);
                let clock = parts[1].trim().parse::<u32>().unwrap_or(210);
                let power = parts[2].trim().parse::<f32>().unwrap_or(8.0);
                let vram_used_mb = parts[3].trim().parse::<f32>().unwrap_or(0.0);
                let vram_tot_mb = parts[4].trim().parse::<f32>().unwrap_or(4096.0);
                return (temp, clock, power, vram_used_mb / 1024.0, vram_tot_mb / 1024.0, true);
            }
        }
    }

    (45.0, 210, 8.0, 0.1, 4.0, true)
}

pub fn get_memory_info() -> (f32, f32, f32, f32) {
    let mut mem_total = 16.0;
    let mut mem_avail = 8.0;
    let mut swap_total = 0.0;
    let mut swap_free = 0.0;

    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let key = parts[0].trim_end_matches(':');
                let val_kb = parts[1].parse::<f32>().unwrap_or(0.0);
                match key {
                    "MemTotal" => mem_total = val_kb / 1024.0 / 1024.0,
                    "MemAvailable" => mem_avail = val_kb / 1024.0 / 1024.0,
                    "SwapTotal" => swap_total = val_kb / 1024.0 / 1024.0,
                    "SwapFree" => swap_free = val_kb / 1024.0 / 1024.0,
                    _ => {}
                }
            }
        }
    }

    let mem_used = (mem_total - mem_avail).max(0.0);
    let ram_pct = if mem_total > 0.0 { (mem_used / mem_total) * 100.0 } else { 0.0 };
    let swap_used = (swap_total - swap_free).max(0.0);
    let swap_pct = if swap_total > 0.0 { (swap_used / swap_total) * 100.0 } else { 0.0 };

    (mem_used, mem_total, ram_pct, swap_pct)
}

pub fn get_nvme_info() -> (f32, u8) {
    let mut temp = 42.0;
    if let Ok(entries) = glob::glob("/sys/class/hwmon/hwmon*/name") {
        for entry in entries.flatten() {
            if let Ok(name) = fs::read_to_string(&entry) {
                if name.trim() == "nvme" {
                    if let Some(dir) = entry.parent() {
                        let temp_path = dir.join("temp1_input");
                        if let Ok(raw) = fs::read_to_string(temp_path) {
                            if let Ok(val) = raw.trim().parse::<f32>() {
                                temp = val / 1000.0;
                            }
                        }
                    }
                }
            }
        }
    }
    (temp, 98)
}

pub fn get_fan_rpms(_is_max: bool) -> (u32, u32) {
    let mut cpu_rpm = 0;
    let mut gpu_rpm = 0;

    // 1. Direct hardware reading from Compal EC Registers (0x19/0x1A CPU, 0x29/0x2A GPU)
    if let (Ok(h), Ok(l)) = (crate::hw::ec::ec_read(0x19), crate::hw::ec::ec_read(0x1A)) {
        let rpm = ((h as u32) << 8) | (l as u32);
        if rpm > 500 && rpm < 8000 {
            cpu_rpm = rpm;
        }
    }

    if let (Ok(h), Ok(l)) = (crate::hw::ec::ec_read(0x29), crate::hw::ec::ec_read(0x2A)) {
        let rpm = ((h as u32) << 8) | (l as u32);
        if rpm > 500 && rpm < 8000 {
            gpu_rpm = rpm;
        }
    }

    // 2. Sysfs Fallback
    if cpu_rpm == 0 {
        if let Ok(entries) = glob::glob("/sys/class/hwmon/hwmon*/fan1_input") {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry) {
                    if let Ok(val) = content.trim().parse::<u32>() {
                        if val > 0 {
                            cpu_rpm = val;
                        }
                    }
                }
            }
        }
    }

    if gpu_rpm == 0 {
        if let Ok(entries) = glob::glob("/sys/class/hwmon/hwmon*/fan2_input") {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry) {
                    if let Ok(val) = content.trim().parse::<u32>() {
                        if val > 0 {
                            gpu_rpm = val;
                        }
                    }
                }
            }
        }
    }

    (if cpu_rpm > 0 { cpu_rpm } else { 2200 }, if gpu_rpm > 0 { gpu_rpm } else { 2400 })
}

pub fn collect_all_telemetry(is_max: bool) -> TelemetryData {
    let (c_rpm, g_rpm) = get_fan_rpms(is_max);
    let (g_temp, g_clock, g_power, g_vram_u, g_vram_tot, g_active) = get_nvidia_telemetry();
    let (r_used, r_tot, r_pct, sw_pct) = get_memory_info();
    let (nvme_t, nvme_h) = get_nvme_info();
    let bat = battery::get_battery_info();

    TelemetryData {
        cpu_temp: get_cpu_temp(),
        cpu_clock: get_cpu_freq_mhz(),
        cpu_power: get_cpu_power_w(),
        cpu_load: get_cpu_usage(),
        gpu_temp: g_temp,
        gpu_clock: g_clock,
        gpu_power: g_power,
        gpu_vram_used: g_vram_u,
        gpu_vram_total: g_vram_tot,
        gpu_active: g_active,
        ram_used: r_used,
        ram_total: r_tot,
        ram_pct: r_pct,
        swap_pct: sw_pct,
        nvme_temp: nvme_t,
        nvme_health: nvme_h,
        cpu_rpm: c_rpm,
        gpu_rpm: g_rpm,
        bat_pct: bat.percentage,
        bat_status: bat.status,
        bat_voltage: bat.voltage,
        bat_health: bat.health_percent,
    }
}
