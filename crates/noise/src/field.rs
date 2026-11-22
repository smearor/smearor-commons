//! `Field` - a scalar noise field sampled at a point.

/// A scalar noise field sampled at a point `P`.
///
/// Implemented by the simplex family ([`crate::Simplex2D`],
/// [`crate::Simplex3D`], the fractal variants) and usable as the base
/// field of [`crate::Fbm::at`].
pub trait Field<P> {
    /// The field value at `p` - deterministic for a given field.
    fn at(&self, p: P) -> f64;
}
