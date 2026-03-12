use diesel::prelude::*;
use std::process;
use tracing::{error, info};

mod config;
mod logging;
mod wallpaper;

use crate::config::get_config;
use crate::logging::init_logging;
use crate::wallpaper::{mode_from_config, set_random_wallpaper};

use rust_digikam_orm::Images;

fn main() {
    init_logging();
    let config = match get_config() {
        Some(config) => config,
        None => {
            process::exit(-1);
        }
    };

    info!(config = ?config, "Running with Config");

    let db_path = shellexpand::full(&config.db_path)
        .unwrap_or_else(|e| {
            error!(error = ?e, "Error expanding database path");
            println!("Error expanding database path");
            process::exit(-1);
        })
        .to_string();

    let connection = &mut SqliteConnection::establish(&db_path).unwrap_or_else(|e| {
        error!(error = ?e, "Error connecting to database");
        println!("Error connecting to database: {}", db_path);
        process::exit(-1);
    });

    let dark_mode_setting = config.dark_mode.setting.as_str();

    let dark_mode_tags = match dark_mode_setting {
        "dark" => config.dark_mode.tags.dark,
        "light" => config.dark_mode.tags.light,
        "system" => match dark_light::detect().unwrap_or_else(|e| {
            error!(error = ?e, "Error detecting system dark mode");
            println!("Error detecting system dark mode");
            process::exit(-1);
        }) {
            dark_light::Mode::Dark => config.dark_mode.tags.dark,
            dark_light::Mode::Light => config.dark_mode.tags.light,
            dark_light::Mode::Unspecified => Vec::new(),
        },
        "none" => Vec::new(),
        _ => Vec::new(),
    };

    if dark_mode_setting != "none" {
        println!("Setting with dark mode set to: {}", dark_mode_setting);
        if dark_mode_setting == "system" {
            println!(
                "System Dark Mode Setting: {}",
                match dark_light::detect().unwrap() {
                    dark_light::Mode::Dark => "Dark",
                    dark_light::Mode::Light => "Light",
                    dark_light::Mode::Unspecified => "Unspecified",
                }
            );
        }
    }

    let tags = config.tags.into_iter().chain(dark_mode_tags).collect();

    let wallpapers = Images::get_by_tag_strings(connection, &tags);

    if wallpapers.is_empty() {
        println!("No images matched the tags given.");
        println!("Please ensure at least one image is tagged with the following:");
        for tag in tags {
            println!("  - {}", tag);
        }
    }

    let wallpaper_options = mode_from_config(&config.wallpaper_mode);
    set_random_wallpaper(wallpapers, wallpaper_options);

    process::exit(0);
}
