//! `cairo` integration: set a [`Color`] as the source of a
//! [`cairo::Context`]. Enabled via the `cairo` feature.

use cairo::Context;

use crate::Color;

impl Color<f64> {
    /// Sets this color as the cairo source (alpha channel ignored).
    pub fn apply(&self, ctx: &Context) {
        ctx.set_source_rgb(self.r, self.g, self.b);
    }

    /// Sets this color as the cairo source, overriding the alpha.
    pub fn apply_alpha(&self, ctx: &Context, alpha: f64) {
        ctx.set_source_rgba(self.r, self.g, self.b, alpha);
    }
}

impl Color<f32> {
    /// Sets this color as the cairo source (alpha channel ignored).
    pub fn apply(&self, ctx: &Context) {
        ctx.set_source_rgb(self.r as f64, self.g as f64, self.b as f64);
    }

    /// Sets this color as the cairo source, overriding the alpha.
    pub fn apply_alpha(&self, ctx: &Context, alpha: f32) {
        ctx.set_source_rgba(self.r as f64, self.g as f64, self.b as f64, alpha as f64);
    }
}

impl Color<u8> {
    /// Sets this color as the cairo source (alpha channel ignored).
    pub fn apply(&self, ctx: &Context) {
        Color::<f64>::from(*self).apply(ctx);
    }

    /// Sets this color as the cairo source, overriding the alpha.
    pub fn apply_alpha(&self, ctx: &Context, alpha: u8) {
        Color::<f64>::from(*self).apply_alpha(ctx, f64::from(alpha) / 255.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_sets_rgb_source() {
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1, 1).unwrap();
        let ctx = Context::new(&surface).unwrap();
        Color::<f64>::new(0.25, 0.5, 0.75).apply(&ctx);
        // source is set without error; painting a 1x1 surface works
        ctx.paint().unwrap();
    }

    #[test]
    fn apply_alpha_sets_rgba_source() {
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1, 1).unwrap();
        let ctx = Context::new(&surface).unwrap();
        Color::<u8>::new(255, 0, 0).apply_alpha(&ctx, 128);
        ctx.paint().unwrap();
    }
}
