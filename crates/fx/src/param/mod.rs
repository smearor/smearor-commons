//! The parameter metadata model - [`ParamSpec`] declares one
//! parameter's key/label/kind, [`ParamKind`] its widget + schema
//! default, [`ParamValue`] is the live read/write currency.

mod kind;
mod spec;
mod value;

pub use kind::ParamKind;
pub use spec::ParamSpec;
pub use value::ParamValue;
