//! The [`Cap`] stroke end style.

/// How an open stroke path ends at its two endpoints (`Butt` cuts at
/// the endpoint, `Round` extends a semicircle, `Square` extends a
/// half-width square).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Cap {
    /// The stroke ends exactly at the endpoint (cairo's default).
    #[default]
    Butt,
    /// A semicircle of half the stroke width extends past the endpoint.
    Round,
    /// A square of half the stroke width extends past the endpoint.
    Square,
}

#[cfg(feature = "cairo")]
impl From<Cap> for cairo::LineCap {
    /// The cairo line-cap matching `cap` one to one.
    fn from(cap: Cap) -> Self {
        match cap {
            Cap::Butt => Self::Butt,
            Cap::Round => Self::Round,
            Cap::Square => Self::Square,
        }
    }
}
