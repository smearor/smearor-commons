use crate::ParseLayerError;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

/// The four layer-shell layers - the stacked order of surfaces from
/// desktop background to overlay.
///
/// The vocabulary shared by Smearor consumers: CLI arguments parse it
/// via [`FromStr`], configuration files deserialize it via `serde`
/// and [`From<&str>`] provides the lenient config-bridge conversion
/// that falls back to [`Top`](Self::Top) on unknown values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmearorLayer {
    /// The bottom-most layer, beneath regular windows.
    #[serde(alias = "background", alias = "Background", alias = "BACKGROUND")]
    Background,
    /// Above the background, beneath regular windows.
    #[serde(alias = "bottom", alias = "Bottom", alias = "BOTTOM")]
    Bottom,
    /// Above regular windows - the default.
    #[serde(alias = "top", alias = "Top", alias = "TOP")]
    #[default]
    Top,
    /// The top-most layer, above everything else.
    #[serde(alias = "overlay", alias = "Overlay", alias = "OVERLAY")]
    Overlay,
}

impl FromStr for SmearorLayer {
    type Err = ParseLayerError;

    /// Parses a layer name case-insensitively; unknown values are
    /// rejected - use [`From<&str>`] for the lenient fallback.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "background" => Ok(Self::Background),
            "bottom" => Ok(Self::Bottom),
            "top" => Ok(Self::Top),
            "overlay" => Ok(Self::Overlay),
            _ => Err(ParseLayerError::new(s)),
        }
    }
}

impl From<&str> for SmearorLayer {
    /// The lenient counterpart of [`FromStr`]: unknown values fall
    /// back to [`Top`](Self::Top). Used where a configuration value
    /// feeds a CLI argument and must not fail.
    fn from(s: &str) -> Self {
        s.parse().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::SmearorLayer;

    /// Layer names parse case-insensitively in all four variants.
    #[test]
    fn parses_all_layers_case_insensitively() {
        assert_eq!("background".parse(), Ok(SmearorLayer::Background));
        assert_eq!("Bottom".parse(), Ok(SmearorLayer::Bottom));
        assert_eq!("TOP".parse(), Ok(SmearorLayer::Top));
        assert_eq!("oVeRlAy".parse(), Ok(SmearorLayer::Overlay));
    }

    /// `FromStr` rejects unknown values while `From<&str>` falls
    /// back to `Top`.
    #[test]
    fn unknown_values_error_or_fall_back() {
        assert!("wallpaper".parse::<SmearorLayer>().is_err());
        assert_eq!(SmearorLayer::from("wallpaper"), SmearorLayer::Top);
    }

    /// Serde aliases accept the same case variants as `FromStr`.
    #[test]
    fn serde_aliases_deserialize() {
        assert_eq!(serde_json::from_str::<SmearorLayer>("\"Overlay\"").ok(), Some(SmearorLayer::Overlay));
        assert_eq!(serde_json::to_string(&SmearorLayer::Bottom).ok().as_deref(), Some("\"Bottom\""));
    }
}
