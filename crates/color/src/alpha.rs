/// A normalized alpha value (0.0-1.0), clamped on construction.
///
/// `Alpha` is a unit: the canonical representation is always `f64`;
/// `u8`/`f32` channel values are encodings of it and convert via
/// `From`/`as_u8`/`as_f32`. The `a` channel of [`crate::Color<T>`]
/// stays `T` — `Alpha` is the standalone scalar for APIs that take
/// or return an opacity.
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Alpha(f64);

impl Alpha {
    /// Fully transparent.
    pub const TRANSPARENT: Self = Self(0.0);

    /// Fully opaque.
    pub const OPAQUE: Self = Self(1.0);

    /// Creates an alpha value, clamped to `0.0-1.0`.
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// The value as an `f32` channel.
    pub fn as_f32(self) -> f32 {
        self.0 as f32
    }

    /// The value as a `u8` channel (0-255, rounded).
    pub fn as_u8(self) -> u8 {
        (self.0 * 255.0).round() as u8
    }

    /// Whether the alpha is fully transparent - the rasterizer can
    /// skip the pixel entirely.
    pub fn is_transparent(self) -> bool {
        self.0 <= 0.0
    }

    /// Whether the alpha is fully opaque - blending can write the
    /// source color directly.
    pub fn is_opaque(self) -> bool {
        self.0 >= 1.0
    }
}

/// Saturating difference - a negative result clamps to [`Alpha::TRANSPARENT`].
impl Sub for Alpha {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

/// Scales the alpha by a plain factor (e.g. per-pixel coverage),
/// clamped back to the unit range.
impl Mul<f64> for Alpha {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.0 * rhs)
    }
}

/// Ratio of two alpha values as a plain factor (e.g. a blend amount).
impl Div for Alpha {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Default for Alpha {
    fn default() -> Self {
        Self::OPAQUE
    }
}

impl From<f64> for Alpha {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<f32> for Alpha {
    fn from(value: f32) -> Self {
        Self::new(f64::from(value))
    }
}

impl From<u8> for Alpha {
    fn from(value: u8) -> Self {
        Self::new(f64::from(value) / 255.0)
    }
}

impl From<Alpha> for f64 {
    fn from(alpha: Alpha) -> Self {
        alpha.0
    }
}

impl From<Alpha> for f32 {
    fn from(alpha: Alpha) -> Self {
        alpha.0 as f32
    }
}

impl From<Alpha> for u8 {
    fn from(alpha: Alpha) -> Self {
        alpha.as_u8()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clamps_to_unit_range() {
        assert_eq!(Alpha::new(1.5), Alpha::OPAQUE);
        assert_eq!(Alpha::new(-0.5), Alpha::TRANSPARENT);
    }

    #[test]
    fn u8_round_trip() {
        assert_eq!(Alpha::from(128u8).as_u8(), 128);
        assert_eq!(u8::from(Alpha::OPAQUE), 255);
    }

    #[test]
    fn default_is_opaque() {
        assert_eq!(Alpha::default(), Alpha::OPAQUE);
    }

    #[test]
    fn sub_saturates_at_transparent() {
        let delta = Alpha::OPAQUE - Alpha::new(0.25);
        assert!((delta.as_f64() - 0.75).abs() < 1e-9);
        assert_eq!(Alpha::new(0.25) - Alpha::OPAQUE, Alpha::TRANSPARENT);
    }

    #[test]
    fn predicates() {
        assert!(Alpha::TRANSPARENT.is_transparent());
        assert!(Alpha::OPAQUE.is_opaque());
        assert!(!Alpha::OPAQUE.is_transparent());
        assert!(!Alpha::TRANSPARENT.is_opaque());
    }

    #[test]
    fn mul_scales_and_clamps() {
        assert_eq!(Alpha::OPAQUE * 0.5, Alpha::new(0.5));
        assert_eq!(Alpha::OPAQUE * 2.0, Alpha::OPAQUE);
    }

    #[test]
    fn div_yields_ratio() {
        assert!((Alpha::new(0.5) / Alpha::new(0.25) - 2.0).abs() < 1e-9);
        assert_eq!(Alpha::new(0.5) / Alpha::OPAQUE, 0.5);
    }
}
