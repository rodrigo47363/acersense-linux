#!/usr/bin/env bash
# ==============================================================================
# AcerSense Pro Linux - Production Installer Script
# Automated hardware support, kernel modules, systemd daemon, and desktop suite
# ==============================================================================

set -euo pipefail

# 0. Drenar y purgar secuencias de escape de teclado (CSI / modifyOtherKeys)
if [ -t 0 ]; then
    stty -echo 2>/dev/null || true
    while read -r -t 0.05 -n 10000 _discard; do :; done 2>/dev/null || true
    stty echo 2>/dev/null || true
fi

CLR_RED="\033[31m"
CLR_GREEN="\033[32m"
CLR_CYAN="\033[36m"
CLR_YELLOW="\033[33m"
CLR_RESET="\033[0m"
CLR_BOLD="\033[1m"

echo -e "${CLR_RED}${CLR_BOLD}"
cat << "BANNER"
    █████╗  ██████╗███████╗██████╗ ███████╗███████╗███╗   ██╗███████╗███████╗
   ██╔══██╗██╔════╝██╔════╝██╔══██╗██╔════╝██╔════╝████╗  ██║██╔════╝██╔════╝
   ███████║██║     █████╗  ██████╔╝███████╗█████╗  ██╔██╗ ██║███████╗█████╗  
   ██╔══██║██║     ██╔══╝  ██╔══██╗╚════██║██╔══╝  ██║╚██╗██║╚════██║██╔══╝  
   ██║  ██║╚██████╗███████╗██║  ██║███████║███████╗██║ ╚████║███████║███████╗
   ╚═╝  ╚═╝ ╚═════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝
BANNER
echo -e "${CLR_RESET}"
echo -e "${CLR_CYAN}Instalando AcerSense Pro v2.0.0 (Rust Edition) en $(uname -s) ($(uname -m))...${CLR_RESET}\n"

if [ "$EUID" -ne 0 ]; then
    echo -e "${CLR_YELLOW}[!] Solicitando permisos de administrador (sudo)...${CLR_RESET}"
    exec sudo bash "$0" "$@"
fi

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 1. Configurar reglas de Hardware tempranas (UDEV & HWDB) para atrapar scancodes
echo -e "${CLR_GREEN}[1/5] Aplicando mapeo de hardware UDEV y HWDB (Tecla física NitroSense)...${CLR_RESET}"
mkdir -p /etc/udev/rules.d /etc/udev/hwdb.d
install -m 644 "$PROJECT_DIR/assets/99-acersense.rules" /etc/udev/rules.d/99-acersense.rules
install -m 644 "$PROJECT_DIR/assets/udev/90-acer-nitrosense.hwdb" /etc/udev/hwdb.d/90-acer-nitrosense.hwdb
install -m 644 "$PROJECT_DIR/assets/udev/99-acer-nitrosense.rules" /etc/udev/rules.d/99-acer-nitrosense.rules

if command -v systemd-hwdb >/dev/null 2>&1; then
    systemd-hwdb update || true
fi
if command -v udevadm >/dev/null 2>&1; then
    udevadm control --reload-rules || true
    udevadm trigger -s input -c change || true
fi
if command -v apt >/dev/null 2>&1; then
    apt-get update -qq || true
    apt-get install -y -qq acpi-call-dkms lm-sensors x11-xserver-utils || true
elif command -v pacman >/dev/null 2>&1; then
    pacman -Sy --noconfirm --needed acpi_call lm_sensors xorg-xrandr || true
elif command -v dnf >/dev/null 2>&1; then
    dnf install -y -q akmod-acpi_call lm_sensors xorg-x11-server-utils || true
fi

# 2. Configurar grupo de sistema y módulo del Kernel (acpi_call)
echo -e "${CLR_GREEN}[2/5] Configurando grupo de sistema acersense, módulo acpi_call y permisos DSDT...${CLR_RESET}"
if ! getent group acersense >/dev/null 2>&1; then
    groupadd -r acersense || true
    echo -e "${CLR_CYAN}[+] Grupo de sistema 'acersense' creado con éxito.${CLR_RESET}"
fi

REAL_USER="${SUDO_USER:-$USER}"
if [ -n "$REAL_USER" ] && [ "$REAL_USER" != "root" ]; then
    usermod -aG acersense "$REAL_USER" || true
    echo -e "${CLR_CYAN}[+] Usuario $REAL_USER agregado al grupo 'acersense'.${CLR_RESET}"
fi

mkdir -p /etc/modules-load.d/ /etc/tmpfiles.d/ /etc/acersense/
echo "acpi_call" > /etc/modules-load.d/acpi_call.conf
echo "acpi_call" > /etc/modules-load.d/acersense.conf
echo "f /proc/acpi/call 0666 root root -" > /etc/tmpfiles.d/acersense.conf
modprobe acpi_call 2>/dev/null || true
if [ -e /proc/acpi/call ]; then
    chmod 666 /proc/acpi/call || true
fi

# 3. Compilar e Instalar binarios nativos en Rust
echo -e "${CLR_GREEN}[3/5] Compilando e instalando ejecutables nativos en Rust en /usr/local/bin/...${CLR_RESET}"
if command -v cargo >/dev/null 2>&1 || [ -f "$HOME/.cargo/bin/cargo" ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
    (cd "$PROJECT_DIR" && cargo build --release)
fi

install -m 755 "$PROJECT_DIR/target/release/acersense" /usr/local/bin/acersense
install -m 755 "$PROJECT_DIR/target/release/acersense-gui" /usr/local/bin/acersense-gui


# 4. Configurar servicio de Sistema (systemd)
echo -e "${CLR_GREEN}[4/5] Habilitando servicio del demonio acersensed.service...${CLR_RESET}"
install -m 644 "$PROJECT_DIR/assets/acersensed.service" /etc/systemd/system/acersensed.service
ln -sf /etc/systemd/system/acersensed.service /etc/systemd/system/acersense.service
systemctl daemon-reload
systemctl enable --now acersensed.service || true

# 5. Lanzador de Escritorio e Iconos
echo -e "${CLR_GREEN}[5/5] Registrando lanzador de escritorio y assets visuales...${CLR_RESET}"
mkdir -p /usr/share/applications /usr/share/icons/hicolor/scalable/apps
install -m 644 "$PROJECT_DIR/assets/acersense-gui.desktop" /usr/share/applications/acersense-gui.desktop
install -m 644 "$PROJECT_DIR/assets/icons/acersense.svg" /usr/share/icons/hicolor/scalable/apps/acersense.svg

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database /usr/share/applications || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor || true
fi

# Restaurar estado del TTY y purgar residuos de teclado
if [ -t 0 ]; then
    stty sane 2>/dev/null || true
    while read -r -t 0.05 -n 10000 _discard; do :; done 2>/dev/null || true
fi

echo -e "\n${CLR_GREEN}${CLR_BOLD}[✔] ¡AcerSense Pro v2.0.0 instalado y activo con éxito!${CLR_RESET}"
echo -e "Comandos disponibles:"
echo -e "  • ${CLR_CYAN}acersense status${CLR_RESET}   (Telemetría completa en consola)"
echo -e "  • ${CLR_CYAN}acersense-gui${CLR_RESET}      (Centro de control gráfico a 60+ FPS)"
echo -e "  • ${CLR_CYAN}acersense fan --mode max${CLR_RESET} (Forzar ventiladores a 5400/6120 RPM)"
echo -e "  • ${CLR_CYAN}acersense profile --set turbo${CLR_RESET} (Perfil de máximo rendimiento)"
