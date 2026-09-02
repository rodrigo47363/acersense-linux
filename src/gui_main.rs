mod cli;
mod core;
mod hw;

use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, Frame, Grid, Margin, Pos2, Rect, RichText, Rounding, ScrollArea, Stroke, Vec2, Visuals};
use core::config::{load_config, save_config, AcerConfig};
use core::fan;
use core::profile;
use hw::battery;
use hw::gaming;
use hw::rgb;
use hw::thermal::{self, TelemetryData};

// Embedded Original Windows NitroSense UI Textures
const ASSET_FAN_BLADE: &[u8] = include_bytes!("../assets/ui/img_fan.png");
const ASSET_LIGHTING_ZONE: &[u8] = include_bytes!("../assets/ui/img_lighting_zone.png");

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    FanControl,
    Monitoring,
    PowerModes,
    KeyboardRgb,
    SystemSettings,
}

#[derive(Debug, Clone)]
pub enum HardwareCommand {
    SetFanMode(String),
    SetCustomFans { cpu_pct: u8, gpu_pct: u8 },
    SetPowerProfile(String),
    SetRgbZone { zone: u8, r: u8, g: u8, b: u8 },
    SetBatteryLimit(bool),
    SetLcdOverdrive(bool),
    SetCoolboost(bool),
}

// Acer Nitro Iconic Color Palette
const COLOR_NITRO_RED: Color32 = Color32::from_rgb(229, 25, 55);       // #E51937
const COLOR_NITRO_RED_GLOW: Color32 = Color32::from_rgb(255, 60, 90);  // #FF3C5A
const COLOR_NITRO_CYAN: Color32 = Color32::from_rgb(0, 229, 255);      // #00E5FF
const COLOR_NITRO_GREEN: Color32 = Color32::from_rgb(0, 230, 118);     // #00E676
const COLOR_NITRO_AMBER: Color32 = Color32::from_rgb(255, 160, 0);     // #FFA000
const COLOR_NITRO_DARK_BG: Color32 = Color32::from_rgb(11, 14, 20);    // #0B0E14
const COLOR_NITRO_PANEL: Color32 = Color32::from_rgb(18, 22, 31);      // #12161F
const COLOR_NITRO_CARD: Color32 = Color32::from_rgb(24, 30, 42);       // #181E2A
const COLOR_NITRO_BORDER: Color32 = Color32::from_rgb(45, 55, 75);     // #2D374B
const COLOR_TEXT_WHITE: Color32 = Color32::from_rgb(240, 244, 248);   // #F0F4F8
const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(140, 150, 168);   // #8C96A8

struct NitroTextures {
    fan_blade: egui::TextureHandle,
    keyboard_zone: egui::TextureHandle,
}

struct AcerSenseApp {
    current_tab: Tab,
    config: AcerConfig,
    
    // Decoupled Background Worker Channels
    cmd_tx: Sender<HardwareCommand>,
    telemetry_rx: Receiver<TelemetryData>,
    telemetry: TelemetryData,

    // Animation physics (60 FPS)
    turbine_cpu_angle: f32,
    turbine_gpu_angle: f32,

    // RGB & Fan sync
    sync_fans: bool,

    // Original Windows Textures
    textures: Option<NitroTextures>,
}

fn load_texture(ctx: &egui::Context, id: &str, bytes: &[u8]) -> egui::TextureHandle {
    let img = image::load_from_memory(bytes).expect("Failed to decode embedded Nitro PNG");
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_flat_samples().as_slice());
    ctx.load_texture(id, color_image, egui::TextureOptions::LINEAR)
}

impl AcerSenseApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(COLOR_TEXT_WHITE);
        visuals.panel_fill = COLOR_NITRO_DARK_BG;
        visuals.window_fill = COLOR_NITRO_DARK_BG;
        visuals.widgets.noninteractive.bg_fill = COLOR_NITRO_PANEL;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, COLOR_NITRO_BORDER);
        visuals.widgets.noninteractive.rounding = Rounding::same(6.0_f32);
        visuals.widgets.inactive.bg_fill = COLOR_NITRO_PANEL;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, COLOR_NITRO_BORDER);
        visuals.widgets.inactive.rounding = Rounding::same(6.0_f32);
        visuals.widgets.hovered.bg_fill = COLOR_NITRO_CARD;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.2_f32, COLOR_NITRO_RED);
        visuals.widgets.hovered.rounding = Rounding::same(6.0_f32);
        visuals.widgets.active.bg_fill = Color32::from_rgb(35, 42, 58);
        visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, COLOR_NITRO_RED);
        visuals.widgets.active.rounding = Rounding::same(6.0_f32);
        cc.egui_ctx.set_visuals(visuals);

        let textures = NitroTextures {
            fan_blade: load_texture(&cc.egui_ctx, "nitro_fan_blade", ASSET_FAN_BLADE),
            keyboard_zone: load_texture(&cc.egui_ctx, "nitro_keyboard_zone", ASSET_LIGHTING_ZONE),
        };

        let (cmd_tx, cmd_rx) = channel::<HardwareCommand>();
        let (telemetry_tx, telemetry_rx) = sync_channel::<TelemetryData>(1);

        // Spawn Dedicated Background Worker
        thread::Builder::new()
            .name("acersense-worker".into())
            .spawn(move || {
                let mut last_poll = Instant::now() - Duration::from_secs(5);
                let mut cached_config = load_config();

                loop {
                    while let Ok(cmd) = cmd_rx.try_recv() {
                        match cmd {
                            HardwareCommand::SetFanMode(mode) => {
                                let _ = fan::set_mode(&mode);
                                cached_config = load_config();
                            }
                            HardwareCommand::SetCustomFans { cpu_pct, gpu_pct } => {
                                let _ = fan::set_custom_speeds(cpu_pct, gpu_pct);
                                cached_config = load_config();
                            }
                            HardwareCommand::SetPowerProfile(prof) => {
                                let _ = profile::set_profile(&prof);
                                cached_config = load_config();
                            }
                            HardwareCommand::SetRgbZone { zone, r, g, b } => {
                                let _ = rgb::set_static_color(zone, r, g, b);
                                cached_config = load_config();
                            }
                            HardwareCommand::SetBatteryLimit(limit) => {
                                let _ = battery::set_battery_80_limit(limit);
                                cached_config.battery_health_80 = limit;
                                let _ = save_config(&cached_config);
                            }
                            HardwareCommand::SetLcdOverdrive(en) => {
                                let _ = gaming::set_lcd_overdrive(en);
                            }
                            HardwareCommand::SetCoolboost(en) => {
                                let _ = fan::set_coolboost_state(en);
                                cached_config = load_config();
                            }
                        }
                    }

                    if last_poll.elapsed() >= Duration::from_millis(1000) {
                        let is_max = cached_config.mode == "max" || cached_config.mode == "turbo";
                        let data = thermal::collect_all_telemetry(is_max);
                        let _ = telemetry_tx.try_send(data);
                        last_poll = Instant::now();
                    }

                    thread::sleep(Duration::from_millis(25));
                }
            })
            .expect("Failed to spawn background worker thread");

        Self {
            current_tab: Tab::FanControl,
            config: load_config(),
            cmd_tx,
            telemetry_rx,
            telemetry: TelemetryData::default(),
            turbine_cpu_angle: 0.0_f32,
            turbine_gpu_angle: 0.0_f32,
            sync_fans: true,
            textures: Some(textures),
        }
    }

    fn draw_nitro_dial(&self, ui: &mut egui::Ui, title: &str, rpm: u32, max_rpm: u32, temp: f32, load: f32, angle: f32, dial_size: f32, accent: Color32) {
        ui.vertical_centered(|ui| {
            // Header with badge
            let temp_color = if temp >= 85.0 { COLOR_NITRO_RED } else if temp >= 75.0 { COLOR_NITRO_AMBER } else { COLOR_TEXT_WHITE };
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 170.0_f32).max(0.0_f32) / 2.0_f32);
                ui.label(RichText::new(title).size(13.5_f32).strong().color(accent));
                ui.label(RichText::new(format!("{:.0}°C", temp)).size(13.5_f32).strong().color(temp_color));
                ui.label(RichText::new(format!("({:.0}%)", load)).size(11.0_f32).color(COLOR_TEXT_MUTED));
            });
            ui.add_space(6.0_f32);

            let (rect, _response) = ui.allocate_exact_size(Vec2::splat(dial_size), egui::Sense::hover());
            let painter = ui.painter_at(rect);
            let center = rect.center();
            let radius = (dial_size / 2.0_f32) - 4.0_f32;

            // 1. Dark Metallic Bezel & Housing
            painter.circle_filled(center, radius, Color32::from_rgb(14, 18, 26));
            painter.circle_stroke(center, radius, Stroke::new(1.8_f32, Color32::from_rgb(38, 46, 62)));

            // 2. Rotating 300x300 AeroBlade Turbine (GPU Mesh Rotation at 60 FPS)
            if let Some(tex) = &self.textures {
                let half_w = radius * 0.88_f32;
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                let rot = |dx: f32, dy: f32| -> Pos2 {
                    Pos2::new(center.x + (dx * cos_a - dy * sin_a), center.y + (dx * sin_a + dy * cos_a))
                };
                
                let mut mesh = egui::Mesh::with_texture(tex.fan_blade.id());
                mesh.add_triangle(0, 1, 2);
                mesh.add_triangle(2, 3, 0);
                mesh.vertices.push(egui::epaint::Vertex { pos: rot(-half_w, -half_w), uv: Pos2::new(0.0, 0.0), color: Color32::WHITE });
                mesh.vertices.push(egui::epaint::Vertex { pos: rot(half_w, -half_w), uv: Pos2::new(1.0, 0.0), color: Color32::WHITE });
                mesh.vertices.push(egui::epaint::Vertex { pos: rot(half_w, half_w), uv: Pos2::new(1.0, 1.0), color: Color32::WHITE });
                mesh.vertices.push(egui::epaint::Vertex { pos: rot(-half_w, half_w), uv: Pos2::new(0.0, 1.0), color: Color32::WHITE });
                painter.add(mesh);
            } else {
                let num_blades = 18;
                for i in 0..num_blades {
                    let blade_angle = angle + (i as f32 * (std::f32::consts::TAU / num_blades as f32));
                    let dir = Vec2::angled(blade_angle);
                    let p1 = center + dir * (radius * 0.35_f32);
                    let p2 = center + dir * (radius * 0.85_f32);
                    painter.line_segment([p1, p2], Stroke::new(2.2_f32, Color32::from_rgba_unmultiplied(220, 230, 245, 150)));
                }
            }

            // 3. Glowing Outer Progress Arc (Radial RPM Meter)
            let rpm_pct = (rpm as f32 / max_rpm as f32).clamp(0.0_f32, 1.0_f32);
            let arc_radius = radius * 0.94_f32;
            let start_ang = -std::f32::consts::FRAC_PI_2;
            let end_ang = start_ang + (rpm_pct * std::f32::consts::TAU);
            let segments = 36;
            for s in 0..segments {
                let a1 = start_ang + (s as f32 / segments as f32) * (rpm_pct * std::f32::consts::TAU);
                let a2 = start_ang + ((s + 1) as f32 / segments as f32) * (rpm_pct * std::f32::consts::TAU);
                if a1 < end_ang {
                    let pt1 = center + Vec2::angled(a1) * arc_radius;
                    let pt2 = center + Vec2::angled(a2.min(end_ang)) * arc_radius;
                    painter.line_segment([pt1, pt2], Stroke::new(3.2_f32, accent));
                }
            }

            // 4. Center Core Hub with Crimson Ring
            painter.circle_filled(center, radius * 0.38_f32, Color32::from_rgb(10, 13, 18));
            painter.circle_stroke(center, radius * 0.38_f32, Stroke::new(1.8_f32, COLOR_NITRO_RED));

            // 5. High-Contrast RPM Text
            painter.text(
                center - Vec2::new(0.0_f32, dial_size * 0.05_f32),
                egui::Align2::CENTER_CENTER,
                format!("{}", rpm),
                egui::FontId::proportional((dial_size * 0.135_f32).clamp(15.0_f32, 22.0_f32)),
                COLOR_TEXT_WHITE,
            );
            painter.text(
                center + Vec2::new(0.0_f32, dial_size * 0.10_f32),
                egui::Align2::CENTER_CENTER,
                "RPM",
                egui::FontId::monospace((dial_size * 0.065_f32).clamp(9.0_f32, 11.5_f32)),
                COLOR_TEXT_MUTED,
            );
        });
    }

    fn nitro_card_frame(&self) -> Frame {
        Frame::none()
            .fill(COLOR_NITRO_CARD)
            .stroke(Stroke::new(1.0_f32, COLOR_NITRO_BORDER))
            .rounding(Rounding::same(8.0_f32))
            .inner_margin(Margin::same(14.0_f32))
    }

    fn render_tactical_telemetry(&self, ui: &mut egui::Ui) {
        Grid::new("nitro_telemetry_grid")
            .num_columns(2)
            .spacing([16.0_f32, 9.0_f32])
            .show(ui, |ui| {
                // CPU Package
                ui.colored_label(COLOR_TEXT_MUTED, "CPU Package:");
                let cpu_c = if self.telemetry.cpu_temp >= 90.0 { COLOR_NITRO_RED } else if self.telemetry.cpu_temp >= 82.0 { COLOR_NITRO_AMBER } else { COLOR_TEXT_WHITE };
                ui.label(RichText::new(format!("{:.1}°C • {} MHz • {:.1}W • Load: {:.1}%", 
                    self.telemetry.cpu_temp, self.telemetry.cpu_clock, self.telemetry.cpu_power, self.telemetry.cpu_load)).color(cpu_c).monospace());
                ui.end_row();

                // Discrete GPU
                ui.colored_label(COLOR_TEXT_MUTED, "Discrete GPU:");
                if self.telemetry.gpu_active {
                    let gpu_c = if self.telemetry.gpu_temp >= 85.0 { COLOR_NITRO_RED } else { COLOR_NITRO_CYAN };
                    ui.label(RichText::new(format!("{:.1}°C • {} MHz • {:.1}W • VRAM: {:.1}/{:.1} GB (RTX 3050)", 
                        self.telemetry.gpu_temp, self.telemetry.gpu_clock, self.telemetry.gpu_power, self.telemetry.gpu_vram_used, self.telemetry.gpu_vram_total)).color(gpu_c).monospace());
                } else {
                    ui.label(RichText::new("PCIe D3cold Sleep (0.0W)").color(COLOR_TEXT_MUTED).monospace());
                }
                ui.end_row();

                // Memory
                ui.colored_label(COLOR_TEXT_MUTED, "RAM & Swap:");
                ui.label(RichText::new(format!("{:.1}/{:.1} GB ({:.0}%) • Swap: {:.0}%", 
                    self.telemetry.ram_used, self.telemetry.ram_total, self.telemetry.ram_pct, self.telemetry.swap_pct)).color(COLOR_TEXT_WHITE).monospace());
                ui.end_row();

                // NVMe
                ui.colored_label(COLOR_TEXT_MUTED, "NVMe SSD:");
                ui.label(RichText::new(format!("{:.1}°C • Health: {}%", self.telemetry.nvme_temp, self.telemetry.nvme_health)).color(COLOR_TEXT_WHITE).monospace());
                ui.end_row();

                // Battery
                ui.colored_label(COLOR_TEXT_MUTED, "Power / Battery:");
                ui.label(RichText::new(format!("{}% ({}) • {:.2}V • Health: {:.0}%", 
                    self.telemetry.bat_pct, self.telemetry.bat_status, self.telemetry.bat_voltage, self.telemetry.bat_health)).color(COLOR_TEXT_WHITE).monospace());
                ui.end_row();
            });
    }
}

impl eframe::App for AcerSenseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Drain Telemetry Channel (Lock-free < 1 µs)
        if let Ok(data) = self.telemetry_rx.try_recv() {
            self.telemetry = data;
        }

        // 2. Physics Animation at 60 FPS
        let speed_cpu = (self.telemetry.cpu_rpm as f32 / 5660.0_f32) * 0.20_f32 + 0.02_f32;
        let speed_gpu = (self.telemetry.gpu_rpm as f32 / 6000.0_f32) * 0.20_f32 + 0.02_f32;
        self.turbine_cpu_angle = (self.turbine_cpu_angle + speed_cpu) % std::f32::consts::TAU;
        self.turbine_gpu_angle = (self.turbine_gpu_angle + speed_gpu) % std::f32::consts::TAU;
        ctx.request_repaint();

        // 3. Hotkeys
        ctx.input(|i| {
            if i.modifiers.ctrl && (i.key_pressed(egui::Key::Q) || i.key_pressed(egui::Key::W)) {
                std::process::exit(0);
            }
            if i.key_pressed(egui::Key::Num1) { self.current_tab = Tab::FanControl; }
            if i.key_pressed(egui::Key::Num2) { self.current_tab = Tab::Monitoring; }
            if i.key_pressed(egui::Key::Num3) { self.current_tab = Tab::PowerModes; }
            if i.key_pressed(egui::Key::Num4) { self.current_tab = Tab::KeyboardRgb; }
            if i.key_pressed(egui::Key::Num5) { self.current_tab = Tab::SystemSettings; }
            if i.key_pressed(egui::Key::M) {
                self.config.mode = "max".into();
                let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("max".into()));
            }
            if i.key_pressed(egui::Key::A) {
                self.config.mode = "auto".into();
                let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("auto".into()));
            }
            if i.key_pressed(egui::Key::C) {
                self.config.mode = "custom".into();
                let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("custom".into()));
            }
        });

        // 4. Windows NitroSense Iconic Crimson Top Header
        egui::TopBottomPanel::top("nitro_header").frame(Frame::none().fill(COLOR_NITRO_PANEL).inner_margin(Margin::symmetric(16.0_f32, 10.0_f32))).show(ctx, |ui| {
            let tabs = [
                (Tab::FanControl, "🌀 FAN CONTROL (1)"),
                (Tab::Monitoring, "📊 MONITORING (2)"),
                (Tab::PowerModes, "⚡ SCENARIOS (3)"),
                (Tab::KeyboardRgb, "🌈 PULSAR LIGHTING (4)"),
                (Tab::SystemSettings, "⚙ SETTINGS (5)"),
            ];

            let avail_w = ui.available_width();
            if avail_w < 680.0_f32 {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("NITROSENSE").color(COLOR_NITRO_RED).strong().size(18.0_f32));
                    ui.label(RichText::new("PRO LINUX v2.0").color(COLOR_TEXT_MUTED).size(10.0_f32));
                });
                ui.add_space(4.0_f32);
                ui.horizontal_wrapped(|ui| {
                    for (t, label) in tabs {
                        let active = self.current_tab == t;
                        let text = RichText::new(label)
                            .color(if active { COLOR_NITRO_RED } else { COLOR_TEXT_MUTED })
                            .strong();
                        if ui.selectable_label(active, text).clicked() {
                            self.current_tab = t;
                        }
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("NITROSENSE").color(COLOR_NITRO_RED).strong().size(20.0_f32));
                    ui.label(RichText::new("ACER GAMING SUITE (100% RUST)").color(COLOR_TEXT_MUTED).size(11.0_f32));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        for (t, label) in tabs.iter().rev() {
                            let active = self.current_tab == *t;
                            let text = RichText::new(*label)
                                .color(if active { COLOR_NITRO_RED } else { COLOR_TEXT_MUTED })
                                .strong();
                            if ui.selectable_label(active, text).clicked() {
                                self.current_tab = *t;
                            }
                        }
                    });
                });
            }
            ui.add_space(4.0_f32);
            ui.separator();
        });

        // 5. Central Viewport with Fluid Scrolling
        egui::CentralPanel::default().frame(Frame::none().fill(COLOR_NITRO_DARK_BG).inner_margin(Margin::same(14.0_f32))).show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                match self.current_tab {
                    Tab::FanControl => {
                        let avail_w = ui.available_width();
                        let is_auto = self.config.mode == "auto";
                        let is_max = self.config.mode == "max" || self.config.mode == "turbo";
                        let is_custom = self.config.mode == "custom";

                        // --- MAIN CARD: FAN SPEED CONTROL ---
                        self.nitro_card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.heading(RichText::new("Fan Speed Control").size(16.0_f32).strong().color(COLOR_TEXT_WHITE));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let mut cb = self.config.coolboost;
                                    let cb_active = cb;
                                    if ui.checkbox(&mut cb, RichText::new("CoolBoost™").color(if cb_active { COLOR_NITRO_GREEN } else { COLOR_TEXT_MUTED }).strong()).changed() {
                                        self.config.coolboost = cb;
                                        let _ = self.cmd_tx.send(HardwareCommand::SetCoolboost(cb));
                                    }
                                });
                            });
                            ui.add_space(4.0_f32);
                            ui.separator();
                            ui.add_space(10.0_f32);

                            let btn_auto = egui::Button::new(RichText::new("AUTO (A)").color(if is_auto { COLOR_NITRO_CYAN } else { COLOR_TEXT_MUTED }).strong())
                                .stroke(Stroke::new(if is_auto { 1.5_f32 } else { 1.0_f32 }, if is_auto { COLOR_NITRO_CYAN } else { COLOR_NITRO_BORDER }));
                            let btn_max = egui::Button::new(RichText::new("MAX (M)").color(if is_max { COLOR_NITRO_RED } else { COLOR_TEXT_MUTED }).strong())
                                .stroke(Stroke::new(if is_max { 1.5_f32 } else { 1.0_f32 }, if is_max { COLOR_NITRO_RED } else { COLOR_NITRO_BORDER }));
                            let btn_custom = egui::Button::new(RichText::new("CUSTOM (C)").color(if is_custom { COLOR_NITRO_RED_GLOW } else { COLOR_TEXT_MUTED }).strong())
                                .stroke(Stroke::new(if is_custom { 1.5_f32 } else { 1.0_f32 }, if is_custom { COLOR_NITRO_RED_GLOW } else { COLOR_NITRO_BORDER }));

                            if avail_w < 580.0_f32 {
                                ui.horizontal(|ui| {
                                    let btn_w = ((ui.available_width() - 16.0_f32) / 3.0_f32).max(75.0_f32);
                                    if ui.add_sized([btn_w, 34.0_f32], btn_auto).clicked() {
                                        self.config.mode = "auto".into();
                                        let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("auto".into()));
                                    }
                                    if ui.add_sized([btn_w, 34.0_f32], btn_max).clicked() {
                                        self.config.mode = "max".into();
                                        let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("max".into()));
                                    }
                                    if ui.add_sized([btn_w, 34.0_f32], btn_custom).clicked() {
                                        self.config.mode = "custom".into();
                                        let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("custom".into()));
                                    }
                                });
                                ui.add_space(12.0_f32);

                                let dial_sz = ((ui.available_width() - 20.0_f32) / 2.0_f32).clamp(115.0_f32, 145.0_f32);
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.set_width((ui.available_width() - 12.0_f32) / 2.0_f32);
                                        self.draw_nitro_dial(ui, "CPU FAN", self.telemetry.cpu_rpm, 5660, self.telemetry.cpu_temp, self.telemetry.cpu_load, self.turbine_cpu_angle, dial_sz, COLOR_NITRO_CYAN);
                                        ui.add_space(6.0_f32);
                                        ui.horizontal(|ui| {
                                            if ui.button("-").clicked() {
                                                self.config.cpu_fan_target = self.config.cpu_fan_target.saturating_sub(5);
                                                if self.sync_fans { self.config.gpu_fan_target = self.config.cpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            let mut c_tgt = self.config.cpu_fan_target;
                                            if ui.add(egui::Slider::new(&mut c_tgt, 0..=100).show_value(false)).changed() {
                                                self.config.cpu_fan_target = c_tgt;
                                                if self.sync_fans { self.config.gpu_fan_target = c_tgt; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: c_tgt, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            if ui.button("+").clicked() {
                                                self.config.cpu_fan_target = (self.config.cpu_fan_target + 5).min(100);
                                                if self.sync_fans { self.config.gpu_fan_target = self.config.cpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            ui.label(RichText::new(format!("{}%", self.config.cpu_fan_target)).color(COLOR_NITRO_CYAN).strong());
                                        });
                                    });

                                    ui.vertical(|ui| {
                                        ui.set_width(ui.available_width());
                                        self.draw_nitro_dial(ui, "GPU FAN", self.telemetry.gpu_rpm, 6000, self.telemetry.gpu_temp, if self.telemetry.gpu_active { 10.0 } else { 0.0 }, self.turbine_gpu_angle, dial_sz, COLOR_NITRO_RED);
                                        ui.add_space(6.0_f32);
                                        ui.horizontal(|ui| {
                                            if ui.button("-").clicked() {
                                                self.config.gpu_fan_target = self.config.gpu_fan_target.saturating_sub(5);
                                                if self.sync_fans { self.config.cpu_fan_target = self.config.gpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            let mut g_tgt = self.config.gpu_fan_target;
                                            if ui.add(egui::Slider::new(&mut g_tgt, 0..=100).show_value(false)).changed() {
                                                self.config.gpu_fan_target = g_tgt;
                                                if self.sync_fans { self.config.cpu_fan_target = g_tgt; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: g_tgt });
                                            }
                                            if ui.button("+").clicked() {
                                                self.config.gpu_fan_target = (self.config.gpu_fan_target + 5).min(100);
                                                if self.sync_fans { self.config.cpu_fan_target = self.config.gpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            ui.label(RichText::new(format!("{}%", self.config.gpu_fan_target)).color(COLOR_NITRO_RED).strong());
                                        });
                                    });
                                });
                            } else {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.set_width(140.0_f32);
                                        if ui.add_sized([135.0_f32, 36.0_f32], btn_auto).clicked() {
                                            self.config.mode = "auto".into();
                                            let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("auto".into()));
                                        }
                                        ui.add_space(6.0_f32);
                                        if ui.add_sized([135.0_f32, 36.0_f32], btn_max).clicked() {
                                            self.config.mode = "max".into();
                                            let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("max".into()));
                                        }
                                        ui.add_space(6.0_f32);
                                        if ui.add_sized([135.0_f32, 36.0_f32], btn_custom).clicked() {
                                            self.config.mode = "custom".into();
                                            let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("custom".into()));
                                        }
                                    });

                                    ui.add_space(20.0_f32);

                                    let remaining_w = ui.available_width();
                                    let dial_col_w = ((remaining_w - 20.0_f32) / 2.0_f32).max(150.0_f32);
                                    let dial_sz = (dial_col_w * 0.78_f32).clamp(120.0_f32, 155.0_f32);

                                    ui.vertical(|ui| {
                                        ui.set_width(dial_col_w);
                                        self.draw_nitro_dial(ui, "CPU FAN", self.telemetry.cpu_rpm, 5660, self.telemetry.cpu_temp, self.telemetry.cpu_load, self.turbine_cpu_angle, dial_sz, COLOR_NITRO_CYAN);
                                        ui.add_space(8.0_f32);
                                        ui.horizontal(|ui| {
                                            if ui.button("-").clicked() {
                                                self.config.cpu_fan_target = self.config.cpu_fan_target.saturating_sub(5);
                                                if self.sync_fans { self.config.gpu_fan_target = self.config.cpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            let mut c_tgt = self.config.cpu_fan_target;
                                            if ui.add(egui::Slider::new(&mut c_tgt, 0..=100).show_value(false)).changed() {
                                                self.config.cpu_fan_target = c_tgt;
                                                if self.sync_fans { self.config.gpu_fan_target = c_tgt; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: c_tgt, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            if ui.button("+").clicked() {
                                                self.config.cpu_fan_target = (self.config.cpu_fan_target + 5).min(100);
                                                if self.sync_fans { self.config.gpu_fan_target = self.config.cpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            ui.label(RichText::new(format!("{}%", self.config.cpu_fan_target)).color(COLOR_NITRO_CYAN).strong());
                                        });
                                    });

                                    ui.vertical(|ui| {
                                        ui.set_width(dial_col_w);
                                        self.draw_nitro_dial(ui, "GPU FAN", self.telemetry.gpu_rpm, 6000, self.telemetry.gpu_temp, if self.telemetry.gpu_active { 10.0 } else { 0.0 }, self.turbine_gpu_angle, dial_sz, COLOR_NITRO_RED);
                                        ui.add_space(8.0_f32);
                                        ui.horizontal(|ui| {
                                            if ui.button("-").clicked() {
                                                self.config.gpu_fan_target = self.config.gpu_fan_target.saturating_sub(5);
                                                if self.sync_fans { self.config.cpu_fan_target = self.config.gpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            let mut g_tgt = self.config.gpu_fan_target;
                                            if ui.add(egui::Slider::new(&mut g_tgt, 0..=100).show_value(false)).changed() {
                                                self.config.gpu_fan_target = g_tgt;
                                                if self.sync_fans { self.config.cpu_fan_target = g_tgt; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: g_tgt });
                                            }
                                            if ui.button("+").clicked() {
                                                self.config.gpu_fan_target = (self.config.gpu_fan_target + 5).min(100);
                                                if self.sync_fans { self.config.cpu_fan_target = self.config.gpu_fan_target; }
                                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                            }
                                            ui.label(RichText::new(format!("{}%", self.config.gpu_fan_target)).color(COLOR_NITRO_RED).strong());
                                        });
                                    });
                                });
                            }
                        });

                        ui.add_space(12.0_f32);

                        // --- BOTTOM ROW: TACTICAL TELEMETRY CARD ---
                        self.nitro_card_frame().show(ui, |ui| {
                            ui.heading(RichText::new("Live System Telemetry").size(15.0_f32).strong().color(COLOR_TEXT_WHITE));
                            ui.add_space(4.0_f32);
                            ui.separator();
                            ui.add_space(8.0_f32);
                            self.render_tactical_telemetry(ui);
                        });
                    }
                    Tab::Monitoring => {
                        self.nitro_card_frame().show(ui, |ui| {
                            ui.heading(RichText::new("📊 Comprehensive Hardware Monitor").size(16.0_f32).strong().color(COLOR_TEXT_WHITE));
                            ui.separator();
                            ui.add_space(10.0_f32);
                            self.render_tactical_telemetry(ui);
                        });
                    }
                    Tab::PowerModes => {
                        self.nitro_card_frame().show(ui, |ui| {
                            ui.heading(RichText::new("⚡ Operating Scenarios & Power Profiles").size(16.0_f32).strong().color(COLOR_TEXT_WHITE));
                            ui.separator();
                            ui.add_space(12.0_f32);

                            let current_p = self.config.profile.clone();
                            let is_quiet = current_p == "quiet" || current_p == "eco";
                            let is_bal = current_p == "balanced" || current_p == "balance";
                            let is_perf = current_p == "performance" || current_p == "turbo";

                            let btn_q = egui::Button::new(RichText::new("🌱 Quiet / Eco Mode\nLow Power & Fan Acoustic Optimization").color(if is_quiet { COLOR_NITRO_GREEN } else { COLOR_TEXT_MUTED }).strong())
                                .stroke(Stroke::new(if is_quiet { 1.5_f32 } else { 1.0_f32 }, if is_quiet { COLOR_NITRO_GREEN } else { COLOR_NITRO_BORDER }));
                            if ui.add_sized([ui.available_width(), 44.0_f32], btn_q).clicked() {
                                self.config.profile = "quiet".into();
                                let _ = self.cmd_tx.send(HardwareCommand::SetPowerProfile("quiet".into()));
                            }
                            ui.add_space(8.0_f32);

                            let btn_b = egui::Button::new(RichText::new("⚡ Default / Balanced Mode\nDynamic Thermal Envelope").color(if is_bal { COLOR_NITRO_CYAN } else { COLOR_TEXT_MUTED }).strong())
                                .stroke(Stroke::new(if is_bal { 1.5_f32 } else { 1.0_f32 }, if is_bal { COLOR_NITRO_CYAN } else { COLOR_NITRO_BORDER }));
                            if ui.add_sized([ui.available_width(), 44.0_f32], btn_b).clicked() {
                                self.config.profile = "balanced".into();
                                let _ = self.cmd_tx.send(HardwareCommand::SetPowerProfile("balanced".into()));
                            }
                            ui.add_space(8.0_f32);

                            let btn_p = egui::Button::new(RichText::new("🔥 Performance / Turbo Mode\nUnlocked TDP & Maximum Fan Speed").color(if is_perf { COLOR_NITRO_RED } else { COLOR_TEXT_MUTED }).strong())
                                .stroke(Stroke::new(if is_perf { 1.5_f32 } else { 1.0_f32 }, if is_perf { COLOR_NITRO_RED } else { COLOR_NITRO_BORDER }));
                            if ui.add_sized([ui.available_width(), 44.0_f32], btn_p).clicked() {
                                self.config.profile = "performance".into();
                                let _ = self.cmd_tx.send(HardwareCommand::SetPowerProfile("performance".into()));
                            }
                        });
                    }
                    Tab::KeyboardRgb => {
                        self.nitro_card_frame().show(ui, |ui| {
                            ui.heading(RichText::new("🌈 4-Zone Pulsar Keyboard Lighting").size(16.0_f32).strong().color(COLOR_TEXT_WHITE));
                            ui.separator();
                            ui.add_space(10.0_f32);

                            // Draw Original Windows 4-Zone Lighting Keyboard Diagram (Preserving Aspect Ratio)
                            if let Some(tex) = &self.textures {
                                let w = ui.available_width().min(540.0_f32);
                                let h = w / 3.58_f32; // 1224x342 Aspect Ratio
                                let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), egui::Sense::hover());
                                ui.painter().image(
                                    tex.keyboard_zone.id(),
                                    rect,
                                    Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                                    Color32::WHITE,
                                );
                            }
                            ui.add_space(14.0_f32);

                            ui.label(RichText::new("Original Acer Lighting Presets:").strong().color(COLOR_TEXT_WHITE));
                            ui.add_space(8.0_f32);

                            let presets = [
                                ("🔴 Nitro Red", 255, 0, 0),
                                ("🔷 Cyber Cyan", 0, 229, 255),
                                ("🟢 Toxic Green", 0, 230, 118),
                                ("🔮 Predator Purple", 180, 0, 255),
                                ("⚪ Pure White", 255, 255, 255),
                            ];

                            ui.horizontal_wrapped(|ui| {
                                for (name, r, g, b) in presets {
                                    if ui.button(RichText::new(name).color(Color32::from_rgb(r, g, b)).strong()).clicked() {
                                        for z in 1..=4 {
                                            let _ = self.cmd_tx.send(HardwareCommand::SetRgbZone { zone: z, r, g, b });
                                        }
                                    }
                                }
                            });
                        });
                    }
                    Tab::SystemSettings => {
                        self.nitro_card_frame().show(ui, |ui| {
                            ui.heading(RichText::new("⚙ Hardware Control & Gaming Locks").size(16.0_f32).strong().color(COLOR_TEXT_WHITE));
                            ui.separator();
                            ui.add_space(12.0_f32);

                            let mut limit = self.config.battery_health_80;
                            if ui.checkbox(&mut limit, RichText::new("🔋 80% Battery Health Limiter (Protects Lithium Cells)").strong()).changed() {
                                self.config.battery_health_80 = limit;
                                let _ = self.cmd_tx.send(HardwareCommand::SetBatteryLimit(limit));
                            }
                            ui.add_space(8.0_f32);

                            let mut win_lock = self.config.winkey_locked;
                            if ui.checkbox(&mut win_lock, RichText::new("🔒 Lock Windows / Super Key during Gaming").strong()).changed() {
                                self.config.winkey_locked = win_lock;
                                let _ = gaming::set_winkey_lock(win_lock);
                                let _ = save_config(&self.config);
                            }
                            ui.add_space(8.0_f32);

                            let mut tp_lock = self.config.touchpad_locked;
                            if ui.checkbox(&mut tp_lock, RichText::new("🚫 Disable Touchpad").strong()).changed() {
                                self.config.touchpad_locked = tp_lock;
                                let _ = gaming::set_touchpad_lock(tp_lock);
                                let _ = save_config(&self.config);
                            }
                            ui.add_space(14.0_f32);

                            if ui.button(RichText::new("🚀 Enable LCD 3ms Response Overdrive").color(COLOR_NITRO_RED).strong()).clicked() {
                                let _ = self.cmd_tx.send(HardwareCommand::SetLcdOverdrive(true));
                            }
                        });
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([880.0_f32, 580.0_f32])
            .with_min_inner_size([480.0_f32, 390.0_f32])
            .with_title("Acer NitroSense (Linux Native v2.0)"),
        ..Default::default()
    };

    eframe::run_native(
        "Acer NitroSense",
        options,
        Box::new(|cc| Ok(Box::new(AcerSenseApp::new(cc)))),
    )
}
