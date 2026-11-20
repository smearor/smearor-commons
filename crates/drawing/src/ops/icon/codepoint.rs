//! The [`Codepoint`] draw spec.

use ab_glyph::FontVec;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// A codepoint draw specification: which glyph, through which font,
/// at which position and size, in which color.
#[derive(Clone, Copy)]
pub struct Codepoint<'a> {
    /// Font containing the glyph.
    pub font: &'a FontVec,
    /// Unicode codepoint to draw.
    pub codepoint: char,
    /// Center position of the glyph.
    pub center: Vec2<Pixels, f32>,
    /// Target icon size; the glyph is scaled to fit.
    pub size: Pixels<f32>,
    /// Fill color.
    pub color: Color<u8>,
}

impl<'a> Codepoint<'a> {
    /// Creates a codepoint spec.
    pub fn new(font: &'a FontVec, codepoint: char, center: Vec2<Pixels, f32>, size: Pixels<f32>, color: impl Into<Color<u8>>) -> Self {
        Self {
            font,
            codepoint,
            center,
            size,
            color: color.into(),
        }
    }
}
