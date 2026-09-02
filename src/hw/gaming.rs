use crate::hw::wmi::call_gaming_method;
use anyhow::Result;

pub fn set_winkey_lock(lock: bool) -> Result<()> {
    let opcode = if lock { 0x01 } else { 0x00 };
    call_gaming_method(11, opcode)?;
    Ok(())
}

pub fn set_touchpad_lock(lock: bool) -> Result<()> {
    let opcode = if lock { 0x01 } else { 0x00 };
    call_gaming_method(12, opcode)?;
    Ok(())
}

pub fn set_lcd_overdrive(enable: bool) -> Result<()> {
    let opcode = if enable { 0x01 } else { 0x00 };
    call_gaming_method(15, opcode)?;
    Ok(())
}
