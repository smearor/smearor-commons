use serde::Deserialize;
use serde::Serialize;
use std::ops::Div;
use std::ops::Sub;

/// An HSV value / brightness (0.0-1.0), clamped on construction.
///
/// `0.0` is black, `1.0` is full brightness. Newtype so a value
/// cannot be confused with a channel value or a percent. Used by
/// [`crate::Hsv`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Value(f64);

impl Value {
    /// Minimum value (black).
    pub const MIN: Self = Self(0.0);

    /// Maximum value (full brightness).
    pub const MAX: Self = Self(1.0);

    /// Creates a value, clamped to `0.0-1.0`.
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

/// Saturating difference - a negative result clamps to [`Value::MIN`].
impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

/// Ratio of two values as a plain factor.
impl Div for Value {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        value.0
    }
}
