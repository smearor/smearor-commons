use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// Which way a regular polygon ([`Hexagon`](crate::Hexagon),
/// [`Octagon`](crate::Octagon)) points.
///
/// `FlatTop` gives the polygon a horizontal top edge (mirrored at
/// the bottom); `PointyTop` gives it a vertex on top. The edge
/// arrangement on the sides follows the polygon's count - left and
/// right *vertices* on a `Hexagon`/`PointyTop` `Octagon`, vertical
/// *edges* on a `FlatTop` `Octagon`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
pub enum PolygonOrientation {
    /// A horizontal top edge.
    #[default]
    FlatTop,
    /// A vertex on top.
    PointyTop,
}

impl fmt::Display for PolygonOrientation {
    /// The spelling accepted by [`FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FlatTop => write!(f, "flat-top"),
            Self::PointyTop => write!(f, "pointy-top"),
        }
    }
}

impl FromStr for PolygonOrientation {
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
    use super::PolygonOrientation;
    use std::str::FromStr;

    /// `Display`/`FromStr` round-trip both orientations.
    #[test]
    fn display_from_str_round_trip() {
        for orientation in [PolygonOrientation::FlatTop, PolygonOrientation::PointyTop] {
            assert_eq!(PolygonOrientation::from_str(&orientation.to_string()).unwrap(), orientation);
        }
        assert!(PolygonOrientation::from_str("diagonal").is_err());
    }
}
