//! Builder pattern for composing multi-element renders onto a pixel buffer.
//!
//! [`RenderContext`] encapsulates the pixel buffer, injected fonts, and
//! default colors/state, then provides methods that delegate to the
//! [`DrawOps`] methods on [`PixelBuffer`]. This eliminates the boilerplate
//! of passing buffers, fonts, and colors to every `draw_*` call.

use crate::ops::Codepoint;
use crate::ops::DrawTarget;
use crate::ops::Icon;
use crate::ops::IconGrid;
use crate::ops::ProgressBar;
use ab_glyph::FontVec;
use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Grid;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::fmt;
use std::fmt::Debug;
use tracing::debug;

/// Builder context for rendering icons, text, and UI elements onto a
/// [`DrawTarget`].
///
/// The context **owns** its target — `T` is typically `&mut PixelBuffer`,
/// `BufferViewMut`, or a backend target like `CairoTarget` (the capability
/// traits and [`DrawTarget`] are implemented for `&mut T` as well, so
/// borrows compose). Owning instead of `&mut T`-borrowing keeps the API
/// honest across backends: buffer views genuinely need exclusive access
/// while cairo targets are shared handles.
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
/// ctx.fill_background().unwrap();
/// ctx.draw_icon('\u{F11B}').unwrap();
/// ctx.draw_label("gamepad").unwrap();
/// ```
pub struct RenderContext<'a, T: DrawTarget> {
    /// The render target this context draws into.
    target: T,
    /// Font for icons (`draw_icon`, `draw_icon_by_name`, `draw_icon_grid`);
    /// also the fallback font for text.
    icon_font: Option<&'a FontVec>,
    /// Font for text (`draw_label`, `draw_text`, `draw_text_sized`);
    /// falls back to `icon_font`, then to the bitmap renderer.
    label_font: Option<&'a FontVec>,
    /// Optional background fill color, applied by [`RenderContext::fill_background`].
    background: Option<Color<f64>>,
    /// Default color for icons and progress bars; `None` defers to the
    /// target's [`DrawTarget::default_text_color`].
    icon_color: Option<Color<f64>>,
    /// Default color for text labels; `None` defers to the target's
    /// [`DrawTarget::default_text_color`].
    text_color: Option<Color<f64>>,
    /// Whether the rendered element is "active" (affects default colors).
    is_active: bool,
}

impl<T: DrawTarget> Debug for RenderContext<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderContext")
            .field("target", &"..")
            .field("icon_font", &self.icon_font.map(|_| ".."))
            .field("label_font", &self.label_font.map(|_| ".."))
            .field("background", &self.background)
            .field("icon_color", &self.icon_color)
            .field("text_color", &self.text_color)
            .field("is_active", &self.is_active)
            .finish()
    }
}

impl<'a, T: DrawTarget> RenderContext<'a, T> {
    /// Creates a new `RenderContext` over `target` with no colors
    /// configured, inactive state and no fonts configured. Colors left
    /// unset resolve to [`DrawTarget::default_text_color`] of the target.
    pub fn new(target: T) -> Self {
        Self {
            target,
            icon_font: None,
            label_font: None,
            background: None,
            icon_color: None,
            text_color: None,
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
        self.icon_color = Some(color);
        self
    }

    /// Sets the default text color for labels and centered text.
    pub fn text_color(mut self, color: Color<f64>) -> Self {
        self.text_color = Some(color);
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
    pub fn fill_background(&mut self) -> Result<(), T::Error> {
        if let Some(color) = self.background {
            self.target.fill_background(color)?;
        }
        Ok(())
    }

    /// Draws an icon by its Unicode codepoint, centered in the upper
    /// portion of the buffer.
    ///
    /// Uses the configured `icon_font` and `icon_color`. Does nothing when
    /// no icon font is configured.
    pub fn draw_icon(&mut self, codepoint: char) -> Result<(), T::Error> {
        let Some(font) = self.icon_font else {
            debug!("no icon font configured, skipping draw_icon");
            return Ok(());
        };
        let size = self.target.size();
        let icon_size = Pixels::new(size.x.min(size.y) as f32 * 0.6);
        let size_f = size.to_f32();
        let color: Color<u8> = self
            .icon_color
            .map(|c| c.into())
            .unwrap_or_else(|| self.target.default_text_color(self.is_active));
        self.target
            .draw_codepoint(Codepoint::new(font, codepoint, Vec2::<Pixels, f32>::new(size_f.half().x, size_f.y * 0.4), icon_size, color))
    }

    /// Draws an icon by name using the configured `icon_font`, resolving
    /// the codepoint via the provided resolver function.
    ///
    /// Uses the configured `icon_color` and `is_active` state. Falls back
    /// to a placeholder if no icon font is configured or the icon is
    /// unavailable.
    pub fn draw_icon_by_name(&mut self, icon_name: &str, resolve_codepoint: impl Fn(&str) -> Option<char>) -> Result<(), T::Error> {
        self.target
            .draw_icon(Icon::new(self.icon_font, icon_name, self.is_active, &resolve_codepoint, self.icon_color))
    }

    /// Draws a text label at the bottom of the buffer using the configured
    /// `label_font` (falling back to `icon_font`, then to the bitmap
    /// renderer).
    ///
    /// Uses the configured `text_color` and `is_active` state.
    pub fn draw_label(&mut self, text: &str) -> Result<(), T::Error> {
        self.target
            .draw_label(self.label_font.or(self.icon_font), text, self.is_active, self.text_color)
    }

    /// Draws text centered horizontally at the given y-baseline.
    ///
    /// Uses the configured `text_color`. The font size is derived from the
    /// buffer height unless overridden.
    pub fn draw_text(&mut self, text: &str, baseline_y: Pixels<f32>) -> Result<(), T::Error> {
        let font_size = Pixels::new((self.target.size().y as f32 * 0.22).max(8.0));
        let color: Color<u8> = self
            .text_color
            .map(|c| c.into())
            .unwrap_or_else(|| self.target.default_text_color(self.is_active));
        self.target
            .draw_text_centered(self.label_font.or(self.icon_font), text, baseline_y, font_size, color)
    }

    /// Draws text centered horizontally at the given y-baseline with a
    /// custom font size.
    pub fn draw_text_sized(&mut self, text: &str, baseline_y: Pixels<f32>, font_size: Pixels<f32>) -> Result<(), T::Error> {
        let color: Color<u8> = self
            .text_color
            .map(|c| c.into())
            .unwrap_or_else(|| self.target.default_text_color(self.is_active));
        self.target
            .draw_text_centered(self.label_font.or(self.icon_font), text, baseline_y, font_size, color)
    }

    /// Draws a horizontal progress bar of the given `height` at the
    /// bottom of the buffer.
    ///
    /// `value` is clamped to `[0.0, 1.0]`. Uses the configured `icon_color`.
    pub fn draw_progress_bar(&mut self, value: Fraction, height: Pixels<f32>) -> Result<(), T::Error> {
        let color: Color<f64> = self.icon_color.unwrap_or_else(|| self.target.default_text_color(self.is_active).into());
        self.target.draw_progress_bar(ProgressBar::new(value, color, height))
    }

    /// Draws a grid of icons by name using the configured `icon_font`,
    /// resolving codepoints via the provided resolver function.
    ///
    /// Uses the configured `is_active` state for default color selection.
    /// Does nothing when no icon font is configured.
    pub fn draw_icon_grid(&mut self, icons: &[&str], grid: Grid, resolve_codepoint: impl Fn(&str) -> Option<char>) -> Result<(), T::Error> {
        let Some(font) = self.icon_font else {
            debug!("no icon font configured, skipping draw_icon_grid");
            return Ok(());
        };
        self.target.draw_icon_grid(IconGrid::new(font, icons, grid, self.is_active, &resolve_codepoint))
    }

    /// Returns the target width in pixels.
    pub fn width(&self) -> u32 {
        self.target.size().x
    }

    /// Returns the target height in pixels.
    pub fn height(&self) -> u32 {
        self.target.size().y
    }

    /// Returns the buffer dimensions.
    pub fn size(&self) -> Size<u32> {
        self.target.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nerd_fonts_rs::NerdFonts;
    use nerd_fonts_rs::fonts::label_font;
    use nerd_fonts_rs::fonts::nerd_font;
    use smearor_commons_buffer::PixelBuffer;

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
        ctx.fill_background().unwrap();

        for px in buf.pixels().as_chunks::<4>().0.iter() {
            assert_eq!(px, &expected);
        }
    }

    #[test]
    fn render_context_fill_background_noop_without_color() {
        let mut buf = PixelBuffer::new(Size::new(2, 2));
        buf.pixels_mut().fill(10);

        let mut ctx = RenderContext::new(&mut buf);
        ctx.fill_background().unwrap();

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
        assert_eq!(ctx.icon_color, Some(ic));
        assert_eq!(ctx.text_color, Some(tc));
        assert!(ctx.icon_font.is_some());
        assert!(ctx.label_font.is_some());
        assert!(ctx.is_active);
        assert_eq!(ctx.background, Some(bg));
    }

    #[test]
    fn render_context_draw_icon_by_name_renders() {
        let mut buf = PixelBuffer::new(Size::new(64, 64));
        let mut ctx = RenderContext::new(&mut buf).icon_font(nerd_font()).active(true);
        ctx.draw_icon_by_name("nf-fa-gamepad", resolve_codepoint).unwrap();

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "icon should render pixels");
    }

    #[test]
    fn render_context_draw_label_renders() {
        let mut buf = PixelBuffer::new(Size::new(96, 96));
        let mut ctx = RenderContext::new(&mut buf).label_font(label_font().or_else(nerd_font));
        ctx.draw_label("hello").unwrap();

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "label should render pixels");
    }
}

#[cfg(test)]
mod generic_target_tests {
    use super::*;
    use smearor_commons_buffer::PixelBuffer;

    #[test]
    fn render_context_works_over_owned_view_target() {
        // T = BufferViewMut: the context owns the view, no &mut buffer borrow.
        let mut buf = PixelBuffer::new(Size::new(8, 8));
        let color = Color::<f64>::new(1.0, 0.0, 0.0);
        {
            let mut ctx = RenderContext::new(buf.view_mut()).background(color);
            ctx.fill_background().unwrap();
            assert_eq!(ctx.size(), Size::new(8, 8));
            assert_eq!(ctx.width(), 8);
        }
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        assert!(buf.pixels().as_chunks::<4>().0.iter().all(|px| px == &expected));
    }

    #[test]
    fn render_context_works_over_mut_reference_target() {
        // T = &mut PixelBuffer: the &mut-T forwarding impls keep the
        // original constructor shape working.
        let mut buf = PixelBuffer::new(Size::new(4, 4));
        let color = Color::<f64>::new(0.0, 1.0, 0.0);
        {
            let mut ctx = RenderContext::new(&mut buf).background(color);
            ctx.fill_background().unwrap();
        }
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        assert!(buf.pixels().iter().all(|&b| expected.contains(&b)));
    }
}
