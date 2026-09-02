use std::sync::OnceLock;
use crate::hw::acpi::call_acpi_raw;
use crate::hw::ec::ec_write;
use anyhow::Result;

pub const WMI_CANDIDATE_METHODS: &[&str] = &[
    "\\_SB.PCI0.WMID.WMAA",
    "\\_SB.PCI0.WMID.WMBH",
    "\\_SB_.PCI0.WMID.WMAA",
];

static CACHED_WMI_METHOD: OnceLock<&'static str> = OnceLock::new();

/// Discovers and caches the active WMI Gaming method.
pub fn get_wmi_method() -> &'static str {
    *CACHED_WMI_METHOD.get_or_init(|| {
        for &method in WMI_CANDIDATE_METHODS {
            let test_cmd = format!("{} 1 1 0x00007", method);
            if let Ok(res) = call_acpi_raw(&test_cmd) {
                if !res.starts_with("Error") && !res.is_empty() {
                    return method;
                }
            }
        }
        "\\_SB.PCI0.WMID.WMAA"
    })
}

/// Executes an Acer Gaming WMI method call (Instance 1).
pub fn call_gaming_method(method_id: u32, opcode: u32) -> Result<String> {
    let method = get_wmi_method();
    let cmd = format!("{} 1 {} 0x{:X}", method, method_id, opcode);
    call_acpi_raw(&cmd)
}

/// Sets fans to full MAX Turbo Overdrive mode (5660+ RPM CPU / 6000+ RPM GPU).
pub fn set_fans_max() -> Result<()> {
    // 1. SMM WSMI / WMBH Method 14 (0x0E): SetGamingFanBehavior (0x820009 = Max Dual Fan)
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x0E 0x820009");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMBH 1 0x0E 0x820009");

    // 2. SMM WSMI / WMBH Method 16 (0x10): SetGamingFanSpeed 100% (CPU=1, GPU=4)
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x10 0x6401"); // CPU 100%
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x10 0x6404"); // GPU 100%
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMBH 1 0x10 0x6401");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMBH 1 0x10 0x6404");

    // 3. WSMI Direct Fallback (Method 0x15 y 0x16)
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WSMI 0x15 0x820009");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WSMI 0x16 0x6401");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WSMI 0x16 0x6404");

    // 4. Activar CoolBoost en WMAA
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMAA 1 1 0x10007");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMAA 1 1 0x10007");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WSMI 0x01 0x10007");

    // 5. WMBH PECM Direct Overrides (TKST Turbo, GPUM Turbo, FTBL Turbo)
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x16 [0x01 0x02]");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x16 [0x02 0x02]");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x12 0x02");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x10 [0x01 0x64]");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x10 [0x04 0x64]");

    // 6. Activar Flags de Anulación en el Embedded Controller (0x10=CPU, 0x20=GPU)
    let _ = ec_write(0x2D, 0x03); // QuickBoost + DSMD
    let _ = ec_write(0x5C, 0x02); // Turbo Fan Table (0x02)
    let _ = ec_write(0x10, 0x02); // CPU Force Max (Override Mode 2)
    let _ = ec_write(0x20, 0x02); // GPU Force Max (Override Mode 2)
    let _ = ec_write(0x24, 0x02); // TKST Turbo (0x02)
    let _ = ec_write(0x25, 0x02); // GPUM Turbo (0x02)
    let _ = ec_write(0x28, 0x02); // CPOC Turbo (0x02)
    let _ = ec_write(0x29, 0x02); // GPOC Turbo (0x02)

    // 7. Inyectar PWM al Máximo Absoluto en el bus Compal
    let _ = ec_write(0x14, 0xFF); // CPU 100% PWM -> 5769+ RPM
    let _ = ec_write(0x24, 0xFF); // GPU 100% PWM -> 6000+ RPM
    let _ = ec_write(0x37, 0x64); // CPUF 100% Target
    let _ = ec_write(0x3A, 0x64); // GPUF 100% Target
    let _ = ec_write(0x3B, 0xFF); // ALTO / HSAS Override

    Ok(())
}

/// Sets fans to dynamic AUTO mode controlled by the BIOS.
pub fn set_fans_auto() -> Result<()> {
    // 1. SMM WSMI / WMBH Method 14 (0x0E): SetGamingFanBehavior (0x410009 = Auto Dual Fan)
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x0E 0x410009");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMBH 1 0x0E 0x410009");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WSMI 0x15 0x410009");

    // 2. Reset fan speed targets en WMBH (0x0001 = CPU Auto, 0x0004 = GPU Auto)
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x10 0x0001");
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x10 0x0004");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMBH 1 0x10 0x0001");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMBH 1 0x10 0x0004");

    // 3. CoolBoost Off en WMAA
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMAA 1 1 0x00007");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMAA 1 1 0x00007");

    // 4. Restaurar registros del EC al modo BIOS
    let _ = ec_write(0x10, 0x00);
    let _ = ec_write(0x14, 0x00);
    let _ = ec_write(0x20, 0x00);
    let _ = ec_write(0x24, 0x00);
    let _ = ec_write(0x25, 0x00);
    let _ = ec_write(0x2D, 0x00);
    let _ = ec_write(0x5C, 0x00);
    let _ = ec_write(0x37, 0x00);
    let _ = ec_write(0x3A, 0x00);
    let _ = ec_write(0x3B, 0x00);

    Ok(())
}

/// Sets custom fan speed targets for CPU and GPU (0-100%).
pub fn set_fans_custom(cpu_pct: u8, gpu_pct: u8) -> Result<()> {
    let c_pct = cpu_pct.min(100);
    let g_pct = gpu_pct.min(100);

    // 1. SMM WSMI / WMBH Method 14 (0x0E): SetGamingFanBehavior (0xC30009 = Custom Dual Fan)
    let _ = call_acpi_raw("\\_SB.PCI0.WMID.WMBH 1 0x0E 0xC30009");
    let _ = call_acpi_raw("\\_SB_.PCI0.WMID.WMBH 1 0x0E 0xC30009");

    // 2. Set individual speeds via WMBH Method 16 (0x10): (Pct << 8) | FanID
    let c_opcode = ((c_pct as u32) << 8) | 1;
    let g_opcode = ((g_pct as u32) << 8) | 4;
    let _ = call_acpi_raw(&format!("\\_SB.PCI0.WMID.WMBH 1 0x10 0x{:04X}", c_opcode));
    let _ = call_acpi_raw(&format!("\\_SB.PCI0.WMID.WMBH 1 0x10 0x{:04X}", g_opcode));
    let _ = call_acpi_raw(&format!("\\_SB.PCI0.WMID.WSMI 0x16 0x{:04X}", c_opcode));
    let _ = call_acpi_raw(&format!("\\_SB.PCI0.WMID.WSMI 0x16 0x{:04X}", g_opcode));

    // 3. Set EC hardware PWM values
    let c_pwm = ((c_pct as u32 * 255) / 100) as u8;
    let g_pwm = ((g_pct as u32 * 255) / 100) as u8;

    let _ = ec_write(0x10, 0x02);
    let _ = ec_write(0x14, c_pwm);
    let _ = ec_write(0x20, 0x02);
    let _ = ec_write(0x24, g_pwm);
    let _ = ec_write(0x37, c_pct);
    let _ = ec_write(0x3A, g_pct);

    Ok(())
}


/// Toggles CoolBoost mode.
pub fn set_coolboost(enable: bool) -> Result<()> {
    let opcode = if enable { 0x10007 } else { 0x00007 };
    call_gaming_method(1, opcode)?;
    Ok(())
}

/// Sets Power Profile.
pub fn set_power_profile(profile: &str) -> Result<()> {
    match profile.to_lowercase().as_str() {
        "quiet" | "saver" | "eco" => {
            let _ = call_gaming_method(1, 0x00007);
            let _ = call_gaming_method(14, 0x00);
            let _ = set_fans_auto();
        }
        "balanced" | "balance" => {
            let _ = call_gaming_method(1, 0x00007);
            let _ = call_gaming_method(14, 0x01);
            let _ = set_fans_auto();
        }
        "performance" | "perf" => {
            let _ = call_gaming_method(1, 0x10007);
            let _ = call_gaming_method(14, 0x02);
        }
        "turbo" => {
            let _ = call_gaming_method(1, 0x30007);
            let _ = call_gaming_method(14, 0x03);
            let _ = set_fans_max();
        }
        _ => anyhow::bail!("Unknown power profile: {}", profile),
    }
    Ok(())
}

/// Sets RGB color for a specific zone (1 to 4).
pub fn set_rgb_zone(zone: u8, r: u8, g: u8, b: u8) -> Result<()> {
    if !(1..=4).contains(&zone) {
        anyhow::bail!("Zone must be between 1 and 4");
    }
    let mask = 1 << (zone - 1);
    let cmd = format!("\\_SB.PCI0.WMID.WMBH 1 6 {{{}, {}, {}, {}}}", mask, r, g, b);
    let _ = call_acpi_raw(&cmd);

    // Direct PECM memory mapped EC registers
    match zone {
        1 => {
            let _ = ec_write(0x3C, r);
            let _ = ec_write(0x3D, g);
            let _ = ec_write(0x3E, b);
        }
        2 => {
            let _ = ec_write(0x3F, r);
            let _ = ec_write(0x40, g);
            let _ = ec_write(0x41, b);
        }
        3 => {
            let _ = ec_write(0x42, r);
            let _ = ec_write(0x43, g);
            let _ = ec_write(0x44, b);
        }
        4 => {
            let _ = ec_write(0x45, r);
            let _ = ec_write(0x46, g);
            let _ = ec_write(0x47, b);
        }
        _ => {}
    }
    Ok(())
}
