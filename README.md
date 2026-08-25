# AcerSense Linux (NitroSense & Care Center for Linux)

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-red.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Python: 3.8+](https://img.shields.io/badge/Python-3.8%2B-blue.svg)](https://www.python.org/)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux%20(Parrot%20%7C%20Debian%20%7C%20Ubuntu%20%7C%20Arch%20%7C%20Fedora)-brightgreen.svg)]()
[![Hardware: Acer Nitro & Predator](https://img.shields.io/badge/Hardware-Acer%20Nitro%20%7C%20Predator%20%7C%20Aspire-orange.svg)]()

**AcerSense Linux** is a complete, open-source Linux software suite and daemon designed to replace proprietary Windows software (**Acer NitroSense**, **Acer Care Center**, and **Quick Access**) on Acer gaming and productivity laptops.

Reverse-engineered from original OEM Windows binaries, **AcerSense Linux** provides full native control over fan curves, cooling modes, power profiles, the 80% battery health charge limiter, and 4-zone RGB keyboard lighting without requiring Windows.

---

## ⚡ Features

*   🌀 **Complete Fan & Cooling Control (NitroSense)**:
    *   **Modes:** `Auto`, `Max` (Full Turbo Fan Speed), and `Custom`.
    *   **Individual Fan Adjustment:** Set exact target speeds (0–100%) for both CPU and GPU fans.
    *   **Acer CoolBoost™:** Toggle hardware fan curve overclocking for maximum thermal dissipation.
*   🔋 **Battery Care & Protection (Acer Care Center)**:
    *   **80% Health Limiter:** Enforce the hardware 80% maximum charge threshold to prevent Li-ion degradation.
    *   **Comprehensive Battery Diagnostics:** Wear level percentage, cycle counts, voltage, wattage, and internal battery temperature.
*   🌈 **4-Zone RGB Keyboard Lighting**:
    *   Independent hex color pickers for **Zone 1 (Left)**, **Zone 2 (Center-Left)**, **Zone 3 (Center-Right)**, and **Zone 4 (Right)**.
    *   Dynamic lighting presets: `Static`, `Breathing`, `Wave`, `Shift`, `Neon`, and `Off`.
    *   Backlight brightness level control (0–100%) and 30-second automatic idle sleep toggle.
*   🚀 **Performance & Power Profiles**:
    *   One-click switching between `Quiet / Eco`, `Balanced`, `Performance`, and `Turbo`.
    *   Integrated with Linux kernel `platform_profile` subsystem.
*   📊 **Real-Time Telemetry & Monitoring**:
    *   Accurate CPU & dedicated Nvidia/AMD GPU temperatures, clocks, VRAM, and power draw.
    *   Real-time fan RPM speed monitoring.
*   🖥️ **Dual Interfaces**:
    *   **Modern Dark Gaming GUI:** Built with modern styling matching the NitroSense / Predator aesthetic.
    *   **Feature-Rich CLI:** Colored terminal tables, scripts-ready subcommands, and a live terminal dashboard (`acersense monitor`).
*   ⚙️ **Systemd Daemon & Persistence**:
    *   Applies user profiles on system boot and restores settings automatically after sleep/hibernation.

---

## 💻 Supported Laptop Models

Tested and confirmed on Acer Nitro and Predator series, including:
*   **Acer Nitro 5:** AN515-42, AN515-43, AN515-44, AN515-45, AN515-46, AN515-47, AN515-51, AN515-52, AN515-53, AN515-54, **AN515-55**, AN515-56, AN515-57, AN515-58
*   **Acer Nitro 16 & 17:** AN516-51, AN517-41, AN517-42, AN517-51, AN517-52, AN517-54, AN517-55
*   **Acer Predator Helios & Triton:** PH315-51/52/53/54, PH317, PT314, PT515, PT516
*   **Acer Aspire Gaming:** A715 series

---

## 📦 Installation

### Automatic 1-Step Installation (Recommended)

Clone the repository and run the installer script:

```bash
git clone https://github.com/rodrigo47363/acersense-linux.git
cd acersense-linux
chmod +x install.sh
sudo ./install.sh
```

The script automatically installs dependencies, copies binaries to `/usr/local/bin`, installs udev rules for non-root control, enables the `acersensed` systemd daemon, and sets up desktop shortcuts.

### Manual Installation

```bash
git clone https://github.com/rodrigo47363/acersense-linux.git
cd acersense-linux
pip install -e .
sudo cp udev/99-acersense.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo cp systemd/acersensed.service /etc/systemd/system/
sudo systemctl enable --now acersensed.service
```

---

## 🚀 Usage Guide

### 1. Graphical Interface (GUI)

Launch the GUI application from your terminal or desktop applications menu:

```bash
acersense-gui
```

---

### 2. Command-Line Interface (CLI)

#### View Complete System Status
```bash
acersense status
```

#### Fan & Cooling Controls
```bash
# Set fan mode to Auto, Max (100% Turbo), or Custom
acersense fan --mode auto
acersense fan --mode max
acersense fan --mode custom

# Set custom target speeds for CPU and GPU fans
acersense fan --cpu 75 --gpu 80

# Toggle Acer CoolBoost
acersense fan --coolboost on
acersense fan --coolboost off
```

#### Power & Performance Profiles
```bash
# List available profiles
acersense profile --list

# Apply profile
acersense profile --set quiet
acersense profile --set balanced
acersense profile --set performance
acersense profile --set turbo
```

#### Battery Health & Care (Acer Care Center 80% Limit)
```bash
# Enable 80% maximum charging protection limit
acersense battery --limit-80 on

# Disable limit (charge to 100%)
acersense battery --limit-80 off

# Detailed battery health diagnostics
acersense battery --info
```

#### 4-Zone RGB Keyboard Lighting
```bash
# Set all zones to a single color (Hex)
acersense rgb --all #FF0000

# Set individual zone colors
acersense rgb --zone 1 #FF0000 --zone 2 #009BF0 --zone 3 #00FF00 --zone 4 #FF00FF

# Apply animation effect (static, breathing, wave, shift, neon, off)
acersense rgb --effect wave --speed 5

# Set brightness (0-100%)
acersense rgb --brightness 80

# Toggle 30-second backlight auto-sleep
acersense rgb --timeout on
acersense rgb --timeout off
```

#### Live Interactive Monitor
```bash
acersense monitor
```

---

## 🔬 Reverse Engineering Documentation

Detailed disassembly notes, ACPI method signatures (`\_SB.WMID.WMAA`), bitmasks, and WMI GUID specifications extracted from Windows binaries are documented in:

📖 **[docs/REVERSE_ENGINEERING.md](docs/REVERSE_ENGINEERING.md)**

---

## 🛠️ Uninstallation

To completely remove AcerSense Linux from your system:

```bash
cd acersense-linux
sudo ./uninstall.sh
```

---

## 📄 License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**. See the [LICENSE](LICENSE) file for details.
