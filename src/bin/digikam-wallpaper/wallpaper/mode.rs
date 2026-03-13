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

impl From<WallpaperMode> for wallpaper_ng::Mode {
    fn from(mode: WallpaperMode) -> Self {
        match mode {
            WallpaperMode::Center => wallpaper_ng::Mode::Center,
            WallpaperMode::Crop => wallpaper_ng::Mode::Crop,
            WallpaperMode::Fit => wallpaper_ng::Mode::Fit,
            WallpaperMode::Span => wallpaper_ng::Mode::Span,
            WallpaperMode::Stretch => wallpaper_ng::Mode::Stretch,
            WallpaperMode::Tile => wallpaper_ng::Mode::Tile,
        }
    }
}

impl From<wallpaper_ng::Mode> for WallpaperMode {
    fn from(mode: wallpaper_ng::Mode) -> Self {
        match mode {
            wallpaper_ng::Mode::Center => WallpaperMode::Center,
            wallpaper_ng::Mode::Crop => WallpaperMode::Crop,
            wallpaper_ng::Mode::Fit => WallpaperMode::Fit,
            wallpaper_ng::Mode::Span => WallpaperMode::Span,
            wallpaper_ng::Mode::Stretch => WallpaperMode::Stretch,
            wallpaper_ng::Mode::Tile => WallpaperMode::Tile,
        }
    }
}
