//! Named color vocabularies.
//!
//! Each vocabulary is a closed set of color names with a canonical
//! string form and a defined sRGB value:
//!
//! - [`CssNamedColor`]: the 149 color keywords of the W3C CSS Color
//!   Module Level 4 (`"rebeccapurple"`, `"transparent"`, ...)
//! - [`PantoneNamedColor`]: the 1342 coated (`C`) and 1335 uncoated
//!   (`U`) spot colors of the Pantone Matching System
//!   (`"Pantone 485 C"`, ...), whose suffix letter is a
//!   [`PantoneSuffix`]
//! - [`PantoneFhiNamedColor`]: the 2310 colors of the Pantone
//!   Fashion, Home + Interiors system (`"Pantone 19-4057 TCX True
//!   Blue"`, ...), whose material code is a [`PantoneFhiSuffix`]

mod css;
mod fhi;
mod fhi_suffix;
mod pantone;
mod pantone_suffix;

pub use css::CssNamedColor;
pub use fhi::PantoneFhiNamedColor;
pub use fhi_suffix::PantoneFhiSuffix;
pub use pantone::PantoneNamedColor;
pub use pantone_suffix::PantoneSuffix;
