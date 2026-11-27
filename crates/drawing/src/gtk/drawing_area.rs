use gtk4::DrawingArea;
use gtk4::cairo::Context;
use gtk4::prelude::DrawingAreaExt;
use gtk4::prelude::DrawingAreaExtManual;
use smearor_commons_geometry::Size;

/// A `DrawingArea` whose draw function is `draw`, invoked each frame
/// with the cairo context and the widget's allocated `Size<f64>` -
/// already floating-point, matching cairo's user space. `size` is the
/// content request in integer pixels - `None` leaves GTK's natural
/// sizing.
pub fn drawing_area(size: Option<Size<i32>>, draw: impl Fn(&Context, Size<f64>) + 'static) -> DrawingArea {
    let area = DrawingArea::new();
    if let Some(size) = size {
        area.set_content_width(size.x);
        area.set_content_height(size.y);
    }
    area.set_draw_func(move |_area, context, width, height| draw(context, Size::new(f64::from(width), f64::from(height))));
    area
}
