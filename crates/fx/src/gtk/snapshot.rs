//! Widget snapshotting - a live widget frozen into a
//! [`PixelBuffer<Rgba>`] texture, once per animation.

use crate::FxError;
use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;
use gtk4::Snapshot;
use gtk4::Widget;
use gtk4::WidgetPaintable;
use gtk4::prelude::Cast;
use gtk4::prelude::IsA;
use gtk4::prelude::PaintableExt;
use gtk4::prelude::SnapshotExt;
use gtk4::prelude::WidgetExt;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_drawing::image_surface_from_buffer;
use smearor_commons_drawing::pixel_buffer_from_surface;
use std::fs::File;
use std::path::Path;

/// Renders `widget` into a straight-alpha [`PixelBuffer<Rgba>`] at
/// its current allocation times the surface scale factor.
///
/// The path: `WidgetPaintable::new(widget)` → `Snapshot::to_node()` → the
/// render node drawn onto an `ARgb32` cairo [`ImageSurface`] →
/// [`pixel_buffer_from_surface`] → unpremultiply → `Bgra` → `Rgba`
/// channel swap.
///
/// Snapshot once per animation - the effect animates a frozen
/// texture; live content is a non-goal by design.
///
/// # Errors
///
/// * [`FxError::SnapshotEmpty`] - zero-size allocation or the widget
///   produced no render node (unrealized/offscreen widgets may still
///   snapshot; GTK decides).
/// * [`FxError::Cairo`] - surface/context status failures.
pub fn snapshot(widget: &impl IsA<Widget>) -> Result<PixelBuffer<Rgba>, FxError> {
    let widget = widget.upcast_ref::<Widget>();
    let scale = i64::from(widget.scale_factor()).max(1);
    let width = i64::from(widget.width()) * scale;
    let height = i64::from(widget.height()) * scale;
    if width <= 0 || height <= 0 {
        return Err(FxError::SnapshotEmpty);
    }

    let paintable = WidgetPaintable::new(Some(widget));
    let snapshot = Snapshot::new();
    paintable.snapshot(&snapshot, f64::from(widget.width()), f64::from(widget.height()));
    let Some(node) = snapshot.to_node() else {
        return Err(FxError::SnapshotEmpty);
    };

    let mut surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).map_err(|e| FxError::Cairo(e.to_string()))?;
    {
        let cr = Context::new(&surface).map_err(|e| FxError::Cairo(e.to_string()))?;
        if scale > 1 {
            cr.scale(scale as f64, scale as f64);
        }
        node.draw(&cr);
    }

    let buffer = pixel_buffer_from_surface(&mut surface).map_err(|e| FxError::Cairo(e.to_string()))?;
    Ok(buffer.into_unpremultiplied().into_rgba())
}

/// Renders `widget` like [`snapshot`] and writes the pixels to `path`
/// as PNG - the probe "render once, dump, quit" idiom in one call.
///
/// # Errors
///
/// * [`FxError::SnapshotEmpty`] / [`FxError::Cairo`] - see [`snapshot`].
/// * [`FxError::PngDump`] - buffer-to-surface conversion or the PNG
///   file write.
pub fn snapshot_to_png(widget: &impl IsA<Widget>, path: impl AsRef<Path>) -> Result<(), FxError> {
    let buffer = snapshot(widget)?;
    let surface = image_surface_from_buffer(&buffer).map_err(|error| FxError::PngDump(error.to_string()))?;
    let mut file = File::create(path.as_ref()).map_err(|error| FxError::PngDump(error.to_string()))?;
    surface.write_to_png(&mut file).map_err(|error| FxError::PngDump(error.to_string()))?;
    Ok(())
}
