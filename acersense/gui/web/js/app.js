// AcerSense Next-Gen Cyberpunk Frontend Controller

const API_BASE = "http://127.0.0.1:18888";
const GAUGE_CIRCUMFERENCE = 440; // 2 * PI * 70

let currentData = null;
let activeTab = "dashboard";

// Initialize UI
document.addEventListener("DOMContentLoaded", () => {
  initTabs();
  initColorPickers();
  initSliders();
  startTelemetryLoop();
});

// Tab Switching
function initTabs() {
  const navBtns = document.querySelectorAll(".nav-btn");
  const tabContents = document.querySelectorAll(".tab-content");

  navBtns.forEach(btn => {
    btn.addEventListener("click", () => {
      const tabId = btn.getAttribute("data-tab");
      activeTab = tabId;

      navBtns.forEach(b => b.classList.remove("active"));
      tabContents.forEach(t => t.classList.remove("active"));

      btn.classList.add("active");
      const target = document.getElementById(`tab-${tabId}`);
      if (target) target.classList.add("active");
    });
  });
}

// Telemetry Polling Engine
function startTelemetryLoop() {
  fetchTelemetry();
  setInterval(fetchTelemetry, 450);
}

async function fetchTelemetry() {
  try {
    const res = await fetch(`${API_BASE}/api/status`);
    if (!res.ok) return;
    const data = await res.json();
    currentData = data;
    renderTelemetry(data);
  } catch (err) {
    console.debug("Telemetry polling error:", err);
  }
}

// Render Telemetry onto Cyberpunk Gauges and Turbines
function renderTelemetry(data) {
  // 1. Header Badges
  const modelEl = document.getElementById("header-model");
  const profileEl = document.getElementById("header-profile");
  if (modelEl) modelEl.textContent = `💻 ${data.hardware.vendor} ${data.hardware.model} (BIOS ${data.hardware.bios})`;
  if (profileEl) profileEl.textContent = `${data.power_profile}`;

  // 2. CPU Dial & Telemetry
  const cpuTemp = data.cpu.temperature || 0;
  const cpuValEl = document.getElementById("cpu-gauge-val");
  const cpuGaugeFill = document.getElementById("cpu-gauge-fill");
  const cpuUsageEl = document.getElementById("cpu-usage-val");
  const cpuFreqEl = document.getElementById("cpu-freq-val");

  if (cpuValEl) cpuValEl.textContent = `${Math.round(cpuTemp)}°`;
  if (cpuUsageEl) cpuUsageEl.textContent = `${data.cpu.usage_percent}%`;
  if (cpuFreqEl) cpuFreqEl.textContent = `${data.cpu.frequency_mhz} MHz`;

  if (cpuGaugeFill) {
    const pct = Math.min(100, Math.max(0, (cpuTemp / 100) * 100));
    const offset = GAUGE_CIRCUMFERENCE - (pct / 100) * GAUGE_CIRCUMFERENCE;
    cpuGaugeFill.style.strokeDashoffset = offset;
    
    // Dynamic color temperature shift
    if (cpuTemp > 85) cpuGaugeFill.style.stroke = "#ff1e27";
    else if (cpuTemp > 70) cpuGaugeFill.style.stroke = "#ffaa00";
    else cpuGaugeFill.style.stroke = "#00f0ff";
  }

  // 3. GPU Dial & Telemetry
  const gpuTemp = data.gpu.temperature || 0;
  const gpuValEl = document.getElementById("gpu-gauge-val");
  const gpuGaugeFill = document.getElementById("gpu-gauge-fill");
  const gpuUsageEl = document.getElementById("gpu-usage-val");
  const gpuVramEl = document.getElementById("gpu-vram-val");

  if (gpuValEl) {
    if (data.gpu.active) {
      gpuValEl.textContent = `${Math.round(gpuTemp)}°`;
    } else {
      gpuValEl.textContent = "IDLE";
      gpuValEl.style.fontSize = "22px";
    }
  }
  if (gpuUsageEl) gpuUsageEl.textContent = data.gpu.active ? `${data.gpu.usage_percent}%` : "Suspended";
  if (gpuVramEl) gpuVramEl.textContent = data.gpu.active ? `${data.gpu.memory_used_mb} / ${data.gpu.memory_total_mb} MB` : "Low Power";

  if (gpuGaugeFill) {
    const pct = data.gpu.active ? Math.min(100, Math.max(0, (gpuTemp / 100) * 100)) : 0;
    const offset = GAUGE_CIRCUMFERENCE - (pct / 100) * GAUGE_CIRCUMFERENCE;
    gpuGaugeFill.style.strokeDashoffset = offset;
  }

  // 4. Twin Animated Fan Turbines
  const cpuRpm = data.fans.cpu_rpm || 0;
  const gpuRpm = data.fans.gpu_rpm || 0;
  const cpuRpmEl = document.getElementById("cpu-rpm-text");
  const gpuRpmEl = document.getElementById("gpu-rpm-text");
  const cpuFanBlade = document.getElementById("cpu-fan-blade");
  const gpuFanBlade = document.getElementById("gpu-fan-blade");

  if (cpuRpmEl) cpuRpmEl.textContent = `${cpuRpm} RPM`;
  if (gpuRpmEl) gpuRpmEl.textContent = `${gpuRpm} RPM`;

  // Physics animation duration calculation: duration = 60 / RPM seconds
  if (cpuFanBlade) {
    if (cpuRpm > 100) {
      const dur = Math.max(0.1, (60 / cpuRpm) * 3);
      cpuFanBlade.style.animationDuration = `${dur}s`;
    } else {
      cpuFanBlade.style.animationDuration = `0s`;
    }
  }
  if (gpuFanBlade) {
    if (gpuRpm > 100) {
      const dur = Math.max(0.1, (60 / gpuRpm) * 3);
      gpuFanBlade.style.animationDuration = `${dur}s`;
    } else {
      gpuFanBlade.style.animationDuration = `0s`;
    }
  }

  // 5. Battery & 80% Limiter
  const batPctEl = document.getElementById("bat-pct-val");
  const batHealthEl = document.getElementById("bat-health-val");
  const batStatusEl = document.getElementById("bat-status-val");
  const batVoltEl = document.getElementById("bat-volt-val");
  const batLimiterSwitch = document.getElementById("switch-battery-80");

  if (batPctEl) batPctEl.textContent = `${data.battery.percentage}%`;
  if (batHealthEl) batHealthEl.textContent = `${data.battery.health_percent}%`;
  if (batStatusEl) batStatusEl.textContent = `${data.battery.status} (${data.battery.power_draw_w} W)`;
  if (batVoltEl) batVoltEl.textContent = `${data.battery.voltage_v} V | ${data.battery.cycle_count} Cycles | ${data.battery.temperature_c}°C`;

  if (batLimiterSwitch && document.activeElement !== batLimiterSwitch) {
    batLimiterSwitch.checked = Boolean(data.battery.health_mode_80_limit);
  }

  // 6. Fan Controls Sync
  const cbSwitch = document.getElementById("switch-coolboost");
  if (cbSwitch && document.activeElement !== cbSwitch) {
    cbSwitch.checked = Boolean(data.fans.coolboost);
  }

  // Sync mode cards
  const modeCards = document.querySelectorAll(".mode-card-btn");
  modeCards.forEach(c => {
    if (c.getAttribute("data-mode") === data.fans.mode) {
      c.classList.add("active");
    } else {
      c.classList.remove("active");
    }
  });

  // 7. RGB Keyboard Sync
  syncKeyboardColors(data.rgb);
}

// 4-Zone RGB Keyboard Color Synchronization
function syncKeyboardColors(rgbData) {
  if (!rgbData || !rgbData.zones) return;
  for (let i = 1; i <= 4; i++) {
    const zoneKey = `zone_${i}`;
    const hex = rgbData.zones[zoneKey]?.hex || "#FF0000";
    const zoneBlock = document.getElementById(`zone-block-${i}`);
    const hexText = document.getElementById(`zone-hex-${i}`);
    const colorInput = document.getElementById(`zone-input-${i}`);

    if (zoneBlock) {
      zoneBlock.style.borderColor = hex;
      zoneBlock.style.boxShadow = `0 0 15px ${hex}66`;
      zoneBlock.style.background = `linear-gradient(180deg, ${hex}22 0%, #151821 100%)`;
    }
    if (hexText) hexText.textContent = hex;
    if (colorInput && document.activeElement !== colorInput) {
      colorInput.value = hex;
    }
  }
}

// Interactive Color Pickers
function initColorPickers() {
  for (let i = 1; i <= 4; i++) {
    const input = document.getElementById(`zone-input-${i}`);
    if (input) {
      input.addEventListener("change", (e) => {
        const hex = e.target.value.toUpperCase();
        setZoneColor(i, hex);
      });
    }
  }

  // Effect buttons
  const effectBtns = document.querySelectorAll(".effect-btn");
  effectBtns.forEach(btn => {
    btn.addEventListener("click", () => {
      effectBtns.forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      const effect = btn.getAttribute("data-effect");
      setRgbEffect(effect);
    });
  });
}

// Sliders Initialization
function initSliders() {
  const cpuSlider = document.getElementById("slider-cpu-fan");
  const gpuSlider = document.getElementById("slider-gpu-fan");
  const rgbSlider = document.getElementById("slider-rgb-brightness");

  if (cpuSlider) {
    cpuSlider.addEventListener("input", (e) => {
      document.getElementById("cpu-fan-pct-text").textContent = `${e.target.value}%`;
    });
    cpuSlider.addEventListener("change", (e) => {
      sendFanSpeed(parseInt(e.target.value), parseInt(gpuSlider.value));
    });
  }

  if (gpuSlider) {
    gpuSlider.addEventListener("input", (e) => {
      document.getElementById("gpu-fan-pct-text").textContent = `${e.target.value}%`;
    });
    gpuSlider.addEventListener("change", (e) => {
      sendFanSpeed(parseInt(cpuSlider.value), parseInt(e.target.value));
    });
  }

  if (rgbSlider) {
    rgbSlider.addEventListener("change", (e) => {
      setRgbBrightness(parseInt(e.target.value));
    });
  }
}

// API Dispatchers
async function setFanMode(mode) {
  try {
    await fetch(`${API_BASE}/api/fan`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ mode: mode })
    });
    fetchTelemetry();
  } catch (err) {
    console.error("Error setting fan mode:", err);
  }
}

async function toggleCoolBoost(cb) {
  try {
    await fetch(`${API_BASE}/api/fan`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ coolboost: cb.checked })
    });
    fetchTelemetry();
  } catch (err) {
    console.error("Error toggling CoolBoost:", err);
  }
}

async function sendFanSpeed(cpuPct, gpuPct) {
  try {
    await fetch(`${API_BASE}/api/fan`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ mode: "custom", cpu_pct: cpuPct, gpu_pct: gpuPct })
    });
  } catch (err) {
    console.error("Error setting fan speed:", err);
  }
}

async function toggleBatteryLimit(cb) {
  try {
    await fetch(`${API_BASE}/api/battery`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ limit_80: cb.checked })
    });
    fetchTelemetry();
  } catch (err) {
    console.error("Error toggling battery limit:", err);
  }
}

async function setZoneColor(zoneIdx, hex) {
  try {
    await fetch(`${API_BASE}/api/rgb`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ zone: zoneIdx, color: hex })
    });
  } catch (err) {
    console.error("Error setting zone color:", err);
  }
}

async function setRgbEffect(effect) {
  try {
    await fetch(`${API_BASE}/api/rgb`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ effect: effect })
    });
  } catch (err) {
    console.error("Error setting RGB effect:", err);
  }
}

async function setRgbBrightness(val) {
  try {
    await fetch(`${API_BASE}/api/rgb`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ brightness: val })
    });
  } catch (err) {
    console.error("Error setting brightness:", err);
  }
}

async function setPowerProfile(profile) {
  try {
    await fetch(`${API_BASE}/api/profile`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ profile: profile })
    });
    fetchTelemetry();
  } catch (err) {
    console.error("Error applying profile:", err);
  }
}
