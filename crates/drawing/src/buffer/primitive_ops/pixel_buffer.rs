//! The [`DrawPrimitiveOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer draws primitives through its packed view.

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
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;

impl DrawPrimitiveOps for PixelBuffer<Rgba> {
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        self.view_mut().fill_background(color)
    }

    fn fill_rect(&mut self, spec: FillRect) -> Result<(), Self::Error> {
        self.view_mut().fill_rect(spec)
    }

    fn stroke_rect(&mut self, spec: StrokeRect) -> Result<(), Self::Error> {
        self.view_mut().stroke_rect(spec)
    }

    fn fill_circle(&mut self, spec: FillCircle) -> Result<(), Self::Error> {
        self.view_mut().fill_circle(spec)
    }

    fn stroke_arc(&mut self, spec: StrokeArc) -> Result<(), Self::Error> {
        self.view_mut().stroke_arc(spec)
    }

    fn stroke_circle(&mut self, spec: StrokeCircle) -> Result<(), Self::Error> {
        self.view_mut().stroke_circle(spec)
    }

    fn stroke_polygon(&mut self, spec: StrokePolygon) -> Result<(), Self::Error> {
        self.view_mut().stroke_polygon(spec)
    }

    fn fill_polygon(&mut self, spec: FillPolygon) -> Result<(), Self::Error> {
        self.view_mut().fill_polygon(spec)
    }

    fn stroke_polyline(&mut self, spec: StrokePolyline) -> Result<(), Self::Error> {
        self.view_mut().stroke_polyline(spec)
    }

    fn stroke_sector(&mut self, spec: StrokeSector) -> Result<(), Self::Error> {
        self.view_mut().stroke_sector(spec)
    }

    fn fill_sector(&mut self, spec: FillSector) -> Result<(), Self::Error> {
        self.view_mut().fill_sector(spec)
    }
}
