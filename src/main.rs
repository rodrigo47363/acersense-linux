mod cli;
mod core;
mod hw;

use clap::Parser;
use colored::*;
use anyhow::Result;

use cli::{Cli, Commands};
use core::config::load_config;
use core::fan;
use core::profile;
use hw::battery;
use hw::gaming;
use hw::rgb;
use hw::thermal;

fn print_polybar() {
    let cfg = load_config();
    let cpu_t = thermal::get_cpu_temp();
    let is_max = cfg.mode == "max" || cfg.mode == "turbo";

    if is_max {
        println!("%{{F#e06c75}}󰈐%{{F-}} %{{F#abb2bf}}{:.0}°C%{{F-}} %{{F#e06c75}}[MAX]%{{F-}}", cpu_t);
    } else if cfg.profile == "quiet" {
        println!("%{{F#98c379}}󰈐%{{F-}} %{{F#abb2bf}}{:.0}°C%{{F-}} %{{F#98c379}}[QUIET]%{{F-}}", cpu_t);
    } else {
        println!("%{{F#61afef}}󰈐%{{F-}} %{{F#abb2bf}}{:.0}°C%{{F-}} %{{F#98c379}}[AUTO]%{{F-}}", cpu_t);
    }
}

fn print_json() -> Result<()> {
    let cfg = load_config();
    let is_max = cfg.mode == "max" || cfg.mode == "turbo";
    let data = thermal::collect_all_telemetry(is_max);
    println!("{}", serde_json::to_string_pretty(&data)?);
    Ok(())
}

fn print_status() {
    let cfg = load_config();
    let is_max = cfg.mode == "max" || cfg.mode == "turbo";
    let data = thermal::collect_all_telemetry(is_max);

    println!("{}", "── AcerSense Pro (Rust Native Engine v2.0) ──────────────────────────".bold().cyan());
    println!("  {}  {} | {} {}", 
        "Fan Mode:".bold(),
        if is_max { "MAX TURBO".bold().red() } else { cfg.mode.to_uppercase().bold().green() },
        "CoolBoost:".bold(),
        if cfg.coolboost { "ON".bold().green() } else { "OFF".normal() }
    );
    println!("  {}  {} | {} {}",
        "Power Profile:".bold(),
        cfg.profile.to_uppercase().bold().yellow(),
        "Battery Health Limit (80%):".bold(),
        if cfg.battery_health_80 { "ENABLED".bold().green() } else { "DISABLED".normal() }
    );
    println!("{}", "── Telemetry & Sensors ──────────────────────────────────────────────".bold().cyan());
    println!("  {}   {:.1} °C | {} {:.1}% | {} {} MHz | {:.1} W", 
        "CPU Package:".bold(), data.cpu_temp, "Load:".bold(), data.cpu_load, "Clock:".bold(), data.cpu_clock, data.cpu_power);
    
    if data.gpu_active {
        println!("  {}   {:.1} °C • {} MHz • {:.1} W • VRAM: {:.1}/{:.1} GB (RTX 3050)", 
            "Discrete GPU:".bold(), data.gpu_temp, data.gpu_clock, data.gpu_power, data.gpu_vram_used, data.gpu_vram_total);
    } else {
        println!("  {}   Standby (PCIe D3cold / 0.0W)", "Discrete GPU:".bold());
    }

    println!("  {}  {:.1}/{:.1} GB ({:.0}%) | {} {:.0}% | {} {:.1} °C",
        "RAM:".bold(), data.ram_used, data.ram_total, data.ram_pct, "Swap:".bold(), data.swap_pct, "NVMe:".bold(), data.nvme_temp);
    
    println!("  {}  {}% ({}) • {:.2}V • Health: {:.0}%", 
        "Battery:".bold(), data.bat_pct, data.bat_status, data.bat_voltage, data.bat_health);
    
    println!("{}", "── Turbines Speed (Hall Effect RPM) ─────────────────────────────────".bold().cyan());
    println!("  {}    {} RPM (Target: {}%)", "CPU Fan:".bold(), data.cpu_rpm.to_string().bold().green(), cfg.cpu_fan_target);
    println!("  {}    {} RPM (Target: {}%)", "GPU Fan:".bold(), data.gpu_rpm.to_string().bold().green(), cfg.gpu_fan_target);
    println!("{}", "─────────────────────────────────────────────────────────────────────".bold().cyan());
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.polybar {
        print_polybar();
        return Ok(());
    }

    if cli.json {
        return print_json();
    }

    match cli.command {
        None | Some(Commands::Status) => {
            print_status();
        }
        Some(Commands::Fan(args)) => {
            if args.toggle {
                let new_mode = fan::toggle_mode()?;
                println!("{} Fan mode toggled to: {}", "[+]".green().bold(), new_mode.bold());
            } else if let Some(mode) = args.mode {
                fan::set_mode(&mode)?;
                println!("{} Fan mode set to: {}", "[+]".green().bold(), mode.to_uppercase().bold());
            } else if let Some(cb) = args.coolboost {
                let en = cb == "on" || cb == "1" || cb == "true";
                fan::set_coolboost_state(en)?;
                println!("{} CoolBoost set to: {}", "[+]".green().bold(), if en { "ON".bold().green() } else { "OFF".normal() });
            } else if args.cpu.is_some() || args.gpu.is_some() {
                let cfg = load_config();
                let c = args.cpu.unwrap_or(cfg.cpu_fan_target);
                let g = args.gpu.unwrap_or(cfg.gpu_fan_target);
                fan::set_custom_speeds(c, g)?;
                println!("{} Custom speeds applied: CPU={}%, GPU={}%", "[+]".green().bold(), c, g);
            } else if args.status {
                print_status();
            } else {
                print_status();
            }
        }
        Some(Commands::Profile(args)) => {
            if args.next {
                let new_p = profile::next_profile()?;
                println!("{} Power profile switched to: {}", "[+]".green().bold(), new_p.to_uppercase().bold());
            } else if let Some(p) = args.set {
                profile::set_profile(&p)?;
                println!("{} Power profile set to: {}", "[+]".green().bold(), p.to_uppercase().bold());
            } else {
                let cfg = load_config();
                println!("Current Power Profile: {}", cfg.profile.to_uppercase().bold().cyan());
            }
        }
        Some(Commands::Mode(args)) => {
            if let Some(target) = args.target {
                match target.to_lowercase().as_str() {
                    "next" => {
                        let new_p = profile::next_profile()?;
                        println!("{} Switched profile: {}", "[+]".green().bold(), new_p.bold());
                    }
                    "toggle" => {
                        let new_m = fan::toggle_mode()?;
                        println!("{} Toggled fan mode: {}", "[+]".green().bold(), new_m.bold());
                    }
                    "max" | "turbo" => {
                        fan::set_mode("max")?;
                        println!("{} Mode set to MAX TURBO", "[+]".green().bold());
                    }
                    "auto" => {
                        fan::set_mode("auto")?;
                        println!("{} Mode set to AUTO", "[+]".green().bold());
                    }
                    "quiet" | "eco" | "saver" => {
                        profile::set_profile("quiet")?;
                        println!("{} Profile set to QUIET", "[+]".green().bold());
                    }
                    "balanced" => {
                        profile::set_profile("balanced")?;
                        println!("{} Profile set to BALANCED", "[+]".green().bold());
                    }
                    "performance" | "perf" => {
                        profile::set_profile("performance")?;
                        println!("{} Profile set to PERFORMANCE", "[+]".green().bold());
                    }
                    other => {
                        eprintln!("{} Unknown mode target: {}", "[-]".red().bold(), other);
                    }
                }
            } else {
                print_status();
            }
        }
        Some(Commands::Battery(args)) => {
            if let Some(lim) = args.limit_80 {
                let en = lim == "on" || lim == "1" || lim == "true";
                battery::set_battery_80_limit(en)?;
                println!("{} Battery 80% charge limit: {}", "[+]".green().bold(), if en { "ENABLED".green().bold() } else { "DISABLED".normal() });
            } else {
                let bat = battery::get_battery_info();
                println!("Battery: {}% ({}) | {:.2}V | Health: {:.1}%", bat.percentage, bat.status, bat.voltage, bat.health_percent);
            }
        }
        Some(Commands::Rgb(args)) => {
            if let Some(preset) = args.preset {
                rgb::apply_preset(&preset)?;
                println!("{} RGB Preset applied: {}", "[+]".green().bold(), preset.bold().magenta());
            } else if let Some(zones) = args.zones {
                if zones.len() == 4 {
                    rgb::set_zones(&zones[0], &zones[1], &zones[2], &zones[3])?;
                    println!("{} 4-Zone RGB colors applied.", "[+]".green().bold());
                } else {
                    eprintln!("{} Exactly 4 zone colors required (e.g. #FF0000 #00FF00 #0000FF #FFFF00)", "[-]".red().bold());
                }
            } else if let Some(color) = args.color {
                rgb::set_zones(&color, &color, &color, &color)?;
                println!("{} All 4 RGB zones set to {}", "[+]".green().bold(), color.bold().cyan());
            } else {
                println!("Usage: acersense rgb --preset <nitro|cyberpunk|ice|toxic|synthwave|white> or --zones <z1> <z2> <z3> <z4>");
            }
        }
        Some(Commands::Gaming(args)) => {
            if let Some(wk) = args.winkey {
                let lock = wk == "lock" || wk == "1" || wk == "true";
                gaming::set_winkey_lock(lock)?;
                println!("{} Windows Key: {}", "[+]".green().bold(), if lock { "LOCKED".yellow().bold() } else { "UNLOCKED".green().bold() });
            }
            if let Some(tp) = args.touchpad {
                let lock = tp == "lock" || tp == "1" || tp == "true";
                gaming::set_touchpad_lock(lock)?;
                println!("{} Touchpad: {}", "[+]".green().bold(), if lock { "LOCKED".yellow().bold() } else { "UNLOCKED".green().bold() });
            }
            if let Some(od) = args.overdrive {
                let en = od == "on" || od == "1" || od == "true";
                gaming::set_lcd_overdrive(en)?;
                println!("{} LCD 3ms Overdrive: {}", "[+]".green().bold(), if en { "ENABLED".green().bold() } else { "DISABLED".normal() });
            }
        }
    }

    Ok(())
}
