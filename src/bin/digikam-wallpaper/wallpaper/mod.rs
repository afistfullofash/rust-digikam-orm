mod mode;
#[allow(clippy::module_inception)]
mod wallpaper;

pub use mode::WallpaperMode;
pub use wallpaper::{set_random_wallpaper, WallpaperOptions};
