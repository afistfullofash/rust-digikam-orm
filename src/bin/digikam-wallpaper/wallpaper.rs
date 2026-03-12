use rand::prelude::IndexedRandom;
use wallpaper_ng as wallpaper;

use rust_digikam_orm::Image;
use tracing::{error, info};

/// Convert the mode defined as a string into a value wallpaper-ng can use
pub fn mode_from_config(mode: &str) -> wallpaper_ng::Mode {
    match mode.trim().to_ascii_lowercase().as_str() {
        "center" => wallpaper_ng::Mode::Center,
        "crop" => wallpaper_ng::Mode::Crop,
        "fit" => wallpaper_ng::Mode::Fit,
        "span" => wallpaper_ng::Mode::Span,
        "stretch" => wallpaper_ng::Mode::Stretch,
        "tile" => wallpaper_ng::Mode::Tile,
        _ => wallpaper_ng::Mode::Center,
    }
}

/// Set a single wallpaper
///
/// # Arguments
/// wallpaper: A Vector of Images to select a wallpaper to set from
/// mode: The mode to set the wallpaper in. Such as "center", "tile"
pub fn set_wallpaper(wallpaper: Image, mode: wallpaper_ng::Mode) {
    let base_image_directory = wallpaper.clone().full_path.unwrap();

    let mut complete_image_path_buf = homedir::my_home().unwrap().unwrap();
    complete_image_path_buf.push(&base_image_directory);

    let complete_image_path = complete_image_path_buf.as_path().display().to_string();

    match wallpaper::set_from_path(&complete_image_path) {
        Ok(_) => {
            println!("Set wallpaper to: {}", complete_image_path);
            match wallpaper_ng::set_mode(mode.clone()) {
                Ok(_) => println!("Set Wallpaper Mode to: {:?}", mode),
                Err(e) => {
                    error!(error = ?e, "Error setting wallpaper mode");
                    println!("Error setting wallpaper mode")
                }
            }
        }
        Err(e) => {
            error!(error = ?e, "Error setting wallpaper");
            println!("Error setting wallpaper: {}", complete_image_path)
        }
    };
}

/// Get a random wallpaper from a selection
///
/// # Arguments
/// wallpapers: A Vector of Images to select from
fn get_random_wallpaper(wallpapers: Vec<Image>) -> Option<Image> {
    let current_wallpaper = wallpaper::get().unwrap_or_else(|_| "".to_string());
    let mut rng = rand::rng();
    match wallpapers.choose(&mut rng) {
        Some(candidate) => match &candidate.full_path {
            Some(path) => {
                info!(
                    current_wallpaper = current_wallpaper,
                    candidate = path,
                    "Checking if the chosen wallpaper is the same as the one already set"
                );

                if path != &current_wallpaper {
                    info!("The wallpaper is new setting");
                    Some(candidate.clone())
                } else {
                    get_random_wallpaper(wallpapers)
                }
            }
            None => {
                error!(wallpaper = ?candidate, "Error getting candidates path trying again");
                get_random_wallpaper(wallpapers)
            }
        },
        None => {
            error!("Could not choose wallpaper");
            None
        }
    }
}

///  Set a Random Wallpaper from a selection
///
/// # Arguments
/// wallpapers: A Vector of Images to select a wallpaper to set from
/// mode: The mode to set the wallpaper in. Such as "center", "tile"
pub fn set_random_wallpaper(wallpapers: Vec<Image>, mode: wallpaper_ng::Mode) {
    if wallpapers.len() == 1 {
        set_wallpaper(wallpapers[0].clone(), mode)
    } else if let Some(wallpaper) = get_random_wallpaper(wallpapers) {
        set_wallpaper(wallpaper, mode)
    }
}
