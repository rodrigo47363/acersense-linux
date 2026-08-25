"""
AcerSense Thermal & Sensor Telemetry Interface
Reads accurate temperatures, fan RPMs, frequencies, and utilizations from sysfs and GPU interfaces.
"""

import os
import glob
import subprocess
import shutil
import psutil
from typing import Dict, Any, List, Optional


class ThermalInterface:
    """Provides real-time hardware telemetry for CPU, GPU, Fans, and System."""

    def __init__(self):
        self.has_nvidia_smi = shutil.which("nvidia-smi") is not None
        self._hwmon_paths = self._scan_hwmon_sensors()

    def _scan_hwmon_sensors(self) -> Dict[str, List[str]]:
        """Scans /sys/class/hwmon/ for temperature and fan sensor nodes."""
        mapping = {"cpu_temp": [], "fans": [], "system_temp": []}
        hwmon_dirs = glob.glob("/sys/class/hwmon/hwmon*")
        
        for d in hwmon_dirs:
            name_file = os.path.join(d, "name")
            name = ""
            if os.path.exists(name_file):
                try:
                    with open(name_file, "r") as f:
                        name = f.read().strip()
                except Exception:
                    pass

            # Fan inputs
            fan_inputs = glob.glob(os.path.join(d, "fan*_input"))
            for fan in fan_inputs:
                mapping["fans"].append(fan)

            # CPU / SOC inputs
            if name in ["coretemp", "k10temp", "zenpower", "cpu_thermal"]:
                temp_inputs = glob.glob(os.path.join(d, "temp*_input"))
                for t in temp_inputs:
                    label_file = t.replace("_input", "_label")
                    label = ""
                    if os.path.exists(label_file):
                        try:
                            with open(label_file, "r") as f:
                                label = f.read().strip()
                        except Exception:
                            pass
                    # Prioritize Package / Tdie / composite temp
                    if any(k in label for k in ["Package", "Tdie", "Composite", "Core 0"]) or not label:
                        mapping["cpu_temp"].append(t)
            elif name in ["acpitz", "pch_cannonlake", "pch_cometlake", "pch_alderlake", "thinkpad"]:
                temp_inputs = glob.glob(os.path.join(d, "temp*_input"))
                mapping["system_temp"].extend(temp_inputs)

        return mapping

    @staticmethod
    def _read_int_file(path: str) -> Optional[int]:
        try:
            if os.path.exists(path):
                with open(path, "r") as f:
                    return int(f.read().strip())
        except Exception:
            pass
        return None

    def get_cpu_telemetry(self) -> Dict[str, Any]:
        """Get CPU Temperature (°C), Utilization (%), and Clock Speed (MHz)."""
        temp_c = 0.0
        temps_found = []
        for path in self._hwmon_paths.get("cpu_temp", []):
            val = self._read_int_file(path)
            if val is not None and val > 0:
                temps_found.append(val / 1000.0)

        if temps_found:
            temp_c = max(temps_found)
        else:
            # Fallback to psutil sensors_temperatures
            try:
                temps = psutil.sensors_temperatures()
                for key in ["coretemp", "k10temp", "acpitz", "cpu_thermal"]:
                    if key in temps and temps[key]:
                        temp_c = max([entry.current for entry in temps[key]])
                        break
            except Exception:
                pass

        # CPU Usage & Frequency
        cpu_usage = psutil.cpu_percent(interval=None)
        cpu_freq_info = psutil.cpu_freq()
        cur_freq_mhz = cpu_freq_info.current if cpu_freq_info else 0.0
        max_freq_mhz = cpu_freq_info.max if cpu_freq_info else 0.0

        return {
            "temperature": round(temp_c, 1),
            "usage_percent": round(cpu_usage, 1),
            "frequency_mhz": round(cur_freq_mhz, 0),
            "max_frequency_mhz": round(max_freq_mhz, 0),
            "core_count": psutil.cpu_count(logical=False) or 4,
            "thread_count": psutil.cpu_count(logical=True) or 8,
        }

    def get_gpu_telemetry(self) -> Dict[str, Any]:
        """Get dedicated Nvidia/AMD GPU Temperature, Utilization, Memory, and Fan Speed."""
        gpu_data = {
            "name": "Discrete GPU",
            "temperature": 0.0,
            "usage_percent": 0.0,
            "memory_used_mb": 0,
            "memory_total_mb": 0,
            "power_draw_w": 0.0,
            "fan_speed_percent": 0,
            "active": False
        }

        # Check Nvidia GPU
        if self.has_nvidia_smi:
            try:
                cmd = [
                    "nvidia-smi",
                    "--query-gpu=name,temperature.gpu,utilization.gpu,memory.used,memory.total,power.draw,fan.speed",
                    "--format=csv,noheader,nounits"
                ]
                output = subprocess.check_output(cmd, stderr=subprocess.DEVNULL, timeout=1.0).decode().strip()
                if output:
                    parts = [p.strip() for p in output.split(",")]
                    if len(parts) >= 6:
                        gpu_data["name"] = parts[0]
                        gpu_data["temperature"] = float(parts[1]) if parts[1] != "[N/A]" else 0.0
                        gpu_data["usage_percent"] = float(parts[2]) if parts[2] != "[N/A]" else 0.0
                        gpu_data["memory_used_mb"] = int(float(parts[3])) if parts[3] != "[N/A]" else 0
                        gpu_data["memory_total_mb"] = int(float(parts[4])) if parts[4] != "[N/A]" else 0
                        gpu_data["power_draw_w"] = float(parts[5]) if parts[5] != "[N/A]" else 0.0
                        if len(parts) >= 7 and parts[6] != "[N/A]":
                            gpu_data["fan_speed_percent"] = int(float(parts[6]))
                        gpu_data["active"] = True
                        return gpu_data
            except Exception:
                pass

        # Fallback to sysfs for AMD/Intel GPU
        amdgpu_dirs = glob.glob("/sys/class/drm/card*/device/hwmon/hwmon*")
        for d in amdgpu_dirs:
            temp_file = os.path.join(d, "temp1_input")
            val = self._read_int_file(temp_file)
            if val is not None and val > 0:
                gpu_data["temperature"] = round(val / 1000.0, 1)
                gpu_data["active"] = True
                break

        return gpu_data

    def get_fan_rpm(self) -> Dict[str, int]:
        """Read active RPM for CPU and GPU fans."""
        fan_speeds = {"cpu_fan_rpm": 0, "gpu_fan_rpm": 0}
        fan_nodes = self._hwmon_paths.get("fans", [])
        
        rpms = []
        for fnode in fan_nodes:
            rpm = self._read_int_file(fnode)
            if rpm is not None and rpm >= 0:
                rpms.append(rpm)

        if len(rpms) >= 1:
            fan_speeds["cpu_fan_rpm"] = rpms[0]
        if len(rpms) >= 2:
            fan_speeds["gpu_fan_rpm"] = rpms[1]
        
        return fan_speeds

    def get_system_telemetry(self) -> Dict[str, Any]:
        """Get RAM and Swap usage stats."""
        mem = psutil.virtual_memory()
        swap = psutil.swap_memory()
        return {
            "ram_used_gb": round(mem.used / (1024**3), 2),
            "ram_total_gb": round(mem.total / (1024**3), 2),
            "ram_percent": mem.percent,
            "swap_percent": swap.percent
        }
