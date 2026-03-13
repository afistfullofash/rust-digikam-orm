use diesel::prelude::*;
use filters::DarkLightFilter;
use std::process;
use tracing::{error, info};

mod config;
mod filters;
mod logging;
mod wallpaper;

use crate::config::get_config;
use crate::logging::init_logging;
use crate::wallpaper::{set_random_wallpaper, WallpaperOptions};

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

    let dark_light_filter = config.dark_mode.setting.detect();

    let dark_mode_tags = match dark_light_filter {
        DarkLightFilter::Dark => config.dark_mode.tags.dark,
        DarkLightFilter::Light => config.dark_mode.tags.light,
        // We should not have System after running detect()
        DarkLightFilter::System => Vec::new(),
        DarkLightFilter::None => Vec::new(),
    };

    if dark_light_filter != DarkLightFilter::None {
        println!(
            "Setting with dark mode set to: {}",
            dark_light_filter.as_str()
        );
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

    let wallpaper_options = WallpaperOptions {
        mode: config.wallpaper_mode,
    };
    set_random_wallpaper(wallpapers, wallpaper_options);

    process::exit(0);
}
