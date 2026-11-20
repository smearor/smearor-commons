//! The [`DrawProgressOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer delegates to its packed view.

use crate::ops::DrawProgressOps;
use crate::ops::Gauge;
use crate::ops::ProgressBar;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;

impl DrawProgressOps for PixelBuffer<Rgba> {
    fn draw_progress_bar(&mut self, bar: ProgressBar) -> Result<(), Self::Error> {
        self.view_mut().draw_progress_bar(bar)
    }

    fn draw_gauge(&mut self, gauge: Gauge) -> Result<(), Self::Error> {
        self.view_mut().draw_gauge(gauge)
    }
}
