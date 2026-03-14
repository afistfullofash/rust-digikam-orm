mod mode;
#[allow(clippy::module_inception)]
mod wallpaper;

pub use mode::WallpaperMode;
pub use wallpaper::{WallpaperOptions, set_random_wallpaper};
