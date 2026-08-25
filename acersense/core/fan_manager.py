"""
AcerSense Fan & Thermal Management Core
Coordinates fan modes, custom RPM target percentages, CoolBoost, and automated curves.
"""

import logging
from typing import Dict, Any, Optional
from acersense.hardware.wmi_interface import AcerWMIInterface
from acersense.hardware.thermal_interface import ThermalInterface

logger = logging.getLogger("acersense.fan")


class FanManager:
    """High-level manager for fan cooling profiles and speed policies."""

    def __init__(self, wmi: Optional[AcerWMIInterface] = None, thermal: Optional[ThermalInterface] = None):
        self.wmi = wmi or AcerWMIInterface()
        self.thermal = thermal or ThermalInterface()
        
        self.current_mode = "auto"
        self.coolboost = False
        self.cpu_custom_pct = 50
        self.gpu_custom_pct = 50

    def set_mode(self, mode: str) -> bool:
        """Set fan mode: 'auto', 'max', 'custom'."""
        mode_lower = mode.lower()
        if mode_lower not in ["auto", "max", "custom"]:
            raise ValueError("Mode must be 'auto', 'max', or 'custom'")
        
        success = self.wmi.set_fan_mode(mode_lower)
        if success:
            self.current_mode = mode_lower
            if mode_lower == "custom":
                self.apply_custom_speeds(self.cpu_custom_pct, self.gpu_custom_pct)
        return success

    def set_coolboost(self, enable: bool) -> bool:
        """Enable or disable Acer CoolBoost."""
        success = self.wmi.set_coolboost(enable)
        if success:
            self.coolboost = enable
        return success

    def set_cpu_speed(self, percentage: int) -> bool:
        """Set manual speed percentage for CPU fan."""
        pct = max(0, min(100, int(percentage)))
        self.cpu_custom_pct = pct
        if self.current_mode == "custom":
            return self.wmi.set_fan_speed(0, pct)
        return True

    def set_gpu_speed(self, percentage: int) -> bool:
        """Set manual speed percentage for GPU fan."""
        pct = max(0, min(100, int(percentage)))
        self.gpu_custom_pct = pct
        if self.current_mode == "custom":
            return self.wmi.set_fan_speed(1, pct)
        return True

    def apply_custom_speeds(self, cpu_pct: int, gpu_pct: int) -> bool:
        """Apply custom speeds to both fans."""
        self.cpu_custom_pct = max(0, min(100, int(cpu_pct)))
        self.gpu_custom_pct = max(0, min(100, int(gpu_pct)))
        s1 = self.wmi.set_fan_speed(0, self.cpu_custom_pct)
        s2 = self.wmi.set_fan_speed(1, self.gpu_custom_pct)
        return s1 and s2

    def get_status(self) -> Dict[str, Any]:
        """Get fan status, modes, and current telemetry."""
        rpms = self.thermal.get_fan_rpm()
        return {
            "mode": self.current_mode,
            "coolboost": self.coolboost,
            "cpu_target_percent": self.cpu_custom_pct,
            "gpu_target_percent": self.gpu_custom_pct,
            "cpu_fan_rpm": rpms.get("cpu_fan_rpm", 0),
            "gpu_fan_rpm": rpms.get("gpu_fan_rpm", 0)
        }
