use crate::hw::wmi::set_rgb_zone;
use anyhow::{bail, Result};

#[allow(dead_code)]
pub fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let clean = hex.trim_start_matches('#');
    if clean.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
    let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
    let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
    Some((r, g, b))
}

#[allow(dead_code)]
pub fn set_static_color(zone: u8, r: u8, g: u8, b: u8) -> Result<()> {
    set_rgb_zone(zone, r, g, b)
}

#[allow(dead_code)]
pub fn set_zones(z1: &str, z2: &str, z3: &str, z4: &str) -> Result<()> {
    let zones = [z1, z2, z3, z4];
    for (i, &hex) in zones.iter().enumerate() {
        if let Some((r, g, b)) = parse_hex(hex) {
            set_rgb_zone((i + 1) as u8, r, g, b)?;
        } else {
            bail!("Invalid hex color: '{}' for zone {}", hex, i + 1);
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn apply_preset(name: &str) -> Result<()> {
    match name.to_lowercase().as_str() {
        "nitro" | "red" => set_zones("#FF0000", "#FF0000", "#FF0000", "#FF0000"),
        "cyberpunk" => set_zones("#00FFFF", "#FF00FF", "#9400D3", "#FFE600"),
        "ice" | "blue" => set_zones("#00FFFF", "#00BFFF", "#1E90FF", "#0000FF"),
        "toxic" | "green" => set_zones("#00FF00", "#32CD32", "#7FFF00", "#00FA9A"),
        "synthwave" => set_zones("#8A2BE2", "#FF1493", "#00FFFF", "#FF4500"),
        "white" => set_zones("#FFFFFF", "#FFFFFF", "#FFFFFF", "#FFFFFF"),
        _ => bail!(
            "Unknown RGB preset: '{}'. Options: nitro, cyberpunk, ice, toxic, synthwave, white",
            name
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_valid() {
        assert_eq!(parse_hex("#FF0000"), Some((255, 0, 0)));
        assert_eq!(parse_hex("00FF00"), Some((0, 255, 0)));
        assert_eq!(parse_hex("#0000FF"), Some((0, 0, 255)));
        assert_eq!(parse_hex("#123456"), Some((0x12, 0x34, 0x56)));
    }

    #[test]
    fn test_parse_hex_invalid() {
        assert_eq!(parse_hex(""), None);
        assert_eq!(parse_hex("#FFF"), None);
        assert_eq!(parse_hex("#FFFFFFF"), None);
        assert_eq!(parse_hex("#GG0000"), None);
        assert_eq!(parse_hex("hello!"), None);
    }

    #[test]
    fn test_apply_preset_invalid() {
        assert!(apply_preset("nonexistent_preset_xyz").is_err());
    }
}
