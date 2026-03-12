use clap::{Parser, ValueEnum};
use confique::{json5, Config};
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use tracing::error;

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

/// Options for applying the system light/dark mode
#[derive(Debug, Deserialize, Clone, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum DarkModeSetting {
    /// Disable light/dark mode filtering
    None,
    /// Apply Dark mode tags
    Dark,
    /// Apply Light mode tags
    Light,
    /// Apply according to system light/dark mode setting
    System,
}

#[derive(Debug, Config, Deserialize)]
pub struct DarkModeConfig {
    /// Light and Dark mode tags to apply depending on MODE
    #[config(nested)]
    pub tags: TagsModeConfig,
    /// The dark/light mode seting to apply
    #[config(default = "None")]
    pub setting: DarkModeSetting,
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
    pub wallpaper_mode: String,
}

impl Default for TagsModeConfig {
    fn default() -> Self {
        Self {
            light: default_light_tags(),
            dark: default_dark_tags(),
        }
    }
}

impl DarkModeSetting {
    pub fn as_str(&self) -> &'static str {
        match self {
            DarkModeSetting::Dark => "dark",
            DarkModeSetting::Light => "light",
            DarkModeSetting::System => "system",
            DarkModeSetting::None => "none",
        }
    }
}

impl fmt::Display for DarkModeSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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

fn default_dark_mode_setting() -> DarkModeSetting {
    DarkModeSetting::None
}

#[derive(Debug, Parser)]
#[command(name = "digikam-wallpaper")]
struct Cli {
    /// Configuration file (TOML)
    #[arg(long)]
    config: Option<PathBuf>,
    /// The path to the digikam sqlite database file
    #[arg(long)]
    db_path: Option<String>,
    /// The wallpaper mode to set
    #[arg(long)]
    wallpaper_mode: Option<String>,
    /// The dark mode settings to apply
    #[arg(long)]
    dark_mode: Option<DarkModeSetting>,
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
