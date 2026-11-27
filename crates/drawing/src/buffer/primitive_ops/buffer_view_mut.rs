//! The [`DrawPrimitiveOps`] implementation for `BufferViewMut<Rgba>`:
//! the actual software-raster logic shared behind a shape-scan helper.

use crate::buffer::raster::blend_pixel;
use crate::ops::Cap;
use crate::ops::DrawPrimitiveOps;
use crate::ops::FillCircle;
use crate::ops::FillPolygon;
use crate::ops::FillRect;
use crate::ops::FillSector;
use crate::ops::StrokeArc;
use crate::ops::StrokeCircle;
use crate::ops::StrokePolygon;
use crate::ops::StrokePolyline;
use crate::ops::StrokeRect;
use crate::ops::StrokeSector;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Segment;
use smearor_commons_geometry::Vec2;

/// Blend `color` at `alpha` onto every pixel whose center lies in
/// `bounds ∩ shape`, sampled at pixel centers like the other raster ops.
fn blend_scanned<F>(view: &mut BufferViewMut<'_, Rgba>, bounds: Rect<Pixels, i32>, color: Color<f64>, alpha: impl Into<Alpha>, shape: F)
where
    F: Fn(&Vec2<Pixels, f64>) -> bool,
{
    let size = view.size();
    let stride = view.stride().as_usize();
    let pixels = view.data_mut();
    let color: Color<u8> = color.into();
    let alpha = alpha.into();
    let (x0, y0) = (bounds.min.x.max(0), bounds.min.y.max(0));
    let (x1, y1) = (bounds.max().x.min(size.x as i32), bounds.max().y.min(size.y as i32));

    for y in y0..y1 {
        for x in x0..x1 {
            if shape(&Vec2::<Pixels>::new(x as f64 + 0.5, y as f64 + 0.5)) {
                let idx = y as usize * stride + x as usize * Rgba::BYTES_PER_PIXEL;
                blend_pixel(pixels, idx, color, alpha);
            }
        }
    }
}

impl DrawPrimitiveOps for BufferViewMut<'_, Rgba> {
    /// Fill the entire view with a solid background color.
    ///
    /// Packed views reinterpret the `&mut [u8]` data as `&mut [u32]` (RGBA
    /// = 4 bytes = one `u32` in little-endian) and use `slice::fill()` for
    /// SIMD-vectorized writes. Strided views fill each row's pixel bytes
    /// and leave row padding untouched.
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        let color: Color<u8> = color.into();
        let mut pixel = [0; Rgba::BYTES_PER_PIXEL];
        Rgba::write_pixel(&mut pixel, color.r, color.g, color.b, color.a);
        self.fill(&pixel);
        Ok(())
    }

    fn fill_rect(&mut self, spec: FillRect) -> Result<(), Self::Error> {
        let bounds = spec.rect.to_i32_ceil();
        blend_scanned(self, bounds, spec.color, spec.alpha, |p| spec.rect.contains(p));
        Ok(())
    }

    /// Stroke centered on the rect edge like cairo: the stroke region is
    /// `rect ± line_width/2` minus the interior — implemented as the
    /// expanded outline minus the shrunken one.
    fn stroke_rect(&mut self, spec: StrokeRect) -> Result<(), Self::Error> {
        if spec.line_width.as_raw() <= 0.0 {
            return Ok(());
        }
        let half = Pixels::new(f64::from(spec.line_width.as_raw()) / 2.0);
        let outer = spec.rect.inset(-half);
        let inner = spec.rect.inset(half);
        let bounds = outer.to_i32_ceil();
        blend_scanned(self, bounds, spec.color, spec.alpha, |p| outer.contains(p) && !inner.contains(p));
        Ok(())
    }

    fn fill_circle(&mut self, spec: FillCircle) -> Result<(), Self::Error> {
        let bounds = spec.circle.bounding_box().to_i32_ceil();
        blend_scanned(self, bounds, spec.color, spec.alpha, |p| spec.circle.contains(p));
        Ok(())
    }

    /// Stroke centered on the radius like cairo: the stroke region is
    /// every point within `line_width/2` of the circle.
    fn stroke_circle(&mut self, spec: StrokeCircle) -> Result<(), Self::Error> {
        let half = f64::from(spec.line_width.as_raw()) / 2.0;
        if half <= 0.0 {
            return Ok(());
        }
        let bounds = spec.circle.bounding_box().outset(Pixels::new(half)).to_i32_ceil();
        let center = spec.circle.center;
        blend_scanned(self, bounds, spec.color, spec.alpha, |p| {
            ((*p - center).magnitude().as_raw() - spec.circle.radius).abs() <= half
        });
        Ok(())
    }

    /// Stroke centered on each edge like cairo - the polygon's
    /// `segments` closes the ring, so the seam strokes like any edge.
    fn stroke_polygon(&mut self, spec: StrokePolygon) -> Result<(), Self::Error> {
        let half = f64::from(spec.line_width.as_raw()) / 2.0;
        if half <= 0.0 {
            return Ok(());
        }
        for segment in spec.polygon.segments() {
            let bounds = segment.bounding_box().outset(Pixels::new(half)).to_i32_ceil();
            blend_scanned(self, bounds, spec.color, spec.alpha, |p| segment.distance(p).as_raw() <= half);
        }
        Ok(())
    }

    fn fill_polygon(&mut self, spec: FillPolygon) -> Result<(), Self::Error> {
        let Some(bounds) = spec.polygon.bounding_box() else {
            return Ok(());
        };
        blend_scanned(self, bounds.to_i32_ceil(), spec.color, spec.alpha, |p| spec.polygon.contains(p));
        Ok(())
    }

    /// Stroke centered on each segment like cairo: a segment's stroke
    /// region is every point within `line_width/2` of it.
    fn stroke_polyline(&mut self, spec: StrokePolyline) -> Result<(), Self::Error> {
        let half = f64::from(spec.line_width.as_raw()) / 2.0;
        if half <= 0.0 {
            return Ok(());
        }
        for segment in spec.polyline.segments() {
            let bounds = segment.bounding_box().outset(Pixels::new(half)).to_i32_ceil();
            blend_scanned(self, bounds, spec.color, spec.alpha, |p| segment.distance(p).as_raw() <= half);
        }
        match spec.cap {
            Cap::Butt => {}
            // Semicircles of radius `half` on the open endpoints -
            // overlapping the band like the segment joints do.
            Cap::Round => {
                for endpoint in [spec.polyline.points().first(), spec.polyline.points().last()].into_iter().flatten() {
                    let bounds = Circle::new(*endpoint, half).bounding_box().to_i32_ceil();
                    blend_scanned(self, bounds, spec.color, spec.alpha, |p| p.distance(endpoint).as_raw() <= half);
                }
            }
            // The `half`-long band beyond each open end, scanned
            // through the extension segment's interior distance -
            // a perpendicular band that stops square at `half`.
            Cap::Square => {
                for (segment, flip) in spec
                    .polyline
                    .segments()
                    .next()
                    .map(|s| (s, false))
                    .into_iter()
                    .chain(spec.polyline.segments().last().map(|s| (s, true)))
                {
                    let axis = Vec2::<Pixels>::new(1.0, 0.0).rotated(segment.direction());
                    let cap = if flip {
                        Segment::new(segment.to, segment.to + axis * half)
                    } else {
                        Segment::new(segment.from - axis * half, segment.from)
                    };
                    let bounds = cap.bounding_box().outset(Pixels::new(half)).to_i32_ceil();
                    blend_scanned(self, bounds, spec.color, spec.alpha, |p| cap.interior_distance(p).is_some_and(|d| d.as_raw() <= half));
                }
            }
        }
        Ok(())
    }

    /// The rim band `|dist − r| ≤ w/2` masked by `arc`'s interval -
    /// caps are the same geometry as `stroke_polyline`'s: round ends
    /// scan endpoint circles, square ends scan a tangent extension.
    fn stroke_arc(&mut self, spec: StrokeArc) -> Result<(), Self::Error> {
        let half = f64::from(spec.line_width.as_raw()) / 2.0;
        if half <= 0.0 {
            return Ok(());
        }
        let (center, radius) = (spec.circle.center, spec.circle.radius);
        let bounds = spec.circle.bounding_box().outset(Pixels::new(half)).to_i32_ceil();
        blend_scanned(self, bounds, spec.color, spec.alpha, |p| {
            (p.distance(&center).as_raw() - radius).abs() <= half && spec.arc.contains((*p - center).direction())
        });
        for (index, end) in [spec.arc.start, spec.arc.end()].iter().enumerate() {
            let radial = Vec2::<Pixels>::new(1.0, 0.0).rotated(*end);
            let endpoint = center + radial * radius;
            match spec.cap {
                Cap::Butt => {}
                Cap::Round => {
                    let cap_bounds = Circle::new(endpoint, half).bounding_box().to_i32_ceil();
                    blend_scanned(self, cap_bounds, spec.color, spec.alpha, |p| p.distance(&endpoint).as_raw() <= half);
                }
                // The tangent continuation: forward past `arc.end`,
                // backward past `arc.start`.
                Cap::Square => {
                    let tangent = radial.perpendicular() * if index == 0 { -half } else { half };
                    let cap = Segment::new(endpoint, endpoint + tangent);
                    let cap_bounds = cap.bounding_box().outset(Pixels::new(half)).to_i32_ceil();
                    blend_scanned(self, cap_bounds, spec.color, spec.alpha, |p| cap.interior_distance(p).is_some_and(|d| d.as_raw() <= half));
                }
            }
        }
        Ok(())
    }

    /// The sector boundary = the `w/2` bands on both rims inside
    /// `arc`'s interval, plus the two radial edge segments.
    fn stroke_sector(&mut self, spec: StrokeSector) -> Result<(), Self::Error> {
        let half = f64::from(spec.line_width.as_raw()) / 2.0;
        if half <= 0.0 {
            return Ok(());
        }
        let (center, inner, outer) = (spec.annulus.center, spec.annulus.inner, spec.annulus.outer);
        let bounds = spec.annulus.outer_circle().bounding_box().outset(Pixels::new(half)).to_i32_ceil();
        blend_scanned(self, bounds, spec.color, spec.alpha, |p| {
            spec.arc.contains((*p - center).direction())
                && ((p.distance(&center).as_raw() - outer).abs() <= half || (p.distance(&center).as_raw() - inner).abs() <= half)
        });
        for direction in [spec.arc.start, spec.arc.end()] {
            let radial = Vec2::<Pixels>::new(1.0, 0.0).rotated(direction);
            let edge = Segment::new(center + radial * inner, center + radial * outer);
            let edge_bounds = edge.bounding_box().outset(Pixels::new(half)).to_i32_ceil();
            blend_scanned(self, edge_bounds, spec.color, spec.alpha, |p| edge.distance(p).as_raw() <= half);
        }
        Ok(())
    }

    /// The sector is `annulus ∩ arc`, sampled like the other shapes.
    fn fill_sector(&mut self, spec: FillSector) -> Result<(), Self::Error> {
        let bounds = spec.annulus.outer_circle().bounding_box().to_i32_ceil();
        let center = spec.annulus.center;
        blend_scanned(self, bounds, spec.color, spec.alpha, |p| {
            spec.annulus.contains(p) && spec.arc.contains((*p - center).direction())
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smearor_commons_buffer::BufferMetadata;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_buffer::Stride;
    use smearor_commons_geometry::Annulus;
    use smearor_commons_geometry::Arc;
    use smearor_commons_geometry::Circle;
    use smearor_commons_geometry::Polygon;
    use smearor_commons_geometry::Polyline;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;

    #[test]
    fn fill_background_fills_all_pixels() {
        let mut buf = PixelBuffer::new(Size::new(4, 3));
        let color = Color::<f64>::new(10.0 / 255.0, 20.0 / 255.0, 30.0 / 255.0);
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color).unwrap();

        for px in buf.pixels().as_chunks::<4>().0.iter() {
            assert_eq!(px, &expected);
        }
    }

    #[test]
    fn fill_background_single_pixel() {
        let mut buf = PixelBuffer::new(Size::new(1, 1));
        let color = Color::<f64>::new_rgba(1.0, 0.0, 128.0 / 255.0, 200.0 / 255.0);
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color).unwrap();
        assert_eq!(buf.pixels(), &expected[..]);
    }

    #[test]
    fn fill_background_strided_view_leaves_padding_untouched() {
        // 2x2 pixels, stride 16 bytes (8 padding bytes per row):
        // the fill must write only the 8 pixel bytes of each row.
        let mut data = vec![0u8; 32];
        let metadata = BufferMetadata::new(Pixels::new(2u32), Pixels::new(2u32), Stride::new(16));
        let color = Color::<u8>::new(255, 128, 64);
        BufferViewMut::<Rgba>::from_raw(&mut data, metadata).unwrap().fill_background(color).unwrap();

        let bytes: [u8; 4] = color.into();
        assert_eq!(&data[0..8], &[bytes, bytes].concat()[..]);
        assert_eq!(&data[8..16], &[0u8; 8]);
        assert_eq!(&data[16..24], &[bytes, bytes].concat()[..]);
        assert_eq!(&data[24..32], &[0u8; 8]);
    }

    #[test]
    fn fill_background_preserves_color_via_u32_round_trip() {
        let mut buf = PixelBuffer::new(Size::new(2, 2));
        let color = Color::<f64>::new_rgba(0xAB as f64 / 255.0, 0xCD as f64 / 255.0, 0xEF as f64 / 255.0, 0x12 as f64 / 255.0);
        let expected_bytes: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color).unwrap();

        let expected = u32::from_le_bytes(expected_bytes);
        for &px in buf.pixels().as_chunks::<4>().0 {
            assert_eq!(u32::from_le_bytes(px), expected);
        }
    }

    fn px(buf: &PixelBuffer<Rgba>, x: u32, y: u32) -> [u8; 4] {
        let idx = (y * buf.size().x + x) as usize * 4;
        buf.pixels()[idx..idx + 4].try_into().unwrap()
    }

    #[test]
    fn fill_rect_fills_only_inside() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        buf.fill_rect(FillRect::new(Rect::<Pixels>::from_xywh(2.0, 2.0, 4.0, 4.0), Color::<f64>::new(1.0, 0.0, 0.0), 1.0));

        assert_eq!(px(&buf, 3, 3), [255, 0, 0, 255]);
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0]);
        assert_eq!(px(&buf, 7, 7), [0, 0, 0, 0]);
    }

    #[test]
    fn fill_rect_blends_alpha_over_background() {
        let mut buf = PixelBuffer::new(Size::new(4, 4));
        buf.fill_background(Color::<u8>::new(0, 0, 255)).unwrap();
        buf.fill_rect(FillRect::new(Rect::<Pixels>::from_xywh(0.0, 0.0, 4.0, 4.0), Color::<f64>::new(1.0, 0.0, 0.0), 0.5));

        let [r, _g, b, a] = px(&buf, 1, 1);
        assert!((120..=135).contains(&r), "half alpha red over blue ~127, got {r}");
        assert!((120..=135).contains(&b));
        assert_eq!(a, 255);
    }

    #[test]
    fn stroke_rect_strokes_border_only() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        buf.stroke_rect(StrokeRect::new(
            Rect::<Pixels>::from_xywh(2.0, 2.0, 6.0, 6.0),
            Color::<f64>::new(0.0, 1.0, 0.0),
            Pixels::new(2.0f32),
            1.0,
        ));

        assert_eq!(px(&buf, 2, 2), [0, 255, 0, 255], "corner border");
        assert_eq!(px(&buf, 4, 4), [0, 0, 0, 0], "interior untouched");
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0], "outside untouched");
    }

    #[test]
    fn fill_polygon_fills_interior() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        let triangle = Polygon::new(vec![Vec2::<Pixels>::new(2.0, 2.0), Vec2::<Pixels>::new(8.0, 2.0), Vec2::<Pixels>::new(5.0, 8.0)]);
        buf.fill_polygon(FillPolygon::new(triangle, Color::<f64>::new(1.0, 0.0, 0.0), 1.0));

        assert_eq!(px(&buf, 5, 4), [255, 0, 0, 255], "inside the triangle");
        assert_eq!(px(&buf, 2, 6), [0, 0, 0, 0], "below the triangle edge");
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0], "outside");
    }

    #[test]
    fn stroke_polyline_marks_segment_band() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        let line = Polyline::new(vec![Vec2::<Pixels>::new(1.0, 5.0), Vec2::<Pixels>::new(9.0, 5.0)]);
        buf.stroke_polyline(StrokePolyline::new(line, Color::<f64>::new(0.0, 0.0, 1.0), Pixels::new(2.0f32), 1.0));

        assert_eq!(px(&buf, 5, 5), [0, 0, 255, 255], "on the segment");
        assert_eq!(px(&buf, 5, 8), [0, 0, 0, 0], "beyond half the line width");
    }

    #[test]
    fn fill_sector_fills_ring_interval_only() {
        use smearor_commons_geometry::RotationRad;
        use std::f64::consts::FRAC_PI_2;

        let mut buf = PixelBuffer::new(Size::new(16, 16));
        let annulus = Annulus::<Pixels>::new(Vec2::<Pixels>::new(8.0, 8.0), 3.0, 6.0);
        let arc = Arc::spanning(RotationRad::new(0.0), RotationRad::new(FRAC_PI_2));
        buf.fill_sector(FillSector::new(annulus, arc, Color::<f64>::new(0.0, 1.0, 0.0), 1.0));

        assert_eq!(px(&buf, 11, 9), [0, 255, 0, 255], "in the sector");
        assert_eq!(px(&buf, 8, 8), [0, 0, 0, 0], "inside the hole");
        assert_eq!(px(&buf, 11, 5), [0, 0, 0, 0], "in the ring but outside the arc");
        assert_eq!(px(&buf, 15, 8), [0, 0, 0, 0], "beyond the outer radius");
    }

    #[test]
    fn fill_circle_fills_inside_not_corners() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        buf.fill_circle(FillCircle::new(
            Circle::<Pixels>::new(Vec2::<Pixels>::new(5.0, 5.0), 3.0),
            Color::<f64>::new(0.0, 0.0, 1.0),
            1.0,
        ));

        assert_eq!(px(&buf, 5, 5), [0, 0, 255, 255]);
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0]);
        assert_eq!(px(&buf, 9, 9), [0, 0, 0, 0]);
    }

    #[test]
    fn fill_rect_respects_strided_padding() {
        // 2x2 view, stride 16 bytes (8 padding per row)
        let mut data = vec![0u8; 32];
        {
            let metadata = BufferMetadata::new(Pixels::new(2u32), Pixels::new(2u32), Stride::new(16));
            let mut view = BufferViewMut::<Rgba>::from_raw(&mut data, metadata).unwrap();
            view.fill_rect(FillRect::new(Rect::<Pixels>::from_xywh(0.0, 0.0, 2.0, 2.0), Color::<f64>::new(1.0, 0.0, 0.0), 1.0));
        }
        assert_eq!(&data[8..16], &[0u8; 8], "padding row untouched");
        assert_eq!(&data[24..32], &[0u8; 8], "padding row untouched");
    }
}
