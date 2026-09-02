#!/usr/bin/env bash
# ==============================================================================
# AcerSense Pro Linux - Uninstaller Script
# Completely removes binaries, services, udev rules, and kernel configurations
# ==============================================================================

set -euo pipefail

if [ "$EUID" -ne 0 ]; then
    echo "Solicitando permisos de administrador para desinstalar..."
    exec sudo bash "$0" "$@"
fi

echo "Desinstalando AcerSense Pro Linux..."

# 1. Detener y deshabilitar demonio
systemctl stop acersensed.service 2>/dev/null || true
systemctl disable acersensed.service 2>/dev/null || true
rm -f /etc/systemd/system/acersensed.service
systemctl daemon-reload

# 2. Eliminar ejecutables y librerías
rm -f /usr/local/bin/acersense
rm -f /usr/local/bin/acersense-gui
rm -f /usr/local/bin/acersense-daemon
rm -f /usr/local/bin/acersense-osd
rm -rf /usr/local/share/acersense

# 3. Eliminar reglas de hardware
rm -f /etc/udev/rules.d/99-acersense.rules
rm -f /etc/udev/rules.d/99-acer-nitrosense.rules
rm -f /etc/udev/hwdb.d/90-acer-nitrosense.hwdb
rm -f /etc/modules-load.d/acersense.conf
rm -f /etc/tmpfiles.d/acersense.conf

# 4. Eliminar lanzador e icono
rm -f /usr/share/applications/acersense-gui.desktop
rm -f /usr/share/applications/acersense.desktop
rm -f /usr/share/icons/hicolor/scalable/apps/acersense.svg

if command -v systemd-hwdb >/dev/null 2>&1; then
    systemd-hwdb update || true
fi
if command -v udevadm >/dev/null 2>&1; then
    udevadm control --reload-rules || true
    udevadm trigger -s input -c change || true
fi

echo "[✔] AcerSense Pro ha sido completamente desinstalado de su sistema."
