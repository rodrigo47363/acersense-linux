"""
AcerSense 4-Zone RGB Keyboard & Backlight Controller
Supports per-zone RGB color adjustment, dynamic lighting effects, timeout rules, and brightness levels.
"""

import os
import glob
import logging
from typing import List, Dict, Any, Tuple, Optional
from acersense.hardware.wmi_interface import AcerWMIInterface

logger = logging.getLogger("acersense.rgb")


class RGBKeyboardController:
    """Controls Acer 4-Zone RGB Keyboard Backlight and effects."""

    def __init__(self, wmi: Optional[AcerWMIInterface] = None):
        self.wmi = wmi or AcerWMIInterface()
        self.led_sysfs_path = self._find_kbd_led_sysfs()
        
        # Default zone colors (Red, Blue, Green, Magenta)
        self.zone_colors = [
            (255, 0, 0),      # Zone 1 (Left)
            (0, 155, 240),    # Zone 2 (Center-Left)
            (0, 255, 0),      # Zone 3 (Center-Right)
            (255, 0, 255)     # Zone 4 (Right)
        ]
        self.current_effect = "static"
        self.brightness = self.get_brightness()
        self.timeout_30s = True

    @staticmethod
    def _find_kbd_led_sysfs() -> Optional[str]:
        candidates = glob.glob("/sys/class/leds/*kbd_backlight*") + glob.glob("/sys/class/leds/*::kbd_backlight")
        return candidates[0] if candidates else None

    @staticmethod
    def hex_to_rgb(hex_str: str) -> Tuple[int, int, int]:
        """Convert '#RRGGBB' or 'RRGGBB' to (R, G, B) tuple."""
        hex_str = hex_str.lstrip("#").strip()
        if len(hex_str) == 3:
            hex_str = "".join([c * 2 for c in hex_str])
        if len(hex_str) != 6:
            return (255, 0, 0)
        try:
            return (int(hex_str[0:2], 16), int(hex_str[2:4], 16), int(hex_str[4:6], 16))
        except ValueError:
            return (255, 0, 0)

    @staticmethod
    def rgb_to_hex(rgb: Tuple[int, int, int]) -> str:
        """Convert (R, G, B) tuple to '#RRGGBB' string."""
        return f"#{rgb[0]:02X}{rgb[1]:02X}{rgb[2]:02X}"

    def get_brightness(self) -> int:
        """Get current backlight brightness (0-100%)."""
        if self.led_sysfs_path:
            b_path = os.path.join(self.led_sysfs_path, "brightness")
            max_b_path = os.path.join(self.led_sysfs_path, "max_brightness")
            if os.path.exists(b_path) and os.path.exists(max_b_path):
                try:
                    with open(b_path, "r") as f:
                        cur = int(f.read().strip())
                    with open(max_b_path, "r") as f:
                        max_b = int(f.read().strip())
                    if max_b > 0:
                        return int((cur / max_b) * 100)
                except Exception:
                    pass
        return 100

    def set_brightness(self, percentage: int) -> bool:
        """Set backlight brightness (0-100%)."""
        percentage = max(0, min(100, int(percentage)))
        self.brightness = percentage

        # 1. Sysfs LED brightness
        if self.led_sysfs_path:
            b_path = os.path.join(self.led_sysfs_path, "brightness")
            max_b_path = os.path.join(self.led_sysfs_path, "max_brightness")
            if os.path.exists(b_path) and os.path.exists(max_b_path):
                try:
                    with open(max_b_path, "r") as f:
                        max_b = int(f.read().strip())
                    val = int((percentage / 100.0) * max_b)
                    self.wmi._write_sysfs(b_path, str(val))
                except Exception as e:
                    logger.debug(f"Failed to set sysfs brightness: {e}")

        # 2. WMI Gaming KB Backlight opcode
        # Reverse-engineered: SetAcerGamingKBBacklight(1 | (percentage << 8))
        if self.wmi.has_acpi_call:
            opcode = 1 | (percentage << 8)
            self.wmi.call_acpi_method(r"\_SB.WMID.WMAA", 0x1C, opcode)

        logger.info(f"Keyboard brightness set to {percentage}%")
        return True

    def set_zone_color(self, zone_index: int, color_hex: str) -> bool:
        """
        Set color for a specific zone (1 to 4).
        Reverse-engineered formula: SetAcerGamingLEDGroupColor(zone_idx | (R<<8) | (G<<16) | (B<<24))
        """
        if zone_index < 1 or zone_index > 4:
            raise ValueError(f"Zone index must be 1, 2, 3, or 4. Got {zone_index}")

        r, g, b = self.hex_to_rgb(color_hex)
        self.zone_colors[zone_index - 1] = (r, g, b)

        # Build WMI opcode: zone | (R << 8) | (G << 16) | (B << 24)
        opcode = (zone_index & 0xFF) | ((r & 0xFF) << 8) | ((g & 0xFF) << 16) | ((b & 0xFF) << 24)

        if self.wmi.has_acpi_call:
            self.wmi.call_acpi_method(r"\_SB.WMID.WMAA", 0x0B, opcode)

        logger.info(f"Zone {zone_index} color set to RGB({r},{g},{b}) / {color_hex}")
        return True

    def set_all_zones(self, color_hex: str) -> bool:
        """Set all 4 zones to the same color."""
        success = True
        for z in range(1, 5):
            if not self.set_zone_color(z, color_hex):
                success = False
        return success

    def set_effect(self, effect_name: str, speed: int = 5, direction: int = 0) -> bool:
        """
        Set dynamic lighting effect: 'static', 'breathing', 'wave', 'neon', 'shift', 'off'.
        """
        effect_name = effect_name.lower()
        effect_map = {
            "static": 0,
            "off": 0,
            "breathing": 1,
            "wave": 2,
            "shift": 3,
            "neon": 4,
            "zoom": 5
        }
        effect_id = effect_map.get(effect_name, 0)
        self.current_effect = effect_name

        if effect_name == "off":
            self.set_brightness(0)
            return True

        if effect_name == "static":
            for z, rgb in enumerate(self.zone_colors, 1):
                self.set_zone_color(z, self.rgb_to_hex(rgb))
            return True

        # Dynamic WMI Effect opcode: SetGamingLEDBehavior(effect_id | (speed << 8) | (direction << 16))
        opcode = (effect_id & 0xFF) | ((speed & 0xFF) << 8) | ((direction & 0xFF) << 16)
        if self.wmi.has_acpi_call:
            self.wmi.call_acpi_method(r"\_SB.WMID.WMAA", 0x1E, opcode)

        logger.info(f"Applied RGB Effect '{effect_name}' (Speed: {speed}, Dir: {direction})")
        return True

    def set_backlight_timeout(self, enable_30s_timeout: bool) -> bool:
        """
        Enable/disable 30-second keyboard backlight automatic timeout.
        Reverse-engineered formula: SetGamingKBBacklight(0x01 | (timeout_val << 8))
        """
        self.timeout_30s = enable_30s_timeout
        opcode = 1 | ((30 if enable_30s_timeout else 0) << 8)
        if self.wmi.has_acpi_call:
            self.wmi.call_acpi_method(r"\_SB.WMID.WMAA", 0x1C, opcode)
        logger.info(f"Backlight 30s timeout set to: {enable_30s_timeout}")
        return True

    def get_status(self) -> Dict[str, Any]:
        """Get complete RGB keyboard state."""
        return {
            "brightness": self.brightness,
            "effect": self.current_effect,
            "timeout_30s": self.timeout_30s,
            "zones": {
                f"zone_{i+1}": {
                    "hex": self.rgb_to_hex(rgb),
                    "rgb": rgb
                } for i, rgb in enumerate(self.zone_colors)
            }
        }
