use clap::{Parser, ValueEnum};
use confique::{json5, Config};
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use tracing::error;

#[derive(Debug, Config, Deserialize)]
pub struct TagsModeConfig {
    #[config(default = ["/Style/Asthetic/Light"])]
    pub light: Vec<String>,
    #[config(default = ["/Style/Asthetic/Dark"])]
    pub dark: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum DarkModeSetting {
    None,
    Dark,
    Light,
    System,
}

#[derive(Debug, Config, Deserialize)]
pub struct DarkModeConfig {
    #[config(nested)]
    pub tags: TagsModeConfig,
    #[config(default = "None")]
    pub setting: DarkModeSetting,
}

#[derive(Debug, Config, Deserialize)]
pub struct AppConfig {
    #[config(default = ["/Size/wallpaper"])]
    pub tags: Vec<String>,
    #[config(nested)]
    pub dark_mode: DarkModeConfig,
    #[config(default = "~/Pictures/digikam4.db")]
    pub db_path: String,
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
    #[arg(long)]
    db_path: Option<String>,
    #[arg(long)]
    wallpaper_mode: Option<String>,
    #[arg(long)]
    dark_mode: Option<DarkModeSetting>,
    #[arg(long)]
    config_template: bool,
}

fn default_config_path() -> PathBuf {
    match dirs::config_dir() {
        Some(mut config_dir) => {
            config_dir.push("digikam-wallpaper/config.json");
            config_dir
        }
        None => PathBuf::from("./config.json"),
    }
}

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
                println!("No cli arguments supplied. Run <command> -h to see options. If this was unintentional.");
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
