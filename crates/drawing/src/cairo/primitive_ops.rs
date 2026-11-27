//! The [`DrawPrimitiveOps`] implementation for `CairoTarget`.
//!
//! One impl for the single cairo target type — unlike the buffer
//! backend there is no second packed-view target to delegate to.

use crate::cairo::CairoTarget;
use crate::cairo::path::polyline_path;
use crate::ops::DrawPrimitiveOps;
use crate::ops::FillCircle;
use crate::ops::FillPolygon;
use crate::ops::FillRect;
use crate::ops::FillSector;
use crate::ops::StrokePolyline;
use crate::ops::StrokeRect;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use std::f64::consts::TAU;

impl DrawPrimitiveOps for CairoTarget {
    /// Fill the entire surface with `color`, preserving its alpha like
    /// the buffer `fill`: `Operator::Source` replaces destination pixels
    /// instead of compositing over them. `restore` runs even when
    /// `paint` fails so the context state stays balanced.
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        let color: Color<u8> = color.into();
        color.apply_alpha(&self.ctx, Alpha::from(color.a));
        self.ctx.save()?;
        self.ctx.set_operator(cairo::Operator::Source);
        let result = self.ctx.paint();
        self.ctx.restore()?;
        result
    }

    fn fill_rect(&mut self, spec: FillRect) -> Result<(), Self::Error> {
        spec.color.apply_alpha(&self.ctx, spec.alpha);
        self.ctx.rectangle(spec.rect.min.x, spec.rect.min.y, spec.rect.width(), spec.rect.height());
        self.ctx.fill()
    }

    /// Stroke centered on the rect edge — cairo already centers strokes
    /// on the path, so the rectangle is emitted at `spec.rect` itself (the
    /// buffer impl needs the `inset(±w/2)` ring for its pixel scan).
    fn stroke_rect(&mut self, spec: StrokeRect) -> Result<(), Self::Error> {
        if spec.line_width.as_raw() <= 0.0 {
            return Ok(());
        }
        spec.color.apply_alpha(&self.ctx, spec.alpha);
        self.ctx.rectangle(spec.rect.min.x, spec.rect.min.y, spec.rect.width(), spec.rect.height());
        self.ctx.set_line_width(f64::from(spec.line_width.as_raw()));
        self.ctx.stroke()
    }

    fn fill_circle(&mut self, spec: FillCircle) -> Result<(), Self::Error> {
        spec.color.apply_alpha(&self.ctx, spec.alpha);
        // `arc` chains onto a stale current point with a stray chord -
        // each spec is a self-contained primitive like on the buffer
        // backend, so it starts on a fresh path.
        self.ctx.new_path();
        self.ctx.arc(spec.circle.center.x, spec.circle.center.y, spec.circle.radius, 0.0, TAU);
        self.ctx.fill()
    }

    fn fill_polygon(&mut self, spec: FillPolygon) -> Result<(), Self::Error> {
        spec.color.apply_alpha(&self.ctx, spec.alpha);
        polyline_path(&self.ctx, &spec.polygon);
        self.ctx.close_path();
        self.ctx.fill()
    }

    /// Stroke centered on the segments like cairo — the open path
    /// gets `line_width` symmetric around the vertex polyline.
    fn stroke_polyline(&mut self, spec: StrokePolyline) -> Result<(), Self::Error> {
        if spec.line_width.as_raw() <= 0.0 {
            return Ok(());
        }
        spec.color.apply_alpha(&self.ctx, spec.alpha);
        polyline_path(&self.ctx, spec.polyline.points());
        self.ctx.set_line_width(f64::from(spec.line_width.as_raw()));
        self.ctx.stroke()
    }

    /// The annulus sector as outer arc swept forward (`arc`) and
    /// inner arc swept back — `arc_negative` handles the wrap across
    /// the full turn through the un-normalized end direction. The spec
    /// is a self-contained primitive: it starts on a fresh path so a
    /// stale current point can't chain a stray chord into the sector.
    fn fill_sector(&mut self, spec: FillSector) -> Result<(), Self::Error> {
        spec.color.apply_alpha(&self.ctx, spec.alpha);
        let center = spec.annulus.center;
        let (start, end) = (spec.arc.start.as_f64(), spec.arc.start.as_f64() + spec.arc.width.as_f64());
        self.ctx.new_path();
        self.ctx.arc(center.x, center.y, spec.annulus.outer, start, end);
        self.ctx.arc_negative(center.x, center.y, spec.annulus.inner, end, start);
        self.ctx.close_path();
        self.ctx.fill()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo::Context;
    use cairo::Format;
    use cairo::ImageSurface;
    use smearor_commons_geometry::Circle;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_geometry::Vec2;

    /// Render through a `CairoTarget` onto a fresh `ImageSurface` and
    /// return the raw surface bytes (ARgb32 = BGRA, premultiplied).
    fn render(size: Size<u32>, draw: impl FnOnce(&mut CairoTarget) -> Result<(), cairo::Error>) -> Vec<u8> {
        let mut surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).expect("image surface");
        {
            let ctx = Context::new(&surface).expect("context");
            let mut target = CairoTarget::new(&ctx, size);
            draw(&mut target).expect("draw operation");
        }
        surface.flush();
        surface.data().expect("surface data").to_vec()
    }

    fn px(data: &[u8], size: Size<u32>, x: u32, y: u32) -> [u8; 4] {
        let idx = (y * size.x + x) as usize * 4;
        data[idx..idx + 4].try_into().expect("pixel")
    }

    #[test]
    fn fill_background_paints_all_pixels() {
        let size = Size::new(8, 8);
        let data = render(size, |t| t.fill_background(Color::<u8>::new_rgba(255, 0, 0, 255)));
        // ARgb32 stores BGRA premultiplied: red = [B=0, G=0, R=255, A=255]
        assert_eq!(px(&data, size, 0, 0), [0, 0, 255, 255]);
        assert_eq!(px(&data, size, 7, 7), [0, 0, 255, 255]);
    }

    #[test]
    fn fill_background_preserves_alpha() {
        let size = Size::new(4, 4);
        let data = render(size, |t| t.fill_background(Color::<u8>::new_rgba(255, 0, 0, 128)));
        let pixel = px(&data, size, 1, 1);
        assert_eq!(pixel[3], 128, "alpha channel should be replaced, not composited");
        assert!((pixel[2] as i32 - 128).abs() <= 1, "premultiplied red should be ~128, got {}", pixel[2]);
    }

    #[test]
    fn fill_rect_covers_interior_only() {
        let size = Size::new(16, 16);
        let rect = Rect::<Pixels>::new(Vec2::new(4.0, 4.0), Vec2::new(8.0, 8.0));
        let data = render(size, |t| t.fill_rect(FillRect::new(rect, Color::<f64>::new(0.0, 1.0, 0.0), Alpha::OPAQUE)));
        assert_eq!(px(&data, size, 8, 8), [0, 255, 0, 255], "interior pixel");
        assert_eq!(px(&data, size, 0, 0), [0, 0, 0, 0], "outside corner");
        assert_eq!(px(&data, size, 15, 15), [0, 0, 0, 0], "outside edge");
    }

    #[test]
    fn stroke_rect_covers_edges_only() {
        let size = Size::new(16, 16);
        let rect = Rect::<Pixels>::new(Vec2::new(4.0, 4.0), Vec2::new(8.0, 8.0));
        let data = render(size, |t| {
            t.stroke_rect(StrokeRect::new(rect, Color::<f64>::new(1.0, 1.0, 1.0), Pixels::new(2.0), Alpha::OPAQUE))
        });
        assert_eq!(px(&data, size, 4, 8)[3], 255, "left edge should be stroked");
        assert_eq!(px(&data, size, 8, 8), [0, 0, 0, 0], "center should be empty");
        assert_eq!(px(&data, size, 0, 0), [0, 0, 0, 0], "outside should be empty");
    }

    #[test]
    fn stroke_rect_zero_width_is_noop() {
        let size = Size::new(8, 8);
        let rect = Rect::<Pixels>::new(Vec2::new(1.0, 1.0), Vec2::new(6.0, 6.0));
        let data = render(size, |t| {
            t.stroke_rect(StrokeRect::new(rect, Color::<f64>::new(1.0, 1.0, 1.0), Pixels::new(0.0), Alpha::OPAQUE))
        });
        assert!(data.iter().all(|&b| b == 0));
    }

    #[test]
    fn fill_circle_covers_center_only() {
        let size = Size::new(16, 16);
        let circle = Circle::<Pixels>::new(Vec2::new(8.0, 8.0), 5.0);
        let data = render(size, |t| t.fill_circle(FillCircle::new(circle, Color::<f64>::new(0.0, 0.0, 1.0), Alpha::OPAQUE)));
        assert_eq!(px(&data, size, 8, 8), [255, 0, 0, 255], "center pixel");
        assert_eq!(px(&data, size, 0, 0), [0, 0, 0, 0], "corner outside");
    }

    #[test]
    fn fill_rect_alpha_composites() {
        let size = Size::new(4, 4);
        let rect = Rect::<Pixels>::new(Vec2::new(0.0, 0.0), Vec2::new(4.0, 4.0));
        let data = render(size, |t| t.fill_rect(FillRect::new(rect, Color::<f64>::new(1.0, 0.0, 0.0), Alpha::new(0.5))));
        let pixel = px(&data, size, 2, 2);
        assert!((pixel[3] as i32 - 128).abs() <= 1, "alpha ~128, got {}", pixel[3]);
        assert!((pixel[2] as i32 - 128).abs() <= 1, "premultiplied red ~128, got {}", pixel[2]);
    }
}
