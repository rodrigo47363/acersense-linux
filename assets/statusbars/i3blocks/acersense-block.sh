#!/usr/bin/env bash
# AcerSense i3blocks Blocklet Script

DATA=$(acersense status --json 2>/dev/null)
if [ -z "$DATA" ]; then
    echo "󰚥 N/A"
    exit 0
fi

CPU_TEMP=$(echo "$DATA" | jq -r '.cpu_temp // 0')
FAN_RPM=$(echo "$DATA" | jq -r '.cpu_fan_rpm // 0')
BAT_PCT=$(echo "$DATA" | jq -r '.battery_pct // 0')
PROFILE=$(echo "$DATA" | jq -r '.profile // "balanced"' | tr '[:lower:]' '[:upper:]')

# Full text
echo " ${CPU_TEMP}°C 🌀 ${FAN_RPM} RPM 󰚥 ${PROFILE}"

# Short text
echo "${CPU_TEMP}°C"

# Color output
if [ "$CPU_TEMP" -ge 80 ]; then
    echo "#F85149" # Critical Red
elif [ "$CPU_TEMP" -ge 65 ]; then
    echo "#D29922" # Warning Yellow
else
    echo "#58A6FF" # Normal Accent Blue
fi

# Click handlers
case $BLOCK_BUTTON in
    1) acersense-gui & ;; # Left click
    2) acersense profile --set balanced ;; # Middle click
    3) acersense profile --set turbo ;; # Right click
esac
