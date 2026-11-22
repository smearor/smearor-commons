//! # smearor-commons-fx
//!
//! Headless, serializable per-pixel effects over frozen textures -
//! the Rust analogue of Burn-My-Windows' shader effects, generalized
//! to any rectangular content (widgets, cards, sprites) a consumer
//! can snapshot into a [`PixelBuffer`](smearor_commons_buffer::PixelBuffer).
//!
//! ## Architecture
//!
//! - [`FxContext`] is the uniform block an effect sees for one frame
//!   (progress, direction, seed, textures) - the crate is clock-free;
//!   consumers drive it with `TimedAnimation`.
//! - [`WindowEffect`] is the effect abstraction: identity, parameter
//!   metadata, frame application.
//! - [`ParamSpec`]/[`ParamKind`]/[`ParamValue`] are the metadata model
//!   that generates GTK controls, serde defaults and codegen.
//! - [`EffectSpec`] is the serde-tagged dispatch root - one variant
//!   per shipped effect, serializing as `{"effect": "crossfade", ...}`.
//!
//! The default feature set is pure-std headless; `gtk4`, `cairo`,
//! `schemars` integrations live behind dependency-named features.

mod context;
mod cycle_speed;
mod direction;
mod effect;
mod effects;
mod error;
mod layer;
mod layers;
mod ops;
mod param;
mod render;
mod spec;
mod spoke_count;

pub use context::FxContext;
pub use cycle_speed::CycleSpeed;
pub use direction::Direction;
pub use effect::WindowEffect;
pub use effects::Apparition;
pub use effects::AuraGlow;
pub use effects::Crossfade;
pub use effects::Doom;
pub use effects::EnergizeA;
pub use effects::EnergizeB;
pub use effects::Fire;
pub use effects::FireColor;
pub use effects::Glide;
pub use effects::Glitch;
pub use effects::GlyphSource;
pub use effects::Hexagon;
pub use effects::Incinerate;
pub use effects::Materialize;
pub use effects::Matrix;
pub use effects::Mushroom;
pub use effects::PixelWheel;
pub use effects::PixelWipe;
pub use effects::Pixelate;
pub use effects::Portal;
pub use effects::ProceduralGlyphAtlas;
pub use effects::RgbWarp;
pub use effects::Shredder;
pub use effects::StarColor;
pub use effects::Tv;
pub use effects::TvGlitch;
pub use effects::VenetianBlinds;
pub use effects::Wisps;
pub use error::FxError;
pub use layer::Layer;
pub use layers::Layers;
pub use ops::absolute_edge_mask;
pub use ops::alpha_over;
pub use ops::darken;
pub use ops::fract;
pub use ops::lighten;
pub use ops::mix;
pub use ops::offset_hue;
pub use ops::relative_edge_mask;
pub use ops::remap;
pub use ops::smoothstep;
pub use ops::tritone;
pub use ops::whirl;
pub use param::ParamKind;
pub use param::ParamSpec;
pub use param::ParamValue;
pub use spec::EffectSpec;
pub use spoke_count::SpokeCount;
