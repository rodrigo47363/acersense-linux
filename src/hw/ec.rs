use crate::hw::acpi::call_acpi_raw;
use anyhow::Result;
use std::sync::OnceLock;

pub const EC_CANDIDATE_PATHS: &[&str] = &[
    "\\_SB_.PCI0.LPCB.EC0_",
    "\\_SB.PCI0.LPCB.EC0",
    "\\_SB.PCI0.LPCB.EC0_",
    "\\_SB.LPCB.EC0",
];

static CACHED_EC_PATH: OnceLock<&'static str> = OnceLock::new();

/// Discovers and caches the active Embedded Controller path in DSDT.
pub fn get_ec_path() -> &'static str {
    CACHED_EC_PATH.get_or_init(|| {
        for &path in EC_CANDIDATE_PATHS {
            let test_cmd = format!("{}.FANG 0x10", path);
            if let Ok(res) = call_acpi_raw(&test_cmd) {
                if !res.starts_with("Error") && !res.is_empty() {
                    return path;
                }
            }
        }
        // Default confirmed fallback for AN515-55
        "\\_SB_.PCI0.LPCB.EC0_"
    })
}

/// Writes an 8-bit value to an Embedded Controller register using FANW method.
pub fn ec_write(reg: u8, val: u8) -> Result<String> {
    let ec = get_ec_path();
    let cmd = format!("{}.FANW 0x{:02X} 0x{:02X}", ec, reg, val);
    call_acpi_raw(&cmd)
}

/// Reads an 8-bit value from an Embedded Controller register using FANG method.
#[allow(dead_code)]
pub fn ec_read(reg: u8) -> Result<u8> {
    let ec = get_ec_path();
    let cmd = format!("{}.FANG 0x{:02X}", ec, reg);
    let res = call_acpi_raw(&cmd)?;

    let clean = res.trim_start_matches('{').trim_end_matches('}').trim();
    if clean.starts_with("0x") || clean.starts_with("0X") {
        let hex_val = u8::from_str_radix(&clean[2..], 16).unwrap_or(0);
        Ok(hex_val)
    } else {
        Ok(clean.parse::<u8>().unwrap_or(0))
    }
}
