//! # smearor-commons-noise
//!
//! Deterministic procedural noise: the Hoskins hash family as a generic
//! [`Hash`] trait, table-free simplex fields ([`Simplex2D`],
//! [`Simplex3D`]) and a generic fractal-Brownian-motion combinator
//! ([`Fbm`]) - all over [`Vec2`]/[`Vec3`]/[`Vec4`] from
//! `smearor-commons-geometry`.
//!
//! ## Determinism
//!
//! The hash family is sine-free (`fract`, multiply, add only - David
//! Hoskins' "hash without sine" recipe), so it is bitwise stable across
//! platforms and math libraries. Both simplex fields build their
//! gradients from `hash22`/`hash33` (the Burn-My-Windows/MIT-licensed
//! variants), which keeps the whole crate deterministic: a given
//! [`Seed`] always replays the same field.

mod fbm;
mod field;
mod hash;
mod seed;
mod simplex_2d;
mod simplex_3d;

pub use fbm::Fbm;
pub use fbm::Gain;
pub use fbm::Lacunarity;
pub use fbm::Octaves;
pub use field::Field;
pub use hash::Hash;
pub use seed::Seed;
pub use simplex_2d::Simplex2D;
pub use simplex_2d::Simplex2DFractal;
pub use simplex_3d::Simplex3D;
pub use simplex_3d::Simplex3DFractal;
