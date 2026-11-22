//! `render` - the shared per-pixel driver behind every `apply`.

use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;

/// Below this pixel count the thread fan-out costs more than it saves.
const MIN_PARALLEL_PIXELS: usize = 32 * 1024;

/// Worker count override for benchmarking (`FX_THREADS=1` forces the
/// sequential path; `0`/unset = `available_parallelism`). Read once.
static FX_THREADS: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| {
    std::env::var("FX_THREADS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()))
});

/// Backend override for A/B measurement: `FX_BACKEND=scoped` keeps the
/// `std::thread::scope` path inside a rayon-enabled build.
#[cfg(feature = "rayon")]
static FX_BACKEND_RAYON: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| std::env::var("FX_BACKEND").map(|v| v != "scoped").unwrap_or(true));

/// Fills `out` pixel-by-pixel with `shade(x, y)` - parallel over row
/// bands when the view is packed and large enough; strided or tiny
/// views take the sequential path. With `feature = "rayon"` the bands
/// run on rayon's work-stealing pool (persistent threads, no per-frame
/// spawn cost); otherwise on [`std::thread::scope`]. Every row is
/// written by exactly one worker, so the output is deterministic and
/// identical to the sequential render.
pub(crate) fn fill<F>(out: &mut BufferViewMut<'_, Rgba>, shade: F)
where
    F: Fn(u32, u32) -> Color<f64> + Sync,
{
    let size = out.size();
    let workers = if out.is_packed() && size.x as usize * size.y as usize >= MIN_PARALLEL_PIXELS {
        (*FX_THREADS).min(size.y as usize)
    } else {
        1
    };
    if workers <= 1 {
        fill_sequential(out, &shade);
        return;
    }
    #[cfg(feature = "rayon")]
    if *FX_BACKEND_RAYON {
        return fill_parallel_rayon(out, &shade, workers);
    }
    fill_parallel(out, &shade, workers);
}

/// The sequential row loop - the reference implementation the
/// parallel path is pinned against.
fn fill_sequential<F>(out: &mut BufferViewMut<'_, Rgba>, shade: &F)
where
    F: Fn(u32, u32) -> Color<f64> + Sync,
{
    let size = out.size();
    for y in 0..size.y {
        for x in 0..size.x {
            out.put_color(Pixels::new(x), Pixels::new(y), shade(x, y));
        }
    }
}

/// Row-band fan-out: `out`'s packed rows are split into `workers`
/// contiguous bands, one scoped thread per band. Bands are disjoint,
/// so the result is byte-identical to [`fill_sequential`].
fn fill_parallel<F>(out: &mut BufferViewMut<'_, Rgba>, shade: &F, workers: usize)
where
    F: Fn(u32, u32) -> Color<f64> + Sync,
{
    let size = out.size();
    let workers = workers.min(size.y as usize);
    let row_bytes = size.x as usize * Rgba::BYTES_PER_PIXEL;
    let band_rows = (size.y as usize).div_ceil(workers) as u32;
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

/// Same banding on rayon's global pool - no per-frame thread spawns
/// and work stealing absorbs uneven row costs (wormhole's bisection
/// rows). `workers` bounds the fan-out via the band count; the pool
/// itself always runs.
#[cfg(feature = "rayon")]
fn fill_parallel_rayon<F>(out: &mut BufferViewMut<'_, Rgba>, shade: &F, workers: usize)
where
    F: Fn(u32, u32) -> Color<f64> + Sync,
{
    use rayon::prelude::IndexedParallelIterator;
    use rayon::prelude::ParallelIterator;
    use rayon::prelude::ParallelSliceMut;
    let size = out.size();
    let workers = workers.min(size.y as usize);
    let row_bytes = size.x as usize * Rgba::BYTES_PER_PIXEL;
    let band_rows = (size.y as usize).div_ceil(workers) as u32;
    out.data_mut()
        .par_chunks_mut(band_rows as usize * row_bytes)
        .enumerate()
        .for_each(|(band, chunk)| {
            let band_height = (chunk.len() / row_bytes) as u32;
            if let Ok(mut view) = BufferViewMut::<Rgba>::from_packed(chunk, Size::new(size.x, band_height)) {
                let y_offset = band as u32 * band_rows;
                for ly in 0..band_height {
                    for x in 0..size.x {
                        view.put_color(Pixels::new(x), Pixels::new(ly), shade(x, y_offset + ly));
                    }
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    use smearor_commons_buffer::PixelBuffer;

    /// `rayon_parity` from the phase-6 test plan, without the rayon
    /// dependency: the banded path must produce byte-identical output
    /// to the sequential reference.
    #[test]
    fn parallel_output_is_byte_identical() {
        let size = Size::new(96, 48);
        let shade =
            |x: u32, y: u32| Color::<f64>::new_rgba(f64::from(x % 7) / 7.0, f64::from(y % 5) / 5.0, f64::from((x + y) % 3) / 3.0, f64::from((x ^ y) % 9) / 9.0);
        let mut a = PixelBuffer::<Rgba>::new(size);
        let mut b = PixelBuffer::<Rgba>::new(size);
        fill_sequential(&mut a.view_mut(), &shade);
        fill_parallel(&mut b.view_mut(), &shade, 7);
        assert_eq!(a.pixels(), b.pixels());
    }

    /// Same pin for the rayon backend - work stealing may reorder
    /// *when* a band runs, never *what* it writes.
    #[cfg(feature = "rayon")]
    #[test]
    fn rayon_output_is_byte_identical() {
        let size = Size::new(96, 48);
        let shade =
            |x: u32, y: u32| Color::<f64>::new_rgba(f64::from(x % 7) / 7.0, f64::from(y % 5) / 5.0, f64::from((x + y) % 3) / 3.0, f64::from((x ^ y) % 9) / 9.0);
        let mut a = PixelBuffer::<Rgba>::new(size);
        let mut b = PixelBuffer::<Rgba>::new(size);
        fill_sequential(&mut a.view_mut(), &shade);
        fill_parallel_rayon(&mut b.view_mut(), &shade, 7);
        assert_eq!(a.pixels(), b.pixels());
    }
}
