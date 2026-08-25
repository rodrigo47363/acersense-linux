"""
AcerSense Profile & Configuration Manager
Manages system presets (Quiet, Balanced, Performance, Turbo) and user custom settings persistence.
"""

import os
import json
import logging
from typing import Dict, Any, Optional
from acersense.hardware.wmi_interface import AcerWMIInterface
from acersense.hardware.battery_interface import BatteryInterface
from acersense.hardware.rgb_keyboard import RGBKeyboardController
from acersense.core.fan_manager import FanManager

logger = logging.getLogger("acersense.profiles")

DEFAULT_CONFIG_PATH = os.path.expanduser("~/.config/acersense/config.json")

PRESET_PROFILES = {
    "quiet": {
        "name": "Quiet / Eco",
        "power_profile": "quiet",
        "fan_mode": "auto",
        "coolboost": False,
        "battery_80_limit": True,
        "rgb_brightness": 30,
        "rgb_effect": "static"
    },
    "balanced": {
        "name": "Balanced / Default",
        "power_profile": "balanced",
        "fan_mode": "auto",
        "coolboost": False,
        "battery_80_limit": True,
        "rgb_brightness": 70,
        "rgb_effect": "static"
    },
    "performance": {
        "name": "Performance",
        "power_profile": "performance",
        "fan_mode": "custom",
        "cpu_fan_pct": 75,
        "gpu_fan_pct": 75,
        "coolboost": True,
        "battery_80_limit": False,
        "rgb_brightness": 100,
        "rgb_effect": "static"
    },
    "turbo": {
        "name": "Turbo / Extreme",
        "power_profile": "performance",
        "fan_mode": "max",
        "coolboost": True,
        "battery_80_limit": False,
        "rgb_brightness": 100,
        "rgb_effect": "wave"
    }
}


class ProfileManager:
    """Manages preset profiles and persists user configuration."""

    def __init__(self, config_path: str = DEFAULT_CONFIG_PATH):
        self.config_path = config_path
        self.wmi = AcerWMIInterface()
        self.fan = FanManager(self.wmi)
        self.battery = BatteryInterface(self.wmi)
        self.rgb = RGBKeyboardController(self.wmi)
        
        self.active_profile = "balanced"
        self.config = self.load_config()

    def load_config(self) -> Dict[str, Any]:
        """Loads configuration from JSON file or initializes defaults."""
        if os.path.exists(self.config_path):
            try:
                with open(self.config_path, "r") as f:
                    return json.load(f)
            except Exception as e:
                logger.error(f"Error reading config {self.config_path}: {e}")
        
        # Default configuration
        default_cfg = {
            "active_profile": "balanced",
            "fan": {
                "mode": "auto",
                "coolboost": False,
                "cpu_pct": 50,
                "gpu_pct": 50
            },
            "power": {
                "profile": "balanced"
            },
            "battery": {
                "health_mode_80_limit": True
            },
            "rgb": {
                "brightness": 80,
                "effect": "static",
                "timeout_30s": True,
                "zones": ["#FF0000", "#009BF0", "#00FF00", "#FF00FF"]
            }
        }
        self.save_config(default_cfg)
        return default_cfg

    def save_config(self, cfg: Optional[Dict[str, Any]] = None) -> bool:
        """Saves current state to JSON config file."""
        if cfg is None:
            cfg = {
                "active_profile": self.active_profile,
                "fan": {
                    "mode": self.fan.current_mode,
                    "coolboost": self.fan.coolboost,
                    "cpu_pct": self.fan.cpu_custom_pct,
                    "gpu_pct": self.fan.gpu_custom_pct
                },
                "power": {
                    "profile": self.wmi.get_power_profile()
                },
                "battery": {
                    "health_mode_80_limit": self.battery.wmi.get_battery_health_mode() or True
                },
                "rgb": {
                    "brightness": self.rgb.brightness,
                    "effect": self.rgb.current_effect,
                    "timeout_30s": self.rgb.timeout_30s,
                    "zones": [self.rgb.rgb_to_hex(c) for c in self.rgb.zone_colors]
                }
            }
        
        try:
            os.makedirs(os.path.dirname(self.config_path), exist_ok=True)
            with open(self.config_path, "w") as f:
                json.dump(cfg, f, indent=4)
            self.config = cfg
            return True
        except Exception as e:
            logger.error(f"Error saving config to {self.config_path}: {e}")
            return False

    def apply_profile(self, profile_name: str) -> bool:
        """Applies a preset profile by name."""
        name_lower = profile_name.lower()
        if name_lower not in PRESET_PROFILES:
            raise ValueError(f"Unknown profile '{profile_name}'. Available: {list(PRESET_PROFILES.keys())}")

        profile = PRESET_PROFILES[name_lower]
        self.active_profile = name_lower

        # Apply Power Profile
        self.wmi.set_power_profile(profile["power_profile"])

        # Apply Fan Settings
        self.fan.set_mode(profile["fan_mode"])
        self.fan.set_coolboost(profile["coolboost"])
        if profile["fan_mode"] == "custom":
            self.fan.apply_custom_speeds(profile.get("cpu_fan_pct", 75), profile.get("gpu_fan_pct", 75))

        # Apply Battery Limit
        self.battery.set_80_percent_limit(profile.get("battery_80_limit", True))

        # Apply RGB Settings
        self.rgb.set_brightness(profile.get("rgb_brightness", 80))
        self.rgb.set_effect(profile.get("rgb_effect", "static"))

        self.save_config()
        logger.info(f"Applied profile '{profile['name']}' successfully.")
        return True
