# AcerSense Pro Linux (v2.0.0 Rust Edition)

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-red.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![CI Pipeline](https://github.com/rodrigo47363/acersense-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/rodrigo47363/acersense-linux/actions/workflows/ci.yml)
[![Language: Rust 2021](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![GUI: egui / eframe](https://img.shields.io/badge/GUI-egui%20%7C%2060%2B%20FPS-00F0FF.svg)](https://github.com/emilk/egui)
[![Themes: 19 Custom Palettes](https://img.shields.io/badge/Themes-19%20Custom%20Palettes-af40ff.svg)](#-curated-19-theme-visual-ergonomics-suite)
[![Hardware: Acer Nitro & Predator](https://img.shields.io/badge/Hardware-Acer%20Nitro%20%7C%20Predator%20%7C%20Aspire-orange.svg)]()
[![Platform: Linux](https://img.shields.io/badge/Platform-Parrot%20%7C%20Debian%20%7C%20Arch%20%7C%20Fedora%20%7C%20Ubuntu-brightgreen.svg)]()
[![Code Quality: Clippy 0 Warnings](https://img.shields.io/badge/Clippy-0%20Warnings-success.svg)]()

**AcerSense Pro Linux** is an ultra-high performance, low-latency open-source hardware control suite and native Rust engine designed to replace proprietary Windows utilities (**Acer NitroSense**, **Acer PredatorSense**, and **Acer Care Center**) on Acer Nitro, Predator, and Aspire Gaming laptops.

```
    █████╗  ██████╗███████╗██████╗ ███████╗███████╗███╗   ██╗███████╗███████╗
   ██╔══██╗██╔════╝██╔════╝██╔══██╗██╔════╝██╔════╝████╗  ██║██╔════╝██╔════╝
   ███████║██║     █████╗  ██████╔╝███████╗█████╗  ██╔██╗ ██║███████╗█████╗  
   ██╔══██║██║     ██╔══╝  ██╔══██╗╚════██║██╔══╝  ██║╚██╗██║╚════██║██╔══╝  
   ██║  ██║╚██████╗███████╗██║  ██║███████║███████╗██║ ╚████║███████║███████╗
   ╚═╝  ╚═╝ ╚═════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝
   Enterprise-Grade Linux Control Suite for Acer Nitro & Predator Laptops
```

<p align="center">
  <img src="assets/screenshots/acersense_gui_dashboard.png" alt="AcerSense Pro Linux GUI Dashboard" width="100%">
</p>

---

## ⚡ Key Highlights & Breakthrough Capabilities

### 1. 🏎️ Dual Fans Peak Hardware Synchronization (5400 / 6120 RPM)
* **Byte-Scale PWM Calibration:** Solves the 8-bit quantization issue by addressing Embedded Controller (EC) registers `0x14` and `0x24` with native 8-bit resolution ($0-255$).
* **Physical Hardware Limits:** CPU Fan locks at **5400 RPM** and GPU Fan (Numpad) reaches **6120 RPM** in Turbo Max burst.
* **Full Multi-Channel Broadcast:** Concurrently orchestrates Method 7 channels `0x00` (CPU), `0x08` (GPU/Numpad), and `0x09` (Dual Sync).

### 2. 🎨 Curated 19-Theme Visual Ergonomics Suite
* **Zero Retinal Fatigue:** Engineered adhering to **WCAG AAA** contrast standards ($>10:1$ luminance ratio), preventing glare burn and eye strain during long-night coding or pentesting sessions.
* **Bimodal Engine (Dark & Light Modes):** Dynamically toggles between `Visuals::dark()` and `Visuals::light()` with contrast inversion for daylight visibility.
* **Live Hotkey Switching `[T]`:** Instant circular cycling across all 19 themes with zero UI reload or latency.
* **Persistent Dotfile Aesthetic:** Includes personal dotfile palettes (**Tokyo Night**, **Neo Tokyo**, **Red Audit Offensive**, **Hacker Matrix**, **Aurora Gradient**, **Kuromi Goth**, **Everforest**, etc.).

### 3. 📈 Real-Time Telemetry & Hardware Graphs
* **Multi-Mode History Charts:** Toggle live graphs between **Thermals** (°C), **Workload** (%), **Power Envelopes** (Watts), and **Turbines** (RPM).
* **High-Precision Sensor Polling:** Reads Intel Core / AMD Ryzen Package temps, NVIDIA NVML GPU temperatures, and NVMe SSD controller health.

### 4. 🦀 High-Performance Native Rust Core (`acersense`)
* **Zero Overhead & Sub-Millisecond Execution:** Built with modern Rust 2021 edition and compiled with full Link-Time Optimization (`lto = true`, `opt-level = 3`, `strip = true`).
* **Direct ACPI Call Interface:** Interacts with `\_SB.PCI0.WMID.WMBH` via `/proc/acpi/call` without bulky interpreter dependencies.
* **Modular Architecture:** Pure decoupling of CLI parsing (`clap 4.5`), hardware abstractions (`hw::*`), and core fan curves.

### 5. 🎛️ Hardware-Accelerated Vector GUI (`acersense-gui` @ 60+ FPS)
* **Native egui / eframe Engine:** High refresh rate, immediate-mode GUI rendered with OpenGL and X11 backends.
* **Zero UI Latency:** Asynchronous telemetry polling decoupled from the main rendering loop via lock-free channels.
* **Physics-Based Gauges:** Dynamic speedometer dials with live RPM calculations and temperature monitoring.
* **4-Zone RGB Lighting Visualizer:** Live interactive preview with preset configurations and custom color picker.

### 6. 🎯 Physical NitroSense Key `[N]` Integration
* Maps keyboard hardware scancode `0xf5` to `KEY_PROG1` (`XF86Launch1`) using custom udev and hwdb rules, launching the AcerSense Pro GUI directly upon pressing the physical **[N]** key.

### 7. 🔋 Battery Health Protection (80% Lithium Care Mode)
* Enforces battery charge limit at **80%** directly in the hardware Embedded Controller, extending battery chemical longevity.

### 8. 🌈 4-Zone RGB Keyboard Lighting & Gaming Tweaks
* **Lighting Presets:** `cyberpunk`, `nitro`, `ice`, `toxic`, `synthwave`, and `white`.
* **Custom Hex Styling:** Assign individual hex colors per zone or broadcast across all 4 zones.
* **Gaming Locks:** Disable Windows key or Touchpad during gaming sessions.

---

## 🎨 The 19 Color Themes Suite

| Theme | Type | Inspiration / Style | Color Identity |
|:---|:---:|:---|:---|
| **🌿 Confort Humano** | Dark | Ergonomic Human-Centric Master | Slate `#0F1219` • Coral `#F43F5E` • Sky `#38BDF8` |
| **🌃 Tokyo Night** | Dark | `simple-tokyonight.rasi` | Indigo `#1A1B26` • Tokyo Blue `#7AA2F7` • Coral `#F7768E` |
| **🏮 Neo Tokyo Cyber** | Dark | `neo_tokyo.rofi` by Rodrigo47363 | Dark Violet `#0F0B1E` • Neon Fuchsia `#FF007C` • Cyan `#00FFFF` |
| **🔴 Red Audit Offensive** | Dark | `red_audit.rofi` by Rodrigo47363 | Pitch Black `#0A0A0E` • Blood Red `#FF3333` • Amber `#FF9E64` |
| **🟢 Hacker Matrix** | Dark | `hacker_green.rofi` by Rodrigo47363 | Pure Black `#000000` • Phosphor Green CRT `#00FF00` |
| **🌌 Stellar Void** | Dark | `stellar_void.rofi` by Rodrigo47363 | Deep Void `#030303` • Plasma Cyan `#00FFFF` • Violet `#8A2BE2` |
| **🔮 Aurora Gradient** | Dark | Modern CSS Gradient Button | Cosmic Night `#05062D` • Purple `#AF40FF` • Cyan `#00DDEB` |
| **💜 Kuromi Goth** | Dark | `kuromi_goth.rofi` by Rodrigo47363 | Absolute Black `#000000` • Goth Lavender `#B48EAD` • Pink `#F5C2E7` |
| **✨ Cinnamoroll Night** | Dark | `cinnamoroll_night.rofi` by Rodrigo47363 | Midnight `#11152C` • Pastel Sky `#8AADF4` • Cheek Pink `#F5BDE6` |
| **🌲 Everforest Soft** | Dark | `squared-everforest.rasi` | Charcoal `#2B3339` • Sage Green `#A7C080` • Aqua `#7FBBB3` |
| **🌊 Nórdico Calmo** | Dark | `squared-nord.rasi` | Arctic Deep `#10141B` • Glacial Blue `#60A5FA` • Teal `#2DD4BF` |
| **🍃 Salvia & Tierra** | Dark | Organic Earth Palette | Forest `#111613` • Earth Amber `#F59E0B` • Sage Mint `#34D399` |
| **⚡ Cyber Nitro** | Dark | Original Acer Nitro Gaming | Pitch `#0A0D12` • Nitro Crimson `#E51937` • Cyan `#00E5FF` |
| **🔷 Modern Blue** | Dark | `modern_blue.rofi` by Rodrigo47363 | Deep Dark Navy `#1A1B26` • Modern Azure `#7AA2F7` |
| **👔 Mantec Corporate** | Dark | `mantec_corporate.rofi` by Rodrigo47363 | Charcoal `#1E1E1E` • Copper Orange `#E67E22` • Steel `#3498DB` |
| **🔺 Material Dark Red** | Dark | `squared-material-red.rasi` | Darker `#212121` • Coral Red `#F07178` • Amber `#FFCB6B` |
| **☁️ Cinnamoroll Cloud** | Light | `cinnamoroll_cloud.rofi` by Rodrigo47363 | Ethereal White `#F5F8FC` • Vibrant Sky Blue `#2E8FD9` |
| **🌸 My Melody Soft** | Light | `mymelody_soft.rofi` by Rodrigo47363 | Soft Cream `#FFF8F0` • Melody Rose `#E85D75` • Apricot `#FFE8D6` |
| **🍮 Pompompurin Café** | Light | `pompompurin_cafe.rofi` by Rodrigo47363 | Warm Vanilla `#FFFBF0` • Purin Caramel `#C97A18` • Cocoa `#4A2E21` |

---

## 💻 Supported Laptop Hardware

Engineered and verified on Compal motherboard platforms:
* **Acer Nitro 5:** AN515-42, AN515-43, AN515-44, AN515-45, AN515-46, AN515-47, AN515-51, AN515-52, AN515-53, AN515-54, **AN515-55**, AN515-56, AN515-57, AN515-58
* **Acer Nitro 16 & 17:** AN516-51, AN517-41, AN517-42, AN517-51, AN517-52, AN517-54, AN517-55
* **Acer Predator Helios & Triton:** PH315, PH317, PT314, PT315, PT515, PT516
* **Acer Aspire 7 Gaming:** A715 series

---

## 📦 Quick Installation

### Option 1: Debian / Parrot OS / Ubuntu Package (.deb)
```bash
# Install the pre-built debian package
sudo dpkg -i acersense_2.0.0_amd64.deb
sudo apt-get install -f
```

### Option 2: Automated Production Installer
```bash
git clone https://github.com/rodrigo47363/acersense-linux.git
cd acersense-linux
sudo ./install.sh
```

### Option 3: Build from Source (Cargo)
```bash
# Prerequisites: cargo, rustc, acpi-call-dkms, lm-sensors
cargo build --release

# Install binaries into user path
cp target/release/acersense target/release/acersense-gui ~/.local/bin/

# (Optional) Install system-wide service binaries
sudo install -m 755 target/release/acersense /usr/local/bin/acersense
sudo install -m 755 target/release/acersense-gui /usr/local/bin/acersense-gui
```

---

## ⌨️ Tactical Commands & Keybindings

| Key / Command | Action |
| :--- | :--- |
| **Physical `[N]` Key** | Launches AcerSense Pro GUI (Identical to Windows NitroSense) |
| **`[T]` Key (in GUI)** | **Cycle color themes in real time** through all 19 palettes |
| **`Win + N`** | Desktop shortcut for AcerSense GUI |
| **`Win + Shift + N`** | Quick-toggles Turbo Max (6120 RPM) $\leftrightarrow$ Silent Auto |
| **`acersense`** / **`acersense status`** | Displays full system thermals, fans, battery & RGB telemetry |
| **`acersense fan --mode max`** | Forces dual fans to maximum physical speed (5400 / 6120 RPM) |
| **`acersense fan --mode auto`** | Restores automatic BIOS fan curve |
| **`acersense fan --toggle`** | Alternates fan profile between Auto and Max |
| **`acersense fan --coolboost on`** | Enables Acer CoolBoost thermal headroom |
| **`acersense profile --set turbo`** | Sets power profile (`quiet`, `balanced`, `performance`, `turbo`) |
| **`acersense battery --limit-80 on`** | Enforces 80% maximum charge health limit |
| **`acersense rgb --preset cyberpunk`** | Applies neon cyberpunk 4-zone keyboard backlight theme |
| **`acersense gaming --winkey lock`** | Locks Windows/Super key to prevent accidental desktop switching |
| **`acersense --polybar`** | Outputs compact color-coded status line for Polybar / status bars |
| **`acersense --json`** | Emits machine-readable telemetry JSON for custom scripting |

---

## 📊 Status Bars Integration (Polybar / Waybar)

### Polybar (`~/.config/polybar/config.ini`)
```ini
[module/acersense]
type = custom/script
exec = acersense --polybar
interval = 2
click-left = acersense fan --toggle
click-right = acersense-gui
```

---

## 📜 Documentation & Technical Papers

* **[Reverse Engineering Technical Report](docs/REVERSE_ENGINEERING.md)**: Deep dive into InsydeH2O UEFI BIOS disassembly, SMM Ring -2 WSMI interrupts, and Windows NitroSense binaries.
* **[Thermals & Fan Control Troubleshooting](docs/TROUBLESHOOTING_THERMALS_AND_FAN_CONTROL.md)**: Embedded Controller register mappings and 8-bit PWM duty cycle calibration.
* **[Technical Failures & Solutions](docs/TECHNICAL_FAILURES_AND_SOLUTIONS.md)**: Comprehensive guide on resolving ACPI method conflicts and udev keyboard scancodes.
* **[Engineering Audit Log](docs/ENGINEERING_AUDIT_LOG.md)**: Chronological hardware benchmarks and architecture milestones.

---

## ⚖️ License

Distributed under the **GNU General Public License v3.0 (GPL-3.0)**.
