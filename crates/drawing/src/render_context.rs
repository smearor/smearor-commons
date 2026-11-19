//! Builder pattern for composing multi-element renders onto a pixel buffer.
//!
//! [`RenderContext`] encapsulates the pixel buffer, injected fonts, and
//! default colors/state, then provides methods that delegate to the
//! [`DrawOps`] methods on [`PixelBuffer`]. This eliminates the boilerplate
//! of passing buffers, fonts, and colors to every `draw_*` call.

use crate::ops::DrawIconOps;
use crate::ops::DrawPrimitiveOps;
use crate::ops::DrawProgressOps;
use crate::ops::DrawTextOps;
use ab_glyph::FontVec;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Grid;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::fmt;
use std::fmt::Debug;
use tracing::debug;

/// Builder context for rendering icons, text, and UI elements onto a raw
/// RGBA pixel buffer.
///
/// Construct with [`RenderContext::new`], chain optional configuration
/// methods, then call draw methods. Fonts are injected via
/// [`icon_font`](Self::icon_font) and [`label_font`](Self::label_font);
/// icon calls fall back to a placeholder and text calls to the bitmap
/// renderer when no font is configured.
///
/// # Examples
///
/// ```no_run
/// use nerd_fonts_rs::fonts::label_font;
/// use nerd_fonts_rs::fonts::nerd_font;
/// use smearor_commons_buffer::PixelBuffer;
/// use smearor_commons_color::Color;
/// use smearor_commons_drawing::RenderContext;
/// use smearor_commons_geometry::Fraction;
/// use smearor_commons_geometry::Size;
///
/// let mut buf = PixelBuffer::new(Size::new(96, 96));
/// let mut ctx = RenderContext::new(&mut buf)
///     .icon_font(nerd_font())
///     .label_font(label_font())
///     .background(Color::<f64>::new(0.12, 0.12, 0.12))
///     .icon_color(Color::<f64>::new(0.4, 0.7, 1.0))
///     .active(true);
/// ctx.fill_background();
/// ctx.draw_icon('\u{F11B}');
/// ctx.draw_label("gamepad");
/// ```
pub struct RenderContext<'a> {
    /// Target pixel buffer (RGBA, row-major).
    buf: &'a mut PixelBuffer<Rgba>,
    /// Font for icons (`draw_icon`, `draw_icon_by_name`, `draw_icon_grid`);
    /// also the fallback font for text.
    icon_font: Option<&'a FontVec>,
    /// Font for text (`draw_label`, `draw_text`, `draw_text_sized`);
    /// falls back to `icon_font`, then to the bitmap renderer.
    label_font: Option<&'a FontVec>,
    /// Optional background fill color, applied by [`RenderContext::fill_background`].
    background: Option<Color<f64>>,
    /// Default color for icons and progress bars.
    icon_color: Color<f64>,
    /// Default color for text labels.
    text_color: Color<f64>,
    /// Whether the rendered element is "active" (affects default colors).
    is_active: bool,
}

impl Debug for RenderContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderContext")
            .field("buf", &self.buf)
            .field("icon_font", &self.icon_font.map(|_| ".."))
            .field("label_font", &self.label_font.map(|_| ".."))
            .field("background", &self.background)
            .field("icon_color", &self.icon_color)
            .field("text_color", &self.text_color)
            .field("is_active", &self.is_active)
            .finish()
    }
}

impl<'a> RenderContext<'a> {
    /// Creates a new `RenderContext` with default colors, inactive state
    /// and no fonts configured.
    pub fn new(buf: &'a mut PixelBuffer<Rgba>) -> Self {
        Self {
            buf,
            icon_font: None,
            label_font: None,
            background: None,
            icon_color: Color::<f64>::new(0.94, 0.94, 0.94),
            text_color: Color::<f64>::new(0.94, 0.94, 0.94),
            is_active: false,
        }
    }

    /// Sets the font used for icons; also the fallback font for text when
    /// no `label_font` is configured.
    pub fn icon_font(mut self, font: Option<&'a FontVec>) -> Self {
        self.icon_font = font;
        self
    }

    /// Sets the font used for labels and centered text; falls back to the
    /// `icon_font`, then to the built-in bitmap renderer.
    pub fn label_font(mut self, font: Option<&'a FontVec>) -> Self {
        self.label_font = font;
        self
    }

    /// Sets the background fill color, applied when [`fill_background`](Self::fill_background) is called.
    pub fn background(mut self, color: Color<f64>) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets the default icon color (also used for progress bars).
    pub fn icon_color(mut self, color: Color<f64>) -> Self {
        self.icon_color = color;
        self
    }

    /// Sets the default text color for labels and centered text.
    pub fn text_color(mut self, color: Color<f64>) -> Self {
        self.text_color = color;
        self
    }

    /// Sets the active state, which affects default color selection.
    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    /// Fills the entire pixel buffer with the configured background color.
    ///
    /// Does nothing if no background color was set.
    pub fn fill_background(&mut self) {
        if let Some(color) = self.background {
            self.buf.fill_background(color);
        }
    }

    /// Draws an icon by its Unicode codepoint, centered in the upper
    /// portion of the buffer.
    ///
    /// Uses the configured `icon_font` and `icon_color`. Does nothing when
    /// no icon font is configured.
    pub fn draw_icon(&mut self, codepoint: char) {
        let Some(font) = self.icon_font else {
            debug!("no icon font configured, skipping draw_icon");
            return;
        };
        let size = self.buf.size();
        let icon_size = Pixels::new(size.x.min(size.y) as f32 * 0.6);
        let size_f = size.to_f32();
        self.buf
            .draw_codepoint(font, codepoint, Vec2::<Pixels, f32>::new(size_f.half().x, size_f.y * 0.4), icon_size, self.icon_color);
    }

    /// Draws an icon by name using the configured `icon_font`, resolving
    /// the codepoint via the provided resolver function.
    ///
    /// Uses the configured `icon_color` and `is_active` state. Falls back
    /// to a placeholder if no icon font is configured or the icon is
    /// unavailable.
    pub fn draw_icon_by_name(&mut self, icon_name: &str, resolve_codepoint: impl Fn(&str) -> Option<char>) {
        self.buf
            .draw_icon(self.icon_font, icon_name, self.is_active, resolve_codepoint, Some(self.icon_color));
    }

    /// Draws a text label at the bottom of the buffer using the configured
    /// `label_font` (falling back to `icon_font`, then to the bitmap
    /// renderer).
    ///
    /// Uses the configured `text_color` and `is_active` state.
    pub fn draw_label(&mut self, text: &str) {
        self.buf
            .draw_label(self.label_font.or(self.icon_font), text, self.is_active, Some(self.text_color));
    }

    /// Draws text centered horizontally at the given y-baseline.
    ///
    /// Uses the configured `text_color`. The font size is derived from the
    /// buffer height unless overridden.
    pub fn draw_text(&mut self, text: &str, baseline_y: Pixels<f32>) {
        let font_size = Pixels::new((self.buf.height().as_raw() as f32 * 0.22).max(8.0));
        self.buf
            .draw_text_centered(self.label_font.or(self.icon_font), text, baseline_y, font_size, self.text_color);
    }

    /// Draws text centered horizontally at the given y-baseline with a
    /// custom font size.
    pub fn draw_text_sized(&mut self, text: &str, baseline_y: Pixels<f32>, font_size: Pixels<f32>) {
        self.buf
            .draw_text_centered(self.label_font.or(self.icon_font), text, baseline_y, font_size, self.text_color);
    }

    /// Draws a horizontal progress bar at the bottom of the buffer.
    ///
    /// `value` is clamped to `[0.0, 1.0]`. Uses the configured `icon_color`.
    pub fn draw_progress_bar(&mut self, value: Fraction) {
        self.buf.draw_progress_bar(value, self.icon_color);
    }

    /// Draws a grid of icons by name using the configured `icon_font`,
    /// resolving codepoints via the provided resolver function.
    ///
    /// Uses the configured `is_active` state for default color selection.
    /// Does nothing when no icon font is configured.
    pub fn draw_icon_grid(&mut self, icons: &[&str], grid: Grid, resolve_codepoint: impl Fn(&str) -> Option<char>) {
        let Some(font) = self.icon_font else {
            debug!("no icon font configured, skipping draw_icon_grid");
            return;
        };
        self.buf.draw_icon_grid(font, icons, grid, self.is_active, resolve_codepoint);
    }

    /// Returns the buffer width in pixels.
    pub fn width(&self) -> u32 {
        self.buf.width().as_raw()
    }

    /// Returns the buffer height in pixels.
    pub fn height(&self) -> u32 {
        self.buf.height().as_raw()
    }

    /// Returns the buffer dimensions.
    pub fn size(&self) -> Size<u32> {
        self.buf.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nerd_fonts_rs::NerdFonts;
    use nerd_fonts_rs::fonts::label_font;
    use nerd_fonts_rs::fonts::nerd_font;

    fn resolve_codepoint(name: &str) -> Option<char> {
        NerdFonts::codepoint(name).map(|c| c.as_char())
    }

    #[test]
    fn render_context_fill_background_applies_configured_color() {
        let size = Size::new(4, 4);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<f64>::new(50.0 / 255.0, 60.0 / 255.0, 70.0 / 255.0);
        let expected: [u8; 4] = Color::<u8>::from(color).into();

        let mut ctx = RenderContext::new(&mut buf).background(color);
        ctx.fill_background();

        for px in buf.pixels().as_chunks::<4>().0.iter() {
            assert_eq!(px, &expected);
        }
    }

    #[test]
    fn render_context_fill_background_noop_without_color() {
        let mut buf = PixelBuffer::new(Size::new(2, 2));
        buf.pixels_mut().fill(10);

        let mut ctx = RenderContext::new(&mut buf);
        ctx.fill_background();

        assert!(buf.pixels().iter().all(|&p| p == 10));
    }

    #[test]
    fn render_context_builder_methods_set_fields() {
        let mut buf = PixelBuffer::new(Size::new(2, 2));
        let bg = Color::<f64>::new_rgba(1.0 / 255.0, 2.0 / 255.0, 3.0 / 255.0, 4.0 / 255.0);
        let ic = Color::<f64>::new_rgba(5.0 / 255.0, 6.0 / 255.0, 7.0 / 255.0, 8.0 / 255.0);
        let tc = Color::<f64>::new_rgba(9.0 / 255.0, 10.0 / 255.0, 11.0 / 255.0, 12.0 / 255.0);
        let ctx = RenderContext::new(&mut buf)
            .background(bg)
            .icon_color(ic)
            .text_color(tc)
            .icon_font(nerd_font())
            .label_font(label_font())
            .active(true);

        assert_eq!(ctx.width(), 2);
        assert_eq!(ctx.height(), 2);
        assert_eq!(ctx.icon_color, ic);
        assert_eq!(ctx.text_color, tc);
        assert!(ctx.icon_font.is_some());
        assert!(ctx.label_font.is_some());
        assert!(ctx.is_active);
        assert_eq!(ctx.background, Some(bg));
    }

    #[test]
    fn render_context_draw_icon_by_name_renders() {
        let mut buf = PixelBuffer::new(Size::new(64, 64));
        let mut ctx = RenderContext::new(&mut buf).icon_font(nerd_font()).active(true);
        ctx.draw_icon_by_name("nf-fa-gamepad", resolve_codepoint);

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "icon should render pixels");
    }

    #[test]
    fn render_context_draw_label_renders() {
        let mut buf = PixelBuffer::new(Size::new(96, 96));
        let mut ctx = RenderContext::new(&mut buf).label_font(label_font().or_else(nerd_font));
        ctx.draw_label("hello");

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "label should render pixels");
    }
}
