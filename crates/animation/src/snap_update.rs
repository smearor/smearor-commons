use smearor_commons_geometry::QuarterTurn;
use smearor_commons_geometry::RotationRad;

/// What one [`RotationSnap::update`] produced for a reported gesture
/// angle.
///
/// Both fields can fire in a single update: a drag jumping straight
/// past the snap threshold still emits its `feedback` first, keeping
/// the "nudge, then snap" order the widget applies them in.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SnapUpdate {
    /// Direction-engagement feedback: add this offset to the current
    /// rotation once, when the drag direction first locks in.
    pub feedback: Option<RotationRad>,
    /// The snap threshold was crossed - the gesture settles on this
    /// quarter turn. Emitted at most once per session.
    pub target: Option<QuarterTurn>,
}
