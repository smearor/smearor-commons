use crate::TimedAnimation;
use crate::Trajectory;
use gtk4::Widget;
use gtk4::glib::ControlFlow;
use gtk4::glib::object::IsA;
use gtk4::prelude::WidgetExtManual;

/// Drives `animation` on `widget`'s frame clock: each frame calls
/// `apply` with the eased value until the animation completes.
/// `apply` returning [`ControlFlow::Break`] stops early.
///
/// The animation is owned by the frame driver - consumer state
/// (what to do with each value) lives in `apply`, not in widget
/// fields. Mutation in `apply` goes through interior mutability
/// (`Cell`/`RefCell`) - the GTK idiom.
///
/// ```ignore
/// let mut animation = TimedAnimation::new(Span::new(from, to), duration, EasingFunction::EaseInOut);
/// animation.start();
/// drive(&widget, animation, |widget, rotation| {
///     widget.imp().rotation.set(rotation);
///     widget.queue_allocate();
///     ControlFlow::Continue
/// });
/// ```
pub fn drive<W, S>(widget: &W, animation: TimedAnimation<S>, apply: impl Fn(&W, S::Value) -> ControlFlow + 'static)
where
    W: IsA<Widget> + 'static,
    S: Trajectory + 'static,
    S::Value: Copy,
{
    widget.add_tick_callback(move |widget, _frame_clock| {
        let Some(value) = animation.value() else {
            return ControlFlow::Break;
        };
        let flow = apply(widget, value);
        if animation.is_complete() { ControlFlow::Break } else { flow }
    });
}
