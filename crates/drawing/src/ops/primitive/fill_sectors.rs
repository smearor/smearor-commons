//! The [`FillSectors`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Annulus;
use smearor_commons_geometry::Arc;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::RotationRad;

/// A band of `count` equal annulus sectors partitioning `arc` - each
/// sector shrunk symmetrically by `gap` around every boundary and
/// filled cycling `colors`: the lifebuoy / donut / dial pattern.
/// `colors.is_empty()` paints nothing.
#[derive(Debug, Clone, PartialEq)]
pub struct FillSectors {
    /// The ring the sectors are cut from.
    pub annulus: Annulus<Pixels>,
    /// The angular interval partitioned into `count` sectors -
    /// `arc.start` is the band's phase.
    pub arc: Arc,
    /// The sector count.
    pub count: usize,
    /// The angular gap around every sector boundary.
    pub gap: RotationRad,
    /// The colors cycled over the sectors.
    pub colors: Vec<Color<f64>>,
    /// Alpha composited on top of each color's own alpha channel.
    pub alpha: Alpha,
}

impl FillSectors {
    /// Creates a sector-band fill spec.
    pub fn new(annulus: Annulus<Pixels>, arc: Arc, count: usize, gap: RotationRad, colors: &[Color<f64>], alpha: impl Into<Alpha>) -> Self {
        Self {
            annulus,
            arc,
            count,
            gap,
            colors: colors.to_vec(),
            alpha: alpha.into(),
        }
    }
}
