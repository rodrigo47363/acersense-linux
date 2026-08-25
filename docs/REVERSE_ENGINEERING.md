# Acer Nitro / Predator Windows Software Reverse Engineering Technical Report

This document details the reverse engineering of Acer proprietary Windows software (**NitroSense 3.01**, **Acer Care Center 4.00**, and **Quick Access 3.00**) to reconstruct complete open-source Linux hardware control implementations.

---

## 1. Target Packages & Disassembly Overview

The target suites provided by the vendor consist of Centennial UWP wrappers, MSI installers, and embedded .NET / Native PE32+ binaries:

*   **`Nitro Sense_Acer_3.01.3056_W11x64_A.zip`**:
    *   `NitroSense.exe` (WPF / .NET Framework 4.5+ GUI)
    *   `TsDotNetLib.dll` (Named Pipe IPC and WMI method marshaler)
    *   `FILE_APP_SVC_EXE` / `PSSvc.exe` (Native x86-64 background service communicating with WMI/ACPI)
    *   `Plugs/` (Model-specific hardware INI tables: `Nitro AN515-55`, `AN515-58`, `AN517-54`, `Predator PH315`, etc.)
*   **`Acer Care Center_Acer_4.00.3060_W11x64_A.zip`**:
    *   `CareCenter.exe` (Main .NET UWP/Win32 Hybrid interface)
    *   `BatteryInformation.dll` / `BatteryDevice.dll` (Battery health mode & threshold controller)
    *   `SysPfMgr.dll` & `SysSwMgr.dll` (System and BIOS policy profiles)
*   **`Quick Access_Acer_3.00.3038_W11x64_A.zip`**:
    *   `CoolBoost.dll` (Acer CoolBoost algorithm toggle)
    *   `SystemUsage.dll` (Power plan & system usage profile switch)
    *   `UsbCharge.dll` (Power-off USB charging settings)

---

## 2. ACPI / WMI Architecture & GUID Mapping

Windows communication flows from the UI -> Named Pipe (`PredatorSense_service_namedpipe` / `treadstone_qa_service`) -> Service -> `\\.\root\WMI` / ACPI Methods.

### Hardware WMI GUIDs

| GUID | Description | Linux Sysfs Node |
| :--- | :--- | :--- |
| `7A4DDFE7-5B5D-40B4-8595-4408E0CC7F56` | **Acer Gaming Function V2** (Fan modes, Fan RPMs, RGB lighting, CoolBoost) | `/sys/bus/wmi/devices/7A4DDFE7-5B5D-40B4-8595-4408E0CC7F56-*` |
| `F75F5666-B8B3-4A5D-A91C-7488F62E5637` | **Acer Gaming Function V1** (Legacy Predator Sense) | `/sys/bus/wmi/devices/F75F5666-B8B3-4A5D-A91C-7488F62E5637-*` |
| `79772EC5-04B1-4BFD-843C-61E7F77B6CC9` | **Acer Battery Care & Health** (80% charge threshold limiter) | `/sys/bus/wmi/drivers/acer-wmi-battery/` |
| `61EF69EA-865C-4BC3-A502-A0DEBA0CB531` | **Acer Standard WMI (AMW0)** (BIOS info, radios, display hotkeys) | `/sys/bus/wmi/devices/61EF69EA-865C-4BC3-A502-A0DEBA0CB531-*` |
| `676AA15E-6A47-4D9F-A2CC-1E6D18D14026` | **Acer Hotkey & Sensor Events** | Handled by kernel `acer_wmi` |

---

## 3. Protocol & Bitmask Reverse-Engineered Specifications

### 3.1. Fan Control Subsystem (`SetAcerGamingFanGroupBehavior`)

Invoked via ACPI method `\_SB.WMID.WMAA` with Method ID `0x15` or WMI Class `AcerGamingFunction.SetGamingFanBehavior`:

$$\text{Opcode} = \text{Base} \mid (\text{CPU\_Behavior} \ll 16) \mid (\text{GPU\_Behavior} \ll 22)$$

*   **Auto Fan Mode:**
    *   Formula: `9 | (1 << 16) | (1 << 22)` = `0x410009` ($4,259,849$)
*   **Max Fan Mode:**
    *   Formula: `9 | (2 << 16) | (2 << 22)` = `0x820009` ($8,519,689$)
*   **Custom Fan Mode:**
    *   Formula: `9 | (3 << 16) | (3 << 22)` = `0xC30009` ($12,779,529$)

### 3.2. Individual Custom Fan Speed (`SetAcerGamingFanGroupSpeed`)

Method ID `0x16` with parameter:
*   **CPU Fan Target %:** `0x01 | (Percentage << 8)`
*   **GPU Fan Target %:** `0x04 | (Percentage << 8)`
*   *Range:* $0 \le \text{Percentage} \le 100$.

### 3.3. CoolBoost™ Toggle (`WMISetFunction` ID 7)

Method ID `0x11` with parameter:
$$\text{Opcode} = 7 \mid (\text{Enable} \ll 16)$$
*   **CoolBoost ON:** `7 | (1 << 16)` = `0x10007` ($65,543$)
*   **CoolBoost OFF:** `7 | (0 << 16)` = `0x00007` ($7$)

### 3.4. 4-Zone RGB Keyboard Lighting (`SetAcerGamingLEDGroupColor`)

Method ID `0x0B` with parameter:
$$\text{Opcode} = (\text{Zone} \ \& \ \text{0xFF}) \mid (\text{R} \ll 8) \mid (\text{G} \ll 16) \mid (\text{B} \ll 24)$$
*   $\text{Zone} \in \{1, 2, 3, 4\}$
*   $\text{R}, \text{G}, \text{B} \in [0, 255]$

### 3.5. Battery Health 80% Charge Limit (Acer Care Center)

Controlled through the kernel `acer-wmi-battery` driver (WMI GUID `79772EC5-04B1-4BFD-843C-61E7F77B6CC9`):
*   `echo 1 > /sys/bus/wmi/drivers/acer-wmi-battery/health_mode` $\rightarrow$ Hardware Embedded Controller caps charge at **80%**.
*   `echo 0 > /sys/bus/wmi/drivers/acer-wmi-battery/health_mode` $\rightarrow$ Full 100% standard charge.

### 3.6. Power Profiles & System Usage (`platform_profile`)

Mapped directly to the ACPI platform profile:
*   `echo quiet > /sys/firmware/acpi/platform_profile` (Quiet / Silent)
*   `echo balanced > /sys/firmware/acpi/platform_profile` (Balanced / Default)
*   `echo performance > /sys/firmware/acpi/platform_profile` (Performance / Turbo)

---

## 4. Linux Implementation Architecture

```
                      +-----------------------------+
                      |   AcerSense GUI (Tk/CTk)    |
                      +--------------+--------------+
                                     |
                      +--------------v--------------+
                      |       AcerSense CLI         |
                      +--------------+--------------+
                                     |
                      +--------------v--------------+
                      |   AcerSense Core Engine     |
                      | (Fan, Battery, RGB, Profile)|
                      +--------------+--------------+
                                     |
            +------------------------+------------------------+
            |                                                 |
+-----------v-----------+                         +-----------v-----------+
|      Linux Sysfs      |                         |       ACPI / WMI      |
| - hwmon / coretemp    |                         | - \_SB.WMID.WMAA      |
| - platform_profile    |                         | - acer-wmi-battery    |
| - nvidia-smi telemetry|                         | - 4-Zone RGB opcodes  |
+-----------------------+                         +-----------------------+
```
