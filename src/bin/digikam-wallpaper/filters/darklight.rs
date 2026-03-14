//! # Light/Dark Mode Filter
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;
use tracing::error;

/// DarkLightFilter
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum DarkLightFilter {
    #[default]
    /// Disable light/dark mode filtering
    None,
    /// Apply Dark mode tags
    Dark,
    /// Apply Light mode tags
    Light,
    /// Apply according to system light/dark mode setting
    System,
}

impl DarkLightFilter {
    /// Given a `DarkLightFilter` resolve the Systems dark/light setting if the current `DarkLightFilter` is set to `DarkLightFilter::System`
    pub fn detect(self) -> DarkLightFilter {
        match self {
            DarkLightFilter::None => DarkLightFilter::None,
            DarkLightFilter::Dark => DarkLightFilter::Dark,
            DarkLightFilter::Light => DarkLightFilter::Light,
            DarkLightFilter::System => dark_light::detect()
                .unwrap_or_else(|e| {
                    error!(error = ?e, "Error detecting system dark mode");
                    println!("Error detecting system dark mode");
                    dark_light::Mode::Unspecified
                })
                .into(),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DarkLightFilter::Dark => "dark",
            DarkLightFilter::Light => "light",
            DarkLightFilter::System => "system",
            DarkLightFilter::None => "none",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseDarkLightFilterError;

impl fmt::Display for ParseDarkLightFilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid wallpaper mode")
    }
}

impl std::error::Error for ParseDarkLightFilterError {}

impl FromStr for DarkLightFilter {
    type Err = ParseDarkLightFilterError;
    /// Create a `DarkLightFilter` enum from a string
    ///
    /// # Arguments
    /// * `s` - A wallpaper mode. It will be converted to lowercase before comparing
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            "system" => Ok(Self::System),
            _ => Err(ParseDarkLightFilterError),
        }
    }
}

impl fmt::Display for DarkLightFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<dark_light::Mode> for DarkLightFilter {
    fn from(mode: dark_light::Mode) -> Self {
        match mode {
            dark_light::Mode::Dark => DarkLightFilter::Dark,
            dark_light::Mode::Light => DarkLightFilter::Light,
            dark_light::Mode::Unspecified => DarkLightFilter::None,
        }
    }
}

impl From<DarkLightFilter> for dark_light::Mode {
    fn from(mode: DarkLightFilter) -> Self {
        match mode {
            DarkLightFilter::Dark => dark_light::Mode::Dark,
            DarkLightFilter::Light => dark_light::Mode::Light,
            DarkLightFilter::None => dark_light::Mode::Unspecified,
            DarkLightFilter::System => mode.detect().into(),
        }
    }
}
