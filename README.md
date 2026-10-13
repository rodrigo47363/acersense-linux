# AcerSense Pro Linux (v2.1.0 Rust Edition)

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-red.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![CI Pipeline](https://github.com/rodrigo47363/acersense-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/rodrigo47363/acersense-linux/actions/workflows/ci.yml)
[![Language: Rust 2021](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![GUI: egui / eframe](https://img.shields.io/badge/GUI-egui%20%7C%2060%2B%20FPS-00F0FF.svg)](https://github.com/emilk/egui)
[![Themes: 25 Custom Palettes](https://img.shields.io/badge/Themes-25%20Custom%20Palettes-af40ff.svg)](#-the-25-color-themes-suite-visual-ergonomics-matrix)
[![i18n: 10 Locales](https://img.shields.io/badge/i18n-10%20Locales-00DDEB.svg)](#6--native-multilingual-engine-10-locales)
[![Hardware: Acer Nitro & Predator](https://img.shields.io/badge/Hardware-Acer%20Nitro%20%7C%20Predator%20%7C%20Aspire-orange.svg)](#-supported-laptop-hardware)
[![Platform: Linux](https://img.shields.io/badge/Platform-Parrot%20%7C%20Debian%20%7C%20Arch%20%7C%20Fedora%20%7C%20Ubuntu-brightgreen.svg)](#-quick-installation--deployment)
[![Code Quality: Clippy 0 Warnings](https://img.shields.io/badge/Clippy-0%20Warnings-success.svg)](#4--high-performance-native-rust-core-acersense)
[![Direct ACPI / SMM: Ring -2 Interconnect](https://img.shields.io/badge/Bus-ACPI%20%7C%20WSMI%20%7C%20EC-blueviolet.svg)](#-system-architecture--hardware-bus-pipeline)

<p align="center">
  <img src="assets/icons/acersense.png" alt="AcerSense Pro Logo" width="128">
</p>

**AcerSense Pro Linux** is an ultra-high performance, low-latency open-source hardware control suite and native Rust engine designed to completely replace proprietary Windows utilities (**Acer NitroSense**, **Acer PredatorSense**, and **Acer Care Center**) on Acer Nitro, Predator, and Aspire Gaming laptops.

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
* **Firmware SMM Closed-Loop Bypass:** Standard Linux drivers are throttled because the InsydeH2O UEFI BIOS executes a System Management Mode (SMM, Ring -2) closed-loop routine that resets GPU registers to ~2400 RPM when idle. AcerSense Pro bypasses this by dispatching WMI Method 14 (`0x0E`) opcode `0x820009` (`SetGamingFanBehavior`) and Method 16 (`0x10`) opcode `0x6404` (`SetGamingFanSpeed` GPU 100%) via WSMI port `0xD0`, unlocking the physical maximum speed inconditionally.
* **Full Multi-Channel Broadcast:** Concurrently orchestrates Method 7 channels `0x00` (CPU), `0x08` (GPU/Numpad), and `0x09` (Dual Sync).

### 2. 🎨 Curated 25-Theme Visual Ergonomics Suite
* **Zero Retinal Fatigue:** Engineered adhering to **WCAG AAA** contrast standards ($>10:1$ luminance ratio), preventing glare burn and eye strain during long-night coding or pentesting sessions.
* **Bimodal Engine (Dark & Light Modes):** Dynamically toggles between dark backgrounds and high-clarity daylight palettes with contrast inversion.
* **Live Hotkey Switching `[T]`:** Instant circular cycling across all **25 themes** with zero UI reload or latency.
* **Persistent Dotfile Aesthetic:** Includes personal dotfile palettes (**Tokyo Night**, **Neo Tokyo**, **Red Audit Offensive**, **Hacker Matrix**, **Aurora Gradient**, **Kuromi Goth**, **Everforest**, **Ado Blue Rose**, **Windows 11 Fluent**, **macOS Spotlight**, **Suika Recon**, etc.).

### 3. 📈 Real-Time Telemetry & Hardware Oscilloscope
* **Multi-Mode History Charts:** Toggle live oscilloscope graphs between **Thermals** (°C), **Workload** (%), **Power Envelopes** (Watts), and **Turbines** (RPM).
* **High-Precision Sensor Polling:** Reads Intel Core / AMD Ryzen Package temps, NVIDIA NVML GPU temperatures, clocks, VRAM & PCIe D3cold power states, and NVMe SSD controller health.

### 4. 🦀 High-Performance Native Rust Core (`acersense`)
* **Zero Overhead & Sub-Millisecond Execution:** Built with modern Rust 2021 edition and compiled with full Link-Time Optimization (`lto = true`, `opt-level = 3`, `strip = true`).
* **Clippy 0 Warnings:** Audited and verified with strict static code analysis for memory safety and zero resource leaks.
* **Direct ACPI Call Interface:** Interacts directly with `\_SB.PCI0.WMID.WMBH` via `/proc/acpi/call` without bulky interpreter dependencies or external runtimes.
* **Modular Architecture:** Pure decoupling of CLI parsing (`clap 4.5`), hardware abstractions (`hw::*`), and core fan curves.

### 5. 🎛️ Hardware-Accelerated Vector GUI (`acersense-gui` @ 60+ FPS)
* **Native egui / eframe Engine:** High refresh rate, immediate-mode GUI rendered with OpenGL and X11 backends.
* **Zero UI Latency:** Asynchronous telemetry polling decoupled from the main rendering loop via lock-free channels.
* **Physics-Based Gauges:** Dynamic speedometer dials with live RPM calculations and temperature monitoring.
* **4-Zone RGB Lighting Visualizer:** Live interactive preview with preset configurations and custom color picker.

### 6. 🌍 Native Multilingual Engine (10 Locales)
* **Zero-Latency Dynamic Switching:** Instant locale translation across all operational dashboards with immediate UI re-rendering and zero restart required (Hotkey `[L]`).
* **10 Global Locales Supported:** Spanish (`es`), English (`en`), German (`de`), French (`fr`), Portuguese (`pt`), Italian (`it`), Russian (`ru`), Chinese Simplified (`zh`), Japanese (`ja`), and Korean (`ko`).
* **CLI Language Management:** Query and change language via `acersense lang [code]` with persistent configuration saved in `~/.config/acersense/config.json`.
* **FreeDesktop Multilingual Metadata:** Fully localized `.desktop` entries with localized names, generic definitions, and regional search tags.

### 7. 🎯 Physical NitroSense Key `[N]` & Modern Launcher Integration
* **Hardware Scancode Mapping:** Maps keyboard scancode `0xf5` (as well as `0x54`, `0xa8`, `0xd5`, `0xec`, `0x85`) to `KEY_PROG1` (`XF86Launch1`) using udev/hwdb rules to launch the GUI directly from the hardware **[N]** key.
* **Rofi & Application Launchers:** Native FreeDesktop XDG standard (`Settings;HardwareSettings;`), single de-duplicated entry, and multilingual fuzzy-search keywords (`acer`, `nitro`, `predator`, `ventiladores`, `fans`, `cooling`, `rgb`, `hardware`, `bateria`).

### 8. 🔋 Battery Health Protection (80% Lithium Care Mode)
* Enforces battery charge limit at **80%** directly in the hardware Embedded Controller, extending battery chemical longevity and preventing thermal swelling under continuous AC power.

### 9. 🌈 4-Zone RGB Keyboard Lighting & Gaming Tweaks
* **Lighting Presets:** `cyberpunk`, `nitro`, `ice`, `toxic`, `synthwave`, and `white`.
* **Custom Hex Styling:** Assign individual hex colors per zone (`--zones`) or broadcast across all 4 zones (`--color`).
* **Tactical Gaming Locks:** Disable Windows key (`--winkey lock`) or Touchpad (`--touchpad lock`) during gaming sessions, plus LCD 3ms overdrive control (`--overdrive on`).

### 10. 🐙 Open Source Transparency & Community Links
* **Direct Repository & Author Access:** Dedicated GitHub community button in the GUI header and interactive project card in Settings.
* **CLI Project Introspection:** `acersense about` outputs official repository links, developer profile ([@rodrigo47363](https://github.com/rodrigo47363)), issue tracker, and releases.

---

## 🏗️ System Architecture & Hardware Bus Pipeline

```mermaid
flowchart TD
    subgraph UI ["User & Desktop Layer"]
        GUI["acersense-gui (egui / eframe @ 60+ FPS)"]
        CLI["acersense CLI (clap v4.5)"]
        BAR["Status Bars (Polybar / Waybar / i3blocks)"]
        HOTKEY["Physical [N] Key (Scancode 0xF5 -> KEY_PROG1)"]
    end

    subgraph ENGINE ["Rust Core Engine"]
        CONF["Config Engine (~/.config/acersense/config.json)"]
        I18N["i18n Engine (10 Locales)"]
        THEMES["Theme Engine (25 Palettes)"]
        TELEM["Telemetry Engine (CPU / GPU / NVMe / Watts)"]
    end

    subgraph KERNEL ["Linux Kernel Space"]
        ACPI_CALL["/proc/acpi/call (acpi_call kernel module)"]
        UDEV["udev / hwdb (99-acersense.rules & 90-acer-nitrosense.hwdb)"]
        SYSFS["sysfs (/sys/class/hwmon & /sys/class/power_supply)"]
        NVML["NVIDIA NVML / Proprietary Driver"]
    end

    subgraph FIRMWARE ["Firmware & System Architecture"]
        DSDT["ACPI DSDT / SSDT (\\_SB.PCI0.WMID.WMBH)"]
        SMM["InsydeH2O SMM / Ring -2 (WSMI Port 0xD0)"]
        EC["Compal Embedded Controller (ITE IT8227E-192)"]
    end

    subgraph HARDWARE ["Target Hardware Components"]
        CPU_FAN["CPU Turbine (5400 RPM Peak)"]
        GPU_FAN["GPU Turbine (6120 RPM Peak)"]
        RGB_KB["4-Zone Pulsar RGB Keyboard"]
        BATTERY["Lithium Battery Charge Controller (80% Ceiling)"]
    end

    GUI --> CONF
    CLI --> CONF
    HOTKEY --> UDEV --> GUI
    BAR --> CLI

    CONF --> KERNEL
    TELEM --> SYSFS
    TELEM --> NVML

    KERNEL --> ACPI_CALL --> DSDT --> SMM --> EC
    EC --> CPU_FAN
    EC --> GPU_FAN
    EC --> RGB_KB
    EC --> BATTERY
```

---

## 🖥️ Core Functional Sections (1080p Fullscreen Showcase)

AcerSense Pro Linux features distinct high-performance operational dashboards rendered at 60+ FPS:

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

<details open>
<summary><b>⚙️ 5. System Settings, 10-Language Selector & Theme Hub (Click to collapse/expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/sections/05_system_settings.png" width="100%" alt="System Settings, 10-Language Selector & Theme Hub">
</p>
Native 10-language internationalization matrix (ES, EN, DE, FR, PT, IT, RU, ZH, JA, KO) with real-time hot-reloading, 25 visual ergonomics themes with contrast indicators, and instant tactile switching.
</details>

<details open>
<summary><b>🐙 6. Open Source Community, GitHub Hub & Battery Care (Click to collapse/expand)</b></summary>
<br>
<p align="center">
  <img src="assets/screenshots/sections/06_community_and_github.png" width="100%" alt="Open Source Community, GitHub Hub & Battery Care">
</p>
Embedded Controller battery health protection (80% lithium ceiling), tactical gaming lockouts (Windows key and Touchpad suppression), LCD 3ms overdrive, hardware diagnostic bus status, and direct one-click GitHub community hub with repository, author profile, issues, and release trackers.
</details>

---

## 🎨 The 25 Color Themes Suite (Visual Ergonomics Matrix)

AcerSense Pro features a curated suite of **25 visual ergonomics themes** engineered to eliminate retinal fatigue, enhance focus, and match personal dotfile setups.

### 🌟 Complete Themes Catalog & Palette Identity

| # | Theme Name | Mode | CLI / Config Alias | Color Identity & Accent Palette | Visual Purpose & Rationale |
|---|:---|:---:|:---|:---|:---|
| **01** | **🌿 Confort Humano** | Dark | `human` / `human_comfort` | Slate `#0F1219` • Coral `#F43F5E` • Sky `#38BDF8` | Warm slate & gentle coral. Zero retinal fatigue for long coding sessions. |
| **02** | **🌃 Tokyo Night** | Dark | `tokyo` / `tokyonight` | Indigo `#1A1B26` • Tokyo Blue `#7AA2F7` • Coral `#F7768E` | Iconic nocturnal urban aesthetic matching Tokyo Night terminal/IDE themes. |
| **03** | **🏮 Neo Tokyo Cyber** | Dark | `neotokyo` / `neo` | Dark Violet `#0F0B1E` • Neon Fuchsia `#FF007C` • Cyan `#00FFFF` | High-contrast synthwave cyberpunk styling. |
| **04** | **🔴 Red Audit Offensive** | Dark | `red_audit` / `red` | Pitch Black `#0A0A0E` • Blood Red `#FF3333` • Amber `#FF9E64` | Pure Red Team & offensive security operational identity. |
| **05** | **🟢 Hacker Matrix** | Dark | `hacker_green` / `matrix` | Pure Black `#000000` • Phosphor Green CRT `#00FF00` | Classic CRT terminal phosphor aesthetics with maximum contrast. |
| **06** | **🌌 Stellar Void** | Dark | `stellar_void` / `space` | Deep Void `#030303` • Plasma Cyan `#00FFFF` • Violet `#8A2BE2` | Deep-space aesthetic with glowing plasma telemetry accents. |
| **07** | **🔮 Aurora Gradient** | Dark | `aurora_gradient` / `aurora` | Cosmic Night `#05062D` • Purple `#AF40FF` • Cyan `#00DDEB` | Cosmic night with radiant neon purple and cyan accents. |
| **08** | **💜 Kuromi Goth** | Dark | `kuromi_goth` / `kuromi` | Absolute Black `#000000` • Goth Lavender `#B48EAD` • Pink `#F5C2E7` | Modern gothic look with muted lavender and soft pink highlights. |
| **09** | **✨ Cinnamoroll Night** | Dark | `cinnamoroll_night` / `cinna_night`| Midnight `#11152C` • Pastel Sky `#8AADF4` • Cheek Pink `#F5BDE6` | Gentle night aesthetic with soft pastel sky and blush tones. |
| **10** | **🌲 Everforest Soft** | Dark | `everforest` / `forest` | Charcoal `#2B3339` • Sage Green `#A7C080` • Aqua `#7FBBB3` | Warm organic charcoal and soothing natural sage harmony. |
| **11** | **🌊 Nórdico Calmo** | Dark | `nordic` | Arctic Deep `#10141B` • Glacial Blue `#60A5FA` • Teal `#2DD4BF` | Minimalist and serene Nordic workstation colorway. |
| **12** | **🍃 Salvia & Tierra** | Dark | `earth` / `earth_sage` | Forest `#111613` • Earth Amber `#F59E0B` • Sage Mint `#34D399` | Forest charcoal, organic sage green, and warm golden amber. |
| **13** | **⚡ Cyber Nitro** | Dark | `cyber` / `cyber_nitro` | Pitch `#0A0D12` • Nitro Crimson `#E51937` • Cyan `#00E5FF` | Original Acer Nitro sporty gaming DNA with electric accents. |
| **14** | **🔷 Modern Blue** | Dark | `modern_blue` / `modern` | Deep Navy `#1A1B26` • Modern Azure `#7AA2F7` | Focused technical readability with crisp modern blue accents. |
| **15** | **👔 Mantec Corporate** | Dark | `mantec_corporate` / `corporate` | Charcoal `#1E1E1E` • Copper Orange `#E67E22` • Steel `#3498DB` | Professional enterprise slate with understated copper flair. |
| **16** | **🔺 Material Dark Red** | Dark | `material_red` / `material` | Darker `#212121` • Coral Red `#F07178` • Amber `#FFCB6B` | Material Design dark gray with refined coral red highlights. |
| **17** | **☁️ Cinnamoroll Cloud** | Light | `cinnamoroll_cloud` / `cloud` | Ethereal White `#F5F8FC` • Vibrant Sky `#2E8FD9` | Ultra-clear daylight theme engineered for sunlit working environments. |
| **18** | **🌸 My Melody Soft** | Light | `mymelody_soft` / `melody` | Soft Cream `#FFF8F0` • Melody Rose `#E85D75` • Apricot `#FFE8D6` | Warm pastel comfort with soft rose accents. |
| **19** | **🍮 Pompompurin Café** | Light | `pompompurin_cafe` / `purin` | Warm Vanilla `#FFFBF0` • Purin Caramel `#C97A18` • Cocoa `#4A2E21` | Cozy coffeehouse vibe with warm vanilla and toasted caramel. |
| **20** | **🌹 Ado Blue Rose** | Dark | `ado_rose` / `ado` / `rose` | Midnight Abyss `#070A12` • Electric Cyan `#00D2FF` • Neon Rose `#FF2A6D` | Inspired by Aoi Bara with electric cyan and neon rose accents. |
| **21** | **🪟 Windows 10 Metro** | Dark | `windows_10` / `win10` / `metro` | Dark Acrylic `#181818` • Metro Gray `#232323` • Metro Blue `#0078D7` | Clean, flat, functional Windows 10 styling. |
| **22** | **🪟 Windows 11 Fluent** | Dark | `windows_11` / `win11` / `fluent` | Sun Valley Mica `#1F1F1F` • Capsule `#2B2B2B` • Fluent Blue `#60CDFF` | Translucent Mica effect with soft rounded capsules. |
| **23** | **🍎 macOS Spotlight Dark** | Dark | `spotlight_dark` / `macos` | Dark Slate `#242424` • Glass Border `#363636` • Cupertino Blue `#0860F2` | Minimalist dark macOS Spotlight look with Cupertino blue. |
| **24** | **🍎 macOS Spotlight Light** | Light | `spotlight_light` / `macos_light` | Pure White `#F5F5F5` • Charcoal `#242424` • Cupertino Blue `#0860F2` | Refined Cupertino light mode with crystal contrast. |
| **25** | **🍉 Suika Recon** | Dark | `suika_melon` / `suika` / `stone` | Rind Green `#52B788` • Quartz Amber `#FFD166` • Pulp Red `#FF5C77` | Dr. Stone tactical organic science and reconnaissance palette. |

---

### 🖼️ Visual Themes Showcase & 1080p Previews

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
| **19. 🍮 Pompompurin Café** | Light | Warm Vanilla `#FFFBF0` • Purin Caramel `#C97A18` • Cocoa `#4A2E21` | [📸 View](assets/screenshots/19_pompompurin_cafe.png) | [Fans](assets/screenshots/by_theme/19_pompompurin_cafe/fans.png) • [Monitor](assets/screenshots/by_theme/19_pompompurin_cafe/monitoring.png) • [Power](assets/screenshots/by_theme/19_pompompurin_cafe/power.png) • [RGB](assets/screenshots/by_theme/19_pompompurin_cafe/rgb.png) • [Settings](assets/screenshots/by_theme/19_pompompurin_cafe/settings.png) |
| **20. 🌹 Ado Blue Rose** | Dark | Midnight `#070A12` • Electric Cyan `#00D2FF` • Neon Rose `#FF2A6D` | [📸 View](assets/screenshots/20_ado_rose.png) | [Fans](assets/screenshots/by_theme/20_ado_rose/fans.png) • [Monitor](assets/screenshots/by_theme/20_ado_rose/monitoring.png) • [Power](assets/screenshots/by_theme/20_ado_rose/power.png) • [RGB](assets/screenshots/by_theme/20_ado_rose/rgb.png) • [Settings](assets/screenshots/by_theme/20_ado_rose/settings.png) |
| **21. 🪟 Windows 10 Metro** | Dark | Dark Acrylic `#181818` • Metro Gray `#232323` • Metro Blue `#0078D7` | [📸 View](assets/screenshots/21_windows_10.png) | [Fans](assets/screenshots/by_theme/21_windows_10/fans.png) • [Monitor](assets/screenshots/by_theme/21_windows_10/monitoring.png) • [Power](assets/screenshots/by_theme/21_windows_10/power.png) • [RGB](assets/screenshots/by_theme/21_windows_10/rgb.png) • [Settings](assets/screenshots/by_theme/21_windows_10/settings.png) |
| **22. 🪟 Windows 11 Fluent** | Dark | Sun Valley Mica `#1F1F1F` • Capsule `#2B2B2B` • Fluent Blue `#60CDFF` | [📸 View](assets/screenshots/22_windows_11.png) | [Fans](assets/screenshots/by_theme/22_windows_11/fans.png) • [Monitor](assets/screenshots/by_theme/22_windows_11/monitoring.png) • [Power](assets/screenshots/by_theme/22_windows_11/power.png) • [RGB](assets/screenshots/by_theme/22_windows_11/rgb.png) • [Settings](assets/screenshots/by_theme/22_windows_11/settings.png) |
| **23. 🍎 macOS Spotlight Dark** | Dark | Dark Slate `#242424` • Glass Border `#363636` • Cupertino Blue `#0860F2` | [📸 View](assets/screenshots/23_spotlight_dark.png) | [Fans](assets/screenshots/by_theme/23_spotlight_dark/fans.png) • [Monitor](assets/screenshots/by_theme/23_spotlight_dark/monitoring.png) • [Power](assets/screenshots/by_theme/23_spotlight_dark/power.png) • [RGB](assets/screenshots/by_theme/23_spotlight_dark/rgb.png) • [Settings](assets/screenshots/by_theme/23_spotlight_dark/settings.png) |
| **24. 🍎 macOS Spotlight Light** | Light | Pure White `#F5F5F5` • Charcoal `#242424` • Cupertino Blue `#0860F2` | [📸 View](assets/screenshots/24_spotlight_light.png) | [Fans](assets/screenshots/by_theme/24_spotlight_light/fans.png) • [Monitor](assets/screenshots/by_theme/24_spotlight_light/monitoring.png) • [Power](assets/screenshots/by_theme/24_spotlight_light/power.png) • [RGB](assets/screenshots/by_theme/24_spotlight_light/rgb.png) • [Settings](assets/screenshots/by_theme/24_spotlight_light/settings.png) |
| **25. 🍉 Suika Recon** | Dark | Rind Green `#52B788` • Quartz Amber `#FFD166` • Pulp Red `#FF5C77` | [📸 View](assets/screenshots/25_suika_melon.png) | [Fans](assets/screenshots/by_theme/25_suika_melon/fans.png) • [Monitor](assets/screenshots/by_theme/25_suika_melon/monitoring.png) • [Power](assets/screenshots/by_theme/25_suika_melon/power.png) • [RGB](assets/screenshots/by_theme/25_suika_melon/rgb.png) • [Settings](assets/screenshots/by_theme/25_suika_melon/settings.png) |

<details open>
<summary><b>✨ Highlighted Signature Themes Gallery (Click to collapse/expand)</b></summary>
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
<p align="center">
  <img src="assets/screenshots/20_ado_rose.png" width="32%" alt="Ado Blue Rose">
  <img src="assets/screenshots/22_windows_11.png" width="32%" alt="Windows 11 Fluent">
  <img src="assets/screenshots/25_suika_melon.png" width="32%" alt="Suika Recon (Dr. Stone)">
</p>
</details>

<details>
<summary><b>🎨 Complete Visual Comparison Grid (Click to expand)</b></summary>
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
| **19. 🍮 Pompompurin Café** | <img src="assets/screenshots/19_pompompurin_cafe.png" width="360" alt="Pompompurin Café"> | **20. 🌹 Ado Blue Rose** | <img src="assets/screenshots/20_ado_rose.png" width="360" alt="Ado Blue Rose"> |
| **21. 🪟 Windows 10 Metro** | <img src="assets/screenshots/21_windows_10.png" width="360" alt="Windows 10 Metro"> | **22. 🪟 Windows 11 Fluent** | <img src="assets/screenshots/22_windows_11.png" width="360" alt="Windows 11 Fluent"> |
| **23. 🍎 macOS Spotlight Dark** | <img src="assets/screenshots/23_spotlight_dark.png" width="360" alt="macOS Spotlight Dark"> | **24. 🍎 macOS Spotlight Light** | <img src="assets/screenshots/24_spotlight_light.png" width="360" alt="macOS Spotlight Light"> |
| **25. 🍉 Suika Recon (Dr. Stone)** | <img src="assets/screenshots/25_suika_melon.png" width="360" alt="Suika Recon"> | **00. 🏎️ AcerSense Main Dashboard** | <img src="assets/screenshots/acersense_gui_dashboard.png" width="360" alt="AcerSense Main Dashboard"> |

</details>

---

## 💻 Supported Laptop Hardware

Engineered and verified on Acer Compal motherboard platforms:
* **Acer Nitro 5:** AN515-42, AN515-43, AN515-44, AN515-45, AN515-46, AN515-47, AN515-51, AN515-52, AN515-53, AN515-54, **AN515-55**, AN515-56, AN515-57, AN515-58
* **Acer Nitro 16 & 17:** AN516-51, AN517-41, AN517-42, AN517-51, AN517-52, AN517-54, AN517-55
* **Acer Predator Helios & Triton:** PH315, PH317, PT314, PT315, PT515, PT516
* **Acer Aspire 7 Gaming:** A715 series

---

## 📦 Quick Installation & Deployment

### Option 1: Debian / Parrot OS / Ubuntu Package (.deb)
The recommended installation method for Debian-based distributions:
```bash
# Install the pre-built Debian package
sudo dpkg -i acersense_2.1.0_amd64.deb
sudo apt-get install -f
```

### Option 2: Automated Production Installer
Automatically configures kernel modules (`acpi_call`), udev rules, system groups, systemd persistence, and compiles release binaries:
```bash
git clone https://github.com/rodrigo47363/acersense-linux.git
cd acersense-linux
sudo ./install.sh
```

### Option 3: Build from Source with Cargo
For Arch Linux, Fedora, Void Linux, or custom installations:

1. **Install Build & Runtime Dependencies:**
   * **Debian / Ubuntu / Parrot:**
     ```bash
     sudo apt-get update
     sudo apt-get install -y cargo rustc acpi-call-dkms lm-sensors libx11-dev libxcursor-dev libxrandr-dev libxi-dev libgl1-mesa-dev
     ```
   * **Arch Linux / Manjaro:**
     ```bash
     sudo pacman -Syu --needed base-devel rust acpi_call lm_sensors libx11 libxcursor libxrandr libxi mesa
     ```
   * **Fedora / RHEL:**
     ```bash
     sudo dnf install -y cargo rust akmod-acpi_call lm_sensors libX11-devel libXcursor-devel libXrandr-devel libXi-devel mesa-libGL-devel
     ```

2. **Compile with Full Optimizations:**
   ```bash
   cargo build --release
   ```

3. **Install Binaries to System Path:**
   ```bash
   sudo install -m 755 target/release/acersense /usr/local/bin/acersense
   sudo install -m 755 target/release/acersense-gui /usr/local/bin/acersense-gui
   ```

4. **Enable acpi_call Kernel Module & Systemd Daemon:**
   ```bash
   echo "acpi_call" | sudo tee /etc/modules-load.d/acpi_call.conf
   sudo modprobe acpi_call
   sudo install -m 644 assets/acersensed.service /etc/systemd/system/acersensed.service
   sudo systemctl daemon-reload
   sudo systemctl enable --now acersensed.service
   ```

### Building Debian Package from Source
```bash
./build_deb.sh
```

### Uninstallation
```bash
sudo ./uninstall.sh
```

---

## ⌨️ Tactical Commands & Keybindings Reference

### 🎮 Global Shortcuts & GUI In-App Hotkeys

| Shortcut | Context | Action |
| :--- | :--- | :--- |
| **Physical `[N]` Key** | Global Hardware | Launches AcerSense Pro GUI (Mapped via scancode `0xF5` $\rightarrow$ `KEY_PROG1`) |
| **`Win + N`** | Desktop / Window Manager | Launches AcerSense Pro GUI |
| **`Win + Shift + N`** | Desktop / Window Manager | Toggles Turbo Max (6120 RPM) $\leftrightarrow$ Silent Auto |
| **`[T]`** | Inside GUI | **Instant circular theme cycling** through all 25 palettes |
| **`[L]`** | Inside GUI | **Instant language cycling** across all 10 supported locales |
| **`[1]` - `[5]`** | Inside GUI | Direct tab jump (`1` Fans, `2` Monitor, `3` Power, `4` RGB, `5` Settings) |
| **`[A]`** | Inside GUI | Sets fans to **Auto mode** (BIOS thermal curve) |
| **`[M]`** | Inside GUI | Sets fans to **Max Turbo mode** (5400 / 6120 RPM) |
| **`[C]`** | Inside GUI | Switches to **Custom sliders mode** |
| **`Ctrl + Q`** | Inside GUI | Exits the GUI application cleanly |

---

### 🖥️ GUI Launch Arguments (`acersense-gui`)

```bash
# Launch GUI in default desktop window (980x680)
acersense-gui

# Launch GUI in true 1080p Fullscreen mode
acersense-gui --fullscreen    # (or -F)

# Launch GUI with window maximized
acersense-gui --maximized

# Launch GUI with a specific theme pre-loaded
acersense-gui --theme tokyo           # (Options: human, tokyo, neotokyo, red_audit, matrix, suika, etc.)

# Launch GUI with a specific locale
acersense-gui --lang es               # (Options: es, en, de, fr, pt, it, ru, zh, ja, ko)

# Launch GUI directly on a specific tab
acersense-gui --tab fans              # Fan control & tachometers
acersense-gui --tab monitoring        # Telemetry oscilloscope (or --monitoring / -m)
acersense-gui --tab power             # Operating power envelopes (or --power / -p)
acersense-gui --tab rgb               # 4-zone keyboard studio (or --rgb / -r)
acersense-gui --tab settings          # System settings & themes (or --settings / -s)
```

---

### ⚙️ CLI Control Engine (`acersense`)

| Command / Flag | Arguments & Options | Technical Description & Practical Output |
| :--- | :--- | :--- |
| **`acersense`** / **`acersense status`** | _None_ | Displays complete system thermals, silicon clock, fan RPM, battery & RGB telemetry |
| **`acersense fan --mode <mode>`** | `auto`, `max`, `custom`, `silent` | Sets fan governor profile (`max` locks hardware at 5400/6120 RPM) |
| **`acersense fan --toggle`** | `-t` | Quick-toggles between Auto BIOS curve and 100% Turbo Max burst |
| **`acersense fan --status`** | `-s` | Inspects live fan speeds, RPM readings, and active cooling state |
| **`acersense fan --coolboost <state>`** | `on`, `off` | Toggles Acer CoolBoost thermal headroom floor via ACPI WMAA |
| **`acersense fan --cpu <pct> --gpu <pct>`** | `0-100` | Applies custom individual PWM duty cycle percentages to CPU and GPU fans |
| **`acersense profile --set <profile>`** | `quiet`, `balanced`, `performance`, `turbo` | Applies power envelope (governor, EPP, turbo boost limits) |
| **`acersense profile --next`** | `-n` | Cycles sequentially to the next platform power profile |
| **`acersense profile --status`** | _None_ | Outputs the currently active power profile |
| **`acersense mode <target>`** | `next`, `toggle`, `max`, `auto`, `quiet`, `balanced`, `performance`, `turbo` | Unified command shorthand for switching power or fan modes |
| **`acersense battery --limit-80 <state>`** | `on`, `off` | Enforces 80% maximum charge ceiling directly in the Compal EC |
| **`acersense battery --status`** | `-s` | Displays battery health, capacity, cycle count, and charging rate |
| **`acersense rgb --preset <preset>`** | `nitro`, `cyberpunk`, `ice`, `toxic`, `synthwave`, `white` | Applies factory curated 4-zone keyboard backlight color scheme |
| **`acersense rgb --color "<hex>"`** | `"#RRGGBB"` (e.g. `"#FF0033"`) | Broadcasts a single custom hex color across all 4 keyboard lighting zones |
| **`acersense rgb --zones <z1> <z2> <z3> <z4>`** | 4 hex codes (e.g. `"#FF0000"` `"#00FF00"` `"#0000FF"` `"#FFFF00"`) | Assigns distinct custom hex colors to Zone 1, 2, 3, and 4 |
| **`acersense gaming --winkey <state>`** | `lock`, `unlock` | Suppresses or enables the Windows/Super key to prevent gaming alt-tabs |
| **`acersense gaming --touchpad <state>`** | `lock`, `unlock` | Disables or enables touchpad input during keyboard heavy gaming |
| **`acersense gaming --overdrive <state>`** | `on`, `off` | Toggles LCD 3ms overdrive panel response acceleration |
| **`acersense lang [code]`** | `es`, `en`, `de`, `fr`, `pt`, `it`, `ru`, `zh`, `ja`, `ko` | Queries or switches persistent UI language configuration |
| **`acersense about`** | _None_ | Displays official repository links, author profile, and community trackers |
| **`acersense restore`** | _None_ | Re-applies saved hardware configuration on system boot or suspend resume |
| **`acersense --polybar`** | _Global flag_ | Emits compact, color-coded status line formatted for Polybar modules |
| **`acersense --json`** | _Global flag_ | Dumps full sensor telemetry as structured JSON for automation scripts |

---

## 📊 Status Bars Integration (Polybar / Waybar / i3blocks)

Pre-built modular configurations are located in [`assets/statusbars/`](assets/statusbars/).

### 1. Polybar (`~/.config/polybar/config.ini`)
```ini
[module/acersense]
type = custom/script
exec = acersense --polybar
interval = 2
format-prefix = "󰚥 "
format-prefix-foreground = #58A6FF
format-background = #161B22
format-foreground = #E6EDF3
format-padding = 1
format-underline = #58A6FF

; Left click opens GUI Control Center
click-left = acersense-gui &

; Right click engages Turbo Max mode
click-right = acersense profile --set turbo

; Middle click restores Balanced mode
click-middle = acersense profile --set balanced
```

### 2. Waybar (`~/.config/waybar/config.jsonc`)
Add `"custom/acersense"` to your `modules-right` or `modules-center`:
```jsonc
"custom/acersense": {
    "format": "{}",
    "return-type": "json",
    "interval": 2,
    "exec": "acersense --json",
    "on-click": "acersense-gui",
    "on-click-right": "acersense fan --toggle",
    "on-click-middle": "acersense profile --set balanced",
    "tooltip": true
}
```

Optional Waybar styling (`~/.config/waybar/style.css`):
```css
#custom-acersense {
    background-color: #161B22;
    color: #58A6FF;
    border: 1px solid #30363D;
    border-radius: 8px;
    padding: 0 12px;
    margin: 4px 6px;
    font-family: monospace;
    font-weight: bold;
}
#custom-acersense.warning { color: #D29922; border-color: #D29922; }
#custom-acersense.critical { color: #F85149; border-color: #F85149; }
```

### 3. i3blocks (`~/.config/i3blocks/config`)
```ini
[acersense]
command=acersense --polybar
interval=2
signal=8
```

---

## 🔧 Hardware Configuration & State Persistence

AcerSense Pro persists configuration in `~/.config/acersense/config.json` (user-specific) with automatic fallback to `/etc/acersense/config.json` (system-wide):

```json
{
  "mode": "auto",
  "profile": "balanced",
  "coolboost": true,
  "cpu_fan_target": 50,
  "gpu_fan_target": 50,
  "sync_fans": true,
  "battery_health_80": false,
  "winkey_locked": false,
  "touchpad_locked": false,
  "theme": "human",
  "language": "es"
}
```

### Automatic Systemd Boot & Suspend Resume Hooks
* **`acersensed.service`**: Systemd unit that executes `acersense restore` on boot to restore fan curves, RGB backlight, and battery charge thresholds.
* **`acersense-sleep`**: Systemd sleep hook located in `/usr/lib/systemd/system-sleep/acersense-sleep` that re-engages the hardware registers 1 second after resuming from suspend/hibernation, ensuring the BIOS doesn't overwrite your preferences.

---

## 🔍 Troubleshooting & Diagnostics

| Symptom | Probable Cause | Technical Fix & Command |
| :--- | :--- | :--- |
| **Error: `/proc/acpi/call` does not exist** | Kernel module `acpi_call` is not loaded | Run `sudo modprobe acpi_call`. To make persistent: `echo "acpi_call" \| sudo tee /etc/modules-load.d/acpi_call.conf`. |
| **Permission denied on `/proc/acpi/call`** | Incorrect permissions on ACPI device | Run `sudo chmod 666 /proc/acpi/call`. Handled automatically by `install.sh` via `/etc/tmpfiles.d/acersense.conf`. |
| **GPU turbine stuck at ~2400 RPM during MAX mode** | SMM closed-loop thermal lock | Ensure `acpi_call` is active. AcerSense Pro unlocks SMM via WMBH Method 14 (`0x820009`) and Method 16 (`0x6404`). Test with `sudo acersense fan --mode max`. |
| **Physical `[N]` key does not launch GUI** | udev/hwdb scancode not indexed | Update hardware database: `sudo systemd-hwdb update && sudo udevadm trigger -s input -c change`. |
| **Discrete GPU telemetry says "PCIe D3cold / 0.0W"** | GPU is in dynamic Linux power saving | Normal behavior! Discrete NVIDIA GPU is powered down to save battery. Telemetry will automatically awaken as soon as a 3D application or CUDA workload starts. |

---

## 📜 Documentation & Technical Papers

* **[Reverse Engineering Technical Report](docs/REVERSE_ENGINEERING.md)**: Deep dive into InsydeH2O UEFI BIOS disassembly, SMM Ring -2 WSMI interrupts, and Windows NitroSense binaries.
* **[Thermals & Fan Control Troubleshooting](docs/TROUBLESHOOTING_THERMALS_AND_FAN_CONTROL.md)**: Embedded Controller register mappings and 8-bit PWM duty cycle calibration.
* **[Technical Failures & Solutions](docs/TECHNICAL_FAILURES_AND_SOLUTIONS.md)**: Comprehensive guide on resolving ACPI method conflicts and udev keyboard scancodes.
* **[Engineering Audit Log](docs/ENGINEERING_AUDIT_LOG.md)**: Chronological hardware benchmarks and architecture milestones.

---

## 🐙 Community & Author

* **Project Repository:** [github.com/rodrigo47363/acersense-linux](https://github.com/rodrigo47363/acersense-linux)
* **Lead Developer:** Rodrigo ([@rodrigo47363](https://github.com/rodrigo47363))
* **Issue Tracker & Bug Reports:** [Issues](https://github.com/rodrigo47363/acersense-linux/issues)
* **Releases & Binary Packages:** [Releases](https://github.com/rodrigo47363/acersense-linux/releases)

---

## ⚖️ License

Distributed under the **GNU General Public License v3.0 (GPL-3.0)**.
