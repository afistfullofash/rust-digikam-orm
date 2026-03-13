use clap::{Parser, ValueEnum};
use confique::{json5, Config};
use serde::Deserialize;
use std::path::PathBuf;
use tracing::error;

use crate::filters::LightDarkFilter;
use crate::wallpaper::WallpaperMode;

/// The dark/light mode seting to apply
#[derive(Debug, Config, Deserialize)]
pub struct TagsModeConfig {
    /// Tags to filter for when applying light mode
    #[config(default = ["/Style/Asthetic/Light"])]
    pub light: Vec<String>,
    /// Tags to filter for when apply dark mode
    #[config(default = ["/Style/Asthetic/Dark"])]
    pub dark: Vec<String>,
}

#[derive(Debug, Config, Deserialize)]
pub struct DarkModeConfig {
    /// Light and Dark mode tags to apply depending on MODE
    #[config(nested)]
    pub tags: TagsModeConfig,
    /// The dark/light mode seting to apply
    #[config(default = "None")]
    pub setting: LightDarkFilter,
}

#[derive(Debug, Config, Deserialize)]
pub struct AppConfig {
    /// Tags applied to every filtering operation
    /// This is most useful for things like generic size/style filtering
    #[config(default = ["/Size/wallpaper"])]
    pub tags: Vec<String>,
    /// Light/Dark Mode configuration
    #[config(nested)]
    pub dark_mode: DarkModeConfig,
    /// The path to the digikam sqlite database file
    #[config(default = "~/Pictures/digikam4.db")]
    pub db_path: String,
    /// The MODE to set the wallpaper in.
    #[config(default = "Center")]
    pub wallpaper_mode: WallpaperMode,
}

impl Default for TagsModeConfig {
    fn default() -> Self {
        Self {
            light: default_light_tags(),
            dark: default_dark_tags(),
        }
    }
}

impl Default for DarkModeConfig {
    fn default() -> Self {
        Self {
            tags: TagsModeConfig::default(),
            setting: default_dark_mode_setting(),
        }
    }
}

fn default_light_tags() -> Vec<String> {
    vec!["/Style/Asthetic/Light".to_string()]
}

fn default_dark_tags() -> Vec<String> {
    vec!["/Style/Asthetic/Dark".to_string()]
}

fn default_dark_mode_setting() -> LightDarkFilter {
    LightDarkFilter::None
}

/// Sets the wallpaper by randomly selecting a image from digiKam's Database
///
/// Features:
/// - Global Tag Filtering
/// - Dark/Light Mode Filtering
/// - Configuration File for runnning headlessly
#[derive(Debug, Parser)]
#[command(name = "digikam-wallpaper", version)]
struct Cli {
    /// Configuration file (TOML)
    #[arg(long)]
    config: Option<PathBuf>,
    /// The path to the digikam sqlite database file
    #[arg(long)]
    db_path: Option<String>,
    /// The wallpaper mode to set
    #[arg(long)]
    wallpaper_mode: Option<WallpaperMode>,
    /// The dark mode settings to apply
    #[arg(long)]
    dark_mode: Option<LightDarkFilter>,
    /// Print a configuration file template with default values
    #[arg(long)]
    config_template: bool,
}

/// Get the default path to the configuration file
/// First we match against the Systems User Configuration directory
/// If we can not find a System User Configuration Directory the current  directory is used
fn default_config_path() -> PathBuf {
    match dirs::config_dir() {
        Some(mut config_dir) => {
            config_dir.push("digikam-wallpaper/config.json");
            config_dir
        }
        None => PathBuf::from("./config.json"),
    }
}

/// Get Application Configuration
pub fn get_config() -> Option<AppConfig> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(default_config_path);

    match AppConfig::builder().env().file(config_path).load() {
        Ok(mut config) => {
            if cli.config_template {
                println!(
                    "{}",
                    json5::template::<AppConfig>(json5::FormatOptions::default())
                );
                return None;
            } else if cli.dark_mode.is_none()
                && cli.wallpaper_mode.is_none()
                && cli.db_path.is_none()
            {
                println!(
                    "No cli arguments supplied. Run <command> -h to see options. If this was unintentional."
                );
            } else {
                if let Some(db_path) = cli.db_path {
                    config.db_path = db_path;
                }

                if let Some(wallpaper_mode) = cli.wallpaper_mode {
                    config.wallpaper_mode = wallpaper_mode;
                }

                if let Some(dark_mode) = cli.dark_mode {
                    config.dark_mode.setting = dark_mode;
                }
            }
            Some(config)
        }
        Err(e) => {
            error!(error = ?e, "Error getting config");
            println!("Error loading config");
            None
        }
    }
}
