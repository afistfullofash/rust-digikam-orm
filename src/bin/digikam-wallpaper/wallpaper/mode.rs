use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

/// The setting mode of the Wallpaper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
pub enum WallpaperMode {
    /// Center the wallpaper
    #[default]
    Center,
    /// Crop the Wallpaper to fit
    Crop,
    /// Force the Wallpaper to fit the Screen
    Fit,
    /// Force the Wallpaper to span the Screen
    Span,
    /// Stretch the Wallpaper to the Screen Size
    Stretch,
    /// Tile the wallpaper
    Tile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseWallpaperModeError;

impl fmt::Display for ParseWallpaperModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid wallpaper mode")
    }
}

impl std::error::Error for ParseWallpaperModeError {}

impl FromStr for WallpaperMode {
    type Err = ParseWallpaperModeError;
    /// Create a `WallpaperMode` enum from a string
    ///
    /// # Arguments
    /// * `s` - A wallpaper mode. It will be converted to lowercase before comparing
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "center" => Ok(Self::Center),
            "crop" => Ok(Self::Crop),
            "fit" => Ok(Self::Fit),
            "span" => Ok(Self::Span),
            "stretch" => Ok(Self::Stretch),
            "tile" => Ok(Self::Tile),
            _ => Err(ParseWallpaperModeError),
        }
    }
}

impl fmt::Display for WallpaperMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Center => "center",
            Self::Crop => "crop",
            Self::Fit => "fit",
            Self::Span => "span",
            Self::Stretch => "stretch",
            Self::Tile => "tile",
        };

        f.write_str(s)
    }
}

impl From<WallpaperMode> for wallpaper_bce::Mode {
    fn from(mode: WallpaperMode) -> Self {
        match mode {
            WallpaperMode::Center => wallpaper_bce::Mode::Center,
            WallpaperMode::Crop => wallpaper_bce::Mode::Crop,
            WallpaperMode::Fit => wallpaper_bce::Mode::Fit,
            WallpaperMode::Span => wallpaper_bce::Mode::Span,
            WallpaperMode::Stretch => wallpaper_bce::Mode::Stretch,
            WallpaperMode::Tile => wallpaper_bce::Mode::Tile,
        }
    }
}

impl From<wallpaper_bce::Mode> for WallpaperMode {
    fn from(mode: wallpaper_bce::Mode) -> Self {
        match mode {
            wallpaper_bce::Mode::Center => WallpaperMode::Center,
            wallpaper_bce::Mode::Crop => WallpaperMode::Crop,
            wallpaper_bce::Mode::Fit => WallpaperMode::Fit,
            wallpaper_bce::Mode::Span => WallpaperMode::Span,
            wallpaper_bce::Mode::Stretch => WallpaperMode::Stretch,
            wallpaper_bce::Mode::Tile => WallpaperMode::Tile,
        }
    }
}
