"""
AcerSense CLI Interface
Command-line utility for managing Acer Nitro, Predator, and Aspire laptops.
"""

import sys
import time
import argparse
from typing import Optional

from acersense.hardware.wmi_interface import AcerWMIInterface
from acersense.hardware.thermal_interface import ThermalInterface
from acersense.hardware.battery_interface import BatteryInterface
from acersense.hardware.rgb_keyboard import RGBKeyboardController
from acersense.core.fan_manager import FanManager
from acersense.core.profile_manager import ProfileManager, PRESET_PROFILES


# ANSI Color Codes
CLR_RESET   = "\033[0m"
CLR_BOLD    = "\033[1m"
CLR_CYAN    = "\033[36m"
CLR_GREEN   = "\033[32m"
CLR_YELLOW  = "\033[33m"
CLR_RED     = "\033[31m"
CLR_MAGENTA = "\033[35m"
CLR_BLUE    = "\033[34m"
CLR_BG_DARK = "\033[40m"


def print_banner():
    banner = f"""{CLR_RED}{CLR_BOLD}
    █████╗  ██████╗███████╗██████╗ ███████╗███████╗███╗   ██╗███████╗███████╗
   ██╔══██╗██╔════╝██╔════╝██╔══██╗██╔════╝██╔════╝████╗  ██║██╔════╝██╔════╝
   ███████║██║     █████╗  ██████╔╝███████╗█████╗  ██╔██╗ ██║███████╗█████╗  
   ██╔══██║██║     ██╔══╝  ██╔══██╗╚════██║██╔══╝  ██║╚██╗██║╚════██║██╔══╝  
   ██║  ██║╚██████╗███████╗██║  ██║███████║███████╗██║ ╚████║███████║███████╗
   ╚═╝  ╚═╝ ╚═════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝{CLR_RESET}
   {CLR_CYAN}Open-Source Linux Control Suite for Acer Nitro & Predator Laptops{CLR_RESET}
"""
    print(banner)


def cmd_status(args):
    """Print full diagnostic status table."""
    wmi = AcerWMIInterface()
    thermal = ThermalInterface()
    battery = BatteryInterface(wmi)
    fan = FanManager(wmi, thermal)
    rgb = RGBKeyboardController(wmi)

    cpu = thermal.get_cpu_telemetry()
    gpu = thermal.get_gpu_telemetry()
    fans = thermal.get_fan_rpm()
    bat = battery.get_battery_telemetry()
    rgb_stat = rgb.get_status()
    power_prof = wmi.get_power_profile()

    print_banner()
    print(f"{CLR_BOLD}Hardware Model:{CLR_RESET} {CLR_GREEN}{wmi.dmi_vendor} {wmi.product_name} (BIOS {wmi.bios_version}){CLR_RESET}")
    print(f"{CLR_BOLD}Power Profile:{CLR_RESET}  {CLR_YELLOW}{power_prof.upper()}{CLR_RESET}\n")

    # Thermal & Frequency Table
    print(f"{CLR_BOLD}{CLR_CYAN}── Thermals & Processor Telemetry ───────────────────────────{CLR_RESET}")
    c_temp_color = CLR_RED if cpu["temperature"] > 80 else (CLR_YELLOW if cpu["temperature"] > 65 else CLR_GREEN)
    g_temp_color = CLR_RED if gpu["temperature"] > 80 else (CLR_YELLOW if gpu["temperature"] > 65 else CLR_GREEN)
    
    print(f"  CPU Temp:       {c_temp_color}{cpu['temperature']} °C{CLR_RESET}  | Usage: {cpu['usage_percent']}%  | Freq: {cpu['frequency_mhz']} MHz")
    if gpu["active"]:
        print(f"  GPU Temp ({gpu['name']}): {g_temp_color}{gpu['temperature']} °C{CLR_RESET}  | Usage: {gpu['usage_percent']}%  | VRAM: {gpu['memory_used_mb']}/{gpu['memory_total_mb']} MB")
    else:
        print(f"  GPU State:      {CLR_YELLOW}Suspended / Idle (Optimus / Power Savings){CLR_RESET}")

    # Fans Table
    print(f"\n{CLR_BOLD}{CLR_CYAN}── Cooling & Fan Subsystem ──────────────────────────────────{CLR_RESET}")
    cb_str = f"{CLR_GREEN}ON (+Turbo Curve){CLR_RESET}" if fan.coolboost else f"{CLR_YELLOW}OFF{CLR_RESET}"
    print(f"  Fan Mode:       {CLR_BOLD}{fan.current_mode.upper()}{CLR_RESET}  | CoolBoost: {cb_str}")
    print(f"  CPU Fan Speed:  {CLR_GREEN}{fans['cpu_fan_rpm']} RPM{CLR_RESET} (Target: {fan.cpu_custom_pct}%)")
    print(f"  GPU Fan Speed:  {CLR_GREEN}{fans['gpu_fan_rpm']} RPM{CLR_RESET} (Target: {fan.gpu_custom_pct}%)")

    # Battery Table
    print(f"\n{CLR_BOLD}{CLR_CYAN}── Battery Health & Power Management ────────────────────────{CLR_RESET}")
    health_limit_str = f"{CLR_GREEN}ACTIVE (Protected at 80% Max){CLR_RESET}" if bat["health_mode_80_limit"] else f"{CLR_YELLOW}DISABLED (Full 100% Charge){CLR_RESET}"
    print(f"  State of Charge: {CLR_BOLD}{bat['percentage']}%{CLR_RESET} [{bat['status']}]  | Health: {CLR_GREEN}{bat['health_percent']}%{CLR_RESET}")
    print(f"  80% Health Limit:{health_limit_str}")
    print(f"  Cycle Count:     {bat['cycle_count']} cycles  | Temperature: {bat['temperature_c']} °C  | Voltage: {bat['voltage_v']} V")

    # RGB Keyboard Table
    print(f"\n{CLR_BOLD}{CLR_CYAN}── 4-Zone RGB Keyboard Backlight ───────────────────────────{CLR_RESET}")
    print(f"  Brightness:     {rgb_stat['brightness']}%  | Active Effect: {rgb_stat['effect'].upper()}  | 30s Timeout: {'ON' if rgb_stat['timeout_30s'] else 'OFF'}")
    zone_str = " | ".join([f"Zone {i[-1]}: {v['hex']}" for i, v in rgb_stat["zones"].items()])
    print(f"  Colors:         {zone_str}\n")


def cmd_fan(args):
    """Manage fan mode and speeds."""
    wmi = AcerWMIInterface()
    fan = FanManager(wmi)

    if getattr(args, "status", False) or args.mode == "status" or (not args.mode and not args.coolboost and args.cpu is None and args.gpu is None):
        thermal = ThermalInterface()
        stat = fan.get_status()
        rpms = thermal.get_fan_rpm()
        print(f"{CLR_BOLD}── Cooling & Fan Subsystem ──────────────────────────────────{CLR_RESET}")
        print(f"  Fan Mode:       {CLR_GREEN}{stat['mode'].upper()}{CLR_RESET}  | CoolBoost: {'ON' if stat['coolboost'] else 'OFF'}")
        print(f"  CPU Fan Target: {stat['cpu_target_percent']}%  | Real Speed: {rpms['cpu_fan_rpm']} RPM")
        print(f"  GPU Fan Target: {stat['gpu_target_percent']}%  | Real Speed: {rpms['gpu_fan_rpm']} RPM")
        return

    if args.mode:
        fan.set_mode(args.mode)
        print(f"{CLR_GREEN}[+] Fan mode set to:{CLR_RESET} {args.mode.upper()}")

    if args.coolboost:
        cb_val = args.coolboost.lower() in ["on", "true", "1", "enable"]
        fan.set_coolboost(cb_val)
        print(f"{CLR_GREEN}[+] CoolBoost set to:{CLR_RESET} {'ON' if cb_val else 'OFF'}")

    if args.cpu is not None or args.gpu is not None:
        cpu_pct = args.cpu if args.cpu is not None else 50
        gpu_pct = args.gpu if args.gpu is not None else 50
        fan.set_mode("custom")
        fan.apply_custom_speeds(cpu_pct, gpu_pct)
        print(f"{CLR_GREEN}[+] Custom fan speeds applied:{CLR_RESET} CPU={cpu_pct}%, GPU={gpu_pct}%")


def cmd_profile(args):
    """Manage power and gaming profiles."""
    pm = ProfileManager()
    if args.list:
        print(f"{CLR_BOLD}Available Acer Profiles:{CLR_RESET}")
        for k, v in PRESET_PROFILES.items():
            print(f"  - {CLR_CYAN}{k:<12}{CLR_RESET}: {v['name']} (Fan: {v['fan_mode']}, Power: {v['power_profile']})")
        return

    if args.set:
        pm.apply_profile(args.set)
        print(f"{CLR_GREEN}[+] Applied profile:{CLR_RESET} {args.set.upper()}")


def cmd_battery(args):
    """Manage battery charge limiter."""
    battery = BatteryInterface()
    if args.limit_80:
        val = args.limit_80.lower() in ["on", "true", "1", "enable"]
        success = battery.set_80_percent_limit(val)
        if success:
            print(f"{CLR_GREEN}[+] Battery 80% Health Limit set to:{CLR_RESET} {'ENABLED' if val else 'DISABLED'}")
        else:
            print(f"{CLR_RED}[-] Failed to set battery health limit. Root or polkit permission required.{CLR_RESET}")

    if args.info:
        info = battery.get_battery_telemetry()
        print(f"{CLR_BOLD}Battery Telemetry:{CLR_RESET}")
        for k, v in info.items():
            print(f"  {k:<22}: {v}")


def cmd_rgb(args):
    """Control 4-Zone RGB keyboard."""
    rgb = RGBKeyboardController()

    if args.all:
        rgb.set_all_zones(args.all)
        print(f"{CLR_GREEN}[+] Set all keyboard zones to:{CLR_RESET} {args.all}")

    if args.zone and args.color:
        rgb.set_zone_color(args.zone, args.color)
        print(f"{CLR_GREEN}[+] Set Zone {args.zone} to:{CLR_RESET} {args.color}")

    if args.effect:
        rgb.set_effect(args.effect, speed=args.speed or 5)
        print(f"{CLR_GREEN}[+] Set RGB Effect to:{CLR_RESET} {args.effect.upper()}")

    if args.brightness is not None:
        rgb.set_brightness(args.brightness)
        print(f"{CLR_GREEN}[+] Set Backlight Brightness to:{CLR_RESET} {args.brightness}%")

    if args.timeout:
        val = args.timeout.lower() in ["on", "true", "1", "enable"]
        rgb.set_backlight_timeout(val)
        print(f"{CLR_GREEN}[+] Set 30s Backlight Timeout to:{CLR_RESET} {'ON' if val else 'OFF'}")


def cmd_monitor(args):
    """Live interactive terminal monitor."""
    thermal = ThermalInterface()
    wmi = AcerWMIInterface()
    battery = BatteryInterface(wmi)

    print("\033[2J\033[H", end="")  # Clear screen
    try:
        while True:
            cpu = thermal.get_cpu_telemetry()
            gpu = thermal.get_gpu_telemetry()
            fans = thermal.get_fan_rpm()
            bat = battery.get_battery_telemetry()

            print("\033[H", end="")  # Move cursor home
            print(f"{CLR_RED}{CLR_BOLD}=== ACERSENSE LIVE MONITOR (Press Ctrl+C to exit) ==={CLR_RESET}")
            print(f"Time: {time.strftime('%Y-%m-%d %H:%M:%S')} | Device: {wmi.product_name}\n")
            print(f"CPU Temp: {cpu['temperature']:>5.1f} °C  | Usage: {cpu['usage_percent']:>4.1f}% | Clock: {cpu['frequency_mhz']:>4.0f} MHz")
            print(f"GPU Temp: {gpu['temperature']:>5.1f} °C  | Usage: {gpu['usage_percent']:>4.1f}% | Power: {gpu['power_draw_w']:>4.1f} W")
            print(f"CPU Fan:  {fans['cpu_fan_rpm']:>5} RPM  | GPU Fan: {fans['gpu_fan_rpm']:>5} RPM")
            print(f"Battery:  {bat['percentage']:>5}% [{bat['status']:<11}] | Power: {bat['power_draw_w']:>4.1f} W | Limit 80%: {'ON' if bat['health_mode_80_limit'] else 'OFF'}")
            time.sleep(1.0)
    except KeyboardInterrupt:
        print(f"\n{CLR_CYAN}Monitor exited.{CLR_RESET}")


def main():
    parser = argparse.ArgumentParser(
        description="AcerSense Linux - Open Source Hardware Controller for Acer Laptops",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    subparsers = parser.add_subparsers(dest="subcommand", help="Available commands")

    # Status
    p_status = subparsers.add_parser("status", help="Show system thermals, fan speeds, battery health, and RGB state")
    p_status.set_defaults(func=cmd_status)

    # Fan
    p_fan = subparsers.add_parser("fan", help="Configure fan mode and custom speeds")
    p_fan.add_argument("--mode", choices=["auto", "max", "custom", "status"], help="Set fan mode")
    p_fan.add_argument("--status", action="store_true", help="Show fan status and RPMs")
    p_fan.add_argument("--coolboost", choices=["on", "off"], help="Toggle CoolBoost")
    p_fan.add_argument("--cpu", type=int, help="CPU fan speed percentage (0-100)")
    p_fan.add_argument("--gpu", type=int, help="GPU fan speed percentage (0-100)")
    p_fan.set_defaults(func=cmd_fan)

    # Profile
    p_prof = subparsers.add_parser("profile", help="Apply system power/gaming profile")
    p_prof.add_argument("--set", choices=["quiet", "balanced", "performance", "turbo"], help="Set profile")
    p_prof.add_argument("--list", action="store_true", help="List available profiles")
    p_prof.set_defaults(func=cmd_profile)

    # Battery
    p_bat = subparsers.add_parser("battery", help="Manage battery health and charge limits")
    p_bat.add_argument("--limit-80", choices=["on", "off"], help="Enable/Disable 80% maximum charge limiter")
    p_bat.add_argument("--info", action="store_true", help="Show detailed battery wear and health telemetry")
    p_bat.set_defaults(func=cmd_battery)

    # RGB
    p_rgb = subparsers.add_parser("rgb", help="Control 4-zone RGB keyboard lighting")
    p_rgb.add_argument("--all", help="Set hex color for all zones (e.g. #FF0000)")
    p_rgb.add_argument("--zone", type=int, choices=[1, 2, 3, 4], help="Target zone index (1-4)")
    p_rgb.add_argument("--color", help="Hex color for target zone (e.g. #00FF00)")
    p_rgb.add_argument("--effect", choices=["static", "breathing", "wave", "shift", "neon", "off"], help="Lighting effect")
    p_rgb.add_argument("--speed", type=int, help="Effect speed (1-10)")
    p_rgb.add_argument("--brightness", type=int, help="Brightness percentage (0-100)")
    p_rgb.add_argument("--timeout", choices=["on", "off"], help="Toggle 30s backlight timeout")
    p_rgb.set_defaults(func=cmd_rgb)

    # Monitor
    p_mon = subparsers.add_parser("monitor", help="Launch live interactive terminal telemetry monitor")
    p_mon.set_defaults(func=cmd_monitor)

    args = parser.parse_args()
    if not args.subcommand:
        cmd_status(args)
    else:
        args.func(args)


if __name__ == "__main__":
    main()
