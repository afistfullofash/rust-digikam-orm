use rand::prelude::IndexedRandom;
use wallpaper_ng as wallpaper;

use rust_digikam_orm::Image;
use tracing::{error, info};

use crate::wallpaper::WallpaperMode;

/// The Options used when setting the Wallpaper
pub struct WallpaperOptions {
    /// How to fit the wallpaper to the screen
    pub mode: WallpaperMode,
}

/// Set a single wallpaper
///
/// # Arguments
/// * `wallpaper` - A `Vec` of `Image` to select a wallpaper to set from
/// * `mode` - The mode to set the wallpaper in. Such as "center", "tile"
pub fn set_wallpaper(wallpaper: Image, options: WallpaperOptions) {
    let Some(base_image_directory) = wallpaper.path() else {
        error!(wallpaper = ?wallpaper, "Cannot set wallpaper without full image path");
        return;
    };

    let mut complete_image_path_buf = homedir::my_home().unwrap().unwrap();
    complete_image_path_buf.push(&base_image_directory);

    let complete_image_path = complete_image_path_buf.as_path().display().to_string();

    match wallpaper::set_from_path(&complete_image_path) {
        Ok(_) => {
            println!("Set wallpaper to: {}", complete_image_path);
            let wallpaper_ng_mode: wallpaper_ng::Mode = options.mode.into();
            match wallpaper_ng::set_mode(wallpaper_ng_mode) {
                Ok(_) => println!("Set Wallpaper Mode to: {:?}", options.mode),
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
/// * `wallpapers` - A `Vec` of `Image` to select from
fn get_random_wallpaper(wallpapers: Vec<Image>) -> Option<Image> {
    let current_wallpaper = wallpaper::get().unwrap_or_else(|_| "".to_string());
    let mut rng = rand::rng();
    match wallpapers.choose(&mut rng) {
        Some(candidate) => match candidate.path() {
            Some(path) => {
                info!(
                    current_wallpaper = current_wallpaper,
                    candidate = path,
                    "Checking if the chosen wallpaper is the same as the one already set"
                );

                if path != current_wallpaper {
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
/// * `wallpapers` - A `Vec` of `Image` to select a wallpaper to set from
/// * `mode` - The mode to set the wallpaper to
pub fn set_random_wallpaper(wallpapers: Vec<Image>, options: WallpaperOptions) {
    if wallpapers.len() == 1 {
        set_wallpaper(wallpapers[0].clone(), options)
    } else if let Some(wallpaper) = get_random_wallpaper(wallpapers) {
        set_wallpaper(wallpaper, options)
    }
}
