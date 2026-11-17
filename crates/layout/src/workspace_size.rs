use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;

/// The extent of a virtual workspace surface.
///
/// A workspace is the coordinate space items live in - larger than,
/// equal to or unbounded relative to the widget rendering it. Which
/// part is actually shown is a [`crate::View`] concern.
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceSize<U> {
    /// The workspace mirrors the rendering widget's allocation - the
    /// historical single-layer behavior and the default.
    #[default]
    FollowWidget,
    /// A fixed extent in workspace units, origin at `(0, 0)`.
    Fixed(Vec2<U>),
    /// Unbounded: items may live anywhere. Without an edge to clamp
    /// against, consumers typically clamp to the visible view region
    /// so items cannot be lost off-screen.
    Infinite,
}

impl<U> WorkspaceSize<U> {
    /// Resolves the workspace extent: `FollowWidget` and `Fixed` yield
    /// a rect starting at the origin, `Infinite` yields `None`.
    ///
    /// `widget_extent` is the rendering widget's size expressed in
    /// workspace units - for `WorkspaceSize<Pixels>` the raw
    /// allocation, for other units the caller converts first.
    pub fn extent(&self, widget_extent: Vec2<U>) -> Option<Rect<U>> {
        let size = match self {
            Self::FollowWidget => widget_extent,
            Self::Fixed(size) => *size,
            Self::Infinite => return None,
        };
        Some(Rect {
            min: Vec2::new(0.0, 0.0),
            size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::WorkspaceSize;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Vec2;

    /// `FollowWidget` mirrors the widget, `Fixed` its size, `Infinite`
    /// resolves to nothing.
    #[test]
    fn extent_resolves_size_modes() {
        let widget = Vec2::<Pixels>::new(800.0, 600.0);
        let follow = WorkspaceSize::<Pixels>::FollowWidget.extent(widget).expect("extent");
        assert_eq!(follow.size, widget);
        let fixed = WorkspaceSize::<Pixels>::Fixed(Vec2::new(2000.0, 1000.0)).extent(widget).expect("extent");
        assert_eq!(fixed.size, Vec2::new(2000.0, 1000.0));
        assert_eq!(WorkspaceSize::<Pixels>::Infinite.extent(widget), None);
    }

    /// The default keeps the historical single-layer behavior.
    #[test]
    fn default_is_follow_widget() {
        assert_eq!(WorkspaceSize::<Pixels>::default(), WorkspaceSize::FollowWidget);
    }

    /// Serde round-trips all variants.
    #[test]
    fn serde_round_trip() {
        for size in [
            WorkspaceSize::<Pixels>::FollowWidget,
            WorkspaceSize::Fixed(Vec2::new(2000.0, 1000.0)),
            WorkspaceSize::Infinite,
        ] {
            let json = serde_json::to_string(&size).expect("serialize");
            assert_eq!(serde_json::from_str::<WorkspaceSize<Pixels>>(&json).expect("deserialize"), size);
        }
    }
}
