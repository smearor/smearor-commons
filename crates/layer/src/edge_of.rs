use gtk4_layer_shell::Edge;
#[cfg(feature = "layout")]
use smearor_commons_layout::TableSide;
use smearor_commons_geometry::QuarterTurn;
use smearor_commons_geometry::Rotation;

/// Converts snapped four-orientation vocabularies into the
/// layer-shell screen [`Edge`] they face.
///
/// [`QuarterTurn`], [`TableSide`](smearor_commons_layout::TableSide)
/// and [`Rotation`] name the same four sides of a surface in
/// different domains - reading rotation, seat position and
/// layer-shell anchor. `edge()` maps them onto that single edge,
/// replacing per-app match tables and float-epsilon comparisons of
/// raw degree values.
pub trait EdgeOf {
    /// The screen edge this orientation faces; `None` for values
    /// that are not snapped to a quarter turn.
    fn edge(self) -> Option<Edge>;
}

impl EdgeOf for QuarterTurn {
    fn edge(self) -> Option<Edge> {
        Some(match self {
            Self::Deg0 => Edge::Bottom,
            Self::Deg90 => Edge::Left,
            Self::Deg180 => Edge::Top,
            Self::Deg270 => Edge::Right,
        })
    }
}

#[cfg(feature = "layout")]
impl EdgeOf for TableSide {
    fn edge(self) -> Option<Edge> {
        Some(match self {
            Self::Bottom => Edge::Bottom,
            Self::Left => Edge::Left,
            Self::Top => Edge::Top,
            Self::Right => Edge::Right,
        })
    }
}

impl EdgeOf for Rotation {
    /// Free rotations have no edge - [`Rotation::quarter_turn`]
    /// returns `None` and so does this.
    fn edge(self) -> Option<Edge> {
        self.quarter_turn()?.edge()
    }
}

#[cfg(test)]
mod tests {
    use super::EdgeOf;
    use gtk4_layer_shell::Edge;
    use smearor_commons_geometry::QuarterTurn;
    use smearor_commons_geometry::Rotation;
    use smearor_commons_geometry::RotationRad;

    /// Each quarter turn faces the edge matching its orientation:
    /// `Deg0` is anchored at the bottom, `Deg90` at the left, and so
    /// on clockwise around the surface.
    #[test]
    fn quarter_turns_face_clockwise_edges() {
        assert_eq!(QuarterTurn::Deg0.edge(), Some(Edge::Bottom));
        assert_eq!(QuarterTurn::Deg90.edge(), Some(Edge::Left));
        assert_eq!(QuarterTurn::Deg180.edge(), Some(Edge::Top));
        assert_eq!(QuarterTurn::Deg270.edge(), Some(Edge::Right));
    }

    /// Snapped rotations yield their turn's edge, free rotations
    /// yield `None`.
    #[test]
    fn rotations_face_edges_only_when_snapped() {
        assert_eq!(Rotation::QuarterTurn(QuarterTurn::Deg270).edge(), Some(Edge::Right));
        assert_eq!(Rotation::Free(RotationRad::from_degrees(45.0)).edge(), None);
    }

    /// Table sides face the same-named edge.
    #[cfg(feature = "layout")]
    #[test]
    fn table_sides_face_same_named_edges() {
        use smearor_commons_layout::TableSide;

        assert_eq!(TableSide::Bottom.edge(), Some(Edge::Bottom));
        assert_eq!(TableSide::Right.edge(), Some(Edge::Right));
    }
}
