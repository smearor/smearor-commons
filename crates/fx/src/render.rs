//! `render` - the shared per-pixel driver behind every `apply`.

use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;

/// Below this pixel count the thread fan-out costs more than it saves.
const MIN_PARALLEL_PIXELS: usize = 32 * 1024;

/// Fills `out` pixel-by-pixel with `shade(x, y)` - parallel over row
/// bands via [`std::thread::scope`] when the view is packed and large
/// enough; strided or tiny views take the sequential path. Every row
/// is written by exactly one worker, so the output is deterministic
/// and identical to the sequential render.
pub(crate) fn fill<F>(out: &mut BufferViewMut<'_, Rgba>, shade: F)
where
    F: Fn(u32, u32) -> Color<f64> + Sync,
{
    let size = out.size();
    let workers = if out.is_packed() && size.x as usize * size.y as usize >= MIN_PARALLEL_PIXELS {
        std::thread::available_parallelism().map_or(1, |n| n.get()).min(size.y as usize)
    } else {
        1
    };
    if workers <= 1 {
        for y in 0..size.y {
            for x in 0..size.x {
                out.put_color(Pixels::new(x), Pixels::new(y), shade(x, y));
            }
        }
        return;
    }
    let row_bytes = size.x as usize * Rgba::BYTES_PER_PIXEL;
    let band_rows = (size.y as usize).div_ceil(workers) as u32;
    let shade = &shade;
    std::thread::scope(|scope| {
        for (band, chunk) in out.data_mut().chunks_mut(band_rows as usize * row_bytes).enumerate() {
            scope.spawn(move || {
                let band_height = (chunk.len() / row_bytes) as u32;
                let Ok(mut view) = BufferViewMut::<Rgba>::from_packed(chunk, Size::new(size.x, band_height)) else {
                    return;
                };
                let y_offset = band as u32 * band_rows;
                for ly in 0..band_height {
                    for x in 0..size.x {
                        view.put_color(Pixels::new(x), Pixels::new(ly), shade(x, y_offset + ly));
                    }
                }
            });
        }
    });
}
