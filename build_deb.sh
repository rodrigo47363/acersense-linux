#!/usr/bin/env bash
# =========================================================================
# AcerSense Linux - Debian / Parrot / Ubuntu .deb Package Builder
# =========================================================================

set -euo pipefail

PACKAGE_NAME="acersense"
VERSION="2.0.0"
ARCH="amd64"
BUILD_DIR="$(pwd)/build/pkg_deb"
PKG_ROOT="${BUILD_DIR}/${PACKAGE_NAME}_${VERSION}_${ARCH}"
OUTPUT_DEB="$(pwd)/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"

echo "================================================================="
echo " BUILDING ACERSENSE PRO LINUX DEBIAN PACKAGE v2.0.0 (.deb)"
echo "================================================================="

# 0. Build Rust native core
echo "[*] Compiling Rust native core..."
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release

# Clean build directory
rm -rf "$BUILD_DIR"
mkdir -p "${PKG_ROOT}/DEBIAN"
mkdir -p "${PKG_ROOT}/usr/local/bin"
mkdir -p "${PKG_ROOT}/etc/udev/rules.d"
mkdir -p "${PKG_ROOT}/etc/systemd/system"
mkdir -p "${PKG_ROOT}/usr/lib/systemd/system-sleep"
mkdir -p "${PKG_ROOT}/usr/share/applications"
mkdir -p "${PKG_ROOT}/usr/share/icons/hicolor/scalable/apps"

# 1. Copy Native Rust Binaries
cp target/release/acersense "${PKG_ROOT}/usr/local/bin/acersense"
cp target/release/acersense-gui "${PKG_ROOT}/usr/local/bin/acersense-gui"
chmod +x "${PKG_ROOT}"/usr/local/bin/*

# 3. Copy System Integration Files
cp udev/99-acersense.rules "${PKG_ROOT}/etc/udev/rules.d/99-acersense.rules"
cp systemd/acersensed.service "${PKG_ROOT}/etc/systemd/system/acersensed.service"
cp systemd/acersense-sleep.sh "${PKG_ROOT}/usr/lib/systemd/system-sleep/acersense-sleep"
chmod +x "${PKG_ROOT}/usr/lib/systemd/system-sleep/acersense-sleep"
cp acersense.desktop "${PKG_ROOT}/usr/share/applications/acersense.desktop"

# 4. Generate SVG Icon
cat << 'EOF' > "${PKG_ROOT}/usr/share/icons/hicolor/scalable/apps/acersense.svg"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" width="128" height="128">
  <defs>
    <linearGradient id="neonG" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#FF1E27"/>
      <stop offset="100%" stop-color="#00F0FF"/>
    </linearGradient>
  </defs>
  <rect width="128" height="128" rx="28" fill="#080A0F"/>
  <polygon points="64,14 114,46 96,112 32,112 14,46" fill="url(#neonG)" opacity="0.85"/>
  <polygon points="64,28 100,52 86,100 42,100 28,52" fill="#0D111A"/>
  <text x="64" y="82" font-family="monospace" font-size="44" font-weight="bold" fill="#00F0FF" text-anchor="middle">A</text>
</svg>
EOF

# 5. Create DEBIAN/control
cat << EOF > "${PKG_ROOT}/DEBIAN/control"
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: AcerSense Linux Team <contact@acersense-linux.org>
Depends: acpi-call-dkms, lm-sensors
Description: Next-Gen Hardware Control Suite for Acer Nitro and Predator Laptops.
 Complete hardware management suite for Linux providing dual-channel 6130 RPM
 fan control, CoolBoost overdrive, 80% battery health protection, 4-zone RGB
 keyboard lighting customization, and native 60 FPS cyberpunk HUD.
EOF

# 6. Create DEBIAN/postinst
cat << 'EOF' > "${PKG_ROOT}/DEBIAN/postinst"
#!/bin/sh
set -e

# Load acpi_call module
modprobe acpi_call 2>/dev/null || true

# Reload udev rules
udevadm control --reload-rules 2>/dev/null || true
udevadm trigger 2>/dev/null || true

# Reload systemd
systemctl daemon-reload 2>/dev/null || true
systemctl enable acersensed.service 2>/dev/null || true
systemctl restart acersensed.service 2>/dev/null || true

echo "[+] AcerSense Linux package installed and service started."
exit 0
EOF
chmod 755 "${PKG_ROOT}/DEBIAN/postinst"

# 7. Create DEBIAN/prerm
cat << 'EOF' > "${PKG_ROOT}/DEBIAN/prerm"
#!/bin/sh
set -e

systemctl stop acersensed.service 2>/dev/null || true
systemctl disable acersensed.service 2>/dev/null || true
exit 0
EOF
chmod 755 "${PKG_ROOT}/DEBIAN/prerm"

# 8. Build the Debian package
dpkg-deb --build --root-owner-group "$PKG_ROOT" "$OUTPUT_DEB"

echo -e "\n================================================================="
echo " [✔] Package Built: ${OUTPUT_DEB}"
echo " Install with: sudo dpkg -i $(basename "$OUTPUT_DEB")"
echo "================================================================="
