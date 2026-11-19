//! The cairo backend: [`CairoTarget`] implements the shared capability
//! vocabulary on a `cairo::Context`.
//!
//! Feature-gated behind `cairo` and headless-safe — `cairo-rs` alone
//! pulls no display dependency, so this backend is testable without a
//! compositor. The capability impls land per trait (`primitive_ops`
//! first); backend-specific extras stay reachable through
//! [`CairoTarget::context`] instead of widening the shared traits.

mod primitive_ops;
mod target;

pub use target::CairoTarget;
