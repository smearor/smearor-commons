//! Layer-shell layer selection for the Smearor ecosystem.
//!
//! [`SmearorLayer`] is the ecosystem's vocabulary for the four
//! layer-shell layers: it parses from CLI arguments ([`FromStr`]),
//! deserializes from configuration files (`serde`) and converts into
//! the backend [`gtk4_layer_shell::Layer`] when the
//! `gtk4-layer-shell` feature is enabled.
//!
//! `serde` is unconditional; backend conversions live behind their
//! feature gate.

#[cfg(feature = "gtk4-layer-shell")]
mod gtk4_layer_shell;
mod parse_layer_error;
mod smearor_layer;

pub use parse_layer_error::ParseLayerError;
pub use smearor_layer::SmearorLayer;
