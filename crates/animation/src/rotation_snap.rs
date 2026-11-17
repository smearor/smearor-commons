use crate::SnapUpdate;
use smearor_commons_geometry::Degrees;
use smearor_commons_geometry::QuarterTurn;
use smearor_commons_geometry::RotationDirection;
use smearor_commons_geometry::RotationRad;

/// The default accumulated drag that settles a snap.
pub const DEFAULT_SNAP: Degrees = Degrees::EIGHTH_TURN;

/// The default accumulated drag that locks the direction and emits
/// engagement feedback.
pub const DEFAULT_DIRECTION: Degrees = Degrees::new(10.0);

/// The default engagement-feedback nudge applied in the drag
/// direction.
pub const DEFAULT_FEEDBACK: Degrees = Degrees::new(2.0);

/// Tracks a rotate gesture and decides when it snaps to the next
/// [`QuarterTurn`].
///
/// Lifecycle per gesture: construct with the rotation captured at
/// gesture begin, feed every reported *cumulative* gesture angle
/// into [`update`](Self::update), then consume the session with
/// [`end`](Self::end) - `end` yields the rotation to restore when
/// the drag never reached the snap threshold.
///
/// Three stages per drag: below `direction` nothing happens;
/// crossing it locks the drag direction and emits a one-shot
/// `feedback` nudge signalling engagement; crossing `snap`
/// emits the `target` quarter turn once and freezes the session.
pub struct RotationSnap {
    /// The rotation captured at gesture begin - restore target and
    /// base of the snap-zone lookup.
    initial: RotationRad,
    /// The previously reported cumulative gesture angle.
    last: Degrees,
    /// The accumulated unwrapped drag - positive is clockwise.
    total: Degrees,
    /// The drag direction when feedback was emitted - `None` until
    /// `direction` is passed. Only locks feedback; the snap target
    /// follows the accumulated total's sign.
    direction: Option<RotationDirection>,
    /// Whether a snap target was emitted - freezes the session.
    snapped: bool,
    /// Accumulated drag that settles the snap.
    snap: Degrees,
    /// Accumulated drag that locks the direction and emits feedback.
    direction_threshold: Degrees,
    /// Engagement-feedback nudge applied in the drag direction.
    feedback: Degrees,
}

impl RotationSnap {
    /// A session with the default thresholds: direction locks at
    /// [`DEFAULT_DIRECTION`], the snap fires at [`DEFAULT_SNAP`] and
    /// engagement feedback nudges [`DEFAULT_FEEDBACK`] in the drag
    /// direction.
    pub fn new(initial: RotationRad) -> Self {
        Self::with_thresholds(initial, DEFAULT_SNAP, DEFAULT_DIRECTION, DEFAULT_FEEDBACK)
    }

    /// A session with explicit thresholds: `snap` is the accumulated
    /// drag that settles the snap, `direction` locks the direction
    /// and emits feedback, `feedback` is the nudge amount applied in
    /// the drag direction.
    ///
    /// `snap` should exceed `direction` so the direction is known
    /// when the snap fires; a `feedback` of [`Degrees::ZERO`] still
    /// reports a (zero) nudge.
    pub fn with_thresholds(initial: RotationRad, snap: Degrees, direction: Degrees, feedback: Degrees) -> Self {
        Self {
            initial,
            last: Degrees::ZERO,
            total: Degrees::ZERO,
            direction: None,
            snapped: false,
            snap,
            direction_threshold: direction,
            feedback,
        }
    }

    /// Feeds the cumulative gesture `angle` - reported relative to
    /// gesture begin - into the session.
    ///
    /// The delta to the previous report is unwrapped across the
    /// `0°`/`360°` boundary before accumulating: raw reports jump
    /// `±360°` there while the drag itself is continuous.
    pub fn update(&mut self, angle: RotationRad) -> SnapUpdate {
        let mut update = SnapUpdate::default();
        if self.snapped {
            return update;
        }

        let current = angle.degrees();
        let mut delta = current - self.last;
        if delta > Degrees::HALF_TURN {
            delta = delta - Degrees::FULL_TURN;
        } else if delta < -Degrees::HALF_TURN {
            delta = delta + Degrees::FULL_TURN;
        }
        self.last = current;
        self.total = self.total + delta;

        if self.direction.is_none() && self.total.abs() > self.direction_threshold {
            self.direction = RotationDirection::of(self.total);
            let feedback = match self.direction {
                Some(RotationDirection::CounterClockwise) => -self.feedback,
                _ => self.feedback,
            };
            update.feedback = Some(RotationRad::from(feedback));
        }

        if self.total.abs() >= self.snap {
            self.snapped = true;
            update.target = RotationDirection::of(self.total).map(|direction| QuarterTurn::snap_target(self.initial, direction));
        }

        update
    }

    /// Finishes the session: the rotation to restore when the drag
    /// never reached the snap threshold, `None` when a snap target
    /// was emitted.
    ///
    /// Restoring `initial` undoes the engagement-feedback nudge the
    /// consumer may have applied mid-drag.
    pub fn end(self) -> Option<RotationRad> {
        if self.snapped { None } else { Some(self.initial) }
    }
}

#[cfg(test)]
mod tests {
    use super::RotationSnap;
    use super::SnapUpdate;
    use smearor_commons_geometry::QuarterTurn;
    use smearor_commons_geometry::RotationRad;

    /// Reports `total` degrees as one cumulative gesture update.
    fn feed(session: &mut RotationSnap, degrees: f64) -> SnapUpdate {
        session.update(RotationRad::from_degrees(degrees))
    }

    /// A drag below the direction threshold produces nothing.
    #[test]
    fn small_drags_stay_pending() {
        let mut session = RotationSnap::new(RotationRad::from_degrees(0.0));
        let update = feed(&mut session, 5.0);
        assert_eq!(update.feedback, None);
        assert_eq!(update.target, None);
        assert_eq!(session.end(), Some(RotationRad::from_degrees(0.0)));
    }

    /// Crossing the direction threshold emits one feedback nudge in
    /// the drag direction, never twice.
    #[test]
    fn direction_lock_emits_feedback_once() {
        let mut session = RotationSnap::new(RotationRad::from_degrees(0.0));
        let update = feed(&mut session, 15.0);
        assert_eq!(update.feedback, Some(RotationRad::from_degrees(2.0)));
        assert_eq!(update.target, None);

        let update = feed(&mut session, 20.0);
        assert_eq!(update.feedback, None);
        assert_eq!(update.target, None);
    }

    /// Crossing the snap threshold emits the quarter turn one step
    /// clockwise from the initial rotation's zone - and freezes.
    #[test]
    fn snap_threshold_emits_target_once() {
        let mut session = RotationSnap::new(RotationRad::from_degrees(0.0));
        feed(&mut session, 15.0);
        let update = feed(&mut session, 50.0);
        assert_eq!(update.feedback, None);
        assert_eq!(update.target, Some(QuarterTurn::Deg90));

        let update = feed(&mut session, 60.0);
        assert_eq!(update.feedback, None);
        assert_eq!(update.target, None);
        assert_eq!(session.end(), None);
    }

    /// Counter-clockwise drags snap to the previous quarter turn and
    /// nudge with a negative feedback offset.
    #[test]
    fn counter_clockwise_drag_snaps_backwards() {
        let mut session = RotationSnap::new(RotationRad::from_degrees(90.0));
        let update = feed(&mut session, -50.0);
        assert_eq!(update.feedback, Some(RotationRad::from_degrees(-2.0)));
        assert_eq!(update.target, Some(QuarterTurn::Deg0));
        assert_eq!(session.end(), None);
    }

    /// A single jump crossing both thresholds emits feedback and
    /// target in the same update, feedback first.
    #[test]
    fn one_jump_emits_feedback_and_target() {
        let mut session = RotationSnap::new(RotationRad::from_degrees(0.0));
        let update = feed(&mut session, 50.0);
        assert_eq!(update.feedback, Some(RotationRad::from_degrees(2.0)));
        assert_eq!(update.target, Some(QuarterTurn::Deg90));
    }

    /// Gesture reports wrap at `0°`/`360°`: dragging 15°
    /// counter-clockwise reports `345`, and reversing back through
    /// zero unwraps each step instead of jumping `±360°`.
    #[test]
    fn angle_reports_unwrap_across_zero() {
        let mut session = RotationSnap::new(RotationRad::from_degrees(0.0));
        let update = feed(&mut session, 345.0);
        assert_eq!(update.feedback, Some(RotationRad::from_degrees(-2.0)));

        feed(&mut session, 5.0); // +20° past the wrap, total +5
        let update = feed(&mut session, 50.0); // +45°, total +50
        // A reversed drag snaps in the direction it accumulated to,
        // even though the feedback nudge went counter-clockwise.
        assert_eq!(update.target, Some(QuarterTurn::Deg90));
        assert_eq!(session.end(), None);
    }

    /// A session abandoned below the snap threshold restores the
    /// initial rotation.
    #[test]
    fn end_restores_initial_when_unsnapped() {
        let mut session = RotationSnap::new(RotationRad::from_degrees(270.0));
        feed(&mut session, -30.0);
        assert_eq!(session.end(), Some(RotationRad::from_degrees(270.0)));
    }
}
