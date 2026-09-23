# AcerSense Pro Linux (v2.1.0 Rust Edition)

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

## 🖥️ Core Functional Sections (1080p Fullscreen Showcase)

AcerSense Pro Linux features 5 distinct high-performance operational dashboards rendered at 60+ FPS:

<details open>
<summary><b>🌀 1. Fan Speed Control & Dual Turbines (Click to collapse/expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/sections/01_fan_control.png" width="100%" alt="Fan Speed Control & Dual Turbines">
</p>
Dual hardware speedometers with live tachometer calibration up to 5400 / 6120 RPM, 60s dynamic ventilation response graph, and multi-mode fan control (Auto, Max Turbo, Custom Sliders).
</details>

<details open>
<summary><b>📊 2. Live Telemetry & Hardware Oscilloscope (Click to collapse/expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/sections/02_telemetry_monitoring.png" width="100%" alt="Live Telemetry & Hardware Oscilloscope">
</p>
Real-time hardware oscilloscope plotting Thermals (°C), Workload (%), Power (W), and Turbines (RPM) with Intel Core silicon telemetry, NVIDIA NVML GPU stats, and NVMe SSD thermal headroom.
</details>

<details>
<summary><b>⚡ 3. Operating Scenarios & Power Envelopes (Click to expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/sections/03_power_scenarios.png" width="100%" alt="Operating Scenarios & Power Envelopes">
</p>
Configurable power governance envelopes: Quiet Eco Stealth (PL1: 15W), Balanced Performance (PL1: 45W), Performance Gaming (CoolBoost Active), and Extreme Combat Turbo (PL2: 65W, GPU TGP: 75W).
</details>

<details>
<summary><b>🌈 4. 4-Zone RGB Keyboard Lighting Studio (Click to expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/sections/04_rgb_keyboard.png" width="100%" alt="4-Zone RGB Keyboard Lighting Studio">
</p>
Interactive 4-zone Pulsar keyboard visualizer with hardware zone selectors (WASD, Center-Left, Center-Right, Numpad), tactile color swatches, custom RGB color mixer, and factory presets.
</details>

<details>
<summary><b>⚙️ 5. System Settings, Battery Care 80% & Theme Selector (Click to expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/sections/05_system_settings.png" width="100%" alt="System Settings & Theme Selector">
</p>
Embedded Controller battery health protection (80% lithium ceiling), Acer CoolBoost thermal expansion, LCD 3ms overdrive, Windows Key lockout, and live visual theme switcher.
</details>

---

## 🎨 The 19 Color Themes Suite (1080p Fullscreen Previews)

| Theme | Type | Color Identity | Main Dashboard | Complete Section Previews |
|:---|:---:|:---|:---:|:---|
| **🌿 Confort Humano** | Dark | Slate `#0F1219` • Coral `#F43F5E` • Sky `#38BDF8` | [📸 View](assets/screenshots/01_human_comfort.png) | [Fans](assets/screenshots/by_theme/01_human_comfort/fans.png) • [Monitor](assets/screenshots/by_theme/01_human_comfort/monitoring.png) • [Power](assets/screenshots/by_theme/01_human_comfort/power.png) • [RGB](assets/screenshots/by_theme/01_human_comfort/rgb.png) • [Settings](assets/screenshots/by_theme/01_human_comfort/settings.png) |
| **🌃 Tokyo Night** | Dark | Indigo `#1A1B26` • Tokyo Blue `#7AA2F7` • Coral `#F7768E` | [📸 View](assets/screenshots/02_tokyo_night.png) | [Fans](assets/screenshots/by_theme/02_tokyo_night/fans.png) • [Monitor](assets/screenshots/by_theme/02_tokyo_night/monitoring.png) • [Power](assets/screenshots/by_theme/02_tokyo_night/power.png) • [RGB](assets/screenshots/by_theme/02_tokyo_night/rgb.png) • [Settings](assets/screenshots/by_theme/02_tokyo_night/settings.png) |
| **🏮 Neo Tokyo Cyber** | Dark | Dark Violet `#0F0B1E` • Neon Fuchsia `#FF007C` • Cyan `#00FFFF` | [📸 View](assets/screenshots/03_neo_tokyo.png) | [Fans](assets/screenshots/by_theme/03_neo_tokyo/fans.png) • [Monitor](assets/screenshots/by_theme/03_neo_tokyo/monitoring.png) • [Power](assets/screenshots/by_theme/03_neo_tokyo/power.png) • [RGB](assets/screenshots/by_theme/03_neo_tokyo/rgb.png) • [Settings](assets/screenshots/by_theme/03_neo_tokyo/settings.png) |
| **🔴 Red Audit Offensive** | Dark | Pitch Black `#0A0A0E` • Blood Red `#FF3333` • Amber `#FF9E64` | [📸 View](assets/screenshots/04_red_audit.png) | [Fans](assets/screenshots/by_theme/04_red_audit/fans.png) • [Monitor](assets/screenshots/by_theme/04_red_audit/monitoring.png) • [Power](assets/screenshots/by_theme/04_red_audit/power.png) • [RGB](assets/screenshots/by_theme/04_red_audit/rgb.png) • [Settings](assets/screenshots/by_theme/04_red_audit/settings.png) |
| **🟢 Hacker Matrix** | Dark | Pure Black `#000000` • Phosphor Green CRT `#00FF00` | [📸 View](assets/screenshots/05_hacker_matrix.png) | [Fans](assets/screenshots/by_theme/05_hacker_matrix/fans.png) • [Monitor](assets/screenshots/by_theme/05_hacker_matrix/monitoring.png) • [Power](assets/screenshots/by_theme/05_hacker_matrix/power.png) • [RGB](assets/screenshots/by_theme/05_hacker_matrix/rgb.png) • [Settings](assets/screenshots/by_theme/05_hacker_matrix/settings.png) |
| **🌌 Stellar Void** | Dark | Deep Void `#030303` • Plasma Cyan `#00FFFF` • Violet `#8A2BE2` | [📸 View](assets/screenshots/06_stellar_void.png) | [Fans](assets/screenshots/by_theme/06_stellar_void/fans.png) • [Monitor](assets/screenshots/by_theme/06_stellar_void/monitoring.png) • [Power](assets/screenshots/by_theme/06_stellar_void/power.png) • [RGB](assets/screenshots/by_theme/06_stellar_void/rgb.png) • [Settings](assets/screenshots/by_theme/06_stellar_void/settings.png) |
| **🔮 Aurora Gradient** | Dark | Cosmic Night `#05062D` • Purple `#AF40FF` • Cyan `#00DDEB` | [📸 View](assets/screenshots/07_aurora_gradient.png) | [Fans](assets/screenshots/by_theme/07_aurora_gradient/fans.png) • [Monitor](assets/screenshots/by_theme/07_aurora_gradient/monitoring.png) • [Power](assets/screenshots/by_theme/07_aurora_gradient/power.png) • [RGB](assets/screenshots/by_theme/07_aurora_gradient/rgb.png) • [Settings](assets/screenshots/by_theme/07_aurora_gradient/settings.png) |
| **💜 Kuromi Goth** | Dark | Absolute Black `#000000` • Goth Lavender `#B48EAD` • Pink `#F5C2E7` | [📸 View](assets/screenshots/08_kuromi_goth.png) | [Fans](assets/screenshots/by_theme/08_kuromi_goth/fans.png) • [Monitor](assets/screenshots/by_theme/08_kuromi_goth/monitoring.png) • [Power](assets/screenshots/by_theme/08_kuromi_goth/power.png) • [RGB](assets/screenshots/by_theme/08_kuromi_goth/rgb.png) • [Settings](assets/screenshots/by_theme/08_kuromi_goth/settings.png) |
| **✨ Cinnamoroll Night** | Dark | Midnight `#11152C` • Pastel Sky `#8AADF4` • Cheek Pink `#F5BDE6` | [📸 View](assets/screenshots/09_cinnamoroll_night.png) | [Fans](assets/screenshots/by_theme/09_cinnamoroll_night/fans.png) • [Monitor](assets/screenshots/by_theme/09_cinnamoroll_night/monitoring.png) • [Power](assets/screenshots/by_theme/09_cinnamoroll_night/power.png) • [RGB](assets/screenshots/by_theme/09_cinnamoroll_night/rgb.png) • [Settings](assets/screenshots/by_theme/09_cinnamoroll_night/settings.png) |
| **🌲 Everforest Soft** | Dark | Charcoal `#2B3339` • Sage Green `#A7C080` • Aqua `#7FBBB3` | [📸 View](assets/screenshots/10_everforest_soft.png) | [Fans](assets/screenshots/by_theme/10_everforest_soft/fans.png) • [Monitor](assets/screenshots/by_theme/10_everforest_soft/monitoring.png) • [Power](assets/screenshots/by_theme/10_everforest_soft/power.png) • [RGB](assets/screenshots/by_theme/10_everforest_soft/rgb.png) • [Settings](assets/screenshots/by_theme/10_everforest_soft/settings.png) |
| **🌊 Nórdico Calmo** | Dark | Arctic Deep `#10141B` • Glacial Blue `#60A5FA` • Teal `#2DD4BF` | [📸 View](assets/screenshots/11_nordic_calm.png) | [Fans](assets/screenshots/by_theme/11_nordic_calm/fans.png) • [Monitor](assets/screenshots/by_theme/11_nordic_calm/monitoring.png) • [Power](assets/screenshots/by_theme/11_nordic_calm/power.png) • [RGB](assets/screenshots/by_theme/11_nordic_calm/rgb.png) • [Settings](assets/screenshots/by_theme/11_nordic_calm/settings.png) |
| **🍃 Salvia & Tierra** | Dark | Forest `#111613` • Earth Amber `#F59E0B` • Sage Mint `#34D399` | [📸 View](assets/screenshots/12_earth_sage.png) | [Fans](assets/screenshots/by_theme/12_earth_sage/fans.png) • [Monitor](assets/screenshots/by_theme/12_earth_sage/monitoring.png) • [Power](assets/screenshots/by_theme/12_earth_sage/power.png) • [RGB](assets/screenshots/by_theme/12_earth_sage/rgb.png) • [Settings](assets/screenshots/by_theme/12_earth_sage/settings.png) |
| **⚡ Cyber Nitro** | Dark | Pitch `#0A0D12` • Nitro Crimson `#E51937` • Cyan `#00E5FF` | [📸 View](assets/screenshots/13_cyber_nitro.png) | [Fans](assets/screenshots/by_theme/13_cyber_nitro/fans.png) • [Monitor](assets/screenshots/by_theme/13_cyber_nitro/monitoring.png) • [Power](assets/screenshots/by_theme/13_cyber_nitro/power.png) • [RGB](assets/screenshots/by_theme/13_cyber_nitro/rgb.png) • [Settings](assets/screenshots/by_theme/13_cyber_nitro/settings.png) |
| **🔷 Modern Blue** | Dark | Deep Dark Navy `#1A1B26` • Modern Azure `#7AA2F7` | [📸 View](assets/screenshots/14_modern_blue.png) | [Fans](assets/screenshots/by_theme/14_modern_blue/fans.png) • [Monitor](assets/screenshots/by_theme/14_modern_blue/monitoring.png) • [Power](assets/screenshots/by_theme/14_modern_blue/power.png) • [RGB](assets/screenshots/by_theme/14_modern_blue/rgb.png) • [Settings](assets/screenshots/by_theme/14_modern_blue/settings.png) |
| **👔 Mantec Corporate** | Dark | Charcoal `#1E1E1E` • Copper Orange `#E67E22` • Steel `#3498DB` | [📸 View](assets/screenshots/15_mantec_corporate.png) | [Fans](assets/screenshots/by_theme/15_mantec_corporate/fans.png) • [Monitor](assets/screenshots/by_theme/15_mantec_corporate/monitoring.png) • [Power](assets/screenshots/by_theme/15_mantec_corporate/power.png) • [RGB](assets/screenshots/by_theme/15_mantec_corporate/rgb.png) • [Settings](assets/screenshots/by_theme/15_mantec_corporate/settings.png) |
| **🔺 Material Dark Red** | Dark | Darker `#212121` • Coral Red `#F07178` • Amber `#FFCB6B` | [📸 View](assets/screenshots/16_material_dark_red.png) | [Fans](assets/screenshots/by_theme/16_material_dark_red/fans.png) • [Monitor](assets/screenshots/by_theme/16_material_dark_red/monitoring.png) • [Power](assets/screenshots/by_theme/16_material_dark_red/power.png) • [RGB](assets/screenshots/by_theme/16_material_dark_red/rgb.png) • [Settings](assets/screenshots/by_theme/16_material_dark_red/settings.png) |
| **☁️ Cinnamoroll Cloud** | Light | Ethereal White `#F5F8FC` • Vibrant Sky Blue `#2E8FD9` | [📸 View](assets/screenshots/17_cinnamoroll_cloud.png) | [Fans](assets/screenshots/by_theme/17_cinnamoroll_cloud/fans.png) • [Monitor](assets/screenshots/by_theme/17_cinnamoroll_cloud/monitoring.png) • [Power](assets/screenshots/by_theme/17_cinnamoroll_cloud/power.png) • [RGB](assets/screenshots/by_theme/17_cinnamoroll_cloud/rgb.png) • [Settings](assets/screenshots/by_theme/17_cinnamoroll_cloud/settings.png) |
| **🌸 My Melody Soft** | Light | Soft Cream `#FFF8F0` • Melody Rose `#E85D75` • Apricot `#FFE8D6` | [📸 View](assets/screenshots/18_mymelody_soft.png) | [Fans](assets/screenshots/by_theme/18_mymelody_soft/fans.png) • [Monitor](assets/screenshots/by_theme/18_mymelody_soft/monitoring.png) • [Power](assets/screenshots/by_theme/18_mymelody_soft/power.png) • [RGB](assets/screenshots/by_theme/18_mymelody_soft/rgb.png) • [Settings](assets/screenshots/by_theme/18_mymelody_soft/settings.png) |
| **🍮 Pompompurin Café** | Light | Warm Vanilla `#FFFBF0` • Purin Caramel `#C97A18` • Cocoa `#4A2E21` | [📸 View](assets/screenshots/19_pompompurin_cafe.png) | [Fans](assets/screenshots/by_theme/19_pompompurin_cafe/fans.png) • [Monitor](assets/screenshots/by_theme/19_pompompurin_cafe/monitoring.png) • [Power](assets/screenshots/by_theme/19_pompompurin_cafe/power.png) • [RGB](assets/screenshots/by_theme/19_pompompurin_cafe/rgb.png) • [Settings](assets/screenshots/by_theme/19_pompompurin_cafe/settings.png) |

### 🖼️ Visual Themes Gallery

<details open>
<summary><b>✨ Highlighted Signature Themes (Click to collapse/expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/07_aurora_gradient.png" width="32%" alt="Aurora Gradient">
  <img src="assets/screenshots/02_tokyo_night.png" width="32%" alt="Tokyo Night">
  <img src="assets/screenshots/04_red_audit.png" width="32%" alt="Red Audit Offensive">
</p>
<p align="center">
  <img src="assets/screenshots/05_hacker_matrix.png" width="32%" alt="Hacker Matrix">
  <img src="assets/screenshots/03_neo_tokyo.png" width="32%" alt="Neo Tokyo">
  <img src="assets/screenshots/08_kuromi_goth.png" width="32%" alt="Kuromi Goth">
</p>
</details>

<details>
<summary><b>🎨 Complete 19-Theme Visual Comparison (1080p Fullscreen Previews)</b></summary>
<br>

| Theme | Preview (1080p) | Theme | Preview (1080p) |
| :--- | :---: | :--- | :---: |
| **01. 🌿 Confort Humano** | <img src="assets/screenshots/01_human_comfort.png" width="360" alt="Confort Humano"> | **02. 🌃 Tokyo Night** | <img src="assets/screenshots/02_tokyo_night.png" width="360" alt="Tokyo Night"> |
| **03. 🏮 Neo Tokyo Cyber** | <img src="assets/screenshots/03_neo_tokyo.png" width="360" alt="Neo Tokyo Cyber"> | **04. 🔴 Red Audit Offensive** | <img src="assets/screenshots/04_red_audit.png" width="360" alt="Red Audit Offensive"> |
| **05. 🟢 Hacker Matrix** | <img src="assets/screenshots/05_hacker_matrix.png" width="360" alt="Hacker Matrix"> | **06. 🌌 Stellar Void** | <img src="assets/screenshots/06_stellar_void.png" width="360" alt="Stellar Void"> |
| **07. 🔮 Aurora Gradient** | <img src="assets/screenshots/07_aurora_gradient.png" width="360" alt="Aurora Gradient"> | **08. 💜 Kuromi Goth** | <img src="assets/screenshots/08_kuromi_goth.png" width="360" alt="Kuromi Goth"> |
| **09. ✨ Cinnamoroll Night** | <img src="assets/screenshots/09_cinnamoroll_night.png" width="360" alt="Cinnamoroll Night"> | **10. 🌲 Everforest Soft** | <img src="assets/screenshots/10_everforest_soft.png" width="360" alt="Everforest Soft"> |
| **11. 🌊 Nórdico Calmo** | <img src="assets/screenshots/11_nordic_calm.png" width="360" alt="Nórdico Calmo"> | **12. 🍃 Salvia & Tierra** | <img src="assets/screenshots/12_earth_sage.png" width="360" alt="Salvia y Tierra"> |
| **13. ⚡ Cyber Nitro** | <img src="assets/screenshots/13_cyber_nitro.png" width="360" alt="Cyber Nitro"> | **14. 🔷 Modern Blue** | <img src="assets/screenshots/14_modern_blue.png" width="360" alt="Modern Blue"> |
| **15. 👔 Mantec Corporate** | <img src="assets/screenshots/15_mantec_corporate.png" width="360" alt="Mantec Corporate"> | **16. 🔺 Material Dark Red** | <img src="assets/screenshots/16_material_dark_red.png" width="360" alt="Material Dark Red"> |
| **17. ☁️ Cinnamoroll Cloud** | <img src="assets/screenshots/17_cinnamoroll_cloud.png" width="360" alt="Cinnamoroll Cloud"> | **18. 🌸 My Melody Soft** | <img src="assets/screenshots/18_mymelody_soft.png" width="360" alt="My Melody Soft"> |
| **19. 🍮 Pompompurin Café** | <img src="assets/screenshots/19_pompompurin_cafe.png" width="360" alt="Pompompurin Café"> | | |

</details>

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
sudo dpkg -i acersense_2.1.0_amd64.deb
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
