//! Pointer wiring - `EventController` → `ctx.pointer` in uv space.

use gtk4::prelude::Cast;
use gtk4::prelude::WidgetExt;
use smearor_commons_geometry::Vec2;
use std::cell::Cell;
use std::rc::Rc;

/// Wires motion and click events on `widget` into `target`.
///
/// Stored positions are widget-relative uv (`x / width`,
/// `y / height`) - the coordinate space [`crate::FxContext::pointer`]
/// expects (BMW's `uStartPos`/`uEpicenter` are actor-local uvs).
/// Leaving the widget clears the pointer to `None`; clicks update the
/// position too, so press-only gestures (the broken-glass epicenter)
/// work without preceding motion.
pub(crate) fn wire_pointer(widget: &impl gtk4::prelude::IsA<gtk4::Widget>, target: Rc<Cell<Option<Vec2<()>>>>) {
    let widget = widget.upcast_ref::<gtk4::Widget>().clone();
    let to_uv = |widget: &gtk4::Widget, x: f64, y: f64| {
        let w = f64::from(widget.width()).max(1.0);
        let h = f64::from(widget.height()).max(1.0);
        Vec2::<()>::new(x / w, y / h)
    };

    let motion = gtk4::EventControllerMotion::new();
    motion.connect_motion({
        let target = Rc::clone(&target);
        let widget = widget.clone();
        move |_, x, y| target.set(Some(to_uv(&widget, x, y)))
    });
    motion.connect_leave({
        let target = Rc::clone(&target);
        move |_| target.set(None)
    });
    widget.add_controller(motion);

    let click = gtk4::GestureClick::new();
    click.connect_pressed({
        let widget = widget.clone();
        move |_, _n, x, y| target.set(Some(to_uv(&widget, x, y)))
    });
    widget.add_controller(click);
}
