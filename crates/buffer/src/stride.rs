use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// Bytes per row of a buffer, including row padding.
///
/// A byte measure, deliberately not `Pixels`: stride compares against
/// the packed row width (`width * bytes_per_pixel`) and carries
/// padding for alignment. Confusing stride with the buffer length or
/// the packed row bytes is the classic shared-memory buffer bug, so
/// the value is typed.
/// Serializes as a bare number.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Stride(pub u32);

impl Stride {
    /// Creates a stride value.
    pub const fn new(stride: u32) -> Self {
        Self(stride)
    }

    /// The stride in bytes as `u32`.
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// The stride in bytes as `usize` for slice arithmetic.
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for Stride {
    fn from(stride: u32) -> Self {
        Self(stride)
    }
}

impl From<Stride> for u32 {
    fn from(stride: Stride) -> Self {
        stride.0
    }
}

impl From<Stride> for usize {
    fn from(stride: Stride) -> Self {
        stride.0 as usize
    }
}

impl Display for Stride {
    /// The bare byte count.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
