"""
AcerSense Battery Care & Health Interface
Manages the 80% battery health charge limit, battery calibration, and detailed telemetry.
"""

import os
import glob
from typing import Dict, Any, Optional
from acersense.hardware.wmi_interface import AcerWMIInterface


class BatteryInterface:
    """Provides complete battery management and diagnostics."""

    def __init__(self, wmi: Optional[AcerWMIInterface] = None):
        self.wmi = wmi or AcerWMIInterface()
        self.bat_path = self._find_primary_battery()

    @staticmethod
    def _find_primary_battery() -> str:
        candidates = glob.glob("/sys/class/power_supply/BAT*")
        return candidates[0] if candidates else "/sys/class/power_supply/BAT1"

    def _read_attr(self, name: str, default: str = "") -> str:
        path = os.path.join(self.bat_path, name)
        if os.path.exists(path):
            try:
                with open(path, "r") as f:
                    return f.read().strip()
            except Exception:
                pass
        return default

    def _read_int(self, name: str, default: int = 0) -> int:
        val = self._read_attr(name, "")
        try:
            return int(val)
        except ValueError:
            return default

    def get_battery_telemetry(self) -> Dict[str, Any]:
        """Returns comprehensive battery health, wear percentage, voltage, and charging stats."""
        status = self._read_attr("status", "Unknown")
        capacity_pct = self._read_int("capacity", 0)
        cycles = self._read_int("cycle_count", 0)
        
        # Charge / Energy readings
        charge_now = self._read_int("charge_now", 0) or self._read_int("energy_now", 0)
        charge_full = self._read_int("charge_full", 0) or self._read_int("energy_full", 0)
        charge_design = self._read_int("charge_full_design", 0) or self._read_int("energy_full_design", 0)
        
        # Calculate battery health / wear level
        health_pct = 100.0
        if charge_design > 0 and charge_full > 0:
            health_pct = round((charge_full / charge_design) * 100.0, 1)

        # Voltage & Current
        voltage_v = round(self._read_int("voltage_now", 0) / 1_000_000.0, 2)
        current_a = round(abs(self._read_int("current_now", 0)) / 1_000_000.0, 2)
        power_w = round(voltage_v * current_a, 2)

        # Acer WMI Battery Temperature
        temp_c = 0.0
        wmi_temp_path = "/sys/bus/wmi/drivers/acer-wmi-battery/temperature"
        if os.path.exists(wmi_temp_path):
            try:
                with open(wmi_temp_path, "r") as f:
                    temp_c = round(int(f.read().strip()) / 1000.0, 1)
            except Exception:
                pass

        # Health Mode Status
        health_mode_active = self.wmi.get_battery_health_mode()

        return {
            "present": self._read_attr("present", "1") == "1",
            "status": status,
            "percentage": capacity_pct,
            "health_percent": health_pct,
            "cycle_count": cycles,
            "voltage_v": voltage_v,
            "current_a": current_a,
            "power_draw_w": power_w,
            "temperature_c": temp_c,
            "manufacturer": self._read_attr("manufacturer", "Acer / OEM"),
            "model_name": self._read_attr("model_name", "Primary"),
            "technology": self._read_attr("technology", "Li-ion"),
            "health_mode_80_limit": health_mode_active,
            "charge_full_mah": round(charge_full / 1000.0, 0) if charge_full else 0,
            "charge_design_mah": round(charge_design / 1000.0, 0) if charge_design else 0,
        }

    def set_80_percent_limit(self, enable: bool) -> bool:
        """Enable or disable the 80% maximum charge limiter (Acer Care Center Battery Health)."""
        return self.wmi.set_battery_health_mode(enable)
