mod cli;
mod core;
mod hw;

use std::collections::VecDeque;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use core::config::{load_config, save_config, AcerConfig};
use core::fan;
use core::profile;
use eframe::egui::{
    self, Color32, Frame, Margin, Pos2, Rect, RichText, Rounding, ScrollArea, Stroke, Vec2, Visuals,
};
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

#[derive(PartialEq, Clone, Copy, Debug)]
enum MonitoringGraphMode {
    Thermals,
    Workload,
    Power,
    Turbines,
}

#[derive(Clone, Copy, Debug)]
struct TelemetryHistoryPoint {
    cpu_temp: f32,
    gpu_temp: f32,
    nvme_temp: f32,
    cpu_load: f32,
    gpu_load: f32,
    cpu_rpm: u32,
    gpu_rpm: u32,
    cpu_power: f32,
    gpu_power: f32,
}

#[derive(Debug, Clone)]
pub enum HardwareCommand {
    SetFanMode(String),
    SetCustomFans { cpu_pct: u8, gpu_pct: u8 },
    SetPowerProfile(String),
    SetRgbZone { zone: u8, r: u8, g: u8, b: u8 },
    SetRgbPreset(String),
    SetBatteryLimit(bool),
    SetLcdOverdrive(bool),
    SetCoolboost(bool),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ThemeMode {
    HumanComfort,
    TokyoNight,
    NeoTokyo,
    Everforest,
    NordicCalm,
    EarthSage,
    CyberNitro,
    RedAudit,
    HackerGreen,
    StellarVoid,
    KuromiGoth,
    CinnamorollNight,
    CinnamorollCloud,
    MyMelodySoft,
    PompompurinCafe,
    MantecCorporate,
    ModernBlue,
    MaterialRed,
    AuroraGradient,
}

impl ThemeMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::HumanComfort => "🌿 Confort Humano",
            Self::TokyoNight => "🌃 Tokyo Night",
            Self::NeoTokyo => "🏮 Neo Tokyo Cyber",
            Self::Everforest => "🌲 Everforest Soft",
            Self::NordicCalm => "🌊 Nórdico Calmo",
            Self::EarthSage => "🍃 Salvia & Tierra",
            Self::CyberNitro => "⚡ Cyber Nitro",
            Self::RedAudit => "🔴 Red Audit Offensive",
            Self::HackerGreen => "🟢 Hacker Matrix",
            Self::StellarVoid => "🌌 Stellar Void",
            Self::AuroraGradient => "🔮 Aurora Gradient",
            Self::KuromiGoth => "💜 Kuromi Goth",
            Self::CinnamorollNight => "✨ Cinnamoroll Night",
            Self::CinnamorollCloud => "☁️ Cinnamoroll Cloud",
            Self::MyMelodySoft => "🌸 My Melody Soft",
            Self::PompompurinCafe => "🍮 Pompompurin Café",
            Self::MantecCorporate => "👔 Mantec Corporate",
            Self::ModernBlue => "🔷 Modern Blue",
            Self::MaterialRed => "🔺 Material Dark Red",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::HumanComfort => "Pizarra cálida, coral suave y esmeralda relajante. Cero fatiga visual para largas sesiones.",
            Self::TokyoNight => "Inspirado en tokyonight.rasi: Índigo noche (#1A1B26), azul cielo (#7AA2F7) y rosa coral (#F7768E).",
            Self::NeoTokyo => "Inspirado en tu neo_tokyo.rofi: Púrpura noche (#0F0B1E), fucsia neón (#FF007C) y cian eléctrico (#00FFFF).",
            Self::Everforest => "Inspirado en squared-everforest.rasi: Carbón cálido (#2B3339), verde bosque suave (#A7C080) y turquesa tenue (#7FBBB3).",
            Self::NordicCalm => "Gris niebla, azul glacial y cian ártico. Estética nórdica minimalista y serena.",
            Self::EarthSage => "Carbón bosque, verde salvia orgánico y ámbar dorado. Tonos naturales reconfortantes.",
            Self::CyberNitro => "Rojo carmesí de alto contraste y cian eléctrico estilo deportivo original de Acer.",
            Self::RedAudit => "Inspirado en tu red_audit.rofi: Pitch Black (#0A0A0E), Blood Red (#FF3333) y alerta ámbar (#FF9E64). Red Team puro.",
            Self::HackerGreen => "Inspirado en tu hacker_green.rofi: Negro terminal (#000000) y fósforo verde CRT (#00FF00). Minimalismo Matrix.",
            Self::StellarVoid => "Inspirado en tu stellar_void.rofi: Vacío cósmico (#030303), plasma cian (#00FFFF) y violeta nebulosa (#8A2BE2).",
            Self::AuroraGradient => "Inspirado en tu botón CSS: Fondo noche cósmica (rgb(5,6,45)), Púrpura (#AF40FF), Índigo (#5B42F3) y Cian (#00DDEB).",
            Self::KuromiGoth => "Inspirado en tu kuromi_goth.rofi: Negro absoluto (#000000), lavanda gótica (#B48EAD) y rosa fucsia (#F5C2E7).",
            Self::CinnamorollNight => "Inspirado en tu cinnamoroll_night.rofi: Noche estrellada (#11152C), azul cielo (#8AADF4) y rosa mejilla (#F5BDE6).",
            Self::CinnamorollCloud => "Inspirado en tu cinnamoroll_cloud.rofi: Blanco etéreo (#F5F8FC) y azul cielo (#2E8FD9). Tema claro diurno.",
            Self::MyMelodySoft => "Inspirado en tu mymelody_soft.rofi: Crema suave (#FFF8F0), rosa pastel (#E85D75) y cálido albaricoque (#FFE8D6).",
            Self::PompompurinCafe => "Inspirado en tu pompompurin_cafe.rofi: Vainilla cálida (#FFFBF0), caramelo (#C97A18) y marrón chocolate (#4A2E21).",
            Self::MantecCorporate => "Inspirado en tu mantec_corporate.rofi: Carbón elegante (#1E1E1E) y naranja cobrizo (#E67E22). Look corporativo.",
            Self::ModernBlue => "Inspirado en tu modern_blue.rofi: Deep Dark Blue (#1A1B26) y azul moderno de alto contraste (#7AA2F7).",
            Self::MaterialRed => "Inspirado en squared-material-red.rasi: Gris oscuro Material (#212121) y rojo coral (#F07178).",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s {
            "tokyo" | "tokyonight" => Self::TokyoNight,
            "neotokyo" | "neo_tokyo" | "neo" => Self::NeoTokyo,
            "everforest" | "forest" => Self::Everforest,
            "nordic" => Self::NordicCalm,
            "earth" => Self::EarthSage,
            "cyber" => Self::CyberNitro,
            "red_audit" | "redaudit" | "audit" | "red" => Self::RedAudit,
            "hacker_green" | "hacker" | "matrix" | "green" => Self::HackerGreen,
            "stellar_void" | "stellar" | "void" | "space" => Self::StellarVoid,
            "aurora" | "gradient" | "aurora_gradient" | "cosmic_gradient" => Self::AuroraGradient,
            "kuromi_goth" | "kuromi" | "goth" => Self::KuromiGoth,
            "cinnamoroll_night" | "cinna_night" | "cinnamon_night" => Self::CinnamorollNight,
            "cinnamoroll_cloud" | "cinna_cloud" | "cinnamon_cloud" | "cloud" => {
                Self::CinnamorollCloud
            }
            "mymelody_soft" | "mymelody" | "melody" => Self::MyMelodySoft,
            "pompompurin_cafe" | "pompompurin" | "purin" | "cafe" => Self::PompompurinCafe,
            "mantec_corporate" | "mantec" | "corporate" => Self::MantecCorporate,
            "modern_blue" | "modern" | "blue" => Self::ModernBlue,
            "material_red" | "material" => Self::MaterialRed,
            _ => Self::HumanComfort,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HumanComfort => "human",
            Self::TokyoNight => "tokyo",
            Self::NeoTokyo => "neotokyo",
            Self::Everforest => "everforest",
            Self::NordicCalm => "nordic",
            Self::EarthSage => "earth",
            Self::CyberNitro => "cyber",
            Self::RedAudit => "red_audit",
            Self::HackerGreen => "hacker_green",
            Self::StellarVoid => "stellar_void",
            Self::AuroraGradient => "aurora_gradient",
            Self::KuromiGoth => "kuromi_goth",
            Self::CinnamorollNight => "cinnamoroll_night",
            Self::CinnamorollCloud => "cinnamoroll_cloud",
            Self::MyMelodySoft => "mymelody_soft",
            Self::PompompurinCafe => "pompompurin_cafe",
            Self::MantecCorporate => "mantec_corporate",
            Self::ModernBlue => "modern_blue",
            Self::MaterialRed => "material_red",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::HumanComfort => Self::TokyoNight,
            Self::TokyoNight => Self::NeoTokyo,
            Self::NeoTokyo => Self::Everforest,
            Self::Everforest => Self::NordicCalm,
            Self::NordicCalm => Self::EarthSage,
            Self::EarthSage => Self::CyberNitro,
            Self::CyberNitro => Self::RedAudit,
            Self::RedAudit => Self::HackerGreen,
            Self::HackerGreen => Self::StellarVoid,
            Self::StellarVoid => Self::AuroraGradient,
            Self::AuroraGradient => Self::KuromiGoth,
            Self::KuromiGoth => Self::CinnamorollNight,
            Self::CinnamorollNight => Self::CinnamorollCloud,
            Self::CinnamorollCloud => Self::MyMelodySoft,
            Self::MyMelodySoft => Self::PompompurinCafe,
            Self::PompompurinCafe => Self::MantecCorporate,
            Self::MantecCorporate => Self::ModernBlue,
            Self::ModernBlue => Self::MaterialRed,
            Self::MaterialRed => Self::HumanComfort,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub name: &'static str,
    pub is_light: bool,
    pub bg: Color32,
    pub panel: Color32,
    pub card: Color32,
    pub card_hover: Color32,
    pub border: Color32,
    pub border_subtle: Color32,
    pub primary: Color32,
    pub primary_glow: Color32,
    pub secondary: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub purple: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub card_rounding: f32,
}

pub fn get_palette(mode: ThemeMode) -> Palette {
    match mode {
        ThemeMode::HumanComfort => Palette {
            name: "Confort Humano",
            is_light: false,
            bg: Color32::from_rgb(15, 18, 25), // #0F1219 (Velvet deep slate)
            panel: Color32::from_rgb(21, 26, 36), // #151A24 (Soft matte panel)
            card: Color32::from_rgb(27, 34, 48), // #1B2230 (Welcoming warm card)
            card_hover: Color32::from_rgb(34, 43, 60), // #222B3C
            border: Color32::from_rgb(46, 58, 80), // #2E3A50
            border_subtle: Color32::from_rgb(34, 42, 58), // #222A3A
            primary: Color32::from_rgb(244, 63, 94), // #F43F5E (Warm Rose-Coral)
            primary_glow: Color32::from_rgb(251, 113, 133), // #FB7185
            secondary: Color32::from_rgb(56, 189, 248), // #38BDF8 (Tranquil Sky)
            success: Color32::from_rgb(52, 211, 153), // #34D399 (Organic Sage)
            warning: Color32::from_rgb(251, 191, 36), // #FBBF24 (Honey Amber)
            purple: Color32::from_rgb(167, 139, 250), // #A78BFA (Soft Iris)
            text_primary: Color32::from_rgb(241, 245, 249), // #F1F5F9 (Soft Pearl White)
            text_secondary: Color32::from_rgb(148, 163, 184), // #94A3B8 (Calm Slate)
            text_muted: Color32::from_rgb(100, 116, 139), // #64748B (Muted Slate)
            card_rounding: 10.0,
        },
        ThemeMode::TokyoNight => Palette {
            name: "Tokyo Night",
            is_light: false,
            bg: Color32::from_rgb(26, 27, 38), // #1A1B26 (Tokyo Night Dark)
            panel: Color32::from_rgb(31, 35, 53), // #1F2335
            card: Color32::from_rgb(36, 40, 59), // #24283B
            card_hover: Color32::from_rgb(47, 53, 78), // #2F354E
            border: Color32::from_rgb(65, 72, 104), // #414868
            border_subtle: Color32::from_rgb(46, 52, 79),
            primary: Color32::from_rgb(247, 118, 142), // #F7768E (Tokyo Red/Coral)
            primary_glow: Color32::from_rgb(255, 158, 175),
            secondary: Color32::from_rgb(122, 162, 247), // #7AA2F7 (Tokyo Blue)
            success: Color32::from_rgb(158, 206, 106),   // #9ECE6A (Tokyo Green)
            warning: Color32::from_rgb(224, 175, 104),   // #E0AF68 (Tokyo Yellow)
            purple: Color32::from_rgb(187, 154, 247),    // #BB9AF7
            text_primary: Color32::from_rgb(192, 202, 245), // #C0CAF5 (Tokyo fg)
            text_secondary: Color32::from_rgb(169, 177, 214), // #A9B1D6
            text_muted: Color32::from_rgb(115, 122, 162), // #737AA2
            card_rounding: 8.0,
        },
        ThemeMode::NeoTokyo => Palette {
            name: "Neo Tokyo Cyber",
            is_light: false,
            bg: Color32::from_rgb(15, 11, 30), // #0F0B1E (Púrpura ultra oscuro)
            panel: Color32::from_rgb(26, 18, 53), // #1A1235
            card: Color32::from_rgb(35, 25, 71), // #231947
            card_hover: Color32::from_rgb(49, 35, 99), // #312363
            border: Color32::from_rgb(74, 56, 128), // #4A3880
            border_subtle: Color32::from_rgb(51, 37, 92),
            primary: Color32::from_rgb(255, 0, 124), // #FF007C (Fucsia Neón)
            primary_glow: Color32::from_rgb(255, 77, 166),
            secondary: Color32::from_rgb(0, 255, 255), // #00FFFF (Cian Eléctrico)
            success: Color32::from_rgb(5, 255, 161),   // #05FFA1 (Verde Neón)
            warning: Color32::from_rgb(255, 230, 0),   // #FFE600 (Amarillo Neón)
            purple: Color32::from_rgb(189, 0, 255),    // #BD00FF
            text_primary: Color32::from_rgb(243, 244, 248),
            text_secondary: Color32::from_rgb(192, 202, 245),
            text_muted: Color32::from_rgb(133, 135, 168),
            card_rounding: 10.0,
        },
        ThemeMode::Everforest => Palette {
            name: "Everforest Soft",
            is_light: false,
            bg: Color32::from_rgb(43, 51, 57), // #2B3339 (Everforest Soft Dark)
            panel: Color32::from_rgb(50, 61, 67), // #323D43
            card: Color32::from_rgb(58, 69, 74), // #3A454A
            card_hover: Color32::from_rgb(68, 81, 87), // #445157
            border: Color32::from_rgb(79, 91, 88), // #4F5B58
            border_subtle: Color32::from_rgb(60, 71, 69),
            primary: Color32::from_rgb(230, 126, 128), // #E67E80 (Everforest Red/Coral)
            primary_glow: Color32::from_rgb(240, 147, 149),
            secondary: Color32::from_rgb(127, 187, 179), // #7FBBB3 (Everforest Aqua/Blue)
            success: Color32::from_rgb(167, 192, 128),   // #A7C080 (Everforest Green)
            warning: Color32::from_rgb(219, 188, 127),   // #DBBC7F (Everforest Amber)
            purple: Color32::from_rgb(214, 153, 182),    // #D699B6
            text_primary: Color32::from_rgb(211, 198, 170), // #D3C6AA
            text_secondary: Color32::from_rgb(157, 169, 160), // #9DA9A0
            text_muted: Color32::from_rgb(122, 132, 120), // #7A8478
            card_rounding: 6.0,
        },
        ThemeMode::NordicCalm => Palette {
            name: "Nórdico Calmo",
            is_light: false,
            bg: Color32::from_rgb(16, 20, 27),
            panel: Color32::from_rgb(23, 29, 39),
            card: Color32::from_rgb(30, 38, 52),
            card_hover: Color32::from_rgb(38, 48, 66),
            border: Color32::from_rgb(48, 62, 84),
            border_subtle: Color32::from_rgb(35, 45, 62),
            primary: Color32::from_rgb(96, 165, 250), // #60A5FA (Arctic Azure)
            primary_glow: Color32::from_rgb(147, 197, 253),
            secondary: Color32::from_rgb(45, 212, 191), // #2DD4BF (Glacial Teal)
            success: Color32::from_rgb(74, 222, 128),
            warning: Color32::from_rgb(250, 204, 21),
            purple: Color32::from_rgb(192, 132, 252),
            text_primary: Color32::from_rgb(243, 244, 246),
            text_secondary: Color32::from_rgb(156, 163, 175),
            text_muted: Color32::from_rgb(107, 114, 128),
            card_rounding: 10.0,
        },
        ThemeMode::EarthSage => Palette {
            name: "Salvia & Tierra",
            is_light: false,
            bg: Color32::from_rgb(17, 22, 19), // Forest Charcoal
            panel: Color32::from_rgb(24, 31, 27),
            card: Color32::from_rgb(31, 41, 35),
            card_hover: Color32::from_rgb(40, 53, 45),
            border: Color32::from_rgb(52, 68, 58),
            border_subtle: Color32::from_rgb(36, 48, 40),
            primary: Color32::from_rgb(245, 158, 11), // Warm Earth Amber
            primary_glow: Color32::from_rgb(251, 191, 36),
            secondary: Color32::from_rgb(52, 211, 153), // Sage Mint
            success: Color32::from_rgb(74, 222, 128),
            warning: Color32::from_rgb(251, 146, 60),
            purple: Color32::from_rgb(167, 139, 250),
            text_primary: Color32::from_rgb(245, 245, 240),
            text_secondary: Color32::from_rgb(163, 173, 162),
            text_muted: Color32::from_rgb(112, 122, 111),
            card_rounding: 10.0,
        },
        ThemeMode::CyberNitro => Palette {
            name: "Cyber Nitro",
            is_light: false,
            bg: Color32::from_rgb(10, 13, 18),
            panel: Color32::from_rgb(16, 20, 28),
            card: Color32::from_rgb(22, 28, 38),
            card_hover: Color32::from_rgb(30, 38, 52),
            border: Color32::from_rgb(38, 48, 66),
            border_subtle: Color32::from_rgb(26, 33, 45),
            primary: Color32::from_rgb(229, 25, 55), // Iconic Nitro Red
            primary_glow: Color32::from_rgb(255, 60, 90),
            secondary: Color32::from_rgb(0, 229, 255), // Cyber Cyan
            success: Color32::from_rgb(0, 230, 118),
            warning: Color32::from_rgb(255, 160, 0),
            purple: Color32::from_rgb(179, 136, 255),
            text_primary: Color32::from_rgb(245, 248, 252),
            text_secondary: Color32::from_rgb(140, 152, 172),
            text_muted: Color32::from_rgb(95, 105, 122),
            card_rounding: 8.0,
        },
        ThemeMode::RedAudit => Palette {
            name: "Red Audit Offensive",
            is_light: false,
            bg: Color32::from_rgb(10, 10, 14), // #0A0A0E (Pitch Black)
            panel: Color32::from_rgb(21, 21, 30), // #15151E
            card: Color32::from_rgb(28, 28, 40), // #1C1C28
            card_hover: Color32::from_rgb(40, 40, 56),
            border: Color32::from_rgb(70, 24, 32), // #461820
            border_subtle: Color32::from_rgb(45, 16, 22),
            primary: Color32::from_rgb(255, 51, 51), // #FF3333 (Blood Red)
            primary_glow: Color32::from_rgb(255, 102, 102),
            secondary: Color32::from_rgb(255, 158, 100), // #FF9E64 (Urgent Orange)
            success: Color32::from_rgb(0, 230, 118),
            warning: Color32::from_rgb(255, 179, 0),
            purple: Color32::from_rgb(224, 64, 251),
            text_primary: Color32::from_rgb(226, 226, 227), // #E2E2E3
            text_secondary: Color32::from_rgb(160, 160, 170),
            text_muted: Color32::from_rgb(92, 92, 102),
            card_rounding: 10.0,
        },
        ThemeMode::HackerGreen => Palette {
            name: "Hacker Matrix",
            is_light: false,
            bg: Color32::from_rgb(0, 0, 0), // #000000 (Pure Terminal Black)
            panel: Color32::from_rgb(5, 15, 5), // #050F05
            card: Color32::from_rgb(10, 27, 10), // #0A1B0A
            card_hover: Color32::from_rgb(17, 46, 17),
            border: Color32::from_rgb(0, 95, 0), // #005F00
            border_subtle: Color32::from_rgb(0, 51, 0),
            primary: Color32::from_rgb(0, 255, 0), // #00FF00 (Phosphor Green)
            primary_glow: Color32::from_rgb(102, 255, 102),
            secondary: Color32::from_rgb(0, 170, 0), // #00AA00 (Terminal Dim Green)
            success: Color32::from_rgb(51, 255, 51),
            warning: Color32::from_rgb(204, 255, 0),
            purple: Color32::from_rgb(0, 229, 255),
            text_primary: Color32::from_rgb(221, 255, 221), // #DDFFDD
            text_secondary: Color32::from_rgb(0, 204, 0),
            text_muted: Color32::from_rgb(0, 119, 0),
            card_rounding: 6.0,
        },
        ThemeMode::StellarVoid => Palette {
            name: "Stellar Void",
            is_light: false,
            bg: Color32::from_rgb(3, 3, 3), // #030303 (Deep Cosmic Void)
            panel: Color32::from_rgb(15, 17, 26), // #0F111A
            card: Color32::from_rgb(22, 25, 38), // #161926
            card_hover: Color32::from_rgb(32, 37, 56),
            border: Color32::from_rgb(138, 43, 226), // #8A2BE2 (Nebula Violet)
            border_subtle: Color32::from_rgb(61, 32, 102),
            primary: Color32::from_rgb(0, 255, 255), // #00FFFF (Plasma Cyan)
            primary_glow: Color32::from_rgb(128, 255, 255),
            secondary: Color32::from_rgb(138, 43, 226), // #8A2BE2
            success: Color32::from_rgb(0, 255, 157),
            warning: Color32::from_rgb(255, 69, 0), // #FF4500 (Supernova Orange)
            purple: Color32::from_rgb(191, 85, 236),
            text_primary: Color32::from_rgb(224, 224, 224), // #E0E0E0
            text_secondary: Color32::from_rgb(158, 158, 174),
            text_muted: Color32::from_rgb(92, 92, 112),
            card_rounding: 12.0,
        },
        ThemeMode::KuromiGoth => Palette {
            name: "Kuromi Goth",
            is_light: false,
            bg: Color32::from_rgb(0, 0, 0), // #000000 (Pure Black)
            panel: Color32::from_rgb(17, 17, 27), // #11111B
            card: Color32::from_rgb(24, 24, 37), // #181825
            card_hover: Color32::from_rgb(37, 37, 56),
            border: Color32::from_rgb(180, 142, 173), // #B48EAD (Goth Lavender)
            border_subtle: Color32::from_rgb(74, 53, 73),
            primary: Color32::from_rgb(180, 142, 173), // #B48EAD
            primary_glow: Color32::from_rgb(212, 174, 207),
            secondary: Color32::from_rgb(245, 194, 231), // #F5C2E7 (Skull Pink)
            success: Color32::from_rgb(166, 227, 161),
            warning: Color32::from_rgb(249, 226, 175),
            purple: Color32::from_rgb(203, 166, 247),
            text_primary: Color32::from_rgb(205, 214, 244), // #CDD6F4
            text_secondary: Color32::from_rgb(166, 173, 200),
            text_muted: Color32::from_rgb(108, 112, 134),
            card_rounding: 10.0,
        },
        ThemeMode::CinnamorollNight => Palette {
            name: "Cinnamoroll Night",
            is_light: false,
            bg: Color32::from_rgb(17, 21, 44), // #11152C (Starry Midnight)
            panel: Color32::from_rgb(26, 33, 65), // #1A2141
            card: Color32::from_rgb(34, 43, 84), // #222B54
            card_hover: Color32::from_rgb(46, 58, 112),
            border: Color32::from_rgb(138, 173, 244), // #8AADF4 (Pastel Sky)
            border_subtle: Color32::from_rgb(50, 64, 117),
            primary: Color32::from_rgb(138, 173, 244), // #8AADF4
            primary_glow: Color32::from_rgb(181, 207, 254),
            secondary: Color32::from_rgb(245, 189, 230), // #F5BDE6 (Pastel Pink)
            success: Color32::from_rgb(166, 209, 137),
            warning: Color32::from_rgb(229, 200, 144),
            purple: Color32::from_rgb(202, 158, 230),
            text_primary: Color32::from_rgb(238, 243, 250), // #EEF3FA
            text_secondary: Color32::from_rgb(186, 194, 222),
            text_muted: Color32::from_rgb(115, 121, 148),
            card_rounding: 10.0,
        },
        ThemeMode::CinnamorollCloud => Palette {
            name: "Cinnamoroll Cloud",
            is_light: true,
            bg: Color32::from_rgb(245, 248, 252), // #F5F8FC (Ethereal White)
            panel: Color32::from_rgb(232, 240, 248), // #E8F0F8
            card: Color32::from_rgb(221, 233, 245), // #DDE9F5
            card_hover: Color32::from_rgb(207, 225, 240),
            border: Color32::from_rgb(181, 211, 238), // #B5D3EE
            border_subtle: Color32::from_rgb(202, 221, 240),
            primary: Color32::from_rgb(46, 143, 217), // #2E8FD9 (Vibrant Sky Blue)
            primary_glow: Color32::from_rgb(93, 176, 238),
            secondary: Color32::from_rgb(232, 112, 158), // #E8709E (Blush Pink)
            success: Color32::from_rgb(46, 160, 95),
            warning: Color32::from_rgb(221, 107, 32),
            purple: Color32::from_rgb(128, 90, 213),
            text_primary: Color32::from_rgb(37, 50, 67), // #253243 (High Contrast Slate)
            text_secondary: Color32::from_rgb(71, 85, 105),
            text_muted: Color32::from_rgb(100, 116, 139),
            card_rounding: 12.0,
        },
        ThemeMode::MyMelodySoft => Palette {
            name: "My Melody Soft",
            is_light: true,
            bg: Color32::from_rgb(255, 248, 240), // #FFF8F0 (Soft Cream)
            panel: Color32::from_rgb(255, 239, 227), // #FFEFE3
            card: Color32::from_rgb(255, 229, 212), // #FFE5D4
            card_hover: Color32::from_rgb(255, 217, 194),
            border: Color32::from_rgb(244, 184, 172), // #F4B8AC
            border_subtle: Color32::from_rgb(249, 208, 199),
            primary: Color32::from_rgb(232, 93, 117), // #E85D75 (Melody Rose)
            primary_glow: Color32::from_rgb(255, 141, 161),
            secondary: Color32::from_rgb(217, 119, 6), // #D97706 (Apricot Gold)
            success: Color32::from_rgb(5, 150, 105),
            warning: Color32::from_rgb(217, 119, 6),
            purple: Color32::from_rgb(147, 51, 234),
            text_primary: Color32::from_rgb(61, 48, 53), // #3D3035 (Warm Dark Cocoa)
            text_secondary: Color32::from_rgb(107, 88, 95),
            text_muted: Color32::from_rgb(140, 119, 127),
            card_rounding: 12.0,
        },
        ThemeMode::PompompurinCafe => Palette {
            name: "Pompompurin Café",
            is_light: true,
            bg: Color32::from_rgb(255, 251, 240), // #FFFBF0 (Warm Vanilla)
            panel: Color32::from_rgb(253, 244, 226), // #FDF4E2
            card: Color32::from_rgb(245, 232, 206), // #F5E8CE
            card_hover: Color32::from_rgb(238, 219, 186),
            border: Color32::from_rgb(223, 196, 158), // #DFC49E
            border_subtle: Color32::from_rgb(235, 215, 184),
            primary: Color32::from_rgb(201, 122, 24), // #C97A18 (Purin Caramel)
            primary_glow: Color32::from_rgb(229, 169, 60),
            secondary: Color32::from_rgb(74, 46, 33), // #4A2E21 (Dark Chocolate)
            success: Color32::from_rgb(46, 125, 50),
            warning: Color32::from_rgb(230, 81, 0),
            purple: Color32::from_rgb(106, 27, 154),
            text_primary: Color32::from_rgb(61, 38, 25), // #3D2619 (Chocolate Brown)
            text_secondary: Color32::from_rgb(92, 64, 51),
            text_muted: Color32::from_rgb(133, 102, 84),
            card_rounding: 12.0,
        },
        ThemeMode::MantecCorporate => Palette {
            name: "Mantec Corporate",
            is_light: false,
            bg: Color32::from_rgb(30, 30, 30), // #1E1E1E (Executive Charcoal)
            panel: Color32::from_rgb(37, 37, 38), // #252526
            card: Color32::from_rgb(45, 45, 48), // #2D2D30
            card_hover: Color32::from_rgb(56, 56, 61),
            border: Color32::from_rgb(62, 62, 66), // #3E3E42
            border_subtle: Color32::from_rgb(51, 51, 55),
            primary: Color32::from_rgb(230, 126, 34), // #E67E22 (Corporate Copper)
            primary_glow: Color32::from_rgb(243, 156, 18),
            secondary: Color32::from_rgb(52, 152, 219), // #3498DB (Corporate Steel Blue)
            success: Color32::from_rgb(46, 204, 113),
            warning: Color32::from_rgb(241, 196, 15),
            purple: Color32::from_rgb(155, 89, 182),
            text_primary: Color32::from_rgb(212, 212, 212), // #D4D4D4
            text_secondary: Color32::from_rgb(160, 160, 160),
            text_muted: Color32::from_rgb(109, 109, 109),
            card_rounding: 8.0,
        },
        ThemeMode::ModernBlue => Palette {
            name: "Modern Blue",
            is_light: false,
            bg: Color32::from_rgb(26, 27, 38), // #1A1B26 (Deep Dark Blue)
            panel: Color32::from_rgb(32, 35, 54), // #202336
            card: Color32::from_rgb(39, 44, 66), // #272C42
            card_hover: Color32::from_rgb(51, 57, 86),
            border: Color32::from_rgb(65, 72, 104), // #414868
            border_subtle: Color32::from_rgb(45, 50, 77),
            primary: Color32::from_rgb(122, 162, 247), // #7AA2F7 (Modern Azure)
            primary_glow: Color32::from_rgb(159, 189, 249),
            secondary: Color32::from_rgb(247, 118, 142), // #F7768E (Soft Rose)
            success: Color32::from_rgb(115, 218, 202),
            warning: Color32::from_rgb(224, 175, 104),
            purple: Color32::from_rgb(187, 154, 247),
            text_primary: Color32::from_rgb(192, 202, 245), // #C0CAF5
            text_secondary: Color32::from_rgb(169, 177, 214),
            text_muted: Color32::from_rgb(86, 95, 137),
            card_rounding: 10.0,
        },
        ThemeMode::MaterialRed => Palette {
            name: "Material Dark Red",
            is_light: false,
            bg: Color32::from_rgb(33, 33, 33), // #212121 (Material Darker)
            panel: Color32::from_rgb(43, 43, 43), // #2B2B2B
            card: Color32::from_rgb(53, 53, 53), // #353535
            card_hover: Color32::from_rgb(66, 66, 66),
            border: Color32::from_rgb(80, 80, 80), // #505050
            border_subtle: Color32::from_rgb(58, 58, 58),
            primary: Color32::from_rgb(240, 113, 120), // #F07178 (Material Coral Red)
            primary_glow: Color32::from_rgb(245, 146, 151),
            secondary: Color32::from_rgb(130, 170, 255), // #82AAFF (Material Blue)
            success: Color32::from_rgb(195, 232, 141),
            warning: Color32::from_rgb(255, 203, 107),
            purple: Color32::from_rgb(199, 146, 234),
            text_primary: Color32::from_rgb(238, 255, 255), // #EEFFFF
            text_secondary: Color32::from_rgb(178, 204, 214),
            text_muted: Color32::from_rgb(113, 124, 180),
            card_rounding: 6.0,
        },
        ThemeMode::AuroraGradient => Palette {
            name: "Aurora Gradient",
            is_light: false,
            bg: Color32::from_rgb(5, 6, 45), // rgb(5, 6, 45) Cosmic Night
            panel: Color32::from_rgb(12, 14, 65), // Elevated deep indigo
            card: Color32::from_rgb(20, 22, 85),
            card_hover: Color32::from_rgb(32, 34, 115),
            border: Color32::from_rgb(91, 66, 243), // #5B42F3 (Electric Indigo)
            border_subtle: Color32::from_rgb(50, 40, 130),
            primary: Color32::from_rgb(175, 64, 255), // #AF40FF (Neon Purple)
            primary_glow: Color32::from_rgb(205, 115, 255),
            secondary: Color32::from_rgb(0, 221, 235), // #00DDEB (Neon Cyan)
            success: Color32::from_rgb(5, 255, 161),   // #05FFA1
            warning: Color32::from_rgb(255, 215, 0),   // #FFD700
            purple: Color32::from_rgb(91, 66, 243),    // #5B42F3
            text_primary: Color32::from_rgb(255, 255, 255), // #FFFFFF Crisp White
            text_secondary: Color32::from_rgb(200, 205, 245),
            text_muted: Color32::from_rgb(130, 138, 185),
            card_rounding: 8.0,
        },
    }
}

pub fn apply_visuals_for_palette(ctx: &egui::Context, pal: &Palette) {
    let mut visuals = if pal.is_light {
        Visuals::light()
    } else {
        Visuals::dark()
    };
    visuals.override_text_color = Some(pal.text_primary);
    visuals.panel_fill = pal.panel;
    visuals.window_fill = pal.bg;
    visuals.widgets.noninteractive.bg_fill = pal.panel;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, pal.border);
    visuals.widgets.noninteractive.rounding = Rounding::same(pal.card_rounding.min(8.0));
    visuals.widgets.inactive.bg_fill = pal.panel;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, pal.border);
    visuals.widgets.inactive.rounding = Rounding::same(pal.card_rounding.min(8.0));
    visuals.widgets.hovered.bg_fill = pal.card_hover;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.2_f32, pal.primary);
    visuals.widgets.hovered.rounding = Rounding::same(pal.card_rounding.min(8.0));
    visuals.widgets.active.bg_fill = pal.card_hover;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, pal.primary);
    visuals.widgets.active.rounding = Rounding::same(pal.card_rounding.min(8.0));
    ctx.set_visuals(visuals);
}

// Ergonomic Human-Centric Master Defaults (backward-compatible fallback)
#[allow(dead_code)]
const COLOR_NITRO_RED: Color32 = Color32::from_rgb(244, 63, 94); // #F43F5E (Warm Rose-Coral)
#[allow(dead_code)]
const COLOR_NITRO_RED_GLOW: Color32 = Color32::from_rgb(251, 113, 133); // #FB7185
#[allow(dead_code)]
const COLOR_NITRO_CYAN: Color32 = Color32::from_rgb(56, 189, 248); // #38BDF8 (Tranquil Sky Azure)
#[allow(dead_code)]
const COLOR_NITRO_GREEN: Color32 = Color32::from_rgb(52, 211, 153); // #34D399 (Organic Sage/Emerald)
#[allow(dead_code)]
const COLOR_NITRO_AMBER: Color32 = Color32::from_rgb(251, 191, 36); // #FBBF24 (Honey Amber)
#[allow(dead_code)]
const COLOR_NITRO_PURPLE: Color32 = Color32::from_rgb(167, 139, 250); // #A78BFA (Soft Iris/Lavender)
#[allow(dead_code)]
const COLOR_NITRO_DARK_BG: Color32 = Color32::from_rgb(15, 18, 25); // #0F1219 (Velvet Deep Slate)
#[allow(dead_code)]
const COLOR_NITRO_PANEL: Color32 = Color32::from_rgb(21, 26, 36); // #151A24 (Soft Matte Surface)
#[allow(dead_code)]
const COLOR_NITRO_CARD: Color32 = Color32::from_rgb(27, 34, 48); // #1B2230 (Warm Slate Card)
#[allow(dead_code)]
const COLOR_NITRO_BORDER: Color32 = Color32::from_rgb(46, 58, 80); // #2E3A50 (Gentle Subtle Border)
#[allow(dead_code)]
const COLOR_TEXT_WHITE: Color32 = Color32::from_rgb(241, 245, 249); // #F1F5F9 (Soft Pearl White, Anti-Glare)
#[allow(dead_code)]
const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184); // #94A3B8 (Calm Slate Text)

struct NitroTextures {
    fan_blade: egui::TextureHandle,
    keyboard_zone: egui::TextureHandle,
}

struct AcerSenseApp {
    current_tab: Tab,
    config: AcerConfig,
    theme_mode: ThemeMode,

    // Decoupled Background Worker Channels
    cmd_tx: Sender<HardwareCommand>,
    telemetry_rx: Receiver<TelemetryData>,
    telemetry: TelemetryData,

    // Telemetry Rolling History Buffer & Oscilloscope controls
    history: VecDeque<TelemetryHistoryPoint>,
    monitoring_graph_mode: MonitoringGraphMode,
    graph_paused: bool,
    history_window_size: usize,

    // Peak metrics tracking (session records)
    peak_cpu_temp: f32,
    peak_gpu_temp: f32,
    peak_cpu_power: f32,
    peak_gpu_power: f32,

    // Animation physics (60 FPS)
    turbine_cpu_angle: f32,
    turbine_gpu_angle: f32,

    // RGB Studio Controls
    rgb_selected_zone: u8, // 0 = All Zones, 1..=4 = Individual Zone
    rgb_custom_r: u8,
    rgb_custom_g: u8,
    rgb_custom_b: u8,
    rgb_last_status: String,

    // System Diagnostics
    diag_status: Option<String>,

    // Fan Dynamics
    delta_cpu_rpm: i32,
    delta_gpu_rpm: i32,

    // Original Windows Textures
    textures: Option<NitroTextures>,
}

fn load_texture(ctx: &egui::Context, id: &str, bytes: &[u8]) -> egui::TextureHandle {
    let img = image::load_from_memory(bytes).expect("Failed to decode embedded Nitro PNG");
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8();
    let color_image =
        egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_flat_samples().as_slice());
    ctx.load_texture(id, color_image, egui::TextureOptions::LINEAR)
}

fn draw_tactical_bar(ui: &mut egui::Ui, pct: f32, fill_color: Color32, height: f32) {
    let avail_w = ui.available_width().max(30.0_f32);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    // Background slot with velvety rounded pill slot
    painter.rect_filled(
        rect,
        Rounding::same(height / 2.0_f32),
        Color32::from_rgb(18, 23, 33),
    );
    painter.rect_stroke(
        rect,
        Rounding::same(height / 2.0_f32),
        Stroke::new(1.0_f32, Color32::from_rgb(42, 54, 76)),
    );

    // Filled progress with smooth rounded pill ends
    let clamped_pct = pct.clamp(0.0_f32, 100.0_f32) / 100.0_f32;
    if clamped_pct > 0.005_f32 {
        let fill_w = (rect.width() * clamped_pct).max(height);
        let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_w, rect.height()));
        painter.rect_filled(fill_rect, Rounding::same(height / 2.0_f32), fill_color);
    }
}

/// Draws an aesthetic shaded curve with translucid gradient fill below line
fn draw_shaded_curve(
    painter: &egui::Painter,
    points: &[Pos2],
    bottom_y: f32,
    line_color: Color32,
    stroke_width: f32,
) {
    let n = points.len();
    if n < 2 {
        return;
    }

    let mut mesh = egui::Mesh::default();
    let top_color =
        Color32::from_rgba_unmultiplied(line_color.r(), line_color.g(), line_color.b(), 35);
    let bot_color =
        Color32::from_rgba_unmultiplied(line_color.r(), line_color.g(), line_color.b(), 2);

    for (i, &pt) in points.iter().enumerate() {
        let bot_pt = Pos2::new(pt.x, bottom_y);
        let idx = (i * 2) as u32;
        mesh.vertices.push(egui::epaint::Vertex {
            pos: pt,
            uv: Pos2::ZERO,
            color: top_color,
        });
        mesh.vertices.push(egui::epaint::Vertex {
            pos: bot_pt,
            uv: Pos2::ZERO,
            color: bot_color,
        });
        if i > 0 {
            mesh.add_triangle(idx - 2, idx - 1, idx);
            mesh.add_triangle(idx - 1, idx + 1, idx);
        }
    }
    painter.add(mesh);

    for i in 0..(n - 1) {
        painter.line_segment(
            [points[i], points[i + 1]],
            Stroke::new(stroke_width, line_color),
        );
    }
}

impl AcerSenseApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let cfg = load_config();
        let theme_arg = std::env::args()
            .position(|a| a == "--theme" || a == "-t")
            .and_then(|idx| std::env::args().nth(idx + 1));
        let theme_mode = if let Some(ref t) = theme_arg {
            ThemeMode::from_str(t)
        } else {
            ThemeMode::from_str(&cfg.theme)
        };
        let pal = get_palette(theme_mode);
        apply_visuals_for_palette(&cc.egui_ctx, &pal);

        let textures = NitroTextures {
            fan_blade: load_texture(&cc.egui_ctx, "nitro_fan_blade", ASSET_FAN_BLADE),
            keyboard_zone: load_texture(&cc.egui_ctx, "nitro_keyboard_zone", ASSET_LIGHTING_ZONE),
        };

        let (cmd_tx, cmd_rx) = channel::<HardwareCommand>();
        let (telemetry_tx, telemetry_rx) = sync_channel::<TelemetryData>(1);

        // Spawn Dedicated Background Worker (decoupled polling)
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
                                let data = thermal::collect_all_telemetry(&cached_config);
                                let _ = telemetry_tx.try_send(data);
                                last_poll = Instant::now();
                            }
                            HardwareCommand::SetCustomFans { cpu_pct, gpu_pct } => {
                                let _ = fan::set_custom_speeds(cpu_pct, gpu_pct);
                                cached_config = load_config();
                                let data = thermal::collect_all_telemetry(&cached_config);
                                let _ = telemetry_tx.try_send(data);
                                last_poll = Instant::now();
                            }
                            HardwareCommand::SetPowerProfile(prof) => {
                                let _ = profile::set_profile(&prof);
                                cached_config = load_config();
                            }
                            HardwareCommand::SetRgbZone { zone, r, g, b } => {
                                let _ = rgb::set_static_color(zone, r, g, b);
                            }
                            HardwareCommand::SetRgbPreset(preset) => {
                                let _ = rgb::apply_preset(&preset);
                            }
                            HardwareCommand::SetBatteryLimit(limit) => {
                                let _ = battery::set_battery_80_limit(limit);
                                cached_config.battery_health_80 = limit;
                                save_config(&cached_config);
                            }
                            HardwareCommand::SetLcdOverdrive(en) => {
                                let _ = gaming::set_lcd_overdrive(en);
                            }
                            HardwareCommand::SetCoolboost(en) => {
                                let _ = fan::set_coolboost_state(en);
                                cached_config = load_config();
                                let data = thermal::collect_all_telemetry(&cached_config);
                                let _ = telemetry_tx.try_send(data);
                                last_poll = Instant::now();
                            }
                        }
                    }

                    if last_poll.elapsed() >= Duration::from_millis(1000) {
                        let data = thermal::collect_all_telemetry(&cached_config);
                        let _ = telemetry_tx.try_send(data);
                        last_poll = Instant::now();
                    }

                    thread::sleep(Duration::from_millis(25));
                }
            })
            .expect("Failed to spawn background worker thread");

        let tab_arg = std::env::args()
            .position(|a| a == "--tab")
            .and_then(|idx| std::env::args().nth(idx + 1));
        let initial_tab = match tab_arg.as_deref() {
            Some("fans") | Some("fan") | Some("1") => Tab::FanControl,
            Some("monitoring") | Some("mon") | Some("2") => Tab::Monitoring,
            Some("power") | Some("scenarios") | Some("3") => Tab::PowerModes,
            Some("rgb") | Some("lighting") | Some("4") => Tab::KeyboardRgb,
            Some("settings") | Some("5") => Tab::SystemSettings,
            _ => {
                if std::env::args().any(|a| a == "--monitoring" || a == "-m" || a == "2") {
                    Tab::Monitoring
                } else if std::env::args().any(|a| a == "--power" || a == "-p" || a == "3") {
                    Tab::PowerModes
                } else if std::env::args().any(|a| a == "--rgb" || a == "-r" || a == "4") {
                    Tab::KeyboardRgb
                } else if std::env::args().any(|a| a == "--settings" || a == "-s" || a == "5") {
                    Tab::SystemSettings
                } else {
                    Tab::FanControl
                }
            }
        };

        Self {
            current_tab: initial_tab,
            config: cfg,
            theme_mode,
            cmd_tx,
            telemetry_rx,
            telemetry: TelemetryData::default(),
            history: VecDeque::with_capacity(150),
            monitoring_graph_mode: MonitoringGraphMode::Thermals,
            graph_paused: false,
            history_window_size: 60,
            peak_cpu_temp: 50.0,
            peak_gpu_temp: 45.0,
            peak_cpu_power: 15.0,
            peak_gpu_power: 8.0,
            turbine_cpu_angle: 0.0_f32,
            turbine_gpu_angle: 0.0_f32,
            rgb_selected_zone: 0,
            rgb_custom_r: 229,
            rgb_custom_g: 25,
            rgb_custom_b: 55,
            rgb_last_status: "Ready".into(),
            diag_status: None,
            delta_cpu_rpm: 0,
            delta_gpu_rpm: 0,
            textures: Some(textures),
        }
    }

    pub fn pal(&self) -> Palette {
        get_palette(self.theme_mode)
    }

    pub fn set_theme(&mut self, ctx: &egui::Context, mode: ThemeMode) {
        self.theme_mode = mode;
        self.config.theme = mode.as_str().to_string();
        save_config(&self.config);
        apply_visuals_for_palette(ctx, &self.pal());
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_nitro_dial(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        rpm: u32,
        max_rpm: u32,
        temp: f32,
        load: f32,
        angle: f32,
        dial_size: f32,
        accent: Color32,
        delta_rpm: i32,
    ) {
        let pal = self.pal();
        ui.vertical_centered(|ui| {
            let temp_color = if temp >= 85.0 { pal.primary } else if temp >= 72.0 { pal.warning } else { pal.success };
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 190.0_f32).max(0.0_f32) / 2.0_f32);
                ui.label(RichText::new(title).size(13.0_f32).strong().color(accent));

                // Rounded temp pill badge with gentle tint
                let badge = Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(temp_color.r(), temp_color.g(), temp_color.b(), 26))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(temp_color.r(), temp_color.g(), temp_color.b(), 110)))
                    .rounding(Rounding::same(5.0_f32))
                    .inner_margin(Margin::symmetric(6.0_f32, 2.0_f32));

                let t_lbl = badge.show(ui, |ui| {
                    ui.label(RichText::new(format!("{:.0}°C", temp)).size(11.5_f32).strong().color(temp_color).monospace());
                }).response;

                let tjmax = if title.contains("CPU") { 100.0_f32 } else { 87.0_f32 };
                let headroom = (tjmax - temp).max(0.0_f32);
                t_lbl.on_hover_text(format!(
                    "{} Telemetría:\n• Temp actual: {:.1}°C\n• Margen térmico: +{:.1}°C antes de Throttling (Tope: {:.0}°C)",
                    title, temp, headroom, tjmax
                ));
                ui.label(RichText::new(format!("({:.0}%)", load)).size(11.0_f32).color(pal.text_muted));
            });
            ui.add_space(6.0_f32);

            let (rect, response) = ui.allocate_exact_size(Vec2::splat(dial_size), egui::Sense::hover());
            let trend_status = if delta_rpm > 35 { "Acelerando ▲" } else if delta_rpm < -35 { "Desacelerando ▼" } else { "Estable •" };
            response.on_hover_text(format!(
                "Dinámica de Turbina {}:\n• Velocidad: {} RPM (Máx: {} RPM)\n• Tendencia: {}\n• Sensor: {:.1}°C",
                title, rpm, max_rpm, trend_status, temp
            ));
            let painter = ui.painter_at(rect);
            let center = rect.center();
            let radius = (dial_size / 2.0_f32) - 4.0_f32;

            // 1. Soft Velvet Bezel & Housing
            painter.circle_filled(center, radius, pal.panel);
            painter.circle_stroke(center, radius, Stroke::new(1.5_f32, pal.border));

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
                    painter.line_segment([p1, p2], Stroke::new(2.2_f32, Color32::from_rgba_unmultiplied(220, 230, 245, 120)));
                }
            }

            // 3. Ambient Track Ring (Full 360 reference track)
            let arc_radius = radius * 0.94_f32;
            let segments = 40;
            for s in 0..segments {
                let a1 = (s as f32 / segments as f32) * std::f32::consts::TAU;
                let a2 = ((s + 1) as f32 / segments as f32) * std::f32::consts::TAU;
                let pt1 = center + Vec2::angled(a1) * arc_radius;
                let pt2 = center + Vec2::angled(a2) * arc_radius;
                painter.line_segment([pt1, pt2], Stroke::new(3.0_f32, Color32::from_rgba_unmultiplied(pal.border.r(), pal.border.g(), pal.border.b(), 90)));
            }

            // 4. Glowing Outer Progress Arc (Radial RPM Meter)
            let rpm_pct = (rpm as f32 / max_rpm as f32).clamp(0.0_f32, 1.0_f32);
            let start_ang = -std::f32::consts::FRAC_PI_2;
            let end_ang = start_ang + (rpm_pct * std::f32::consts::TAU);
            let active_segs = (36.0 * rpm_pct).ceil() as usize;
            for s in 0..active_segs {
                let a1 = start_ang + (s as f32 / 36.0_f32) * (rpm_pct * std::f32::consts::TAU);
                let a2 = start_ang + ((s + 1) as f32 / 36.0_f32) * (rpm_pct * std::f32::consts::TAU);
                if a1 < end_ang {
                    let pt1 = center + Vec2::angled(a1) * arc_radius;
                    let pt2 = center + Vec2::angled(a2.min(end_ang)) * arc_radius;
                    painter.line_segment([pt1, pt2], Stroke::new(3.2_f32, accent));
                }
            }

            // 5. Center Core Hub with Accent Ring (Refined breathing room)
            let hub_radius = radius * 0.44_f32;
            painter.circle_filled(center, hub_radius, pal.bg);
            painter.circle_stroke(center, hub_radius, Stroke::new(1.4_f32, Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 140)));

            // 6. High-Contrast, Ergonomic RPM Text
            painter.text(
                center - Vec2::new(0.0_f32, dial_size * 0.055_f32),
                egui::Align2::CENTER_CENTER,
                format!("{}", rpm),
                egui::FontId::proportional((dial_size * 0.135_f32).clamp(15.0_f32, 22.0_f32)),
                pal.text_primary,
            );

            // 7. Modern Trend Capsule Badge (Clean Micro-Pill)
            let (trend_str, trend_col) = if delta_rpm > 35 {
                (format!("▲ +{}", delta_rpm), pal.warning)
            } else if delta_rpm < -35 {
                (format!("▼ {}", delta_rpm), pal.secondary)
            } else {
                ("• ESTABLE".to_string(), pal.text_muted)
            };

            let badge_y = center.y + dial_size * 0.10_f32;
            let badge_rect = Rect::from_center_size(Pos2::new(center.x, badge_y), Vec2::new(dial_size * 0.40_f32, dial_size * 0.10_f32));
            painter.rect_filled(badge_rect, Rounding::same(badge_rect.height() / 2.0), Color32::from_rgba_unmultiplied(trend_col.r(), trend_col.g(), trend_col.b(), 24));
            painter.rect_stroke(badge_rect, Rounding::same(badge_rect.height() / 2.0), Stroke::new(0.8_f32, Color32::from_rgba_unmultiplied(trend_col.r(), trend_col.g(), trend_col.b(), 90)));

            painter.text(
                Pos2::new(center.x, badge_y),
                egui::Align2::CENTER_CENTER,
                trend_str,
                egui::FontId::monospace((dial_size * 0.054_f32).clamp(7.5_f32, 9.5_f32)),
                trend_col,
            );
        });
    }

    fn nitro_card_frame(&self) -> Frame {
        let pal = self.pal();
        Frame::none()
            .fill(pal.card)
            .stroke(Stroke::new(1.0_f32, pal.border))
            .rounding(Rounding::same(pal.card_rounding))
            .inner_margin(Margin::same(14.0_f32))
    }

    /// Real-time live oscilloscope drawing component with multi-series shaded graphing
    fn render_telemetry_chart(&self, ui: &mut egui::Ui, mode: MonitoringGraphMode, height: f32) {
        let pal = self.pal();
        let avail_w = ui.available_width();
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(avail_w, height), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // 1. Technical oscilloscope background
        painter.rect_filled(rect, Rounding::same(8.0_f32), pal.bg);
        painter.rect_stroke(
            rect,
            Rounding::same(8.0_f32),
            Stroke::new(1.0_f32, pal.border),
        );

        // Increased left margin (66px) so Y-axis labels like "6000 RPM" never truncate
        let chart_left = rect.left() + 66.0_f32;
        let chart_right = rect.right() - 16.0_f32;
        let chart_top = rect.top() + 14.0_f32;
        let chart_bottom = rect.bottom() - 20.0_f32;
        let chart_w = (chart_right - chart_left).max(10.0_f32);
        let chart_h = (chart_bottom - chart_top).max(10.0_f32);

        let (val_min, val_max, steps, unit) = match mode {
            MonitoringGraphMode::Thermals => (20.0_f32, 100.0_f32, 4, "°C"),
            MonitoringGraphMode::Workload => (0.0_f32, 100.0_f32, 4, "%"),
            MonitoringGraphMode::Power => (0.0_f32, 80.0_f32, 4, " W"),
            MonitoringGraphMode::Turbines => (0.0_f32, 6000.0_f32, 3, " RPM"),
        };

        // 2. Horizontal reference grid lines
        for i in 0..=steps {
            let frac = i as f32 / steps as f32;
            let y = chart_bottom - (frac * chart_h);
            let val = val_min + (frac * (val_max - val_min));

            painter.line_segment(
                [Pos2::new(chart_left, y), Pos2::new(chart_right, y)],
                Stroke::new(
                    1.0_f32,
                    Color32::from_rgba_unmultiplied(
                        pal.border.r(),
                        pal.border.g(),
                        pal.border.b(),
                        70,
                    ),
                ),
            );

            painter.text(
                Pos2::new(chart_left - 8.0_f32, y),
                egui::Align2::RIGHT_CENTER,
                format!("{:.0}{}", val, unit),
                egui::FontId::monospace(9.5_f32),
                pal.text_muted,
            );
        }

        // 3. Danger Threshold line for thermals (85°C)
        if mode == MonitoringGraphMode::Thermals {
            let alert_y = chart_bottom - ((85.0_f32 - val_min) / (val_max - val_min) * chart_h);
            painter.line_segment(
                [
                    Pos2::new(chart_left, alert_y),
                    Pos2::new(chart_right, alert_y),
                ],
                Stroke::new(
                    1.2_f32,
                    Color32::from_rgba_unmultiplied(
                        pal.primary.r(),
                        pal.primary.g(),
                        pal.primary.b(),
                        140,
                    ),
                ),
            );
            painter.text(
                Pos2::new(chart_right - 6.0_f32, alert_y - 6.0_f32),
                egui::Align2::RIGHT_BOTTOM,
                "LÍMITE TÉRMICO (85°C)",
                egui::FontId::monospace(8.5_f32),
                Color32::from_rgba_unmultiplied(
                    pal.primary.r(),
                    pal.primary.g(),
                    pal.primary.b(),
                    180,
                ),
            );
        }

        // 4. Polylines with shaded areas
        let window_len = self.history_window_size.min(self.history.len());
        if window_len >= 2 {
            let slice_start = self.history.len() - window_len;
            let history_slice: Vec<_> = self.history.iter().skip(slice_start).cloned().collect();
            let n = history_slice.len();

            let get_pos = |idx: usize, val: f32| -> Pos2 {
                let x_frac = idx as f32 / (n - 1) as f32;
                let y_frac = ((val - val_min) / (val_max - val_min)).clamp(0.0_f32, 1.0_f32);
                Pos2::new(
                    chart_left + (x_frac * chart_w),
                    chart_bottom - (y_frac * chart_h),
                )
            };

            match mode {
                MonitoringGraphMode::Thermals => {
                    let pts_cpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].cpu_temp))
                        .collect();
                    let pts_gpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].gpu_temp))
                        .collect();
                    let pts_nvme: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].nvme_temp))
                        .collect();
                    draw_shaded_curve(&painter, &pts_cpu, chart_bottom, pal.secondary, 2.0_f32);
                    draw_shaded_curve(&painter, &pts_gpu, chart_bottom, pal.primary, 2.0_f32);
                    draw_shaded_curve(&painter, &pts_nvme, chart_bottom, pal.warning, 1.4_f32);
                }
                MonitoringGraphMode::Workload => {
                    let pts_cpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].cpu_load))
                        .collect();
                    let pts_gpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].gpu_load))
                        .collect();
                    draw_shaded_curve(&painter, &pts_cpu, chart_bottom, pal.secondary, 2.0_f32);
                    draw_shaded_curve(&painter, &pts_gpu, chart_bottom, pal.primary, 2.0_f32);
                }
                MonitoringGraphMode::Power => {
                    let pts_cpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].cpu_power))
                        .collect();
                    let pts_gpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].gpu_power))
                        .collect();
                    draw_shaded_curve(&painter, &pts_cpu, chart_bottom, pal.secondary, 2.0_f32);
                    draw_shaded_curve(&painter, &pts_gpu, chart_bottom, pal.primary, 2.0_f32);
                }
                MonitoringGraphMode::Turbines => {
                    let pts_cpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].cpu_rpm as f32))
                        .collect();
                    let pts_gpu: Vec<Pos2> = (0..n)
                        .map(|i| get_pos(i, history_slice[i].gpu_rpm as f32))
                        .collect();
                    draw_shaded_curve(&painter, &pts_cpu, chart_bottom, pal.secondary, 2.0_f32);
                    draw_shaded_curve(&painter, &pts_gpu, chart_bottom, pal.primary, 2.0_f32);
                }
            }
        }

        // 5. Hover inspection vertical reticle
        if let Some(mouse_pos) = response.hover_pos() {
            if mouse_pos.x >= chart_left
                && mouse_pos.x <= chart_right
                && mouse_pos.y >= chart_top
                && mouse_pos.y <= chart_bottom
            {
                painter.line_segment(
                    [
                        Pos2::new(mouse_pos.x, chart_top),
                        Pos2::new(mouse_pos.x, chart_bottom),
                    ],
                    Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(240, 244, 248, 80)),
                );
            }
        }
    }

    /// Modernized tactical telemetry matrix for the bottom card (guaranteed 2x2 grid when compact)
    fn render_tactical_telemetry(&self, ui: &mut egui::Ui) {
        let avail_w = ui.available_width();
        let is_wide = avail_w >= 780.0_f32;

        if is_wide {
            ui.columns(4, |columns| {
                self.draw_tile_cpu(&mut columns[0]);
                self.draw_tile_gpu(&mut columns[1]);
                self.draw_tile_memory(&mut columns[2]);
                self.draw_tile_power(&mut columns[3]);
            });
        } else {
            ui.columns(2, |columns| {
                self.draw_tile_cpu(&mut columns[0]);
                self.draw_tile_gpu(&mut columns[1]);
            });
            ui.add_space(8.0_f32);
            ui.columns(2, |columns| {
                self.draw_tile_memory(&mut columns[0]);
                self.draw_tile_power(&mut columns[1]);
            });
        }
    }

    fn draw_tile_cpu(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        ui.vertical(|ui| {
            let temp_c = if self.telemetry.cpu_temp >= 85.0 { pal.primary } else if self.telemetry.cpu_temp >= 75.0 { pal.warning } else { pal.secondary };
            ui.label(RichText::new("CPU PACKAGE").size(10.5_f32).color(pal.text_muted).strong());
            ui.horizontal(|ui| {
                let t_lbl = ui.label(RichText::new(format!("{:.1}°C", self.telemetry.cpu_temp)).size(16.0_f32).color(temp_c).strong());
                let headroom = (100.0_f32 - self.telemetry.cpu_temp).max(0.0_f32);
                t_lbl.on_hover_text(format!(
                    "Intel Core i5-10300H Telemetry:\n• Thermal Headroom: +{:.1}°C before TjMax (100.0°C)\n• Active Governor: {}\n• EPP: {}\n• Turbo Boost: {}",
                    headroom,
                    self.telemetry.cpu_governor,
                    self.telemetry.cpu_epp,
                    if self.telemetry.cpu_turbo { "Active" } else { "Disabled" }
                ));
                ui.label(RichText::new(format!("({:.0}%)", self.telemetry.cpu_load)).size(12.0_f32).color(pal.text_muted));
            });
            draw_tactical_bar(ui, self.telemetry.cpu_load, temp_c, 5.0_f32);
            ui.add_space(2.0_f32);
            ui.label(RichText::new(format!("{} MHz • {:.1}W", self.telemetry.cpu_clock, self.telemetry.cpu_power)).size(10.5_f32).color(pal.text_muted).monospace());
        });
    }

    fn draw_tile_gpu(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        ui.vertical(|ui| {
            ui.label(RichText::new("DISCRETE GPU").size(10.5_f32).color(pal.text_muted).strong());
            if self.telemetry.gpu_active {
                let temp_c = if self.telemetry.gpu_temp >= 80.0 { pal.primary } else { pal.secondary };
                ui.horizontal(|ui| {
                    let t_lbl = ui.label(RichText::new(format!("{:.1}°C", self.telemetry.gpu_temp)).size(16.0_f32).color(temp_c).strong());
                    let headroom = (87.0_f32 - self.telemetry.gpu_temp).max(0.0_f32);
                    t_lbl.on_hover_text(format!(
                        "NVIDIA RTX 3050 Telemetry:\n• Thermal Headroom: +{:.1}°C before Throttle (87.0°C)\n• Driver: {}\n• VRAM: {:.2} / {:.1} GB",
                        headroom,
                        self.telemetry.driver_version,
                        self.telemetry.gpu_vram_used,
                        self.telemetry.gpu_vram_total
                    ));
                    ui.label(RichText::new(format!("({:.0}%)", self.telemetry.gpu_load)).size(12.0_f32).color(pal.text_muted));
                });
                draw_tactical_bar(ui, self.telemetry.gpu_load, pal.primary, 5.0_f32);
                ui.add_space(2.0_f32);
                ui.label(RichText::new(format!("{} MHz • {:.1}W • VRAM {:.1}G", self.telemetry.gpu_clock, self.telemetry.gpu_power, self.telemetry.gpu_vram_used)).size(10.5_f32).color(pal.text_muted).monospace());
            } else {
                ui.label(RichText::new("D3cold Standby").size(16.0_f32).color(pal.text_muted).strong());
                draw_tactical_bar(ui, 0.0_f32, pal.border, 5.0_f32);
                ui.add_space(2.0_f32);
                ui.label(RichText::new("0.0 W • PCIe Sleep").size(10.5_f32).color(pal.text_muted).monospace());
            }
        });
    }

    fn draw_tile_memory(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        ui.vertical(|ui| {
            ui.label(
                RichText::new("RAM & NVMe")
                    .size(10.5_f32)
                    .color(pal.text_muted)
                    .strong(),
            );
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{:.1} GB", self.telemetry.ram_used))
                        .size(16.0_f32)
                        .color(pal.text_primary)
                        .strong(),
                );
                ui.label(
                    RichText::new(format!("({:.0}%)", self.telemetry.ram_pct))
                        .size(12.0_f32)
                        .color(pal.text_muted),
                );
            });
            draw_tactical_bar(ui, self.telemetry.ram_pct, pal.success, 5.0_f32);
            ui.add_space(2.0_f32);
            ui.label(
                RichText::new(format!(
                    "NVMe: {:.1}°C • H: {}%",
                    self.telemetry.nvme_temp, self.telemetry.nvme_health
                ))
                .size(10.5_f32)
                .color(pal.text_muted)
                .monospace(),
            );
        });
    }

    fn draw_tile_power(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        ui.vertical(|ui| {
            ui.label(
                RichText::new("POWER / BATTERY")
                    .size(10.5_f32)
                    .color(pal.text_muted)
                    .strong(),
            );
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{}%", self.telemetry.bat_pct))
                        .size(16.0_f32)
                        .color(pal.text_primary)
                        .strong(),
                );
                let (pwr_badge, pwr_col) = if self.telemetry.ac_connected {
                    ("⚡ AC", pal.success)
                } else {
                    ("🔋 BAT", pal.warning)
                };
                ui.label(
                    RichText::new(pwr_badge)
                        .size(12.0_f32)
                        .color(pwr_col)
                        .strong(),
                );
            });
            draw_tactical_bar(
                ui,
                self.telemetry.bat_pct as f32,
                if self.telemetry.ac_connected {
                    pal.success
                } else {
                    pal.warning
                },
                5.0_f32,
            );
            ui.add_space(2.0_f32);
            ui.label(
                RichText::new(format!(
                    "{:.2}V • H: {:.0}%",
                    self.telemetry.bat_voltage, self.telemetry.bat_health
                ))
                .size(10.5_f32)
                .color(pal.text_muted)
                .monospace(),
            );
        });
    }

    /// Renders the Dedicated Fan Control Tab (Tab::FanControl)
    fn render_fans_tab(&mut self, ui: &mut egui::Ui) {
        let pal = self.pal();
        let avail_w = ui.available_width();
        let is_auto = self.config.mode == "auto";
        let is_max = self.config.mode == "max" || self.config.mode == "turbo";
        let is_custom = self.config.mode == "custom";

        // --- MAIN CARD: FAN SPEED CONTROL & MODE SELECTION ---
        self.nitro_card_frame().show(ui, |ui| {
            // 1. Header Row: Title, Mode Pill, Fan Sync, and CoolBoost Toggle
            ui.horizontal(|ui| {
                ui.heading(RichText::new("Fan Speed Control").size(16.0_f32).strong().color(pal.text_primary));
                ui.add_space(8.0_f32);

                // Operational Mode Badge / Pill
                let (mode_text, mode_color, mode_desc) = if is_auto {
                    ("• BIOS CLOSED LOOP", pal.secondary, "Compal EC autonomous closed loop. Fans dynamically ramp according to thermal curves.")
                } else if is_max {
                    ("• TURBO SMM OVERDRIVE", pal.primary, "Overdrive active. Compal EC registers locked to 0xFF (Max 12V fan rail voltage).")
                } else {
                    ("• MANUAL PWM TARGET", pal.warning, "Manual PWM duty cycle override. Autonomous closed loop slipped to target duty.")
                };

                let badge = egui::Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(mode_color.r(), mode_color.g(), mode_color.b(), 25))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(mode_color.r(), mode_color.g(), mode_color.b(), 120)))
                    .rounding(Rounding::same(4.0_f32))
                    .inner_margin(Margin::symmetric(6.0_f32, 2.0_f32));

                badge.show(ui, |ui| {
                    ui.label(RichText::new(mode_text).size(10.0_f32).strong().color(mode_color));
                }).response.on_hover_text(mode_desc);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // CoolBoost Pill Toggle
                    let cb = self.config.coolboost;
                    let cb_text = if cb { "❄ CoolBoost™ ON" } else { "❄ CoolBoost™ OFF" };
                    let cb_color = if cb { pal.success } else { pal.text_muted };
                    let cb_bg = if cb { Color32::from_rgba_unmultiplied(pal.success.r(), pal.success.g(), pal.success.b(), 24) } else { pal.panel };
                    let cb_border = if cb { pal.success } else { pal.border };

                    let cb_btn = egui::Button::new(RichText::new(cb_text).size(11.0_f32).strong().color(cb_color))
                        .fill(cb_bg)
                        .stroke(Stroke::new(1.0_f32, cb_border))
                        .rounding(Rounding::same(4.0_f32));

                    if ui.add(cb_btn).on_hover_text(
                        "Compal EC CoolBoost offset: elevates dynamic fan curve baseline +400-500 RPM for lower thermal hysteresis under transient load spikes. Click to toggle."
                    ).clicked() {
                        self.config.coolboost = !cb;
                        let _ = self.cmd_tx.send(HardwareCommand::SetCoolboost(self.config.coolboost));
                        save_config(&self.config);
                    }

                    ui.add_space(10.0_f32);

                    // Fan Sync Toggle (Tactical Link / Decouple)
                    let sync = self.config.sync_fans;
                    let sync_text = if sync { "🔗 Fans Linked" } else { "⚡ Independent" };
                    let sync_color = if sync { pal.secondary } else { pal.text_muted };
                    let sync_bg = if sync { Color32::from_rgba_unmultiplied(pal.secondary.r(), pal.secondary.g(), pal.secondary.b(), 24) } else { pal.panel };
                    let sync_border = if sync { pal.secondary } else { pal.border };

                    let sync_btn = egui::Button::new(RichText::new(sync_text).size(11.0_f32).strong().color(sync_color))
                        .fill(sync_bg)
                        .stroke(Stroke::new(1.0_f32, sync_border))
                        .rounding(Rounding::same(4.0_f32));

                    if ui.add(sync_btn).on_hover_text(if sync {
                        "Fans are LINKED: CPU and GPU target speeds move synchronously. Click to decouple."
                    } else {
                        "Fans are INDEPENDENT: CPU and GPU target speeds are adjusted separately. Click to link."
                    }).clicked() {
                        self.config.sync_fans = !sync;
                        if self.config.sync_fans {
                            self.config.gpu_fan_target = self.config.cpu_fan_target;
                            let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans {
                                cpu_pct: self.config.cpu_fan_target,
                                gpu_pct: self.config.gpu_fan_target,
                            });
                        }
                        save_config(&self.config);
                    }
                });
            });

            ui.add_space(4.0_f32);
            ui.separator();
            ui.add_space(8.0_f32);

            // 2. High-Visibility Mode Selectors (Auto, Max, Custom)
            let btn_w = ((ui.available_width() - 16.0_f32) / 3.0_f32).max(85.0_f32);

            let btn_auto = egui::Button::new(
                RichText::new(if avail_w < 520.0 { "AUTO (A)" } else { "AUTO (A)\nBIOS Closed Loop" })
                    .size(11.5_f32)
                    .strong()
                    .color(if is_auto { pal.secondary } else { pal.text_muted })
            )
            .fill(if is_auto { Color32::from_rgba_unmultiplied(pal.secondary.r(), pal.secondary.g(), pal.secondary.b(), 28) } else { pal.panel })
            .stroke(Stroke::new(if is_auto { 1.6_f32 } else { 1.0_f32 }, if is_auto { pal.secondary } else { pal.border }))
            .rounding(Rounding::same(6.0_f32));

            let btn_max = egui::Button::new(
                RichText::new(if avail_w < 520.0 { "MAX (M)" } else { "MAX (M)\nTurbo 100% SMM" })
                    .size(11.5_f32)
                    .strong()
                    .color(if is_max { pal.primary } else { pal.text_muted })
            )
            .fill(if is_max { Color32::from_rgba_unmultiplied(pal.primary.r(), pal.primary.g(), pal.primary.b(), 28) } else { pal.panel })
            .stroke(Stroke::new(if is_max { 1.6_f32 } else { 1.0_f32 }, if is_max { pal.primary } else { pal.border }))
            .rounding(Rounding::same(6.0_f32));

            let btn_custom = egui::Button::new(
                RichText::new(if avail_w < 520.0 { "CUSTOM (C)" } else { "CUSTOM (C)\nManual Target" })
                    .size(11.5_f32)
                    .strong()
                    .color(if is_custom { pal.warning } else { pal.text_muted })
            )
            .fill(if is_custom { Color32::from_rgba_unmultiplied(pal.warning.r(), pal.warning.g(), pal.warning.b(), 28) } else { pal.panel })
            .stroke(Stroke::new(if is_custom { 1.6_f32 } else { 1.0_f32 }, if is_custom { pal.warning } else { pal.border }))
            .rounding(Rounding::same(6.0_f32));

            ui.horizontal(|ui| {
                if ui.add_sized([btn_w, 40.0_f32], btn_auto).clicked() {
                    self.config.mode = "auto".into();
                    let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("auto".into()));
                }
                if ui.add_sized([btn_w, 40.0_f32], btn_max).clicked() {
                    self.config.mode = "max".into();
                    let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("max".into()));
                }
                if ui.add_sized([btn_w, 40.0_f32], btn_custom).clicked() {
                    self.config.mode = "custom".into();
                    let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("custom".into()));
                }
            });

            ui.add_space(10.0_f32);

            // 3. Tactical Speed Presets (Quick 1-click duty overrides)
            ui.horizontal(|ui| {
                ui.label(RichText::new("Presets:").size(11.0_f32).strong().color(pal.text_muted));
                let presets: [(&str, u8, &str); 4] = [
                    ("20% Quiet", 20, "20% Target (~2100 RPM) • Low acoustic footprint"),
                    ("45% Balanced", 45, "45% Target (~3200 RPM) • Daily browsing & dev"),
                    ("75% High Flow", 75, "75% Target (~4500 RPM) • Sustained compile & gaming"),
                    ("100% Max", 100, "100% Target (Max RPM) • Thermal throttling prevention"),
                ];

                for (label, val, desc) in presets {
                    let is_active_preset = is_custom && self.config.cpu_fan_target == val && self.config.gpu_fan_target == val;
                    let p_btn = egui::Button::new(
                        RichText::new(label)
                            .size(10.5_f32)
                            .strong()
                            .color(if is_active_preset { pal.warning } else { pal.text_primary })
                    )
                    .fill(if is_active_preset { Color32::from_rgba_unmultiplied(pal.warning.r(), pal.warning.g(), pal.warning.b(), 32) } else { pal.card })
                    .stroke(Stroke::new(1.0_f32, if is_active_preset { pal.warning } else { pal.border }))
                    .rounding(Rounding::same(4.0_f32));

                    let res = ui.add(p_btn);
                    if res.clicked() {
                        self.config.mode = "custom".into();
                        self.config.cpu_fan_target = val;
                        self.config.gpu_fan_target = val;
                        let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("custom".into()));
                        let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: val, gpu_pct: val });
                        save_config(&self.config);
                    }
                    res.on_hover_text(desc);
                }
            });

            ui.add_space(12.0_f32);

            // 4. Dials and Sliders Area
            let dial_sz = if avail_w < 580.0_f32 {
                ((ui.available_width() - 20.0_f32) / 2.0_f32).clamp(115.0_f32, 140.0_f32)
            } else {
                ((ui.available_width() - 30.0_f32) / 2.0_f32 * 0.72_f32).clamp(120.0_f32, 148.0_f32)
            };

            let col_w = (ui.available_width() - 16.0_f32) / 2.0_f32;

            ui.horizontal(|ui| {
                // CPU FAN COLUMN
                ui.vertical(|ui| {
                    ui.set_width(col_w);
                    self.draw_nitro_dial(
                        ui,
                        "CPU FAN",
                        self.telemetry.cpu_rpm,
                        5660,
                        self.telemetry.cpu_temp,
                        self.telemetry.cpu_load,
                        self.turbine_cpu_angle,
                        dial_sz,
                        pal.secondary,
                        self.delta_cpu_rpm,
                    );

                    let cpu_duty = (self.telemetry.cpu_rpm as f32 / 5660.0_f32 * 100.0_f32).clamp(0.0, 100.0);
                    let cpu_pwm = (cpu_duty * 2.55_f32) as u8;
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(format!("Duty: {:.0}% • PWM: 0x{:02X}", cpu_duty, cpu_pwm)).size(10.5_f32).color(pal.text_muted).monospace());
                    });
                    ui.add_space(4.0_f32);

                    if is_custom {
                        ui.horizontal(|ui| {
                            let step_minus = egui::Button::new(RichText::new(" - ").size(11.0_f32).strong().color(pal.secondary))
                                .fill(pal.card)
                                .stroke(Stroke::new(1.0_f32, pal.border))
                                .rounding(Rounding::same(4.0_f32));
                            if ui.add(step_minus).on_hover_text("Reducir -5%").clicked() {
                                self.config.cpu_fan_target = self.config.cpu_fan_target.saturating_sub(5);
                                if self.config.sync_fans { self.config.gpu_fan_target = self.config.cpu_fan_target; }
                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                save_config(&self.config);
                            }
                            let mut c_tgt = self.config.cpu_fan_target;
                            if ui.add(egui::Slider::new(&mut c_tgt, 0..=100).show_value(false)).changed() {
                                self.config.cpu_fan_target = c_tgt;
                                if self.config.sync_fans { self.config.gpu_fan_target = c_tgt; }
                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: c_tgt, gpu_pct: self.config.gpu_fan_target });
                                save_config(&self.config);
                            }
                            let step_plus = egui::Button::new(RichText::new(" + ").size(11.0_f32).strong().color(pal.secondary))
                                .fill(pal.card)
                                .stroke(Stroke::new(1.0_f32, pal.border))
                                .rounding(Rounding::same(4.0_f32));
                            if ui.add(step_plus).on_hover_text("Aumentar +5%").clicked() {
                                self.config.cpu_fan_target = (self.config.cpu_fan_target + 5).min(100);
                                if self.config.sync_fans { self.config.gpu_fan_target = self.config.cpu_fan_target; }
                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                save_config(&self.config);
                            }
                            ui.label(RichText::new(format!("{}%", self.config.cpu_fan_target)).color(pal.secondary).strong());
                        });
                    } else {
                        let (badge_txt, badge_col, badge_hover) = if is_max {
                            ("⚡ Overdrive Turbo (100% SMM)", pal.primary, "SMM Turbo activo forzando raíles a máxima potencia de disipación.")
                        } else {
                            ("🔒 Curva Térmica BIOS Autónoma", pal.secondary, "Compal EC gestiona dinámicamente el ciclo PWM según la curva térmica.")
                        };
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(badge_col.r(), badge_col.g(), badge_col.b(), 18))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(badge_col.r(), badge_col.g(), badge_col.b(), 70)))
                            .rounding(Rounding::same(5.0_f32))
                            .inner_margin(Margin::symmetric(8.0_f32, 4.0_f32))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new(badge_txt).size(10.5_f32).strong().color(badge_col));
                                });
                            }).response.on_hover_text(badge_hover);
                    }
                });

                // GPU FAN COLUMN
                ui.vertical(|ui| {
                    ui.set_width(col_w);
                    self.draw_nitro_dial(
                        ui,
                        "GPU FAN",
                        self.telemetry.gpu_rpm,
                        6000,
                        self.telemetry.gpu_temp,
                        self.telemetry.gpu_load,
                        self.turbine_gpu_angle,
                        dial_sz,
                        pal.primary,
                        self.delta_gpu_rpm,
                    );

                    let gpu_duty = (self.telemetry.gpu_rpm as f32 / 6000.0_f32 * 100.0_f32).clamp(0.0, 100.0);
                    let gpu_pwm = (gpu_duty * 2.55_f32) as u8;
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(format!("Duty: {:.0}% • PWM: 0x{:02X}", gpu_duty, gpu_pwm)).size(10.5_f32).color(pal.text_muted).monospace());
                    });
                    ui.add_space(4.0_f32);

                    if is_custom {
                        ui.horizontal(|ui| {
                            let step_minus = egui::Button::new(RichText::new(" - ").size(11.0_f32).strong().color(pal.primary))
                                .fill(pal.card)
                                .stroke(Stroke::new(1.0_f32, pal.border))
                                .rounding(Rounding::same(4.0_f32));
                            if ui.add(step_minus).on_hover_text("Reducir -5%").clicked() {
                                self.config.gpu_fan_target = self.config.gpu_fan_target.saturating_sub(5);
                                if self.config.sync_fans { self.config.cpu_fan_target = self.config.gpu_fan_target; }
                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                save_config(&self.config);
                            }
                            let mut g_tgt = self.config.gpu_fan_target;
                            if ui.add(egui::Slider::new(&mut g_tgt, 0..=100).show_value(false)).changed() {
                                self.config.gpu_fan_target = g_tgt;
                                if self.config.sync_fans { self.config.cpu_fan_target = g_tgt; }
                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: g_tgt });
                                save_config(&self.config);
                            }
                            let step_plus = egui::Button::new(RichText::new(" + ").size(11.0_f32).strong().color(pal.primary))
                                .fill(pal.card)
                                .stroke(Stroke::new(1.0_f32, pal.border))
                                .rounding(Rounding::same(4.0_f32));
                            if ui.add(step_plus).on_hover_text("Aumentar +5%").clicked() {
                                self.config.gpu_fan_target = (self.config.gpu_fan_target + 5).min(100);
                                if self.config.sync_fans { self.config.cpu_fan_target = self.config.gpu_fan_target; }
                                let _ = self.cmd_tx.send(HardwareCommand::SetCustomFans { cpu_pct: self.config.cpu_fan_target, gpu_pct: self.config.gpu_fan_target });
                                save_config(&self.config);
                            }
                            ui.label(RichText::new(format!("{}%", self.config.gpu_fan_target)).color(pal.primary).strong());
                        });
                    } else {
                        let (badge_txt, badge_col, badge_hover) = if is_max {
                            ("⚡ Overdrive Turbo (100% SMM)", pal.primary, "SMM Turbo activo forzando raíles a máxima potencia de disipación.")
                        } else {
                            ("🔒 Curva Térmica BIOS Autónoma", pal.secondary, "Compal EC gestiona dinámicamente el ciclo PWM según la curva térmica.")
                        };
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(badge_col.r(), badge_col.g(), badge_col.b(), 18))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(badge_col.r(), badge_col.g(), badge_col.b(), 70)))
                            .rounding(Rounding::same(5.0_f32))
                            .inner_margin(Margin::symmetric(8.0_f32, 4.0_f32))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new(badge_txt).size(10.5_f32).strong().color(badge_col));
                                });
                            }).response.on_hover_text(badge_hover);
                    }
                });
            });

            ui.add_space(10.0_f32);

            // 5. Aerodynamic Acoustic & Airflow Estimate Strip
            let max_rpm = self.telemetry.cpu_rpm.max(self.telemetry.gpu_rpm);
            let (noise_dba, noise_desc, noise_color) = if max_rpm == 0 {
                (18, "0 RPM • Passive Convection / Pure Silence", pal.success)
            } else if max_rpm < 2200 {
                (24, "Whisper Flow • Near Inaudible / Office & Dev", pal.success)
            } else if max_rpm < 3400 {
                (33, "Gentle Draft • Mild Ambient Whir", pal.secondary)
            } else if max_rpm < 4500 {
                (43, "Forced Induction • Audible Balanced Gaming", pal.warning)
            } else if max_rpm < 5400 {
                (51, "High Turbine Flow • Heavy Thermal Dissipation", pal.primary)
            } else {
                (56, "Max Jet Induction • Full 12V Turbo Overdrive", pal.primary)
            };

            let approx_cfm = ((self.telemetry.cpu_rpm + self.telemetry.gpu_rpm) as f32 * 0.0031_f32).clamp(0.0_f32, 36.0_f32);

            egui::Frame::none()
                .fill(pal.panel)
                .stroke(Stroke::new(1.0_f32, pal.border))
                .rounding(Rounding::same(6.0_f32))
                .inner_margin(Margin::symmetric(10.0_f32, 6.0_f32))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Acoustic Footprint:").size(11.0_f32).color(pal.text_primary).strong());
                        ui.label(RichText::new(format!("~{} dBA", noise_dba)).size(11.5_f32).strong().color(noise_color));
                        ui.label(RichText::new(format!("• {}", noise_desc)).size(11.0_f32).color(pal.text_muted));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("Est. Airflow: {:.1} CFM", approx_cfm)).size(11.0_f32).color(pal.secondary).monospace());
                        });
                    });
                });
        });

        ui.add_space(10.0_f32);

        // --- EMBEDDED REAL-TIME VENTILATION RESPONSE OSCILLOSCOPE ---
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Ventilation Dynamics Curve")
                        .size(13.5_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                let avail = ui.available_width();
                if avail > 340.0_f32 {
                    ui.label(
                        RichText::new("• Dual-Turbine Response (60s)")
                            .size(11.0_f32)
                            .color(pal.text_muted),
                    );
                } else if avail > 220.0_f32 {
                    ui.label(RichText::new("(60s)").size(11.0_f32).color(pal.text_muted));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "CPU: {} RPM | GPU: {} RPM",
                            self.telemetry.cpu_rpm, self.telemetry.gpu_rpm
                        ))
                        .size(11.0_f32)
                        .color(pal.text_secondary)
                        .monospace(),
                    );
                });
            });
            ui.add_space(4.0_f32);
            ui.separator();
            ui.add_space(4.0_f32);
            self.render_telemetry_chart(ui, MonitoringGraphMode::Turbines, 115.0_f32);
        });

        ui.add_space(10.0_f32);

        // --- BOTTOM ROW: TACTICAL TELEMETRY CARD ---
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("Live System Telemetry Matrix")
                        .size(15.0_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(RichText::new("PURGE TURBO").color(pal.primary).strong())
                        .clicked()
                    {
                        self.config.mode = "max".into();
                        let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("max".into()));
                    }
                    if ui
                        .button(RichText::new("RESET AUTO").color(pal.secondary).strong())
                        .clicked()
                    {
                        self.config.mode = "auto".into();
                        let _ = self.cmd_tx.send(HardwareCommand::SetFanMode("auto".into()));
                    }
                });
            });
            ui.add_space(4.0_f32);
            ui.separator();
            ui.add_space(6.0_f32);
            self.render_tactical_telemetry(ui);
        });
    }

    /// Renders the Comprehensive Hardware Observatory tab (Tab::Monitoring)
    fn render_monitoring_tab(&mut self, ui: &mut egui::Ui) {
        let pal = self.pal();
        let avail_w = ui.available_width();

        // 1. TOP KPI SUMMARY RIBBON (4 Executive Metrics)
        let total_w = self.telemetry.cpu_power
            + if self.telemetry.gpu_active {
                self.telemetry.gpu_power
            } else {
                0.0_f32
            };
        let thermal_headroom =
            (100.0_f32 - self.telemetry.cpu_temp.max(self.telemetry.gpu_temp)).max(0.0_f32);
        let cpu_duty = (self.telemetry.cpu_rpm as f32 / 5660.0_f32 * 100.0_f32).clamp(0.0, 100.0);
        let gpu_duty = (self.telemetry.gpu_rpm as f32 / 6000.0_f32 * 100.0_f32).clamp(0.0, 100.0);

        let render_kpi = |ui: &mut egui::Ui, idx: usize| match idx {
            0 => {
                self.nitro_card_frame().show(ui, |ui| {
                    ui.label(
                        RichText::new("THERMAL HEADROOM")
                            .size(9.5_f32)
                            .strong()
                            .color(pal.text_muted),
                    );
                    let h_col = if thermal_headroom < 12.0 {
                        pal.primary
                    } else if thermal_headroom < 20.0 {
                        pal.warning
                    } else {
                        pal.success
                    };
                    ui.label(
                        RichText::new(format!("+{:.1} °C", thermal_headroom))
                            .size(18.0_f32)
                            .strong()
                            .color(h_col),
                    );
                    ui.label(
                        RichText::new(format!(
                            "Peak: {:.0}°C CPU / {:.0}°C GPU",
                            self.peak_cpu_temp, self.peak_gpu_temp
                        ))
                        .size(9.5_f32)
                        .color(pal.text_muted),
                    );
                });
            }
            1 => {
                self.nitro_card_frame().show(ui, |ui| {
                    ui.label(
                        RichText::new("TOTAL POWER DRAW")
                            .size(9.5_f32)
                            .strong()
                            .color(pal.text_muted),
                    );
                    ui.label(
                        RichText::new(format!("{:.1} W", total_w))
                            .size(18.0_f32)
                            .strong()
                            .color(pal.secondary),
                    );
                    ui.label(
                        RichText::new(format!(
                            "Peak: {:.1} W / 135W AC",
                            self.peak_cpu_power + self.peak_gpu_power
                        ))
                        .size(9.5_f32)
                        .color(pal.text_muted),
                    );
                });
            }
            2 => {
                self.nitro_card_frame().show(ui, |ui| {
                    ui.label(
                        RichText::new("DUAL TURBINES")
                            .size(9.5_f32)
                            .strong()
                            .color(pal.text_muted),
                    );
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{}", self.telemetry.cpu_rpm))
                                .size(16.0_f32)
                                .strong()
                                .color(pal.secondary),
                        );
                        ui.label(RichText::new("/").size(13.0_f32).color(pal.text_muted));
                        ui.label(
                            RichText::new(format!("{}", self.telemetry.gpu_rpm))
                                .size(16.0_f32)
                                .strong()
                                .color(pal.primary),
                        );
                        ui.label(RichText::new("RPM").size(11.0_f32).color(pal.text_muted));
                    });
                    ui.label(
                        RichText::new(format!("Duty: {:.0}% CPU • {:.0}% GPU", cpu_duty, gpu_duty))
                            .size(9.5_f32)
                            .color(pal.text_muted),
                    );
                });
            }
            3 => {
                self.nitro_card_frame().show(ui, |ui| {
                    ui.label(
                        RichText::new("POWER SOURCE")
                            .size(9.5_f32)
                            .strong()
                            .color(pal.text_muted),
                    );
                    let (p_txt, p_col) = if self.telemetry.ac_connected {
                        ("135W AC MAINS", pal.success)
                    } else {
                        ("ON BATTERY", pal.warning)
                    };
                    ui.label(RichText::new(p_txt).size(15.0_f32).strong().color(p_col));
                    ui.label(
                        RichText::new(format!(
                            "Bat: {}% • Salud: {:.0}%",
                            self.telemetry.bat_pct, self.telemetry.bat_health
                        ))
                        .size(9.5_f32)
                        .color(pal.text_muted),
                    );
                });
            }
            _ => {}
        };

        if avail_w < 620.0_f32 {
            ui.columns(2, |cols| {
                render_kpi(&mut cols[0], 0);
                render_kpi(&mut cols[1], 1);
            });
            ui.add_space(8.0_f32);
            ui.columns(2, |cols| {
                render_kpi(&mut cols[0], 2);
                render_kpi(&mut cols[1], 3);
            });
        } else {
            ui.columns(4, |cols| {
                render_kpi(&mut cols[0], 0);
                render_kpi(&mut cols[1], 1);
                render_kpi(&mut cols[2], 2);
                render_kpi(&mut cols[3], 3);
            });
        }

        ui.add_space(10.0_f32);

        // 2. Real-time Telemetry Oscilloscope Card
        self.nitro_card_frame().show(ui, |ui| {
            // Row 1: Heading and Mode Selector
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("📊 Live Hardware Oscilloscope")
                        .size(15.5_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let is_fans = self.monitoring_graph_mode == MonitoringGraphMode::Turbines;
                    if ui
                        .selectable_label(
                            is_fans,
                            RichText::new("🌀 Turbines")
                                .color(if is_fans {
                                    pal.secondary
                                } else {
                                    pal.text_muted
                                })
                                .strong(),
                        )
                        .clicked()
                    {
                        self.monitoring_graph_mode = MonitoringGraphMode::Turbines;
                    }
                    let is_pwr = self.monitoring_graph_mode == MonitoringGraphMode::Power;
                    if ui
                        .selectable_label(
                            is_pwr,
                            RichText::new("🔌 Power (W)")
                                .color(if is_pwr {
                                    pal.secondary
                                } else {
                                    pal.text_muted
                                })
                                .strong(),
                        )
                        .clicked()
                    {
                        self.monitoring_graph_mode = MonitoringGraphMode::Power;
                    }
                    let is_work = self.monitoring_graph_mode == MonitoringGraphMode::Workload;
                    if ui
                        .selectable_label(
                            is_work,
                            RichText::new("⚡ Load (%)")
                                .color(if is_work {
                                    pal.secondary
                                } else {
                                    pal.text_muted
                                })
                                .strong(),
                        )
                        .clicked()
                    {
                        self.monitoring_graph_mode = MonitoringGraphMode::Workload;
                    }
                    let is_therm = self.monitoring_graph_mode == MonitoringGraphMode::Thermals;
                    if ui
                        .selectable_label(
                            is_therm,
                            RichText::new("🌡 Thermals")
                                .color(if is_therm {
                                    pal.secondary
                                } else {
                                    pal.text_muted
                                })
                                .strong(),
                        )
                        .clicked()
                    {
                        self.monitoring_graph_mode = MonitoringGraphMode::Thermals;
                    }
                });
            });
            ui.add_space(4.0_f32);

            // Row 2: Timeline controls & live indicator
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Window:")
                        .size(10.5_f32)
                        .color(pal.text_muted),
                );
                for &win in &[30, 60, 120] {
                    let sel = self.history_window_size == win;
                    if ui
                        .selectable_label(
                            sel,
                            RichText::new(format!("{}s", win))
                                .size(10.5_f32)
                                .color(if sel { pal.secondary } else { pal.text_muted }),
                        )
                        .clicked()
                    {
                        self.history_window_size = win;
                    }
                }
                ui.add_space(6.0_f32);
                let p_label = if self.graph_paused {
                    "▶ Resume"
                } else {
                    "⏸ Pause"
                };
                if ui
                    .button(
                        RichText::new(p_label)
                            .size(10.5_f32)
                            .color(if self.graph_paused {
                                pal.success
                            } else {
                                pal.text_muted
                            }),
                    )
                    .clicked()
                {
                    self.graph_paused = !self.graph_paused;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.graph_paused {
                        ui.label(
                            RichText::new("⏸ FROZEN")
                                .size(10.0_f32)
                                .strong()
                                .color(pal.warning),
                        );
                    } else {
                        ui.label(
                            RichText::new("🟢 LIVE 60 FPS")
                                .size(10.0_f32)
                                .strong()
                                .color(pal.success),
                        );
                    }
                });
            });

            ui.add_space(4.0_f32);
            ui.separator();
            ui.add_space(6.0_f32);

            // Render live chart
            self.render_telemetry_chart(ui, self.monitoring_graph_mode, 155.0_f32);

            ui.add_space(8.0_f32);
            // Dynamic legend row (wrapped)
            ui.horizontal_wrapped(|ui| match self.monitoring_graph_mode {
                MonitoringGraphMode::Thermals => {
                    ui.label(
                        RichText::new("● CPU Package:")
                            .color(pal.secondary)
                            .strong(),
                    );
                    ui.label(
                        RichText::new(format!("{:.1}°C", self.telemetry.cpu_temp))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                    ui.add_space(14.0_f32);
                    ui.label(RichText::new("● Discrete GPU:").color(pal.primary).strong());
                    ui.label(
                        RichText::new(format!("{:.1}°C", self.telemetry.gpu_temp))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                    ui.add_space(14.0_f32);
                    ui.label(RichText::new("● NVMe SSD:").color(pal.warning).strong());
                    ui.label(
                        RichText::new(format!("{:.1}°C", self.telemetry.nvme_temp))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                }
                MonitoringGraphMode::Workload => {
                    ui.label(RichText::new("● CPU Load:").color(pal.secondary).strong());
                    ui.label(
                        RichText::new(format!("{:.1}%", self.telemetry.cpu_load))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                    ui.add_space(14.0_f32);
                    ui.label(
                        RichText::new("● GPU Core Load:")
                            .color(pal.primary)
                            .strong(),
                    );
                    ui.label(
                        RichText::new(format!("{:.1}%", self.telemetry.gpu_load))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                }
                MonitoringGraphMode::Power => {
                    ui.label(
                        RichText::new("● CPU Package:")
                            .color(pal.secondary)
                            .strong(),
                    );
                    ui.label(
                        RichText::new(format!("{:.1} W", self.telemetry.cpu_power))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                    ui.add_space(14.0_f32);
                    ui.label(RichText::new("● Discrete GPU:").color(pal.primary).strong());
                    ui.label(
                        RichText::new(format!(
                            "{:.1} W",
                            if self.telemetry.gpu_active {
                                self.telemetry.gpu_power
                            } else {
                                0.0
                            }
                        ))
                        .color(pal.text_primary)
                        .monospace(),
                    );
                    ui.add_space(14.0_f32);
                    let total_w = self.telemetry.cpu_power
                        + if self.telemetry.gpu_active {
                            self.telemetry.gpu_power
                        } else {
                            0.0
                        };
                    ui.label(RichText::new("● Combined:").color(pal.success).strong());
                    ui.label(
                        RichText::new(format!("{:.1} W", total_w))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                }
                MonitoringGraphMode::Turbines => {
                    ui.label(RichText::new("● CPU Fan:").color(pal.secondary).strong());
                    ui.label(
                        RichText::new(format!("{} RPM", self.telemetry.cpu_rpm))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                    ui.add_space(14.0_f32);
                    ui.label(RichText::new("● GPU Fan:").color(pal.primary).strong());
                    ui.label(
                        RichText::new(format!("{} RPM", self.telemetry.gpu_rpm))
                            .color(pal.text_primary)
                            .monospace(),
                    );
                }
            });
        });

        ui.add_space(10.0_f32);

        // 3. Primary Silicon Cards (CPU & GPU)
        if avail_w < 650.0_f32 {
            self.render_cpu_card(ui);
            ui.add_space(10.0_f32);
            self.render_gpu_card(ui);
        } else {
            ui.columns(2, |cols| {
                self.render_cpu_card(&mut cols[0]);
                self.render_gpu_card(&mut cols[1]);
            });
        }

        ui.add_space(10.0_f32);

        // 4. Subsystem Cards (Memory & Storage, Power & Turbines)
        if avail_w < 650.0_f32 {
            self.render_memory_card(ui);
            ui.add_space(10.0_f32);
            self.render_storage_card(ui);
            ui.add_space(10.0_f32);
            self.render_power_card(ui);
            ui.add_space(10.0_f32);
            self.render_turbines_card(ui);
        } else {
            ui.columns(2, |cols| {
                self.render_memory_card(&mut cols[0]);
                cols[0].add_space(10.0_f32);
                self.render_storage_card(&mut cols[0]);

                self.render_power_card(&mut cols[1]);
                cols[1].add_space(10.0_f32);
                self.render_turbines_card(&mut cols[1]);
            });
        }
    }

    fn render_cpu_card(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("🖥 CPU Silicon Architecture")
                        .size(14.5_f32)
                        .strong()
                        .color(pal.secondary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let temp_c = if self.telemetry.cpu_temp >= 85.0 {
                        pal.primary
                    } else if self.telemetry.cpu_temp >= 75.0 {
                        pal.warning
                    } else {
                        pal.success
                    };
                    ui.label(
                        RichText::new(format!("{:.1} °C", self.telemetry.cpu_temp))
                            .size(14.0_f32)
                            .strong()
                            .color(temp_c),
                    );
                });
            });
            ui.label(
                RichText::new(format!("{} (4 Cores / 8 Threads)", self.telemetry.cpu_name))
                    .size(11.0_f32)
                    .color(pal.text_muted),
            );
            ui.add_space(4.0_f32);
            ui.separator();
            ui.add_space(6.0_f32);

            // Utilization bar
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Instant Load:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.label(
                    RichText::new(format!("{:.1}%", self.telemetry.cpu_load))
                        .size(11.5_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} MHz • {:.1} W",
                            self.telemetry.cpu_clock, self.telemetry.cpu_power
                        ))
                        .size(11.0_f32)
                        .monospace()
                        .color(pal.secondary),
                    );
                });
            });
            ui.add_space(2.0_f32);
            draw_tactical_bar(ui, self.telemetry.cpu_load, pal.secondary, 7.0_f32);
            ui.add_space(6.0_f32);

            // Frequency bar (vs 4500 MHz Max Boost)
            let freq_pct = (self.telemetry.cpu_clock as f32 / 4500.0_f32) * 100.0_f32;
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Dynamic Boost Frequency:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{:.2} GHz / 4.50 GHz Max",
                            self.telemetry.cpu_clock as f32 / 1000.0
                        ))
                        .size(10.5_f32)
                        .monospace()
                        .color(pal.text_primary),
                    );
                });
            });
            ui.add_space(2.0_f32);
            draw_tactical_bar(ui, freq_pct, pal.success, 5.0_f32);
            ui.add_space(6.0_f32);

            // Linux Governance
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Gov: {} | EPP: {}",
                        self.telemetry.cpu_governor, self.telemetry.cpu_epp
                    ))
                    .size(10.0_f32)
                    .color(pal.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let turbo_text = if self.telemetry.cpu_turbo {
                        "INTEL TURBO: ACTIVE"
                    } else {
                        "TURBO: OFF"
                    };
                    let turbo_col = if self.telemetry.cpu_turbo {
                        pal.success
                    } else {
                        pal.warning
                    };
                    ui.label(
                        RichText::new(turbo_text)
                            .size(9.5_f32)
                            .strong()
                            .color(turbo_col),
                    );
                });
            });
        });
    }

    fn render_gpu_card(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("🎮 Discrete NVIDIA Graphics").size(14.5_f32).strong().color(pal.primary));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.telemetry.gpu_active {
                        let temp_c = if self.telemetry.gpu_temp >= 80.0 { pal.primary } else { pal.success };
                        ui.label(RichText::new(format!("{:.1} °C", self.telemetry.gpu_temp)).size(14.0_f32).strong().color(temp_c));
                    } else {
                        ui.label(RichText::new("PCIe D3cold STANDBY").size(11.0_f32).strong().color(pal.text_muted));
                    }
                });
            });
            ui.label(RichText::new(format!("{} (Ampere GA107)", self.telemetry.gpu_name)).size(11.0_f32).color(pal.text_muted));
            ui.add_space(4.0_f32);
            ui.separator();
            ui.add_space(6.0_f32);

            if self.telemetry.gpu_active {
                // Silicon core load
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Silicon Utilization:").size(11.0_f32).color(pal.text_muted));
                    ui.label(RichText::new(format!("{:.1}%", self.telemetry.gpu_load)).size(11.5_f32).strong().color(pal.text_primary));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{} MHz • {:.1} W", self.telemetry.gpu_clock, self.telemetry.gpu_power)).size(11.0_f32).monospace().color(pal.primary));
                    });
                });
                ui.add_space(2.0_f32);
                draw_tactical_bar(ui, self.telemetry.gpu_load, pal.primary, 7.0_f32);
                ui.add_space(6.0_f32);

                // VRAM Bar
                let vram_pct = if self.telemetry.gpu_vram_total > 0.0 { (self.telemetry.gpu_vram_used / self.telemetry.gpu_vram_total) * 100.0 } else { 0.0 };
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Dedicated VRAM:").size(11.0_f32).color(pal.text_muted));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{:.2} / {:.1} GB GDDR6 ({:.0}%)", self.telemetry.gpu_vram_used, self.telemetry.gpu_vram_total, vram_pct)).size(10.5_f32).monospace().color(pal.text_primary));
                    });
                });
                ui.add_space(2.0_f32);
                draw_tactical_bar(ui, vram_pct, pal.warning, 5.0_f32);
                ui.add_space(6.0_f32);

                ui.label(RichText::new(format!("Driver: v{} • Bus PCIe 0000:01:00.0 (Active)", self.telemetry.driver_version)).size(10.0_f32).color(pal.text_muted));
            } else {
                ui.label(RichText::new("PCIe D3cold Ultra-Low Power State (0.0 W)").size(12.0_f32).color(pal.text_muted));
                ui.label(RichText::new("GPU turbine is in Zero-RPM silent mode to conserve energy and eliminate noise.").size(10.5_f32).color(pal.text_muted));
            }
        });
    }

    fn render_memory_card(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        self.nitro_card_frame().show(ui, |ui| {
            ui.heading(
                RichText::new("💾 Memory Subsystem")
                    .size(13.5_f32)
                    .strong()
                    .color(pal.text_primary),
            );
            ui.separator();
            ui.add_space(4.0_f32);

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Physical RAM:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{:.1} / {:.1} GB ({:.0}%)",
                            self.telemetry.ram_used,
                            self.telemetry.ram_total,
                            self.telemetry.ram_pct
                        ))
                        .size(11.0_f32)
                        .color(pal.text_primary)
                        .monospace(),
                    );
                });
            });
            ui.add_space(2.0_f32);
            draw_tactical_bar(ui, self.telemetry.ram_pct, pal.success, 6.0_f32);
            ui.add_space(6.0_f32);

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Virtual Swap:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{:.1} / {:.1} GB ({:.0}%)",
                            self.telemetry.swap_used,
                            self.telemetry.swap_total,
                            self.telemetry.swap_pct
                        ))
                        .size(11.0_f32)
                        .color(pal.text_primary)
                        .monospace(),
                    );
                });
            });
            ui.add_space(2.0_f32);
            draw_tactical_bar(ui, self.telemetry.swap_pct, pal.secondary, 5.0_f32);
        });
    }

    fn render_storage_card(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("💽 NVMe Solid State Drive")
                        .size(13.5_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{:.1} °C", self.telemetry.nvme_temp))
                            .size(13.0_f32)
                            .strong()
                            .color(pal.warning),
                    );
                });
            });
            ui.label(
                RichText::new(&self.telemetry.nvme_name)
                    .size(10.5_f32)
                    .color(pal.text_muted),
            );
            ui.separator();
            ui.add_space(4.0_f32);

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Controller State:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.label(
                    RichText::new("Online / Optimal")
                        .size(11.0_f32)
                        .color(pal.success)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("Life Integrity: {}%", self.telemetry.nvme_health))
                            .size(11.0_f32)
                            .color(pal.text_primary),
                    );
                });
            });
        });
    }

    fn render_power_card(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("⚡ Power Delivery & Battery")
                        .size(13.5_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (src_txt, src_col) = if self.telemetry.ac_connected {
                        ("⚡ 135W AC MAINS", pal.success)
                    } else {
                        ("🔋 BATTERY", pal.warning)
                    };
                    ui.label(
                        RichText::new(src_txt)
                            .size(11.5_f32)
                            .strong()
                            .color(src_col),
                    );
                });
            });
            ui.separator();
            ui.add_space(4.0_f32);

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Level: {}% ({})",
                        self.telemetry.bat_pct, self.telemetry.bat_status
                    ))
                    .size(11.0_f32)
                    .color(pal.text_primary)
                    .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{:.2} V • Health: {:.0}%",
                            self.telemetry.bat_voltage, self.telemetry.bat_health
                        ))
                        .size(10.5_f32)
                        .color(pal.text_muted)
                        .monospace(),
                    );
                });
            });
            ui.add_space(2.0_f32);
            draw_tactical_bar(
                ui,
                self.telemetry.bat_pct as f32,
                if self.telemetry.ac_connected {
                    pal.success
                } else {
                    pal.warning
                },
                6.0_f32,
            );
            ui.add_space(4.0_f32);

            ui.label(
                RichText::new(format!(
                    "Cell: {} • Cycles: {} • Care 80%: {}",
                    self.telemetry.bat_model,
                    self.telemetry.bat_cycles,
                    if self.config.battery_health_80 {
                        "ON"
                    } else {
                        "OFF"
                    }
                ))
                .size(10.0_f32)
                .color(pal.text_muted),
            );
        });
    }

    fn render_turbines_card(&self, ui: &mut egui::Ui) {
        let pal = self.pal();
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("🌀 AeroBlade Turbines")
                        .size(13.5_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let cb_txt = if self.config.coolboost {
                        "CoolBoost ON"
                    } else {
                        "CoolBoost OFF"
                    };
                    let cb_col = if self.config.coolboost {
                        pal.success
                    } else {
                        pal.text_muted
                    };
                    ui.label(RichText::new(cb_txt).size(11.0_f32).strong().color(cb_col));
                });
            });
            ui.separator();
            ui.add_space(4.0_f32);

            let cpu_pct = (self.telemetry.cpu_rpm as f32 / 5660.0_f32) * 100.0_f32;
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("CPU Turbine:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} RPM ({:.0}%)", self.telemetry.cpu_rpm, cpu_pct))
                            .size(11.0_f32)
                            .strong()
                            .color(pal.secondary)
                            .monospace(),
                    );
                });
            });
            ui.add_space(2.0_f32);
            draw_tactical_bar(ui, cpu_pct, pal.secondary, 5.0_f32);
            ui.add_space(6.0_f32);

            let gpu_pct = (self.telemetry.gpu_rpm as f32 / 6000.0_f32) * 100.0_f32;
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("GPU Turbine:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} RPM ({:.0}%)", self.telemetry.gpu_rpm, gpu_pct))
                            .size(11.0_f32)
                            .strong()
                            .color(pal.primary)
                            .monospace(),
                    );
                });
            });
            ui.add_space(2.0_f32);
            draw_tactical_bar(ui, gpu_pct, pal.primary, 5.0_f32);
        });
    }

    /// Renders Tab 3: Tactical Operating Scenarios & Power Profiles
    fn render_scenarios_tab(&mut self, ui: &mut egui::Ui) {
        let pal = self.pal();
        let current_p = self.config.profile.clone();

        // 1. Scenarios Header Card
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("⚡ Operating Scenarios & Power Envelopes").size(16.0_f32).strong().color(pal.text_primary));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("CURRENT: {}", current_p.to_uppercase())).size(12.0_f32).strong().color(pal.primary));
                });
            });
            ui.separator();
            ui.add_space(6.0_f32);

            let scenarios = [
                ("quiet", "🌱 Quiet / Eco Stealth", "Maximum battery endurance & acoustic silence (<22 dBA). Limits CPU to 15W TDP and maintains fans at whisper speed.", pal.success, "PL1: 15W | Fan: Silent Min | Temp: Cold"),
                ("balanced", "⚡ Balanced Performance", "Optimal daily configuration for security audits, development, and adaptive cooling under varying loads.", pal.secondary, "PL1: 45W | Fan: Auto BIOS | Temp: Nominal"),
                ("performance", "🔥 Performance Gaming", "Unlocked CPU clock speeds (up to 4.50 GHz boost) with accelerated cooling curve for heavy multitasking.", pal.warning, "PL1: 45W+ | CoolBoost: Active | Temp: Active"),
                ("turbo", "🚀 Extreme Combat Turbo", "Full 100% PWM fan saturation (5660/6000 RPM) with maximum PL2 power envelope for password cracking and stress loads.", pal.primary, "PL2: 65W | GPU TGP: 75W | Fan: 100% Turbo"),
            ];

            for (id, title, desc, col, specs) in scenarios {
                let is_active = current_p == id;
                let bg_col = if is_active { pal.card_hover } else { pal.panel };
                let border_col = if is_active { col } else { pal.border };

                Frame::none()
                    .fill(bg_col)
                    .stroke(Stroke::new(if is_active { 1.6_f32 } else { 1.0_f32 }, border_col))
                    .rounding(Rounding::same(8.0_f32))
                    .inner_margin(Margin::same(12.0_f32))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(title).size(14.0_f32).strong().color(if is_active { col } else { pal.text_primary }));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if is_active {
                                    ui.label(RichText::new("● ACTIVE").size(10.5_f32).strong().color(col));
                                } else {
                                    let btn = egui::Button::new(RichText::new("Engage").size(10.5_f32).color(pal.text_primary))
                                        .fill(pal.card)
                                        .stroke(Stroke::new(1.0_f32, pal.border))
                                        .rounding(Rounding::same(4.0_f32));
                                    if ui.add(btn).clicked() {
                                        self.config.profile = id.into();
                                        let _ = self.cmd_tx.send(HardwareCommand::SetPowerProfile(id.into()));
                                    }
                                }
                            });
                        });
                        ui.add_space(2.0_f32);
                        ui.label(RichText::new(desc).size(11.0_f32).color(pal.text_muted));
                        ui.add_space(4.0_f32);
                        ui.label(RichText::new(specs).size(10.0_f32).monospace().color(col));
                    });
                ui.add_space(6.0_f32);
            }
        });

        ui.add_space(10.0_f32);

        // 2. Real-Time Power State Card
        self.nitro_card_frame().show(ui, |ui| {
            ui.heading(
                RichText::new("🔌 Energy Governance & Linux Power State")
                    .size(14.0_f32)
                    .strong()
                    .color(pal.text_primary),
            );
            ui.separator();
            ui.add_space(6.0_f32);

            let total_w = self.telemetry.cpu_power
                + if self.telemetry.gpu_active {
                    self.telemetry.gpu_power
                } else {
                    0.0_f32
                };
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Real-Time Dissipation:")
                        .size(11.0_f32)
                        .color(pal.text_muted),
                );
                ui.label(
                    RichText::new(format!(
                        "{:.1} W (CPU {:.1}W + GPU {:.1}W)",
                        total_w,
                        self.telemetry.cpu_power,
                        if self.telemetry.gpu_active {
                            self.telemetry.gpu_power
                        } else {
                            0.0
                        }
                    ))
                    .size(12.0_f32)
                    .strong()
                    .color(pal.secondary),
                );
            });
            ui.add_space(4.0_f32);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Active Governor: {} | EPP: {}",
                        self.telemetry.cpu_governor, self.telemetry.cpu_epp
                    ))
                    .size(10.5_f32)
                    .color(pal.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let t_txt = if self.telemetry.cpu_turbo {
                        "INTEL TURBO: ACTIVE"
                    } else {
                        "INTEL TURBO: LOCKED"
                    };
                    let t_col = if self.telemetry.cpu_turbo {
                        pal.success
                    } else {
                        pal.warning
                    };
                    ui.label(RichText::new(t_txt).size(10.0_f32).strong().color(t_col));
                });
            });
        });
    }

    /// Renders Tab 4: 4-Zone Pulsar Keyboard Lighting Studio
    fn render_keyboard_rgb_tab(&mut self, ui: &mut egui::Ui) {
        let pal = self.pal();
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("🌈 4-Zone Pulsar Keyboard Lighting Studio")
                        .size(16.0_f32)
                        .strong()
                        .color(pal.text_primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(&self.rgb_last_status)
                            .size(11.0_f32)
                            .color(pal.secondary),
                    );
                });
            });
            ui.separator();
            ui.add_space(8.0_f32);

            // 1. Draw Original Windows 4-Zone Lighting Keyboard Diagram
            if let Some(tex) = &self.textures {
                let w = ui.available_width().min(600.0_f32);
                let h = w / 3.58_f32; // 1224x342 Aspect Ratio
                let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), egui::Sense::hover());
                ui.painter().image(
                    tex.keyboard_zone.id(),
                    rect,
                    Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
            }
            ui.add_space(10.0_f32);

            // 2. Zone Selection Bar
            ui.label(
                RichText::new("Select Target Keyboard Zone:")
                    .strong()
                    .size(12.5_f32)
                    .color(pal.text_primary),
            );
            ui.add_space(4.0_f32);
            ui.horizontal_wrapped(|ui| {
                let zones = [
                    (0, "🌐 All Zones (1-4)"),
                    (1, "🎮 Zone 1 (WASD)"),
                    (2, "⌨ Zone 2 (Center-Left)"),
                    (3, "⌨ Zone 3 (Center-Right)"),
                    (4, "🔢 Zone 4 (Numpad)"),
                ];

                for (z_idx, z_label) in zones {
                    let is_sel = self.rgb_selected_zone == z_idx;
                    let btn = ui.selectable_label(
                        is_sel,
                        RichText::new(z_label)
                            .color(if is_sel {
                                pal.secondary
                            } else {
                                pal.text_muted
                            })
                            .strong(),
                    );
                    if btn.clicked() {
                        self.rgb_selected_zone = z_idx;
                    }
                }
            });

            ui.add_space(10.0_f32);
            ui.separator();
            ui.add_space(8.0_f32);

            // 3. Tactical Color Palette (Quick-Apply Swatches)
            ui.label(
                RichText::new("Quick Tactical Palette:")
                    .strong()
                    .size(12.5_f32)
                    .color(pal.text_primary),
            );
            ui.add_space(6.0_f32);

            let palette = [
                ("🔴 Nitro Red", 229, 25, 55),
                ("🔷 Cyber Cyan", 0, 229, 255),
                ("🟢 Toxic Green", 0, 230, 118),
                ("🔮 Predator Violet", 179, 136, 255),
                ("⚡ Neon Orange", 255, 109, 0),
                ("🟡 Electric Gold", 255, 214, 0),
                ("💖 Hot Pink", 255, 64, 129),
                ("🌊 Deep Aqua", 0, 184, 212),
                ("⚪ Pure White", 255, 255, 255),
                ("🌑 Stealth Off", 0, 0, 0),
            ];

            ui.horizontal_wrapped(|ui| {
                for (name, r, g, b) in palette {
                    let btn = egui::Button::new(
                        RichText::new(name)
                            .color(Color32::from_rgb(r, g, b))
                            .strong(),
                    )
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(r, g, b)));
                    if ui.add(btn).clicked() {
                        self.rgb_custom_r = r;
                        self.rgb_custom_g = g;
                        self.rgb_custom_b = b;
                        if self.rgb_selected_zone == 0 {
                            for z in 1..=4 {
                                let _ = self.cmd_tx.send(HardwareCommand::SetRgbZone {
                                    zone: z,
                                    r,
                                    g,
                                    b,
                                });
                            }
                            self.rgb_last_status = format!("Applied {} to All Zones", name);
                        } else {
                            let _ = self.cmd_tx.send(HardwareCommand::SetRgbZone {
                                zone: self.rgb_selected_zone,
                                r,
                                g,
                                b,
                            });
                            self.rgb_last_status =
                                format!("Applied {} to Zone {}", name, self.rgb_selected_zone);
                        }
                    }
                }
            });

            ui.add_space(12.0_f32);

            // 4. Custom RGB Sliders & Live Swatch
            ui.label(
                RichText::new("Custom Color Mixer:")
                    .strong()
                    .size(12.5_f32)
                    .color(pal.text_primary),
            );
            ui.add_space(6.0_f32);
            ui.horizontal(|ui| {
                let (swatch_rect, _) =
                    ui.allocate_exact_size(Vec2::new(44.0_f32, 44.0_f32), egui::Sense::hover());
                ui.painter().rect_filled(
                    swatch_rect,
                    Rounding::same(6.0_f32),
                    Color32::from_rgb(self.rgb_custom_r, self.rgb_custom_g, self.rgb_custom_b),
                );
                ui.painter().rect_stroke(
                    swatch_rect,
                    Rounding::same(6.0_f32),
                    Stroke::new(1.2_f32, pal.border),
                );

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("R:").color(pal.primary).strong());
                        ui.add(egui::Slider::new(&mut self.rgb_custom_r, 0..=255));
                        ui.label(RichText::new("G:").color(pal.success).strong());
                        ui.add(egui::Slider::new(&mut self.rgb_custom_g, 0..=255));
                        ui.label(RichText::new("B:").color(pal.secondary).strong());
                        ui.add(egui::Slider::new(&mut self.rgb_custom_b, 0..=255));
                    });
                    ui.add_space(2.0_f32);
                    let push_btn = egui::Button::new(
                        RichText::new("⚡ Push Color to Selected Zone")
                            .strong()
                            .color(pal.text_primary),
                    )
                    .fill(pal.card)
                    .stroke(Stroke::new(1.0_f32, pal.border))
                    .rounding(Rounding::same(5.0_f32));
                    if ui.add(push_btn).clicked() {
                        let (r, g, b) = (self.rgb_custom_r, self.rgb_custom_g, self.rgb_custom_b);
                        if self.rgb_selected_zone == 0 {
                            for z in 1..=4 {
                                let _ = self.cmd_tx.send(HardwareCommand::SetRgbZone {
                                    zone: z,
                                    r,
                                    g,
                                    b,
                                });
                            }
                            self.rgb_last_status =
                                format!("Pushed #{:02X}{:02X}{:02X} to All Zones", r, g, b);
                        } else {
                            let _ = self.cmd_tx.send(HardwareCommand::SetRgbZone {
                                zone: self.rgb_selected_zone,
                                r,
                                g,
                                b,
                            });
                            self.rgb_last_status = format!(
                                "Pushed #{:02X}{:02X}{:02X} to Zone {}",
                                r, g, b, self.rgb_selected_zone
                            );
                        }
                    }
                });
            });

            ui.add_space(12.0_f32);
            ui.separator();
            ui.add_space(8.0_f32);

            // 5. Curated Multi-Zone Master Presets
            ui.label(
                RichText::new("Acer Nitro Multi-Zone Presets:")
                    .strong()
                    .size(12.5_f32)
                    .color(pal.text_primary),
            );
            ui.add_space(6.0_f32);

            let presets = [
                ("nitro", "🔴 Nitro Crimson"),
                ("cyberpunk", "🌆 Cyberpunk 2077"),
                ("ice", "❄ Arctic Ice"),
                ("toxic", "🧪 Toxic Acid"),
                ("synthwave", "🌌 Deep Synthwave"),
                ("white", "⚪ Pure White"),
            ];

            ui.horizontal_wrapped(|ui| {
                for (id, label) in presets {
                    let p_btn =
                        egui::Button::new(RichText::new(label).strong().color(pal.text_primary))
                            .fill(pal.card)
                            .stroke(Stroke::new(1.0_f32, pal.border))
                            .rounding(Rounding::same(5.0_f32));
                    if ui.add(p_btn).clicked() {
                        let _ = self.cmd_tx.send(HardwareCommand::SetRgbPreset(id.into()));
                        self.rgb_last_status = format!("Applied preset: {}", label);
                    }
                }
            });
        });
    }

    /// Renders Tab 5: Hardware Settings, Gaming Locks & Self-Diagnostics
    fn render_settings_tab(&mut self, ui: &mut egui::Ui) {
        let pal = self.pal();

        // 0. Visual Ergonomics & Human Color Themes
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("🎨 Paleta de Color & Ergonomía Visual").size(15.0_f32).strong().color(pal.text_primary));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("ACTIVO: {}", self.theme_mode.name())).size(11.5_f32).strong().color(pal.primary));
                });
            });
            ui.label(RichText::new("Diseño optimizado para confort humano, reduciendo el estrés retinal y mejorando la legibilidad técnica en sesiones prolongadas.").size(11.0_f32).color(pal.text_muted));
            ui.add_space(6.0_f32);
            ui.separator();
            ui.add_space(8.0_f32);

            let all_themes = [
                ThemeMode::HumanComfort,
                ThemeMode::TokyoNight,
                ThemeMode::NeoTokyo,
                ThemeMode::RedAudit,
                ThemeMode::HackerGreen,
                ThemeMode::StellarVoid,
                ThemeMode::AuroraGradient,
                ThemeMode::KuromiGoth,
                ThemeMode::CinnamorollNight,
                ThemeMode::Everforest,
                ThemeMode::NordicCalm,
                ThemeMode::EarthSage,
                ThemeMode::CyberNitro,
                ThemeMode::ModernBlue,
                ThemeMode::MantecCorporate,
                ThemeMode::MaterialRed,
                ThemeMode::CinnamorollCloud,
                ThemeMode::MyMelodySoft,
                ThemeMode::PompompurinCafe,
            ];

            for t in all_themes {
                let is_sel = self.theme_mode == t;
                let t_pal = get_palette(t);
                let bg_col = if is_sel { pal.card_hover } else { pal.panel };
                let stroke_col = if is_sel { pal.primary } else { pal.border };

                Frame::none()
                    .fill(bg_col)
                    .stroke(Stroke::new(if is_sel { 1.5_f32 } else { 1.0_f32 }, stroke_col))
                    .rounding(Rounding::same(8.0_f32))
                    .inner_margin(Margin::same(10.0_f32))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(t.name()).size(13.0_f32).strong().color(if is_sel { pal.primary } else { pal.text_primary }));

                            // Swatch sample circles
                            ui.horizontal(|ui| {
                                let swatches = [t_pal.primary, t_pal.secondary, t_pal.success, t_pal.warning, t_pal.card];
                                for col in swatches {
                                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(12.0_f32), egui::Sense::hover());
                                    ui.painter().circle_filled(rect.center(), 5.0_f32, col);
                                    ui.painter().circle_stroke(rect.center(), 5.0_f32, Stroke::new(1.0_f32, pal.border));
                                }
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if is_sel {
                                    ui.label(RichText::new("✔ ACTIVO").size(11.0_f32).strong().color(pal.success));
                                } else {
                                    let btn = egui::Button::new(RichText::new("Seleccionar").size(11.0_f32).color(pal.text_primary))
                                        .fill(pal.card)
                                        .stroke(Stroke::new(1.0_f32, pal.border))
                                        .rounding(Rounding::same(5.0_f32));
                                    if ui.add(btn).clicked() {
                                        self.set_theme(ui.ctx(), t);
                                    }
                                }
                            });
                        });
                        ui.add_space(2.0_f32);
                        ui.label(RichText::new(t.description()).size(10.5_f32).color(pal.text_muted));
                    });
                ui.add_space(6.0_f32);
            }
        });

        ui.add_space(10.0_f32);

        // 1. Battery & Power Delivery Protection Card
        self.nitro_card_frame().show(ui, |ui| {
            ui.heading(RichText::new("🔋 Battery & Lithium Care System").size(15.0_f32).strong().color(pal.text_primary));
            ui.separator();
            ui.add_space(6.0_f32);

            let mut limit = self.config.battery_health_80;
            let is_active = limit;
            if ui.checkbox(&mut limit, RichText::new("80% Battery Health Protection Limiter").size(13.0_f32).strong().color(if is_active { pal.success } else { pal.text_primary })).changed() {
                self.config.battery_health_80 = limit;
                let _ = self.cmd_tx.send(HardwareCommand::SetBatteryLimit(limit));
            }
            ui.label(RichText::new("Stops charging at ~80% capacity to prevent chemical degradation and battery swelling during continuous AC mains usage.").size(10.5_f32).color(pal.text_muted));
            ui.add_space(6.0_f32);

            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Cell Model: {} • Cycles: {} • Chemistry Health: {:.0}%", self.telemetry.bat_model, self.telemetry.bat_cycles, self.telemetry.bat_health)).size(10.5_f32).monospace().color(pal.text_muted));
            });
        });

        ui.add_space(10.0_f32);

        // 2. Tactical Gaming Locks & Display Enhancement
        self.nitro_card_frame().show(ui, |ui| {
            ui.heading(RichText::new("🎮 Tactical Gaming Locks & Hardware Features").size(15.0_f32).strong().color(pal.text_primary));
            ui.separator();
            ui.add_space(6.0_f32);

            let mut win_lock = self.config.winkey_locked;
            if ui.checkbox(&mut win_lock, RichText::new("🔒 Lock Windows / Super Key during Gaming").size(12.5_f32).color(pal.text_primary).strong()).changed() {
                self.config.winkey_locked = win_lock;
                let _ = gaming::set_winkey_lock(win_lock);
                save_config(&self.config);
            }
            ui.label(RichText::new("Disables the Super key scancode to avoid desktop dropouts during full-screen operations.").size(10.5_f32).color(pal.text_muted));
            ui.add_space(8.0_f32);

            let mut tp_lock = self.config.touchpad_locked;
            if ui.checkbox(&mut tp_lock, RichText::new("🚫 Disable Internal Touchpad").size(12.5_f32).color(pal.text_primary).strong()).changed() {
                self.config.touchpad_locked = tp_lock;
                let _ = gaming::set_touchpad_lock(tp_lock);
                save_config(&self.config);
            }
            ui.label(RichText::new("Suppresses touchpad hardware interrupts to eliminate accidental palm touches when using an external mouse.").size(10.5_f32).color(pal.text_muted));
            ui.add_space(10.0_f32);

            let od_btn = egui::Button::new(RichText::new("🚀 Trigger LCD 3ms Response Overdrive").color(pal.primary).strong())
                .fill(pal.card)
                .stroke(Stroke::new(1.0_f32, pal.border))
                .rounding(Rounding::same(5.0_f32));
            if ui.add(od_btn).clicked() {
                let _ = self.cmd_tx.send(HardwareCommand::SetLcdOverdrive(true));
            }
            ui.label(RichText::new("Applies accelerated voltage overdrive to the panel to eliminate ghosting in high-FPS gaming.").size(10.5_f32).color(pal.text_muted));
        });

        ui.add_space(10.0_f32);

        // 3. Embedded Controller (EC) & ACPI Hardware Diagnostics
        self.nitro_card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("🐧 Linux Kernel & Hardware Bus Diagnostics").size(15.0_f32).strong().color(pal.text_primary));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let test_btn = egui::Button::new(RichText::new("🩺 Run Hardware Self-Test").strong().color(pal.secondary))
                        .fill(pal.card)
                        .stroke(Stroke::new(1.0_f32, pal.border))
                        .rounding(Rounding::same(5.0_f32));
                    if ui.add(test_btn).clicked() {
                        let acpi_ok = std::path::Path::new("/proc/acpi/call").exists();
                        let nvme_ok = std::path::Path::new("/sys/class/nvme").exists();
                        let bat_ok = self.telemetry.bat_pct > 0;
                        let gpu_ok = self.telemetry.driver_version != "N/A";

                        self.diag_status = Some(format!(
                            "Diagnostic Result: ACPI Bus: {}, NVMe Controller: {}, Battery Sensor: {}, NVIDIA Bus: {}",
                            if acpi_ok { "✔ OPTIMAL" } else { "⚠ ERROR" },
                            if nvme_ok { "✔ ACTIVE" } else { "⚠ UNKNOWN" },
                            if bat_ok { "✔ ONLINE" } else { "⚠ OFF" },
                            if gpu_ok { "✔ D0/D3 LINK OK" } else { "⚠ SUSPENDED" }
                        ));
                    }
                });
            });
            ui.separator();
            ui.add_space(6.0_f32);

            ui.label(RichText::new("• ACPI Call Driver: /proc/acpi/call (Operational)").size(11.0_f32).color(pal.text_muted));
            ui.label(RichText::new("• Embedded Controller: Compal EC \\_SB.PCI0.LPCB.EC0 (Fang/Fanw Ports Active)").size(11.0_f32).color(pal.text_muted));
            ui.label(RichText::new("• WMI Gaming Interface: \\_SB.PCI0.WMID.WMAA (Instance 1)").size(11.0_f32).color(pal.text_muted));
            ui.label(RichText::new("• Tachometers: EC Register 0x19/0x1A (CPU) & 0x29/0x2A (GPU)").size(11.0_f32).color(pal.text_muted));

            if let Some(res) = &self.diag_status {
                ui.add_space(6.0_f32);
                Frame::none()
                    .fill(pal.card_hover)
                    .stroke(Stroke::new(1.0_f32, pal.secondary))
                    .rounding(Rounding::same(6.0_f32))
                    .inner_margin(Margin::same(8.0_f32))
                    .show(ui, |ui| {
                        ui.label(RichText::new(res).size(11.0_f32).monospace().color(pal.secondary));
                    });
            }
        });
    }
}

impl eframe::App for AcerSenseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Drain Telemetry Channel (Lock-free < 1 µs) and update rolling history & peaks
        if let Ok(data) = self.telemetry_rx.try_recv() {
            if self.telemetry.cpu_rpm > 0 {
                self.delta_cpu_rpm = data.cpu_rpm as i32 - self.telemetry.cpu_rpm as i32;
            }
            if self.telemetry.gpu_rpm > 0 {
                self.delta_gpu_rpm = data.gpu_rpm as i32 - self.telemetry.gpu_rpm as i32;
            }
            if !self.graph_paused {
                self.history.push_back(TelemetryHistoryPoint {
                    cpu_temp: data.cpu_temp,
                    gpu_temp: data.gpu_temp,
                    nvme_temp: data.nvme_temp,
                    cpu_load: data.cpu_load,
                    gpu_load: data.gpu_load,
                    cpu_rpm: data.cpu_rpm,
                    gpu_rpm: data.gpu_rpm,
                    cpu_power: data.cpu_power,
                    gpu_power: data.gpu_power,
                });
                if self.history.len() > 150 {
                    self.history.pop_front();
                }
            }
            if data.cpu_temp > self.peak_cpu_temp {
                self.peak_cpu_temp = data.cpu_temp;
            }
            if data.gpu_temp > self.peak_gpu_temp {
                self.peak_gpu_temp = data.gpu_temp;
            }
            if data.cpu_power > self.peak_cpu_power {
                self.peak_cpu_power = data.cpu_power;
            }
            if data.gpu_power > self.peak_gpu_power {
                self.peak_gpu_power = data.gpu_power;
            }
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
            if i.key_pressed(egui::Key::Num1) {
                self.current_tab = Tab::FanControl;
            }
            if i.key_pressed(egui::Key::Num2) {
                self.current_tab = Tab::Monitoring;
            }
            if i.key_pressed(egui::Key::Num3) {
                self.current_tab = Tab::PowerModes;
            }
            if i.key_pressed(egui::Key::Num4) {
                self.current_tab = Tab::KeyboardRgb;
            }
            if i.key_pressed(egui::Key::Num5) {
                self.current_tab = Tab::SystemSettings;
            }
            if i.key_pressed(egui::Key::T) {
                let next = self.theme_mode.next();
                self.set_theme(ctx, next);
            }
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
                let _ = self
                    .cmd_tx
                    .send(HardwareCommand::SetFanMode("custom".into()));
            }
        });

        let pal = self.pal();

        // 4. Iconic AcerSense Two-Tier Tactical Header (Zero collision guarantee)
        egui::TopBottomPanel::top("acersense_header")
            .frame(
                Frame::none()
                    .fill(pal.panel)
                    .inner_margin(Margin::symmetric(16.0_f32, 8.0_f32)),
            )
            .show(ctx, |ui| {
                let avail_w = ui.available_width();

                // ROW 1: Identity Branding (Left) & Global Controls (Right)
                ui.horizontal(|ui| {
                    ui.heading(
                        RichText::new("ACERSENSE")
                            .color(pal.primary)
                            .strong()
                            .size(19.0_f32),
                    );

                    let badge = Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(
                            pal.success.r(),
                            pal.success.g(),
                            pal.success.b(),
                            25,
                        ))
                        .stroke(Stroke::new(
                            1.0_f32,
                            Color32::from_rgba_unmultiplied(
                                pal.success.r(),
                                pal.success.g(),
                                pal.success.b(),
                                100,
                            ),
                        ))
                        .rounding(Rounding::same(4.0_f32))
                        .inner_margin(Margin::symmetric(5.0_f32, 2.0_f32));
                    badge.show(ui, |ui| {
                        ui.label(
                            RichText::new("PRO LINUX v2.1")
                                .color(pal.success)
                                .size(10.0_f32)
                                .strong(),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Quick theme cycle pill button
                        let theme_text = format!("🎨 {}", self.theme_mode.name());
                        let theme_btn = egui::Button::new(
                            RichText::new(theme_text)
                                .size(11.0_f32)
                                .strong()
                                .color(pal.text_secondary),
                        )
                        .fill(pal.card)
                        .stroke(Stroke::new(1.0_f32, pal.border))
                        .rounding(Rounding::same(6.0_f32));

                        if ui
                            .add(theme_btn)
                            .on_hover_text(
                                "Clic o presiona [T] para alternar entre paletas de confort",
                            )
                            .clicked()
                        {
                            let next = self.theme_mode.next();
                            self.set_theme(ctx, next);
                        }
                    });
                });

                ui.add_space(6.0_f32);
                ui.separator();
                ui.add_space(6.0_f32);

                // ROW 2: Balanced Segmented Navigation Tab Bar
                let tabs = [
                    (Tab::FanControl, "🌀 VENTILADORES (1)"),
                    (Tab::Monitoring, "📊 MONITOREO (2)"),
                    (Tab::PowerModes, "⚡ ESCENARIOS (3)"),
                    (Tab::KeyboardRgb, "🌈 ILUMINACIÓN (4)"),
                    (Tab::SystemSettings, "⚙ AJUSTES (5)"),
                ];

                let tab_count = tabs.len() as f32;
                let spacing = 6.0_f32;
                let total_spacing = spacing * (tab_count - 1.0);
                let tab_w = ((avail_w - total_spacing) / tab_count).max(75.0_f32);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;
                    for (t, label) in &tabs {
                        let active = self.current_tab == *t;
                        let tab_bg = if active {
                            Color32::from_rgba_unmultiplied(
                                pal.primary.r(),
                                pal.primary.g(),
                                pal.primary.b(),
                                28,
                            )
                        } else {
                            pal.card
                        };
                        let tab_stroke = if active {
                            Stroke::new(1.3_f32, pal.primary)
                        } else {
                            Stroke::new(1.0_f32, pal.border)
                        };
                        let tab_text = RichText::new(*label)
                            .size(if avail_w < 680.0 { 10.0_f32 } else { 11.2_f32 })
                            .color(if active {
                                pal.primary
                            } else {
                                pal.text_secondary
                            })
                            .strong();

                        let btn = egui::Button::new(tab_text)
                            .fill(tab_bg)
                            .stroke(tab_stroke)
                            .rounding(Rounding::same(6.0_f32));

                        if ui.add_sized([tab_w, 28.0_f32], btn).clicked() {
                            self.current_tab = *t;
                        }
                    }
                });
            });

        // 5. Always-Visible Bottom Tactical Status Bar with Modern Status Chips
        egui::TopBottomPanel::bottom("acersense_footer")
            .frame(
                Frame::none()
                    .fill(pal.panel)
                    .inner_margin(Margin::symmetric(14.0_f32, 6.0_f32)),
            )
            .show(ctx, |ui| {
                let total_w = self.telemetry.cpu_power
                    + if self.telemetry.gpu_active {
                        self.telemetry.gpu_power
                    } else {
                        0.0_f32
                    };
                let avail_w = ui.available_width();

                ui.horizontal(|ui| {
                    let render_chip = |ui: &mut egui::Ui, text: &str, col: Color32| {
                        Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(
                                col.r(),
                                col.g(),
                                col.b(),
                                22,
                            ))
                            .stroke(Stroke::new(
                                1.0_f32,
                                Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), 90),
                            ))
                            .rounding(Rounding::same(4.0_f32))
                            .inner_margin(Margin::symmetric(6.0_f32, 2.0_f32))
                            .show(ui, |ui| {
                                ui.label(RichText::new(text).color(col).strong().size(10.5_f32));
                            });
                    };

                    if self.telemetry.ac_connected {
                        render_chip(ui, "⚡ AC MAINS (135W)", pal.success);
                    } else {
                        render_chip(
                            ui,
                            &format!("🔋 BATERÍA ({}%)", self.telemetry.bat_pct),
                            pal.warning,
                        );
                    }

                    ui.add_space(4.0_f32);
                    render_chip(
                        ui,
                        &format!("🌀 FAN: {}", self.config.mode.to_uppercase()),
                        pal.secondary,
                    );

                    ui.add_space(4.0_f32);
                    render_chip(
                        ui,
                        &format!("⚡ PERFIL: {}", self.config.profile.to_uppercase()),
                        pal.primary,
                    );

                    ui.add_space(6.0_f32);
                    ui.label(
                        RichText::new(format!("DISIPACIÓN: {:.1} W", total_w))
                            .color(pal.text_primary)
                            .monospace()
                            .size(10.5_f32),
                    );

                    if avail_w >= 720.0_f32 {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(
                                    "[1-5] Pestañas  [T] Tema  [A/M/C] Modo  [Ctrl+Q] Salir",
                                )
                                .color(pal.text_muted)
                                .size(10.0_f32),
                            );
                        });
                    }
                });
            });

        // 6. Central Viewport with Fluid Scrolling
        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(pal.bg)
                    .inner_margin(Margin::same(14.0_f32)),
            )
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| match self.current_tab {
                        Tab::FanControl => {
                            self.render_fans_tab(ui);
                        }
                        Tab::Monitoring => {
                            self.render_monitoring_tab(ui);
                        }
                        Tab::PowerModes => {
                            self.render_scenarios_tab(ui);
                        }
                        Tab::KeyboardRgb => {
                            self.render_keyboard_rgb_tab(ui);
                        }
                        Tab::SystemSettings => {
                            self.render_settings_tab(ui);
                        }
                    });
            });
    }
}

fn main() -> eframe::Result<()> {
    let is_fullscreen = std::env::args().any(|a| a == "--fullscreen" || a == "-F");
    let is_maximized = std::env::args().any(|a| a == "--maximized");
    let mut vp = egui::ViewportBuilder::default()
        .with_inner_size([980.0_f32, 680.0_f32])
        .with_min_inner_size([540.0_f32, 440.0_f32])
        .with_title("AcerSense Linux Pro v2.1 — Hardware Suite");

    if is_fullscreen {
        vp = vp.with_fullscreen(true);
    } else if is_maximized {
        vp = vp.with_maximized(true);
    }

    let options = eframe::NativeOptions {
        viewport: vp,
        ..Default::default()
    };

    eframe::run_native(
        "AcerSense Linux",
        options,
        Box::new(|cc| Ok(Box::new(AcerSenseApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_parsing() {
        assert_eq!(ThemeMode::from_str("human"), ThemeMode::HumanComfort);
        assert_eq!(ThemeMode::from_str("tokyo"), ThemeMode::TokyoNight);
        assert_eq!(ThemeMode::from_str("tokyonight"), ThemeMode::TokyoNight);
        assert_eq!(ThemeMode::from_str("neotokyo"), ThemeMode::NeoTokyo);
        assert_eq!(ThemeMode::from_str("neo_tokyo"), ThemeMode::NeoTokyo);
        assert_eq!(ThemeMode::from_str("neo"), ThemeMode::NeoTokyo);
        assert_eq!(ThemeMode::from_str("everforest"), ThemeMode::Everforest);
        assert_eq!(ThemeMode::from_str("forest"), ThemeMode::Everforest);
        assert_eq!(ThemeMode::from_str("nordic"), ThemeMode::NordicCalm);
        assert_eq!(ThemeMode::from_str("earth"), ThemeMode::EarthSage);
        assert_eq!(ThemeMode::from_str("cyber"), ThemeMode::CyberNitro);
        assert_eq!(ThemeMode::from_str("red_audit"), ThemeMode::RedAudit);
        assert_eq!(ThemeMode::from_str("redaudit"), ThemeMode::RedAudit);
        assert_eq!(ThemeMode::from_str("hacker_green"), ThemeMode::HackerGreen);
        assert_eq!(ThemeMode::from_str("matrix"), ThemeMode::HackerGreen);
        assert_eq!(ThemeMode::from_str("stellar_void"), ThemeMode::StellarVoid);
        assert_eq!(ThemeMode::from_str("space"), ThemeMode::StellarVoid);
        assert_eq!(ThemeMode::from_str("kuromi_goth"), ThemeMode::KuromiGoth);
        assert_eq!(ThemeMode::from_str("kuromi"), ThemeMode::KuromiGoth);
        assert_eq!(
            ThemeMode::from_str("cinnamoroll_night"),
            ThemeMode::CinnamorollNight
        );
        assert_eq!(
            ThemeMode::from_str("cinnamoroll_cloud"),
            ThemeMode::CinnamorollCloud
        );
        assert_eq!(
            ThemeMode::from_str("mymelody_soft"),
            ThemeMode::MyMelodySoft
        );
        assert_eq!(
            ThemeMode::from_str("pompompurin_cafe"),
            ThemeMode::PompompurinCafe
        );
        assert_eq!(
            ThemeMode::from_str("mantec_corporate"),
            ThemeMode::MantecCorporate
        );
        assert_eq!(ThemeMode::from_str("modern_blue"), ThemeMode::ModernBlue);
        assert_eq!(ThemeMode::from_str("material_red"), ThemeMode::MaterialRed);
        assert_eq!(ThemeMode::from_str("aurora"), ThemeMode::AuroraGradient);
        assert_eq!(ThemeMode::from_str("gradient"), ThemeMode::AuroraGradient);
        assert_eq!(ThemeMode::from_str("unknown"), ThemeMode::HumanComfort);
    }

    #[test]
    fn test_theme_mode_cycling() {
        let mut curr = ThemeMode::HumanComfort;
        let expected_sequence = [
            ThemeMode::TokyoNight,
            ThemeMode::NeoTokyo,
            ThemeMode::Everforest,
            ThemeMode::NordicCalm,
            ThemeMode::EarthSage,
            ThemeMode::CyberNitro,
            ThemeMode::RedAudit,
            ThemeMode::HackerGreen,
            ThemeMode::StellarVoid,
            ThemeMode::AuroraGradient,
            ThemeMode::KuromiGoth,
            ThemeMode::CinnamorollNight,
            ThemeMode::CinnamorollCloud,
            ThemeMode::MyMelodySoft,
            ThemeMode::PompompurinCafe,
            ThemeMode::MantecCorporate,
            ThemeMode::ModernBlue,
            ThemeMode::MaterialRed,
            ThemeMode::HumanComfort,
        ];

        for expected in expected_sequence {
            curr = curr.next();
            assert_eq!(curr, expected);
        }
    }

    #[test]
    fn test_palette_generation() {
        for mode in [
            ThemeMode::HumanComfort,
            ThemeMode::TokyoNight,
            ThemeMode::NeoTokyo,
            ThemeMode::Everforest,
            ThemeMode::NordicCalm,
            ThemeMode::EarthSage,
            ThemeMode::CyberNitro,
            ThemeMode::RedAudit,
            ThemeMode::HackerGreen,
            ThemeMode::StellarVoid,
            ThemeMode::AuroraGradient,
            ThemeMode::KuromiGoth,
            ThemeMode::CinnamorollNight,
            ThemeMode::CinnamorollCloud,
            ThemeMode::MyMelodySoft,
            ThemeMode::PompompurinCafe,
            ThemeMode::MantecCorporate,
            ThemeMode::ModernBlue,
            ThemeMode::MaterialRed,
        ] {
            let p = get_palette(mode);
            assert!(!p.name.is_empty());
            assert!(p.card_rounding > 0.0);
        }
    }
}
