use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// Which way a regular [`Hexagon`](crate::Hexagon) points.
///
/// `FlatTop` gives the hexagon horizontal top and bottom edges with
/// the vertices pointing left and right; `PointyTop` gives it
/// vertical left and right edges with a vertex on top and bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
pub enum HexOrientation {
    /// Horizontal top and bottom edges - vertices left and right.
    #[default]
    FlatTop,
    /// Vertical left and right edges - a vertex on top and bottom.
    PointyTop,
}

impl fmt::Display for HexOrientation {
    /// The spelling accepted by [`FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FlatTop => write!(f, "flat-top"),
            Self::PointyTop => write!(f, "pointy-top"),
        }
    }
}

impl FromStr for HexOrientation {
    type Err = String;

    /// Accepts `flat-top` or `pointy-top`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "flat-top" => Ok(Self::FlatTop),
            "pointy-top" => Ok(Self::PointyTop),
            _ => Err(format!("invalid hexagon orientation '{s}' - use flat-top or pointy-top")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HexOrientation;
    use std::str::FromStr;

    /// `Display`/`FromStr` round-trip both orientations.
    #[test]
    fn display_from_str_round_trip() {
        for orientation in [HexOrientation::FlatTop, HexOrientation::PointyTop] {
            assert_eq!(HexOrientation::from_str(&orientation.to_string()).unwrap(), orientation);
        }
        assert!(HexOrientation::from_str("diagonal").is_err());
    }
}
