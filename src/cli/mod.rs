use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "acersense")]
#[command(author = "Rodrigo <rodrigo@parrot-sec.local>")]
#[command(version = "2.1.0 (Rust Edition)")]
#[command(about = "Ultra-fast low-level hardware control suite for Acer Nitro & Predator in Linux")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Print single-line format optimized for Polybar module
    #[arg(long, global = true)]
    pub polybar: bool,

    /// Output full telemetry as JSON
    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show full system, cooling and hardware status
    Status,

    /// Control fan subsystem (Modes, Speeds, CoolBoost)
    Fan(FanArgs),

    /// Manage platform power profiles
    Profile(ProfileArgs),

    /// Quick shorthand for mode switching
    Mode(ModeArgs),

    /// Battery health and charging limits
    Battery(BatteryArgs),

    /// 4-Zone RGB keyboard lighting control
    Rgb(RgbArgs),

    /// Gaming enhancements (WinKey lock, Touchpad, Overdrive)
    Gaming(GamingArgs),

    /// Restore hardware states from saved configuration (used on system boot / resume)
    Restore,
}

#[derive(Args, Debug)]
pub struct FanArgs {
    /// Fan mode: auto, max, custom, silent
    #[arg(short, long)]
    pub mode: Option<String>,

    /// Toggle between auto and max
    #[arg(short, long)]
    pub toggle: bool,

    /// Show fan cooling status
    #[arg(short, long)]
    pub status: bool,

    /// CoolBoost state: on, off
    #[arg(short, long)]
    pub coolboost: Option<String>,

    /// CPU fan speed percentage (0-100)
    #[arg(long)]
    pub cpu: Option<u8>,

    /// GPU fan speed percentage (0-100)
    #[arg(long)]
    pub gpu: Option<u8>,
}

#[derive(Args, Debug)]
pub struct ProfileArgs {
    /// Set power profile: quiet, balanced, performance, turbo
    #[arg(short, long)]
    pub set: Option<String>,

    /// Cycle to next power profile
    #[arg(short, long)]
    pub next: bool,

    /// Show current profile
    #[arg(short, long)]
    pub status: bool,
}

#[derive(Args, Debug)]
pub struct ModeArgs {
    /// Mode target: next, auto, max, quiet, balanced, performance, turbo
    pub target: Option<String>,
}

#[derive(Args, Debug)]
pub struct BatteryArgs {
    /// 80% charge limit: on, off
    #[arg(long)]
    pub limit_80: Option<String>,

    /// Show battery health and statistics
    #[arg(short, long)]
    pub status: bool,
}

#[derive(Args, Debug)]
pub struct RgbArgs {
    /// Lighting preset: nitro, cyberpunk, ice, toxic, synthwave, white
    #[arg(short, long)]
    pub preset: Option<String>,

    /// Set 4 individual zone colors (e.g., --zones "#FF0000" "#00FF00" "#0000FF" "#FFFF00")
    #[arg(short, long, num_args = 4)]
    pub zones: Option<Vec<String>>,

    /// Set all 4 zones to a single hex color (e.g., --color "#FF0000")
    #[arg(short, long)]
    pub color: Option<String>,
}

#[derive(Args, Debug)]
pub struct GamingArgs {
    /// Lock/unlock Windows key: lock, unlock
    #[arg(long)]
    pub winkey: Option<String>,

    /// Lock/unlock Touchpad: lock, unlock
    #[arg(long)]
    pub touchpad: Option<String>,

    /// LCD Overdrive: on, off
    #[arg(long)]
    pub overdrive: Option<String>,
}
