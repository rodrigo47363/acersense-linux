# AcerSense Pro Linux (v2.0.0 Rust Edition)

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-red.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust 2021](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![GUI: egui / eframe](https://img.shields.io/badge/GUI-egui%20%7C%2060%2B%20FPS-00F0FF.svg)](https://github.com/emilk/egui)
[![Hardware: Acer Nitro & Predator](https://img.shields.io/badge/Hardware-Acer%20Nitro%20%7C%20Predator%20%7C%20Aspire-orange.svg)]()
[![Platform: Linux](https://img.shields.io/badge/Platform-Parrot%20%7C%20Debian%20%7C%20Arch%20%7C%20Fedora%20%7C%20Ubuntu-brightgreen.svg)]()
[![Performance: Zero-Overhead Native](https://img.shields.io/badge/Performance-Zero--Overhead%20Native-blueviolet.svg)]()

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

---

## ⚡ Key Highlights & Breakthrough Capabilities

### 1. 🏎️ Dual Fans Peak Hardware Synchronization (5400 / 6120 RPM)
* **Byte-Scale PWM Calibration:** Solves the 8-bit quantization issue by addressing Embedded Controller (EC) registers `0x14` and `0x24` with native 8-bit resolution ($0-255$).
* **Physical Hardware Limits:** CPU Fan locks at **5400 RPM** and GPU Fan (Numpad) reaches **6120 RPM** in Turbo Max burst.
* **Full Multi-Channel Broadcast:** Concurrently orchestrates Method 7 channels `0x00` (CPU), `0x08` (GPU/Numpad), and `0x09` (Dual Sync).

### 2. 🦀 High-Performance Native Rust Core (`acersense`)
* **Zero Overhead & Sub-Millisecond Execution:** Built with modern Rust 2021 edition and compiled with full Link-Time Optimization (`lto = true`, `opt-level = 3`, `strip = true`).
* **Direct ACPI Call Interface:** Interacts with `\_SB.PCI0.WMID.WMBH` via `/proc/acpi/call` without bulky interpreter dependencies.
* **Modular Architecture:** Pure decoupling of CLI parsing (`clap 4.5`), hardware abstractions (`hw::*`), and core fan curves.

### 3. 🎛️ Hardware-Accelerated Vector GUI (`acersense-gui` @ 60+ FPS)
* **Native egui / eframe Engine:** High refresh rate, immediate-mode GUI rendered with OpenGL and X11 backends.
* **Zero UI Latency:** Asynchronous telemetry polling decoupled from the main rendering loop.
* **Physics-Based Gauges:** Dynamic speedometer dials with live RPM calculations and temperature monitoring.
* **4-Zone RGB Lighting Visualizer:** Live interactive preview with preset configurations and custom color picker.

### 4. 🎯 Physical NitroSense Key `[N]` Integration
* Maps keyboard hardware scancode `0xf5` to `KEY_PROG1` (`XF86Launch1`) using custom udev and hwdb rules, launching the AcerSense Pro GUI directly upon pressing the physical **[N]** key.

### 5. 🔋 Battery Health Protection (80% Lithium Care Mode)
* Enforces battery charge limit at **80%** directly in the hardware Embedded Controller, extending battery chemical longevity.

### 6. 🌈 4-Zone RGB Keyboard Lighting & Gaming Tweaks
* **Lighting Presets:** `cyberpunk`, `nitro`, `ice`, `toxic`, `synthwave`, and `white`.
* **Custom Hex Styling:** Assign individual hex colors per zone or broadcast across all 4 zones.
* **Gaming Locks:** Disable Windows key or Touchpad during gaming sessions.

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

# Install binaries into system path
sudo install -m 755 target/release/acersense /usr/local/bin/acersense
sudo install -m 755 target/release/acersense-gui /usr/local/bin/acersense-gui
```

---

## ⌨️ Tactical Commands & Keybindings

| Key / Command | Action |
| :--- | :--- |
| **Physical `[N]` Key** | Launches AcerSense Pro GUI (Identical to Windows NitroSense) |
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
