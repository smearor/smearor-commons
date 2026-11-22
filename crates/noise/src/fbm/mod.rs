//! `Fbm` - generic fractal Brownian motion over any [`Field`].

mod gain;
mod lacunarity;
mod octaves;

pub use gain::Gain;
pub use lacunarity::Lacunarity;
pub use octaves::Octaves;

use crate::field::Field;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Mul;

/// Generic fractal Brownian motion over any base [`Field`]: sums
/// `octaves` scaled-and-faded copies of the field, `lacunarity` scaling
/// frequency and `gain` scaling amplitude per octave.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Fbm {
    /// How many octaves are summed.
    pub octaves: Octaves,
    /// Amplitude falloff per octave.
    pub gain: Gain,
    /// Frequency growth per octave.
    pub lacunarity: Lacunarity,
}

impl Fbm {
    /// Classic parity configuration: 4 octaves, gain 0.5, lacunarity 2.0.
    pub const FOUR_OCTAVES: Fbm = Fbm {
        octaves: Octaves(4),
        gain: Gain(0.5),
        lacunarity: Lacunarity(2.0),
    };

    /// Sums `field.at(p * frequency) * amplitude` over the octaves.
    /// `Octaves(0)` behaves as one octave - the field is always sampled.
    pub fn at<P: Copy + Mul<f64, Output = P>>(&self, field: &impl Field<P>, p: P) -> f64 {
        let mut sum = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        for _ in 0..self.octaves.0.max(1) {
            sum += field.at(p * frequency) * amplitude;
            amplitude *= self.gain.0;
            frequency *= self.lacunarity.0;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Simplex2D;
    use smearor_commons_geometry::Vec2;

    #[test]
    fn more_octaves_add_detail() {
        let p = Vec2::<()>::new(0.37, -0.61);
        let one = Fbm {
            octaves: Octaves(1),
            ..Fbm::FOUR_OCTAVES
        }
        .at(&Simplex2D, p);
        let four = Fbm::FOUR_OCTAVES.at(&Simplex2D, p);
        assert_ne!(one, four);
    }

    #[test]
    fn fbm_is_deterministic() {
        let p = Vec2::<()>::new(1.25, 3.5);
        assert_eq!(Fbm::FOUR_OCTAVES.at(&Simplex2D, p), Fbm::FOUR_OCTAVES.at(&Simplex2D, p));
    }

    #[test]
    fn serde_roundtrip() {
        let fbm = Fbm {
            octaves: Octaves(6),
            gain: Gain(0.4),
            lacunarity: Lacunarity(1.9),
        };
        let json = serde_json::to_string(&fbm).unwrap();
        assert_eq!(json, r#"{"octaves":6,"gain":0.4,"lacunarity":1.9}"#);
        assert_eq!(serde_json::from_str::<Fbm>(&json).unwrap(), fbm);
    }
}
