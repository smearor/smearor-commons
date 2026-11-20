//! The [`DrawProgressOps`] implementation for [`CairoTarget`].

use crate::cairo::CairoTarget;
use crate::ops::DrawProgressOps;
use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

impl DrawProgressOps for CairoTarget {
    /// The same 4px strip the raster backend draws, `size.y - 6` from
    /// the top, filled fraction in `color` and the rest darkened to a
    /// third — rendered as two rectangles.
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        let size = self.size();
        let bar_height = 4u32;
        let bar_y = f64::from(size.y.saturating_sub(bar_height + 2));
        let fill_width = (f64::from(size.x) * value.clamped().as_f64()).min(f64::from(size.x));
        let rgba: Color<u8> = color.into();

        Color::<f64>::from(rgba).apply(&self.ctx);
        self.ctx.rectangle(0.0, bar_y, fill_width, f64::from(bar_height));
        self.ctx.fill()?;

        let rest_width = f64::from(size.x) - fill_width;
        if rest_width > 0.0 {
            let track = Color::<u8>::new_rgba(rgba.r / 3, rgba.g / 3, rgba.b / 3, 255);
            Color::<f64>::from(track).apply(&self.ctx);
            self.ctx.rectangle(fill_width, bar_y, rest_width, f64::from(bar_height));
            self.ctx.fill()?;
        }
        Ok(())
    }

    /// The gauge as two cairo arcs: the full `track` circle stroked,
    /// then the `value` arc stroked on top from 12 o'clock clockwise —
    /// cairo `arc` angles run clockwise in y-down user space, matching
    /// the raster backend's `RotationRad` convention.
    fn draw_gauge(
        &mut self,
        center: Vec2<Pixels, f32>,
        radius: Pixels<f32>,
        line_width: Pixels<f32>,
        value: Fraction,
        color: impl Into<Color<f64>>,
        track: impl Into<Color<f64>>,
    ) -> Result<(), Self::Error> {
        let (center, radius, line_width) = (center.to_f64(), f64::from(radius.as_raw()), f64::from(line_width.as_raw()));
        if line_width <= 0.0 || radius <= line_width / 2.0 {
            return Ok(());
        }
        self.ctx.set_line_width(line_width);

        track.into().apply(&self.ctx);
        self.ctx.arc(center.x, center.y, radius, 0.0, std::f64::consts::TAU);
        self.ctx.stroke()?;

        let sweep = value.clamped().as_f64() * std::f64::consts::TAU;
        if sweep > 0.0 {
            color.into().apply(&self.ctx);
            self.ctx
                .arc(center.x, center.y, radius, -std::f64::consts::FRAC_PI_2, -std::f64::consts::FRAC_PI_2 + sweep);
            self.ctx.stroke()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo::Context;
    use cairo::Format;
    use cairo::ImageSurface;
    use smearor_commons_geometry::Size;

    #[test]
    fn draw_progress_bar_fills_fraction() {
        let size = Size::new(10, 10);
        let surface = ImageSurface::create(Format::ARgb32, 10, 10).unwrap();
        let mut target = CairoTarget::new(&Context::new(&surface).unwrap(), size);
        target.draw_progress_bar(Fraction::new(0.5), Color::<f64>::new(1.0, 0.0, 0.0)).unwrap();
        drop(target);

        let mut surface = surface;
        surface.flush();
        let stride = surface.stride() as usize;
        let data = surface.data().unwrap();
        // Bar row y = 4 (10 - 4 - 2): left half filled red, right half
        // dimmed to a third. ARgb32 memory order is B,G,R,A.
        let filled = 4 * stride + 2 * 4;
        assert_eq!(data[filled + 2], 255, "filled part should be the bar color");
        let track = 4 * stride + 8 * 4;
        assert_eq!(data[track + 2], 85, "unfilled part should be a third of the color");
        assert_eq!(data[track + 3], 255, "track stays opaque");
    }

    #[test]
    fn draw_gauge_strokes_track_and_arc() {
        let size = Size::new(48, 48);
        let surface = ImageSurface::create(Format::ARgb32, 48, 48).unwrap();
        let mut target = CairoTarget::new(&Context::new(&surface).unwrap(), size);
        target
            .draw_gauge(
                Vec2::<Pixels, f32>::new(24.0, 24.0),
                Pixels::new(16.0),
                Pixels::new(4.0),
                Fraction::new(0.25),
                Color::<f64>::new(1.0, 0.0, 0.0),
                Color::<f64>::new(0.3, 0.3, 0.3),
            )
            .unwrap();
        drop(target);

        let mut surface = surface;
        surface.flush();
        let stride = surface.stride() as usize;
        let data = surface.data().unwrap();
        // 12 o'clock point on the ring (24, 8): quarter arc covers it → red.
        let top = 8 * stride + 24 * 4;
        assert_eq!(data[top + 2], 255, "arc should paint the top of the ring");
        // 6 o'clock point on the ring (24, 40): outside the quarter →
        // the gray track (0.3 → ~76 per channel, R == B).
        let bottom = 40 * stride + 24 * 4;
        assert!(data[bottom + 3] > 0, "track should paint the bottom of the ring");
        assert_eq!(data[bottom + 2], data[bottom], "track is gray — equal red and blue channels");
    }
}
