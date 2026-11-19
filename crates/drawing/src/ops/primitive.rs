//! The [`DrawPrimitiveOps`] capability trait.

use smearor_commons_color::Color;

/// Primitive surface operations: background fills and (in later phases)
/// rect/ring/dot highlight primitives over `Rect`/`Circle` geometry.
///
/// The methods only write the declared pixel area; row padding and memory
/// beyond the view extent stay untouched.
pub trait DrawPrimitiveOps {
    /// Fill the entire view with a solid background color.
    fn fill_background(&mut self, color: impl Into<Color<u8>>);
}
