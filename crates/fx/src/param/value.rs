use serde::Deserialize;
use serde::Serialize;
use smearor_commons_color::Color;

/// A live parameter value - the UI's read/write currency.
///
/// Untagged: JSON has no kind tag; the pairing with [`super::ParamKind`]
/// (same index in `params()`) is what makes a value meaningful.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamValue {
    /// A float parameter.
    Float(f64),
    /// A dropdown index.
    Choice(usize),
    /// An integer parameter.
    Int(i64),
    /// A toggle.
    Bool(bool),
    /// A color parameter (serializes as the `Color<f64>` string form).
    Color(Color<f64>),
}

impl ParamValue {
    /// The float payload - `None` when this is a different kind
    /// (`Int` is *not* widened; the kinds stay distinct).
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// The integer payload - `None` when this is a different kind.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// The bool payload - `None` when this is a different kind.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// The color payload - `None` when this is a different kind.
    pub fn as_color(&self) -> Option<Color<f64>> {
        match self {
            Self::Color(value) => Some(*value),
            _ => None,
        }
    }

    /// The choice index payload - `None` when this is a different kind.
    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Self::Choice(value) => Some(*value),
            _ => None,
        }
    }
}
