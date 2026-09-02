use std::fs::OpenOptions;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;
use std::process::Command;
use anyhow::{Context, Result};

pub const ACPI_CALL_PATH: &str = "/proc/acpi/call";

/// Ensures that the acpi_call kernel module is loaded and permissions are accessible.
pub fn ensure_acpi_call_loaded() -> bool {
    if Path::new(ACPI_CALL_PATH).exists() {
        return true;
    }
    // Attempt to load the module
    let _ = Command::new("modprobe").arg("acpi_call").status();
    Path::new(ACPI_CALL_PATH).exists()
}

/// Executes a raw ACPI call by writing to /proc/acpi/call and reading back the result.
pub fn call_acpi_raw(cmd: &str) -> Result<String> {
    if !ensure_acpi_call_loaded() {
        anyhow::bail!("acpi_call kernel module is not loaded and /proc/acpi/call does not exist");
    }

    // Attempt write
    let write_res = OpenOptions::new().write(true).open(ACPI_CALL_PATH);
    let mut file = match write_res {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            // Self-healing: try to fix permissions via pkexec/sudo if running in desktop
            let _ = Command::new("sudo").args(["chmod", "666", ACPI_CALL_PATH]).status();
            OpenOptions::new()
                .write(true)
                .open(ACPI_CALL_PATH)
                .with_context(|| "Permission denied writing to /proc/acpi/call. Run: sudo chmod 666 /proc/acpi/call")?
        }
        Err(e) => return Err(e).with_context(|| format!("Failed to open {} for writing", ACPI_CALL_PATH)),
    };

    file.write_all(cmd.as_bytes())
        .with_context(|| format!("Failed to write command '{}' to {}", cmd, ACPI_CALL_PATH))?;
    file.flush()?;

    // Read response
    let mut read_file = OpenOptions::new()
        .read(true)
        .open(ACPI_CALL_PATH)
        .with_context(|| format!("Failed to open {} for reading", ACPI_CALL_PATH))?;
    
    let mut buf = Vec::new();
    read_file.read_to_end(&mut buf)?;

    let res_str = String::from_utf8_lossy(&buf)
        .trim_matches(|c: char| c == '\0' || c == '\n' || c == '\r' || c.is_whitespace())
        .to_string();

    Ok(res_str)
}
