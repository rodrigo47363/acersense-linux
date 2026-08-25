"""
AcerSense GUI Application
Modern Dark Gaming UI for Acer Nitro / Predator / Aspire hardware control.
"""

import sys
import os
import threading
import time
from typing import Optional

try:
    import customtkinter as ctk
    USE_CTK = True
except ImportError:
    import tkinter as tk
    from tkinter import ttk, colorchooser
    USE_CTK = False

from tkinter import colorchooser

from acersense.hardware.wmi_interface import AcerWMIInterface
from acersense.hardware.thermal_interface import ThermalInterface
from acersense.hardware.battery_interface import BatteryInterface
from acersense.hardware.rgb_keyboard import RGBKeyboardController
from acersense.core.fan_manager import FanManager
from acersense.core.profile_manager import ProfileManager


class AcerSenseGUI:
    """Main graphical user interface for AcerSense Linux."""

    def __init__(self, root):
        self.root = root
        self.root.title("AcerSense Linux - Nitro & Predator Control Center")
        self.root.geometry("980x720")
        self.root.minsize(880, 640)

        # Hardware interfaces
        self.wmi = AcerWMIInterface()
        self.thermal = ThermalInterface()
        self.battery = BatteryInterface(self.wmi)
        self.fan = FanManager(self.wmi, self.thermal)
        self.rgb = RGBKeyboardController(self.wmi)
        self.profile_mgr = ProfileManager()

        # Theme Configuration
        if USE_CTK:
            ctk.set_appearance_mode("Dark")
            ctk.set_default_color_theme("dark-blue")

        self.color_accent = "#FF2233"     # Nitro Red Accent
        self.color_bg = "#121214"         # Dark Canvas
        self.color_card = "#1C1C22"       # Card Dark
        self.color_card_hover = "#282832" # Card Hover
        self.color_text_dim = "#A0A0B0"   # Muted text

        self._build_ui()
        self._start_telemetry_thread()

    def _build_ui(self):
        """Construct the UI layout."""
        if USE_CTK:
            self._build_ctk_ui()
        else:
            self._build_tk_ui()

    def _build_ctk_ui(self):
        """Build modern UI using CustomTkinter."""
        # Top Header Bar
        self.header_frame = ctk.CTkFrame(self.root, height=70, corner_radius=0, fg_color=self.color_card)
        self.header_frame.pack(fill="x", side="top", padx=0, pady=0)

        # Title Label
        title_lbl = ctk.CTkLabel(
            self.header_frame,
            text="ACERSENSE",
            font=ctk.CTkFont(size=24, weight="bold"),
            text_color=self.color_accent
        )
        title_lbl.pack(side="left", padx=25, pady=15)

        # Device Model Badge
        model_text = f"💻 {self.wmi.dmi_vendor} {self.wmi.product_name} | BIOS {self.wmi.bios_version}"
        self.model_lbl = ctk.CTkLabel(
            self.header_frame,
            text=model_text,
            font=ctk.CTkFont(size=13),
            text_color=self.color_text_dim
        )
        self.model_lbl.pack(side="right", padx=25, pady=15)

        # Tabview
        self.tabview = ctk.CTkTabview(self.root, fg_color=self.color_bg, segmented_button_selected_color=self.color_accent)
        self.tabview.pack(fill="both", expand=True, padx=15, pady=10)

        self.tab_dash = self.tabview.add("📊 Dashboard")
        self.tab_fan = self.tabview.add("🌀 Fan & Cooling")
        self.tab_battery = self.tabview.add("🔋 Battery Care")
        self.tab_rgb = self.tabview.add("🌈 RGB Keyboard")

        self._build_dashboard_tab(self.tab_dash)
        self._build_fan_tab(self.tab_fan)
        self._build_battery_tab(self.tab_battery)
        self._build_rgb_tab(self.tab_rgb)

    def _build_dashboard_tab(self, parent):
        """Dashboard overview cards."""
        parent.grid_columnconfigure((0, 1), weight=1)

        # CPU Card
        self.card_cpu = ctk.CTkFrame(parent, corner_radius=12, fg_color=self.color_card)
        self.card_cpu.grid(row=0, column=0, padx=10, pady=10, sticky="nsew")

        ctk.CTkLabel(self.card_cpu, text="Processor (CPU)", font=ctk.CTkFont(size=18, weight="bold"), text_color=self.color_accent).pack(anchor="w", padx=20, pady=(15, 5))
        self.lbl_cpu_temp = ctk.CTkLabel(self.card_cpu, text="Temp: -- °C", font=ctk.CTkFont(size=28, weight="bold"))
        self.lbl_cpu_temp.pack(anchor="w", padx=20, pady=2)
        self.lbl_cpu_usage = ctk.CTkLabel(self.card_cpu, text="Usage: --% | Clock: -- MHz", font=ctk.CTkFont(size=13), text_color=self.color_text_dim)
        self.lbl_cpu_usage.pack(anchor="w", padx=20, pady=(0, 15))

        # GPU Card
        self.card_gpu = ctk.CTkFrame(parent, corner_radius=12, fg_color=self.color_card)
        self.card_gpu.grid(row=0, column=1, padx=10, pady=10, sticky="nsew")

        ctk.CTkLabel(self.card_gpu, text="Dedicated GPU", font=ctk.CTkFont(size=18, weight="bold"), text_color="#00D26A").pack(anchor="w", padx=20, pady=(15, 5))
        self.lbl_gpu_temp = ctk.CTkLabel(self.card_gpu, text="Temp: -- °C", font=ctk.CTkFont(size=28, weight="bold"))
        self.lbl_gpu_temp.pack(anchor="w", padx=20, pady=2)
        self.lbl_gpu_usage = ctk.CTkLabel(self.card_gpu, text="Usage: --% | VRAM: -- MB", font=ctk.CTkFont(size=13), text_color=self.color_text_dim)
        self.lbl_gpu_usage.pack(anchor="w", padx=20, pady=(0, 15))

        # Fans Telemetry Card
        self.card_fans = ctk.CTkFrame(parent, corner_radius=12, fg_color=self.color_card)
        self.card_fans.grid(row=1, column=0, padx=10, pady=10, sticky="nsew")

        ctk.CTkLabel(self.card_fans, text="Active Fan Speeds", font=ctk.CTkFont(size=18, weight="bold"), text_color="#009BF0").pack(anchor="w", padx=20, pady=(15, 5))
        self.lbl_fan_cpu_rpm = ctk.CTkLabel(self.card_fans, text="CPU Fan: -- RPM", font=ctk.CTkFont(size=18, weight="bold"))
        self.lbl_fan_cpu_rpm.pack(anchor="w", padx=20, pady=2)
        self.lbl_fan_gpu_rpm = ctk.CTkLabel(self.card_fans, text="GPU Fan: -- RPM", font=ctk.CTkFont(size=18, weight="bold"))
        self.lbl_fan_gpu_rpm.pack(anchor="w", padx=20, pady=(0, 15))

        # System / Battery Card
        self.card_sys = ctk.CTkFrame(parent, corner_radius=12, fg_color=self.color_card)
        self.card_sys.grid(row=1, column=1, padx=10, pady=10, sticky="nsew")

        ctk.CTkLabel(self.card_sys, text="Power & Battery Status", font=ctk.CTkFont(size=18, weight="bold"), text_color="#F5A623").pack(anchor="w", padx=20, pady=(15, 5))
        self.lbl_bat_status = ctk.CTkLabel(self.card_sys, text="Battery: --% [--]", font=ctk.CTkFont(size=18, weight="bold"))
        self.lbl_bat_status.pack(anchor="w", padx=20, pady=2)
        self.lbl_bat_health = ctk.CTkLabel(self.card_sys, text="Health: --% | 80% Limit: --", font=ctk.CTkFont(size=13), text_color=self.color_text_dim)
        self.lbl_bat_health.pack(anchor="w", padx=20, pady=(0, 15))

    def _build_fan_tab(self, parent):
        """Fan modes and speed sliders."""
        card = ctk.CTkFrame(parent, corner_radius=12, fg_color=self.color_card)
        card.pack(fill="both", expand=True, padx=15, pady=15)

        ctk.CTkLabel(card, text="Cooling Policy & Fan Modes", font=ctk.CTkFont(size=20, weight="bold")).pack(anchor="w", padx=25, pady=(20, 10))

        # Fan Mode Segmented Buttons
        self.fan_mode_var = ctk.StringVar(value="auto")
        self.seg_fan_mode = ctk.CTkSegmentedButton(
            card,
            values=["auto", "max", "custom"],
            variable=self.fan_mode_var,
            command=self._on_fan_mode_changed,
            selected_color=self.color_accent,
            height=40,
            font=ctk.CTkFont(size=15, weight="bold")
        )
        self.seg_fan_mode.pack(fill="x", padx=25, pady=10)

        # CoolBoost Toggle
        self.coolboost_var = ctk.BooleanVar(value=False)
        self.switch_coolboost = ctk.CTkSwitch(
            card,
            text="Acer CoolBoost™ (Increases maximum fan RPM ceiling)",
            variable=self.coolboost_var,
            command=self._on_coolboost_changed,
            progress_color=self.color_accent,
            font=ctk.CTkFont(size=14)
        )
        self.switch_coolboost.pack(anchor="w", padx=25, pady=15)

        # Custom Speed Sliders
        ctk.CTkLabel(card, text="Manual Fan Target Speeds (Custom Mode)", font=ctk.CTkFont(size=16, weight="bold"), text_color=self.color_text_dim).pack(anchor="w", padx=25, pady=(15, 5))

        # CPU Fan Slider
        self.lbl_cpu_slider = ctk.CTkLabel(card, text="CPU Fan Speed: 50%", font=ctk.CTkFont(size=13))
        self.lbl_cpu_slider.pack(anchor="w", padx=25, pady=(5, 0))
        self.slider_cpu_fan = ctk.CTkSlider(card, from_=0, to=100, number_of_steps=100, command=self._on_cpu_slider_changed, button_color=self.color_accent)
        self.slider_cpu_fan.set(50)
        self.slider_cpu_fan.pack(fill="x", padx=25, pady=(0, 15))

        # GPU Fan Slider
        self.lbl_gpu_slider = ctk.CTkLabel(card, text="GPU Fan Speed: 50%", font=ctk.CTkFont(size=13))
        self.lbl_gpu_slider.pack(anchor="w", padx=25, pady=(5, 0))
        self.slider_gpu_fan = ctk.CTkSlider(card, from_=0, to=100, number_of_steps=100, command=self._on_gpu_slider_changed, button_color=self.color_accent)
        self.slider_gpu_fan.set(50)
        self.slider_gpu_fan.pack(fill="x", padx=25, pady=(0, 20))

    def _build_battery_tab(self, parent):
        """Acer Care Center Battery Health Tab."""
        card = ctk.CTkFrame(parent, corner_radius=12, fg_color=self.color_card)
        card.pack(fill="both", expand=True, padx=15, pady=15)

        ctk.CTkLabel(card, text="Battery Health & Charge Protection", font=ctk.CTkFont(size=20, weight="bold")).pack(anchor="w", padx=25, pady=(20, 10))

        # 80% Health Limiter Switch
        is_80_on = self.battery.wmi.get_battery_health_mode() or False
        self.battery_80_var = ctk.BooleanVar(value=is_80_on)
        self.switch_bat_80 = ctk.CTkSwitch(
            card,
            text="Acer Battery Health Protection (Caps charging at 80% to prolong battery lifespan)",
            variable=self.battery_80_var,
            command=self._on_battery_limit_changed,
            progress_color="#00D26A",
            font=ctk.CTkFont(size=14, weight="bold")
        )
        self.switch_bat_80.pack(anchor="w", padx=25, pady=20)

        # Telemetry info display
        self.bat_info_frame = ctk.CTkFrame(card, corner_radius=8, fg_color="#141418")
        self.bat_info_frame.pack(fill="both", expand=True, padx=25, pady=(0, 20))

        self.lbl_bat_wear = ctk.CTkLabel(self.bat_info_frame, text="🔋 Battery Wear Level / Health: Checking...", font=ctk.CTkFont(size=14))
        self.lbl_bat_wear.pack(anchor="w", padx=15, pady=8)

        self.lbl_bat_voltage = ctk.CTkLabel(self.bat_info_frame, text="⚡ Voltage & Current: Checking...", font=ctk.CTkFont(size=14))
        self.lbl_bat_voltage.pack(anchor="w", padx=15, pady=8)

        self.lbl_bat_cycles = ctk.CTkLabel(self.bat_info_frame, text="🔄 Charge Cycles: Checking...", font=ctk.CTkFont(size=14))
        self.lbl_bat_cycles.pack(anchor="w", padx=15, pady=8)

    def _build_rgb_tab(self, parent):
        """4-Zone RGB Keyboard Tab."""
        card = ctk.CTkFrame(parent, corner_radius=12, fg_color=self.color_card)
        card.pack(fill="both", expand=True, padx=15, pady=15)

        ctk.CTkLabel(card, text="4-Zone RGB Keyboard Illumination", font=ctk.CTkFont(size=20, weight="bold")).pack(anchor="w", padx=25, pady=(20, 10))

        # Zone Colors Display Row
        zones_frame = ctk.CTkFrame(card, fg_color="transparent")
        zones_frame.pack(fill="x", padx=25, pady=10)

        self.zone_buttons = []
        for i in range(1, 5):
            btn = ctk.CTkButton(
                zones_frame,
                text=f"Zone {i}\n{self.rgb.rgb_to_hex(self.rgb.zone_colors[i-1])}",
                fg_color=self.rgb.rgb_to_hex(self.rgb.zone_colors[i-1]),
                text_color="#FFFFFF",
                font=ctk.CTkFont(size=13, weight="bold"),
                height=60,
                corner_radius=8,
                command=lambda z=i: self._pick_zone_color(z)
            )
            btn.pack(side="left", fill="x", expand=True, padx=5)
            self.zone_buttons.append(btn)

        # Lighting Effects Row
        ctk.CTkLabel(card, text="Lighting Animation Preset", font=ctk.CTkFont(size=15, weight="bold")).pack(anchor="w", padx=25, pady=(15, 5))
        self.rgb_effect_var = ctk.StringVar(value="static")
        self.seg_rgb_effect = ctk.CTkSegmentedButton(
            card,
            values=["static", "breathing", "wave", "shift", "neon", "off"],
            variable=self.rgb_effect_var,
            command=self._on_rgb_effect_changed,
            selected_color=self.color_accent
        )
        self.seg_rgb_effect.pack(fill="x", padx=25, pady=5)

        # Brightness Slider
        self.lbl_rgb_bright = ctk.CTkLabel(card, text=f"Brightness: {self.rgb.brightness}%", font=ctk.CTkFont(size=13))
        self.lbl_rgb_bright.pack(anchor="w", padx=25, pady=(15, 0))
        self.slider_rgb_bright = ctk.CTkSlider(card, from_=0, to=100, command=self._on_rgb_bright_changed, button_color=self.color_accent)
        self.slider_rgb_bright.set(self.rgb.brightness)
        self.slider_rgb_bright.pack(fill="x", padx=25, pady=(0, 15))

    # Event Handlers
    def _on_fan_mode_changed(self, mode):
        self.fan.set_mode(mode)

    def _on_coolboost_changed(self):
        self.fan.set_coolboost(self.coolboost_var.get())

    def _on_cpu_slider_changed(self, val):
        pct = int(val)
        self.lbl_cpu_slider.configure(text=f"CPU Fan Speed: {pct}%")
        self.fan.set_cpu_speed(pct)

    def _on_gpu_slider_changed(self, val):
        pct = int(val)
        self.lbl_gpu_slider.configure(text=f"GPU Fan Speed: {pct}%")
        self.fan.set_gpu_speed(pct)

    def _on_battery_limit_changed(self):
        self.battery.set_80_percent_limit(self.battery_80_var.get())

    def _pick_zone_color(self, zone_idx):
        cur_hex = self.rgb.rgb_to_hex(self.rgb.zone_colors[zone_idx - 1])
        color = colorchooser.askcolor(color=cur_hex, title=f"Select Color for Zone {zone_idx}")
        if color and color[1]:
            hex_val = color[1].upper()
            self.rgb.set_zone_color(zone_idx, hex_val)
            self.zone_buttons[zone_idx - 1].configure(
                text=f"Zone {zone_idx}\n{hex_val}",
                fg_color=hex_val
            )

    def _on_rgb_effect_changed(self, effect):
        self.rgb.set_effect(effect)

    def _on_rgb_bright_changed(self, val):
        pct = int(val)
        self.lbl_rgb_bright.configure(text=f"Brightness: {pct}%")
        self.rgb.set_brightness(pct)

    def _start_telemetry_thread(self):
        """Background thread updating telemetry gauges."""
        self._running = True
        self.thread = threading.Thread(target=self._telemetry_loop, daemon=True)
        self.thread.start()

    def _telemetry_loop(self):
        while self._running:
            try:
                cpu = self.thermal.get_cpu_telemetry()
                gpu = self.thermal.get_gpu_telemetry()
                fans = self.thermal.get_fan_rpm()
                bat = self.battery.get_battery_telemetry()

                # Schedule UI update on main thread
                self.root.after(0, self._update_ui_telemetry, cpu, gpu, fans, bat)
            except Exception:
                pass
            time.sleep(1.2)

    def _update_ui_telemetry(self, cpu, gpu, fans, bat):
        try:
            # CPU
            self.lbl_cpu_temp.configure(text=f"Temp: {cpu['temperature']} °C")
            self.lbl_cpu_usage.configure(text=f"Usage: {cpu['usage_percent']}% | Clock: {cpu['frequency_mhz']} MHz")

            # GPU
            if gpu["active"]:
                self.lbl_gpu_temp.configure(text=f"Temp: {gpu['temperature']} °C")
                self.lbl_gpu_usage.configure(text=f"Usage: {gpu['usage_percent']}% | VRAM: {gpu['memory_used_mb']}/{gpu['memory_total_mb']} MB")
            else:
                self.lbl_gpu_temp.configure(text="State: Idle / Sleep")
                self.lbl_gpu_usage.configure(text="Nvidia GPU in low-power idle mode")

            # Fans
            self.lbl_fan_cpu_rpm.configure(text=f"CPU Fan: {fans['cpu_fan_rpm']} RPM")
            self.lbl_fan_gpu_rpm.configure(text=f"GPU Fan: {fans['gpu_fan_rpm']} RPM")

            # Battery
            self.lbl_bat_status.configure(text=f"Battery: {bat['percentage']}% [{bat['status']}]")
            self.lbl_bat_health.configure(text=f"Health: {bat['health_percent']}% | Wear: {round(100 - bat['health_percent'], 1)}%")

            self.lbl_bat_wear.configure(text=f"🔋 Battery Health: {bat['health_percent']}% ({bat['charge_full_mah']}/{bat['charge_design_mah']} mAh)")
            self.lbl_bat_voltage.configure(text=f"⚡ Voltage: {bat['voltage_v']} V | Current: {bat['current_a']} A | Power: {bat['power_draw_w']} W")
            self.lbl_bat_cycles.configure(text=f"🔄 Cycles: {bat['cycle_count']} | Temp: {bat['temperature_c']} °C | Tech: {bat['technology']}")
        except Exception:
            pass


def main():
    if USE_CTK:
        root = ctk.CTk()
    else:
        root = tk.Tk()
    app = AcerSenseGUI(root)
    root.mainloop()


if __name__ == "__main__":
    main()
