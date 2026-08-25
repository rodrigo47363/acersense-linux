#!/usr/bin/env bash
# AcerSense Linux - Uninstaller Script

set -e

if [ "$EUID" -ne 0 ]; then
    exec sudo bash "$0" "$@"
fi

echo "Uninstalling AcerSense Linux..."

systemctl stop acersensed.service 2>/dev/null || true
systemctl disable acersensed.service 2>/dev/null || true
rm -f /etc/systemd/system/acersensed.service
systemctl daemon-reload

rm -f /usr/local/bin/acersense
rm -f /usr/local/bin/acersense-gui
rm -f /usr/local/bin/acersense-daemon
rm -rf /usr/local/share/acersense
rm -f /etc/udev/rules.d/99-acersense.rules
rm -f /usr/share/applications/acersense.desktop

udevadm control --reload-rules && udevadm trigger || true

echo "AcerSense Linux has been completely removed."
