//! The `Fire` effect - `Fire` plus its private machinery: the
//! [`FireColor`] ramp palette, the dual-phase [`mask`], and the
//! 5-stop [`ramp`] evaluator.

mod color;
mod effect;
mod mask;
mod ramp;

pub use color::FireColor;
pub use effect::Fire;
