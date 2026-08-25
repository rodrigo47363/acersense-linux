#!/usr/bin/env bash
# AcerSense Linux - One-Step Installer Script
# Automates dependency resolution, permissions setup, systemd daemon, and desktop integration.

set -e

CLR_RED="\033[31m"
CLR_GREEN="\033[32m"
CLR_CYAN="\033[36m"
CLR_YELLOW="\033[33m"
CLR_RESET="\033[0m"
CLR_BOLD="\033[1m"

echo -e "${CLR_RED}${CLR_BOLD}"
echo "    █████╗  ██████╗███████╗██████╗ ███████╗███████╗███╗   ██╗███████╗███████╗"
echo "   ██╔══██╗██╔════╝██╔════╝██╔══██╗██╔════╝██╔════╝████╗  ██║██╔════╝██╔════╝"
echo "   ███████║██║     █████╗  ██████╔╝███████╗█████╗  ██╔██╗ ██║███████╗█████╗  "
echo "   ██╔══██║██║     ██╔══╝  ██╔══██╗╚════██║██╔══╝  ██║╚██╗██║╚════██║██╔══╝  "
echo "   ██║  ██║╚██████╗███████╗██║  ██║███████║███████╗██║ ╚████║███████║███████╗"
echo "   ╚═╝  ╚═╝ ╚═════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝"
echo -e "${CLR_RESET}"
echo -e "${CLR_CYAN}Installing AcerSense Linux on $(uname -s) (${ARCH:-$(uname -m)})...${CLR_RESET}\n"

# Check root privileges
if [ "$EUID" -ne 0 ]; then
    echo -e "${CLR_YELLOW}[!] Running with sudo to configure system files and permissions...${CLR_RESET}"
    exec sudo bash "$0" "$@"
fi

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 1. Install Python dependencies
echo -e "${CLR_GREEN}[+] Checking and installing Python dependencies...${CLR_RESET}"
python3 -m pip install psutil customtkinter --break-system-packages 2>/dev/null || python3 -m pip install psutil customtkinter

# 2. Configure kernel modules (acpi_call for fan & RGB control)
echo -e "${CLR_GREEN}[+] Enabling acpi_call kernel module...${CLR_RESET}"
mkdir -p /etc/modules-load.d/
echo "acpi_call" > /etc/modules-load.d/acersense.conf
modprobe acpi_call 2>/dev/null || true
if [ -e /proc/acpi/call ]; then
    chmod 666 /proc/acpi/call
fi

# 3. Install binaries to /usr/local/bin
echo -e "${CLR_GREEN}[+] Installing executable binaries to /usr/local/bin...${CLR_RESET}"
mkdir -p /usr/local/share/acersense
cp -rf "$PROJECT_DIR/acersense" /usr/local/share/acersense/
cp -f "$PROJECT_DIR/bin/acersense" /usr/local/bin/acersense
cp -f "$PROJECT_DIR/bin/acersense-gui" /usr/local/bin/acersense-gui
cp -f "$PROJECT_DIR/bin/acersense-daemon" /usr/local/bin/acersense-daemon

chmod +x /usr/local/bin/acersense
chmod +x /usr/local/bin/acersense-gui
chmod +x /usr/local/bin/acersense-daemon

# 4. Configure udev rules for non-root hardware access
echo -e "${CLR_GREEN}[+] Installing udev rules (/etc/udev/rules.d/99-acersense.rules)...${CLR_RESET}"
cp -f "$PROJECT_DIR/udev/99-acersense.rules" /etc/udev/rules.d/99-acersense.rules
udevadm control --reload-rules && udevadm trigger || true

# 5. Install systemd service
echo -e "${CLR_GREEN}[+] Installing systemd background service (acersensed.service)...${CLR_RESET}"
cp -f "$PROJECT_DIR/systemd/acersensed.service" /etc/systemd/system/acersensed.service
systemctl daemon-reload
systemctl enable acersensed.service --now || true

# 6. Install Desktop Launcher
echo -e "${CLR_GREEN}[+] Installing desktop application launcher...${CLR_RESET}"
cp -f "$PROJECT_DIR/acersense.desktop" /usr/share/applications/acersense.desktop
chmod +x /usr/share/applications/acersense.desktop

echo -e "\n${CLR_GREEN}${CLR_BOLD}[✔] AcerSense Linux installed successfully!${CLR_RESET}"
echo -e "You can now run:"
echo -e "  - ${CLR_CYAN}acersense status${CLR_RESET}   (View thermals, fans, battery & RGB)"
echo -e "  - ${CLR_CYAN}acersense-gui${CLR_RESET}      (Open Dark Gaming GUI Dashboard)"
echo -e "  - ${CLR_CYAN}acersense --help${CLR_RESET}   (Explore all CLI options)"
