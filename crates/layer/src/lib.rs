//! Layer-shell vocabulary for the Smearor ecosystem.
//!
//! [`SmearorLayer`] is the shared name for the four layer-shell
//! layers: it parses from CLI arguments ([`FromStr`]), deserializes
//! from configuration files (`serde`) and converts into the backend
//! [`gtk4_layer_shell::Layer`] when the `gtk4-layer-shell` feature
//! is enabled. [`LayerConfig`] is the config quartet apps repeat
//! across arguments, files and state.
//!
//! With `gtk4-layer-shell` enabled, [`EdgeOf`] maps the snapped
//! four-orientation vocabularies - [`smearor_commons_geometry`]'s
//! `QuarterTurn`/`Rotation` and, behind the `layout` feature,
//! [`smearor_commons_layout`]'s `TableSide` - onto the layer-shell
//! `Edge` a surface anchors to.
//!
//! `serde` is unconditional; backend and domain integrations live
//! behind their feature gates.

#[cfg(feature = "gtk4-layer-shell")]
mod edge_of;
#[cfg(feature = "gtk4-layer-shell")]
mod gtk4_layer_shell;
mod layer_config;
mod parse_layer_error;
mod smearor_layer;

#[cfg(feature = "gtk4-layer-shell")]
pub use edge_of::EdgeOf;
pub use layer_config::LayerConfig;
pub use parse_layer_error::ParseLayerError;
pub use smearor_layer::SmearorLayer;
