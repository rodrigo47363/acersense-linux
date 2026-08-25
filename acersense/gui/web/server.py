"""
AcerSense Local Web API & Static Server
Provides lightning-fast REST endpoints for the modern UI.
"""

import os
import json
import logging
from http.server import HTTPServer, SimpleHTTPRequestHandler
import urllib.parse
from typing import Optional

from acersense.hardware.wmi_interface import AcerWMIInterface
from acersense.hardware.thermal_interface import ThermalInterface
from acersense.hardware.battery_interface import BatteryInterface
from acersense.hardware.rgb_keyboard import RGBKeyboardController
from acersense.core.fan_manager import FanManager
from acersense.core.profile_manager import ProfileManager

logger = logging.getLogger("acersense.web")

WEB_ROOT = os.path.dirname(__file__)


class AcerSenseRequestHandler(SimpleHTTPRequestHandler):
    """Handles static web requests and REST API hardware commands."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=WEB_ROOT, **kwargs)

    @property
    def wmi(self):
        return self.server.wmi

    @property
    def thermal(self):
        return self.server.thermal

    @property
    def battery(self):
        return self.server.battery

    @property
    def fan(self):
        return self.server.fan

    @property
    def rgb(self):
        return self.server.rgb

    @property
    def profile_mgr(self):
        return self.server.profile_mgr

    def log_message(self, format, *args):
        # Silence routine static request logging to avoid terminal clutter
        pass

    def _send_json(self, data: dict, status: int = 200):
        body = json.dumps(data).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()
        self.wfile.write(body)

    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()

    def do_GET(self):
        url = urllib.parse.urlparse(self.path)
        if url.path == "/api/status":
            try:
                cpu = self.thermal.get_cpu_telemetry()
                gpu = self.thermal.get_gpu_telemetry()
                fans = self.thermal.get_fan_rpm()
                bat = self.battery.get_battery_telemetry()
                fan_stat = self.fan.get_status()
                rgb_stat = self.rgb.get_status()
                power_profile = self.wmi.get_power_profile()

                data = {
                    "hardware": {
                        "vendor": self.wmi.dmi_vendor,
                        "model": self.wmi.product_name,
                        "bios": self.wmi.bios_version
                    },
                    "cpu": cpu,
                    "gpu": gpu,
                    "fans": {
                        "cpu_rpm": fans.get("cpu_fan_rpm", 0),
                        "gpu_rpm": fans.get("gpu_fan_rpm", 0),
                        "mode": fan_stat["mode"],
                        "coolboost": fan_stat["coolboost"],
                        "cpu_target_pct": fan_stat["cpu_target_percent"],
                        "gpu_target_pct": fan_stat["gpu_target_percent"]
                    },
                    "battery": bat,
                    "rgb": rgb_stat,
                    "power_profile": power_profile
                }
                self._send_json(data)
            except Exception as e:
                logger.error(f"Error serving /api/status: {e}")
                self._send_json({"error": str(e)}, 500)
            return

        # Serve static web files
        super().do_GET()

    def do_POST(self):
        url = urllib.parse.urlparse(self.path)
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length).decode("utf-8") if length > 0 else "{}"
        try:
            payload = json.loads(body)
        except Exception:
            payload = {}

        if url.path == "/api/fan":
            mode = payload.get("mode")
            coolboost = payload.get("coolboost")
            cpu_pct = payload.get("cpu_pct")
            gpu_pct = payload.get("gpu_pct")

            if mode:
                self.fan.set_mode(mode)
            if coolboost is not None:
                self.fan.set_coolboost(bool(coolboost))
            if cpu_pct is not None or gpu_pct is not None:
                c = cpu_pct if cpu_pct is not None else self.fan.cpu_custom_pct
                g = gpu_pct if gpu_pct is not None else self.fan.gpu_custom_pct
                self.fan.apply_custom_speeds(c, g)

            self._send_json({"success": True, "status": self.fan.get_status()})
            return

        elif url.path == "/api/profile":
            profile = payload.get("profile")
            if profile:
                self.profile_mgr.apply_profile(profile)
                self._send_json({"success": True, "profile": profile})
            else:
                self._send_json({"error": "Profile name required"}, 400)
            return

        elif url.path == "/api/battery":
            limit_80 = payload.get("limit_80")
            if limit_80 is not None:
                self.battery.set_80_percent_limit(bool(limit_80))
                self._send_json({"success": True, "limit_80": bool(limit_80)})
            else:
                self._send_json({"error": "limit_80 boolean required"}, 400)
            return

        elif url.path == "/api/rgb":
            zone = payload.get("zone")
            color = payload.get("color")
            all_color = payload.get("all_color")
            effect = payload.get("effect")
            brightness = payload.get("brightness")
            timeout = payload.get("timeout_30s")

            if all_color:
                self.rgb.set_all_zones(all_color)
            if zone and color:
                self.rgb.set_zone_color(int(zone), color)
            if effect:
                self.rgb.set_effect(effect)
            if brightness is not None:
                self.rgb.set_brightness(int(brightness))
            if timeout is not None:
                self.rgb.set_backlight_timeout(bool(timeout))

            self._send_json({"success": True, "status": self.rgb.get_status()})
            return

        self._send_json({"error": "Not Found"}, 404)


class AcerSenseServer(HTTPServer):
    def __init__(self, server_address, RequestHandlerClass):
        super().__init__(server_address, RequestHandlerClass)
        self.wmi = AcerWMIInterface()
        self.thermal = ThermalInterface()
        self.battery = BatteryInterface(self.wmi)
        self.fan = FanManager(self.wmi, self.thermal)
        self.rgb = RGBKeyboardController(self.wmi)
        self.profile_mgr = ProfileManager()


def run_server(port: int = 18888) -> AcerSenseServer:
    server = AcerSenseServer(("127.0.0.1", port), AcerSenseRequestHandler)
    logger.info(f"AcerSense Web Server running on http://127.0.0.1:{port}")
    return server
