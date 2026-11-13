use crate::EasingFunction;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use std::time::Duration;
use std::time::Instant;

/// An eased, timed animation over any [`Trajectory`]: pairs the
/// trajectory with a [`Duration`] and an [`EasingFunction`] and
/// produces eased values once [`Self::start`]ed.
///
/// `TimedAnimation<Span<RotationRad>>` is the shortest-arc rotation
/// animation; `TimedAnimation<Span<ScaleFactor>>` the zoom
/// animation; `TimedAnimation<(Span<RotationRad>,
/// ThreePhaseSpan<ScaleFactor>)>` a rotation-zoom composite - one
/// clock, two trajectories.
///
/// The clock state (`start_time`) is runtime-only and skipped in
/// serialization - a deserialized animation is unstarted.
#[derive(Debug, Serialize, Deserialize)]
pub struct TimedAnimation<S> {
    /// The value trajectory - the shape of the animated output.
    pub trajectory: S,
    /// How long the animation runs after [`Self::start`].
    pub duration: Duration,
    /// The progress curve applied to linear time.
    pub easing_function: EasingFunction,
    #[serde(skip, default)]
    start_time: Option<Instant>,
}

impl<S: Trajectory> TimedAnimation<S> {
    /// Creates an animation over `trajectory` running `duration`
    /// and eased by `easing_function`. Call [`Self::start`] to
    /// begin.
    pub const fn new(trajectory: S, duration: Duration, easing_function: EasingFunction) -> Self {
        Self {
            trajectory,
            duration,
            easing_function,
            start_time: None,
        }
    }

    /// Starts the animation clock.
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Whether [`Self::start`] has been called.
    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    /// Whether the animation duration has elapsed since
    /// [`Self::start`].
    pub fn is_complete(&self) -> bool {
        self.start_time.is_some_and(|start| start.elapsed() >= self.duration)
    }

    /// The linear time progress `0.0..=1.0` since [`Self::start`] -
    /// `None` before it. Values `>= 1.0` mean the animation has run
    /// through.
    pub fn progress(&self) -> Option<Fraction> {
        self.start_time
            .map(|start| Fraction::new(start.elapsed().as_secs_f64() / self.duration.as_secs_f64()))
    }

    /// The eased trajectory value at the current point in time -
    /// `None` before [`Self::start`]. At or past the end the exact
    /// `t = 1.0` value is returned.
    pub fn value(&self) -> Option<S::Value> {
        let progress = self.progress()?;
        if progress >= Fraction::ONE {
            return Some(self.trajectory.value_at(Fraction::ONE));
        }
        Some(self.trajectory.value_at(self.easing_function.apply(progress)))
    }
}

#[cfg(test)]
mod tests {
    use super::TimedAnimation;
    use crate::EasingFunction;
    use crate::Span;
    use smearor_commons_geometry::RotationRad;
    use std::thread::sleep;
    use std::time::Duration;

    /// An unstarted animation yields nothing.
    #[test]
    fn unstarted_yields_no_value() {
        let animation = TimedAnimation::new(Span::new(0.0_f64, 10.0), Duration::from_millis(10), EasingFunction::Linear);
        assert_eq!(animation.value(), None);
        assert!(!animation.is_started());
        assert!(!animation.is_complete());
    }

    /// Half-way through a linear animation the value is near the
    /// midpoint.
    #[test]
    fn linear_animation_tracks_elapsed_time() {
        let mut animation = TimedAnimation::new(Span::new(0.0_f64, 10.0), Duration::from_millis(100), EasingFunction::Linear);
        animation.start();
        sleep(Duration::from_millis(50));
        let value = animation.value();
        assert!(value.is_some_and(|v| (3.0..7.0).contains(&v)), "value {value:?} should be near the midpoint");
    }

    /// Past the duration the animation reports `t = 1` - the exact
    /// trajectory end, not an eased approximation.
    #[test]
    fn completed_animation_returns_the_exact_end() {
        let mut animation = TimedAnimation::new(Span::new(RotationRad::ZERO, RotationRad::QUARTER_TURN), Duration::from_millis(10), EasingFunction::Linear);
        animation.start();
        sleep(Duration::from_millis(20));
        assert!(animation.is_complete());
        assert_eq!(animation.value(), Some(RotationRad::QUARTER_TURN));
    }
}
