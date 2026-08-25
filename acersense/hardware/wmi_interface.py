"""
AcerSense WMI & ACPI Hardware Interface
Directly communicates with Acer ACPI/WMI methods reverse-engineered from Windows NitroSense/PredatorSense.
"""

import os
import subprocess
import logging
from typing import Optional, Dict

logger = logging.getLogger("acersense.wmi")

# Verified WMI Method IDs from Acer DSDT / WMI mapping:
# Method 0x0F (Decimal 15) -> SetGamingFanBehavior: 1=Auto, 2=Max, 3=Custom
# Method 0x10 (Decimal 16) -> SetGamingFanSpeed: (FanID | (Pct << 8))
# Method 0x11 (Decimal 17) -> WMISetFunction: (FunctionID | (Value << 16))
# Method 0x0B (Decimal 11) -> SetGamingLEDGroupColor: (Zone | (R<<8) | (G<<16) | (B<<24))
# Method 0x1E (Decimal 30) -> SetGamingLEDBehavior: (Effect | (Speed<<8) | (Dir<<16))
# Method 0x1C (Decimal 28) -> SetGamingKBBacklight: (1 | (Timeout<<8))

WMI_METHOD_FAN_BEHAVIOR = 0x0F
WMI_METHOD_FAN_SPEED    = 0x10
WMI_METHOD_WMI_FUNCTION = 0x11
WMI_METHOD_RGB_COLOR    = 0x0B
WMI_METHOD_RGB_EFFECT   = 0x1E
WMI_METHOD_KB_TIMEOUT   = 0x1C

# Reverse-Engineered Fan Behavior Opcodes
FAN_OPCODE_AUTO   = 1
FAN_OPCODE_MAX    = 2
FAN_OPCODE_CUSTOM = 3


class AcerWMIInterface:
    """Interface to communicate with Acer proprietary WMI & ACPI subsystems."""

    def __init__(self):
        self.dmi_vendor = self._read_sysfs("/sys/class/dmi/id/sys_vendor", "Unknown")
        self.product_name = self._read_sysfs("/sys/class/dmi/id/product_name", "Unknown")
        self.product_version = self._read_sysfs("/sys/class/dmi/id/product_version", "Unknown")
        self.bios_version = self._read_sysfs("/sys/class/dmi/id/bios_version", "Unknown")
        
        self._ensure_acpi_call_loaded()
        self.has_acpi_call = os.path.exists("/proc/acpi/call")
        self.primary_acpi_method = r"\_SB.PCI0.WMID.WMBH"
        
        logger.info(f"Initialized Acer WMI for {self.product_name} (BIOS: {self.bios_version}, acpi_call: {self.has_acpi_call})")

    def _ensure_acpi_call_loaded(self):
        """Attempts to ensure acpi_call kernel module is loaded."""
        if not os.path.exists("/proc/acpi/call"):
            try:
                subprocess.run(["modprobe", "acpi_call"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            except Exception:
                pass

    @staticmethod
    def _read_sysfs(path: str, default: str = "") -> str:
        try:
            if os.path.exists(path):
                with open(path, "r", encoding="utf-8", errors="ignore") as f:
                    return f.read().strip()
        except Exception as e:
            logger.debug(f"Error reading sysfs path {path}: {e}")
        return default

    @staticmethod
    def _write_sysfs(path: str, value: str) -> bool:
        try:
            if os.path.exists(path):
                with open(path, "w", encoding="utf-8") as f:
                    f.write(value)
                return True
        except PermissionError:
            res = subprocess.run(["pkexec", "tee", path], input=value.encode(), stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            if res.returncode == 0:
                return True
            res_sudo = subprocess.run(["sudo", "-n", "tee", path], input=value.encode(), stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            return res_sudo.returncode == 0
        except Exception as e:
            logger.error(f"Failed to write '{value}' to {path}: {e}")
        return False

    def call_acpi_raw(self, call_str: str) -> Optional[str]:
        """Executes a single raw string command to /proc/acpi/call."""
        if not os.path.exists("/proc/acpi/call"):
            self._ensure_acpi_call_loaded()
        if not os.path.exists("/proc/acpi/call"):
            return None

        try:
            with open("/proc/acpi/call", "w") as f:
                f.write(call_str)
            with open("/proc/acpi/call", "r") as f:
                res = f.read().strip().rstrip("\x00")
                logger.debug(f"ACPI Call '{call_str}' -> '{res}'")
                return res
        except PermissionError:
            try:
                subprocess.run(["pkexec", "tee", "/proc/acpi/call"], input=call_str.encode(), stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                with open("/proc/acpi/call", "r") as f:
                    return f.read().strip().rstrip("\x00")
            except Exception:
                pass
        except Exception as e:
            logger.debug(f"Failed ACPI Call '{call_str}': {e}")
        return None

    def call_gaming_method(self, method_id: int, opcode: int) -> bool:
        """Calls verified Acer Gaming ACPI WMBH method."""
        call_str = f"{self.primary_acpi_method} 1 0x{method_id:X} 0x{opcode:X}"
        res = self.call_acpi_raw(call_str)
        if res and not res.startswith("Error") and (res.startswith("{0x00") or res == "0x0" or res.startswith("{")):
            return True
        return False

    def set_fan_mode(self, mode: str) -> bool:
        """
        Set fan behavior mode: 'auto', 'max', 'custom'.
        """
        mode_lower = mode.lower()
        if mode_lower == "auto":
            logger.info("Applying Fan Mode: AUTO")
            self.call_gaming_method(WMI_METHOD_FAN_BEHAVIOR, FAN_OPCODE_AUTO)
            self.call_gaming_method(WMI_METHOD_FAN_BEHAVIOR, 0x410009)
            self.set_coolboost(False)
            self.set_power_profile("balanced")
            return True
        elif mode_lower == "max":
            logger.info("Applying Fan Mode: MAX TURBO (100% ~5500 RPM)")
            # 1. Set mode to MAX in EC
            self.call_gaming_method(WMI_METHOD_FAN_BEHAVIOR, FAN_OPCODE_MAX)
            self.call_gaming_method(WMI_METHOD_FAN_BEHAVIOR, 0x820009)
            # 2. Lock CPU and GPU fan speed targets to 100%
            self.set_fan_speed(0, 100)
            self.set_fan_speed(1, 100)
            # 3. Enable CoolBoost ceiling
            self.set_coolboost(True)
            self.set_power_profile("performance")
            return True
        elif mode_lower == "custom":
            logger.info("Applying Fan Mode: CUSTOM")
            self.call_gaming_method(WMI_METHOD_FAN_BEHAVIOR, FAN_OPCODE_CUSTOM)
            self.call_gaming_method(WMI_METHOD_FAN_BEHAVIOR, 0xC30009)
            return True
        else:
            raise ValueError(f"Unknown fan mode: {mode}")

    def set_fan_speed(self, fan_index: int, percentage: int) -> bool:
        """
        Set individual fan target speed (0 = CPU, 1 = GPU).
        """
        percentage = max(0, min(100, int(percentage)))
        if fan_index == 0:  # CPU
            opcode = 0x01 | (percentage << 8)
        elif fan_index == 1:  # GPU
            opcode = 0x04 | (percentage << 8)
        else:
            raise ValueError(f"Invalid fan index: {fan_index}")

        logger.info(f"Applying Fan Speed index {fan_index}: {percentage}% (Opcode 0x{opcode:X})")
        return self.call_gaming_method(WMI_METHOD_FAN_SPEED, opcode)

    def set_coolboost(self, enable: bool) -> bool:
        """
        Enable/Disable Acer CoolBoost (+500-1000 RPM fan curve ceiling).
        """
        opcode = 7 | ((1 if enable else 0) << 16)
        logger.info(f"Applying CoolBoost: {enable} (Opcode 0x{opcode:X})")
        return self.call_gaming_method(WMI_METHOD_WMI_FUNCTION, opcode)

    def set_rgb_zone_color(self, zone_index: int, r: int, g: int, b: int) -> bool:
        """
        Set 4-Zone RGB keyboard color.
        """
        opcode = (zone_index & 0xFF) | ((r & 0xFF) << 8) | ((g & 0xFF) << 16) | ((b & 0xFF) << 24)
        return self.call_gaming_method(WMI_METHOD_RGB_COLOR, opcode)

    def set_rgb_behavior(self, effect_id: int, speed: int, direction: int) -> bool:
        """
        Set RGB lighting dynamic effect.
        """
        opcode = (effect_id & 0xFF) | ((speed & 0xFF) << 8) | ((direction & 0xFF) << 16)
        return self.call_gaming_method(WMI_METHOD_RGB_EFFECT, opcode)

    def set_power_profile(self, profile: str) -> bool:
        """
        Set power profile: 'quiet', 'balanced', 'performance', 'turbo'.
        """
        profile_map = {
            "quiet": "quiet",
            "balanced": "balanced",
            "performance": "performance",
            "turbo": "performance"
        }
        target = profile_map.get(profile.lower(), "balanced")
        
        # 1. Linux sysfs platform profile
        sysfs_profile = "/sys/firmware/acpi/platform_profile"
        if os.path.exists(sysfs_profile):
            self._write_sysfs(sysfs_profile, target)

        # 2. WMI Power Profile Opcode
        mode_idx = {"quiet": 0, "balanced": 1, "performance": 2, "turbo": 3}.get(profile.lower(), 1)
        opcode = 7 | (mode_idx << 16)
        self.call_gaming_method(WMI_METHOD_WMI_FUNCTION, opcode)
        return True

    def get_power_profile(self) -> str:
        """Read current platform profile from kernel sysfs."""
        sysfs_profile = "/sys/firmware/acpi/platform_profile"
        return self._read_sysfs(sysfs_profile, "balanced")

    def get_battery_health_mode(self) -> Optional[bool]:
        """Check if 80% charge limiter is active."""
        health_path = "/sys/bus/wmi/drivers/acer-wmi-battery/health_mode"
        if os.path.exists(health_path):
            val = self._read_sysfs(health_path, "0")
            return val == "1"
        return None

    def set_battery_health_mode(self, enable: bool) -> bool:
        """Enable (80% charge limit) or Disable (100% full charge) battery health mode."""
        health_path = "/sys/bus/wmi/drivers/acer-wmi-battery/health_mode"
        if os.path.exists(health_path):
            val = "1" if enable else "0"
            return self._write_sysfs(health_path, val)
        return False
