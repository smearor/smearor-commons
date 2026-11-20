//! The [`Icon`] draw spec — an icon addressed by name.

use ab_glyph::FontVec;
use smearor_commons_color::Color;

/// An icon draw specification: which icon, resolved through which
/// font and codepoint resolver, in which state and color.
///
/// The resolver is borrowed dynamically — it runs once per call, so
/// dynamic dispatch costs nothing and the spec stays concrete (no
/// `Icon<F>` generic threaded through forwarding impls).
#[derive(Clone, Copy)]
pub struct Icon<'a> {
    /// Icon font; `None` falls back to the placeholder.
    pub font: Option<&'a FontVec>,
    /// Icon name resolved through `resolve`.
    pub name: &'a str,
    /// Active-state flag for the default color policy.
    pub is_active: bool,
    /// Consumer-side name → codepoint resolver.
    pub resolve: &'a dyn Fn(&str) -> Option<char>,
    /// Optional color override — applies to the placeholder as well.
    pub color: Option<Color<f64>>,
}

impl<'a> Icon<'a> {
    /// Creates an icon spec.
    pub fn new(font: Option<&'a FontVec>, name: &'a str, is_active: bool, resolve: &'a dyn Fn(&str) -> Option<char>, color: Option<Color<f64>>) -> Self {
        Self {
            font,
            name,
            is_active,
            resolve,
            color,
        }
    }
}
