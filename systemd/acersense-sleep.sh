#!/usr/bin/env bash
# =========================================================================
# AcerSense Linux - Systemd Sleep & Resume Hook
# Restores fan curves, power profile, 80% battery limits, and RGB on wakeup.
# Placed in: /usr/lib/systemd/system-sleep/acersense-sleep
# =========================================================================

case "$1/$2" in
    pre/*)
        # Actions before suspend/hibernate (if any)
        ;;
    post/*)
        # System resumed from suspend / hibernation
        # Give EC and ACPI subsystem 1 second to stabilize
        sleep 1
        if command -v acersense-daemon >/dev/null 2>&1; then
            acersense-daemon --resume || true
        elif [ -x /usr/local/bin/acersense-daemon ]; then
            /usr/local/bin/acersense-daemon --resume || true
        fi
        ;;
esac

exit 0
