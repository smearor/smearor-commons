//! Widget snapshotting - a live widget frozen into a
//! [`PixelBuffer<Rgba>`] texture, once per animation.

use crate::FxError;
use gtk4::prelude::IsA;
use gtk4::prelude::PaintableExt;
use gtk4::prelude::SnapshotExt;
use gtk4::prelude::WidgetExt;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_drawing::pixel_buffer_from_surface;

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
pub fn snapshot(widget: &impl IsA<gtk4::Widget>) -> Result<PixelBuffer<Rgba>, FxError> {
    use gtk4::prelude::Cast;
    let widget = widget.upcast_ref::<gtk4::Widget>();
    let scale = i64::from(widget.scale_factor()).max(1);
    let width = i64::from(widget.width()) * scale;
    let height = i64::from(widget.height()) * scale;
    if width <= 0 || height <= 0 {
        return Err(FxError::SnapshotEmpty);
    }

    let paintable = gtk4::WidgetPaintable::new(Some(widget));
    let snapshot = gtk4::Snapshot::new();
    paintable.snapshot(&snapshot, f64::from(widget.width()), f64::from(widget.height()));
    let Some(node) = snapshot.to_node() else {
        return Err(FxError::SnapshotEmpty);
    };

    let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32).map_err(|e| FxError::Cairo(e.to_string()))?;
    {
        let cr = cairo::Context::new(&surface).map_err(|e| FxError::Cairo(e.to_string()))?;
        if scale > 1 {
            cr.scale(scale as f64, scale as f64);
        }
        node.draw(&cr);
    }

    let buffer = pixel_buffer_from_surface(&mut surface).map_err(|e| FxError::Cairo(e.to_string()))?;
    Ok(buffer.into_unpremultiplied().into_rgba())
}
