//! `Seed` - deterministic per-run seed for noise-driven effects.

use crate::Hash;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Deterministic per-run seed - wraps the raw `u64` so call sites
/// cannot accidentally feed an arbitrary offset where the run's seed
/// belongs. Serializes as a plain number.
///
/// Seeding follows the same convention the shader sources use: the
/// seed is folded into the *coordinates* (`offset_2d`/`offset_3d`)
/// rather than re-randomizing the field - which keeps one field
/// instance reusable for any number of seeded runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Seed(pub u64);

impl Seed {
    /// A nondeterministic seed from the system clock - the
    /// `Math.random()` equivalent for effect runs.
    pub fn random() -> Self {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or_default();
        Self(nanos ^ 0x9E37_79B9_7F4A_7C15)
    }

    /// A decorrelated sibling seed - for particle systems that want
    /// several independent fields from one run seed.
    pub fn mix(&self, other: &Seed) -> Seed {
        let mut z = self.0 ^ other.0.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Seed(z ^ (z >> 31))
    }

    /// The seed as a `[0, 1)` scalar - the `uSeed` float equivalent.
    pub fn as_f64(&self) -> f64 {
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }

    /// A deterministic `Vec2` coordinate offset derived from the seed -
    /// add it to sampled coordinates to vary a run without touching the
    /// field.
    pub fn offset_2d(&self) -> Vec2<()> {
        let unit: Vec2<()> = self.as_f64().hash();
        unit * 1000.0
    }

    /// A deterministic `Vec3` coordinate offset derived from the seed.
    pub fn offset_3d(&self) -> Vec3<()> {
        let unit: Vec3<()> = self.as_f64().hash();
        unit * 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_is_deterministic_and_decorrelated() {
        assert_eq!(Seed(1).mix(&Seed(2)), Seed(1).mix(&Seed(2)));
        assert_ne!(Seed(1).mix(&Seed(2)), Seed(2).mix(&Seed(1)));
    }

    #[test]
    fn serde_roundtrip_is_a_plain_number() {
        let json = serde_json::to_string(&Seed(42)).unwrap();
        assert_eq!(json, "42");
        assert_eq!(serde_json::from_str::<Seed>(&json).unwrap(), Seed(42));
    }
}
