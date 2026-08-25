"""
AcerSense Native CustomTkinter GUI Module
Pure Python desktop window fallback.
"""

import threading
import time
from tkinter import colorchooser

try:
    import customtkinter as ctk
    USE_CTK = True
except ImportError:
    import tkinter as tk
    USE_CTK = False

from acersense.hardware.wmi_interface import AcerWMIInterface
from acersense.hardware.thermal_interface import ThermalInterface
from acersense.hardware.battery_interface import BatteryInterface
from acersense.hardware.rgb_keyboard import RGBKeyboardController
from acersense.core.fan_manager import FanManager
from acersense.core.profile_manager import ProfileManager


class AcerSenseGUI:
    def __init__(self, root):
        self.root = root
        self.root.title("AcerSense Linux (Native UI)")
        self.root.geometry("960x700")

        self.wmi = AcerWMIInterface()
        self.thermal = ThermalInterface()
        self.battery = BatteryInterface(self.wmi)
        self.fan = FanManager(self.wmi, self.thermal)
        self.rgb = RGBKeyboardController(self.wmi)
        self.profile_mgr = ProfileManager()

        if USE_CTK:
            ctk.set_appearance_mode("Dark")
            ctk.set_default_color_theme("dark-blue")

        self.color_accent = "#FF1E27"
        self.color_card = "#1C1C22"
        self._build_ui()

    def _build_ui(self):
        if not USE_CTK:
            return
        header = ctk.CTkFrame(self.root, height=60, fg_color=self.color_card)
        header.pack(fill="x", padx=0, pady=0)
        ctk.CTkLabel(header, text="🔥 ACERSENSE NATIVE", font=ctk.CTkFont(size=20, weight="bold"), text_color=self.color_accent).pack(side="left", padx=20, pady=10)

        tabs = ctk.CTkTabview(self.root, segmented_button_selected_color=self.color_accent)
        tabs.pack(fill="both", expand=True, padx=15, pady=10)
        
        t_dash = tabs.add("Dashboard")
        t_fan = tabs.add("Fan Control")
        t_bat = tabs.add("Battery")
        t_rgb = tabs.add("RGB")

        # Telemetry
        self.lbl_cpu = ctk.CTkLabel(t_dash, text="CPU: Loading...", font=ctk.CTkFont(size=18, weight="bold"))
        self.lbl_cpu.pack(pady=10)
        self.lbl_gpu = ctk.CTkLabel(t_dash, text="GPU: Loading...", font=ctk.CTkFont(size=18, weight="bold"))
        self.lbl_gpu.pack(pady=10)

        # Fan
        ctk.CTkSegmentedButton(t_fan, values=["auto", "max", "custom"], command=self.fan.set_mode, selected_color=self.color_accent).pack(fill="x", padx=20, pady=15)
        
        # Battery
        ctk.CTkSwitch(t_bat, text="80% Battery Limit", command=lambda: self.battery.set_80_percent_limit(True), progress_color="#00D26A").pack(pady=20)

        threading.Thread(target=self._telemetry_loop, daemon=True).start()

    def _telemetry_loop(self):
        while True:
            try:
                c = self.thermal.get_cpu_telemetry()
                g = self.thermal.get_gpu_telemetry()
                self.root.after(0, lambda: self.lbl_cpu.configure(text=f"CPU: {c['temperature']} °C | {c['frequency_mhz']} MHz | Usage: {c['usage_percent']}%"))
                self.root.after(0, lambda: self.lbl_gpu.configure(text=f"GPU: {g['temperature']} °C | Usage: {g['usage_percent']}%" if g['active'] else "GPU: Idle"))
            except Exception:
                pass
            time.sleep(1.0)
