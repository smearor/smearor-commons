//! The shipped effects - one struct per file, the `EffectSpec`
//! variant payloads.

mod aura_glow;
mod crossfade;
mod fire;
mod glide;
mod matrix;
mod pixel_wheel;

pub use aura_glow::AuraGlow;
pub use crossfade::Crossfade;
pub use fire::Fire;
pub use fire::FireColor;
pub use glide::Glide;
pub use matrix::GlyphSource;
pub use matrix::Matrix;
pub use matrix::ProceduralGlyphAtlas;
pub use pixel_wheel::PixelWheel;
