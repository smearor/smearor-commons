//! Layer-shell vocabulary for the Smearor ecosystem.
//!
//! [`SmearorLayer`] is the ecosystem's vocabulary for the four
//! layer-shell layers: it parses from CLI arguments ([`FromStr`]),
//! deserializes from configuration files (`serde`) and converts into
//! the backend [`gtk4_layer_shell::Layer`] when the
//! `gtk4-layer-shell` feature is enabled.
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
mod parse_layer_error;
mod smearor_layer;

pub use parse_layer_error::ParseLayerError;
pub use smearor_layer::SmearorLayer;
