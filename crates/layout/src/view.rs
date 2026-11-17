use serde::Deserialize;
use serde::Serialize;
use smearor_commons_animation::Interpolate;
use smearor_commons_geometry::AspectRatio;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::PixelsPerMm;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;

/// The render scale (pixels per millimeter) that makes one workspace
/// millimeter one physical millimeter on a screen of `dpi` dots per
/// inch.
///
/// `dpi` comes from `GdkMonitor` (`width_mm`/`height_mm` plus pixel
/// geometry) or a calibration override for displays reporting wrong
/// physical sizes (projectors). Only meaningful for
/// [`View`]s over millimeter workspaces.
pub fn physical_zoom(dpi: f64) -> PixelsPerMm {
    PixelsPerMm::new(dpi / 25.4)
}

/// The camera over a workspace: which `Rect<U>` region maps onto the
/// widget, and at which on-screen rotation.
///
/// The region is the workspace area *requested* to be visible; when
/// its aspect ratio differs from the widget's it is grown (never
/// shrunk) to match - see [`Self::shown_region`]. Rotation turns the
/// workspace on screen: at a multi-user table, rotating the view is
/// "walking around the table" while seats and item orientations stay
/// anchored to the workspace.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct View<U> {
    /// The workspace region requested to be visible.
    pub region: Rect<U>,
    /// The workspace's rotation on screen - `RotationRad::ZERO` shows
    /// it as stored.
    pub rotation: RotationRad,
}

impl<U> View<U> {
    /// A view covering `region` without rotation.
    pub const fn covering(region: Rect<U>) -> Self {
        Self {
            region,
            rotation: RotationRad::ZERO,
        }
    }

    /// Moves the shown region by `delta` workspace units.
    pub fn pan(&mut self, delta: Vec2<U>) {
        self.region.min += delta;
    }

    /// Zooms `factor`-fold around `anchor`: a factor of `2` halves the
    /// shown region (2x closer). The anchor point keeps its widget
    /// position (zoom-to-point).
    pub fn zoom_at(&mut self, anchor: Vec2<U>, factor: ScaleFactor) {
        debug_assert!(factor.as_f64() > 0.0);
        let shrink = factor.inverse();
        let center = self.region.center();
        self.region = Rect::centered(anchor + (center - anchor) * shrink, self.region.size * shrink);
    }

    /// Rotates the workspace by `delta` around the workspace `pivot` -
    /// the pivot keeps its widget position.
    pub fn rotate_around(&mut self, pivot: Vec2<U>, delta: RotationRad) {
        let center = self.region.center();
        self.region = Rect::centered(pivot - (pivot - center).rotated(-delta), self.region.size);
        self.rotation = self.rotation.rotated(delta);
    }

    /// The region actually shown on a widget of `widget_size`: the
    /// configured region expanded (never shrunk) to the widget's
    /// aspect ratio, centered. A degenerate widget size falls back to
    /// the configured region.
    pub fn shown_region(&self, widget_size: Vec2<Pixels>) -> Rect<U> {
        if self.region.is_degenerate() {
            return self.region;
        }
        match widget_size.aspect() {
            Some(aspect) => expand_to_aspect(self.region, aspect),
            None => self.region,
        }
    }

    /// Workspace point → widget coordinate (the render path).
    ///
    /// The shown region's center maps to the widget's center, scaled
    /// uniformly and rotated by [`Self::rotation`].
    pub fn to_widget(&self, point: Vec2<U>, widget: Rect<Pixels>) -> Vec2<Pixels> {
        let shown = self.shown_region(widget.size);
        let scale = ScaleFactor::new(widget.size.x / shown.size.x);
        let delta = (point - shown.center()).rotated(self.rotation) * scale;
        widget.center() + delta.with_unit::<Pixels>()
    }

    /// Widget coordinate → workspace point (the input path) - the
    /// inverse of [`Self::to_widget`].
    pub fn to_workspace(&self, point: Vec2<Pixels>, widget: Rect<Pixels>) -> Vec2<U> {
        let shown = self.shown_region(widget.size);
        let scale = ScaleFactor::new(widget.size.x / shown.size.x);
        let delta = (point - widget.center()) * scale.inverse();
        shown.center() + delta.with_unit::<U>().rotated(-self.rotation)
    }
}

impl<U> Default for View<U> {
    fn default() -> Self {
        Self::covering(Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(0.0, 0.0),
        })
    }
}

impl<U> Interpolate for View<U> {
    /// Interpolates region corners and rotation component-wise, so a
    /// view transition glides through pan, zoom and rotation at once.
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self {
            region: Rect {
                min: self.region.min.interpolate(target.region.min, t),
                size: self.region.size.interpolate(target.region.size, t),
            },
            rotation: self.rotation.interpolate(target.rotation, t),
        }
    }
}

/// `region` grown (never shrunk) to `aspect`, centered on the
/// region's center.
fn expand_to_aspect<U>(region: Rect<U>, aspect: AspectRatio) -> Rect<U> {
    let ratio = aspect.as_f64();
    let Some(current) = region.size.aspect() else { return region };
    let size = if (current.as_f64() - ratio).abs() < 1e-9 {
        return region;
    } else if current < aspect {
        Vec2::new(region.size.y * ratio, region.size.y)
    } else {
        Vec2::new(region.size.x, region.size.x / ratio)
    };
    Rect::centered(region.center(), size)
}

#[cfg(test)]
mod tests {
    use super::View;
    use super::expand_to_aspect;
    use super::physical_zoom;
    use smearor_commons_animation::Interpolate;
    use smearor_commons_geometry::AspectRatio;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::PixelsPerMm;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::RotationRad;
    use smearor_commons_geometry::ScaleFactor;
    use smearor_commons_geometry::Vec2;

    fn widget() -> Rect<Pixels> {
        Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(800.0, 600.0),
        }
    }

    /// `pan` shifts the region without touching size or rotation.
    #[test]
    fn pan_moves_region() {
        let mut view = View::covering(Rect {
            min: Vec2::<Pixels>::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
        });
        view.pan(Vec2::new(10.0, 20.0));
        assert_eq!(view.region.min, Vec2::new(10.0, 20.0));
    }

    /// A zoom around an anchor keeps that anchor's widget position -
    /// the zoom-to-point contract of touch UIs.
    #[test]
    fn zoom_at_keeps_anchor_fixed_on_widget() {
        let anchor = Vec2::<Pixels>::new(25.0, 25.0);
        let mut view = View::covering(Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
        });
        let before = view.to_widget(anchor, widget());
        view.zoom_at(anchor, ScaleFactor::new(2.0));
        assert_eq!(view.region.size, Vec2::new(50.0, 50.0));
        let after = view.to_widget(anchor, widget());
        assert!((before.x - after.x).abs() < 1e-9);
        assert!((before.y - after.y).abs() < 1e-9);
    }

    /// Rotating the view around a pivot keeps that pivot's widget
    /// position while `rotation` accumulates.
    #[test]
    fn rotate_around_keeps_pivot_fixed_on_widget() {
        let pivot = Vec2::<Pixels>::new(10.0, 40.0);
        let mut view = View::covering(Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
        });
        let before = view.to_widget(pivot, widget());
        view.rotate_around(pivot, RotationRad::from_degrees(45.0));
        let after = view.to_widget(pivot, widget());
        assert!((before.x - after.x).abs() < 1e-9);
        assert!((before.y - after.y).abs() < 1e-9);
        assert!((view.rotation.to_degrees() - 45.0).abs() < 1e-9);
    }

    /// `to_workspace` inverts `to_widget`, with rotation applied.
    #[test]
    fn to_workspace_inverts_to_widget() {
        let mut view = View::covering(Rect {
            min: Vec2::<Pixels>::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
        });
        view.rotation = RotationRad::from_degrees(30.0);
        let point = Vec2::new(37.0, 82.0);
        let round_trip = view.to_workspace(view.to_widget(point, widget()), widget());
        assert!((round_trip.x - point.x).abs() < 1e-9);
        assert!((round_trip.y - point.y).abs() < 1e-9);
    }

    /// A mismatched aspect grows the shown region, never shrinks it.
    #[test]
    fn shown_region_expands_to_widget_aspect() {
        let view = View::covering(Rect {
            min: Vec2::<Pixels>::new(0.0, 0.0),
            size: Vec2::new(100.0, 50.0),
        });
        // Square widget, region is wider (aspect 2 vs 1): height grows.
        let shown = view.shown_region(Vec2::new(500.0, 500.0));
        assert_eq!(shown.size, Vec2::new(100.0, 100.0));
        assert_eq!(shown.center(), view.region.center());
    }

    /// `expand_to_aspect` leaves a matching region untouched.
    #[test]
    fn expand_to_aspect_keeps_matching_region() {
        let region = Rect {
            min: Vec2::<Pixels>::new(0.0, 0.0),
            size: Vec2::new(100.0, 50.0),
        };
        assert_eq!(expand_to_aspect(region, AspectRatio::new(2.0).expect("valid aspect")), region);
    }

    /// Interpolation glides through pan, zoom and rotation at once.
    #[test]
    fn interpolate_moves_region_and_rotation() {
        let from = View::<Pixels>::covering(Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
        });
        let to = View::<Pixels> {
            region: Rect {
                min: Vec2::new(40.0, 80.0),
                size: Vec2::new(50.0, 50.0),
            },
            rotation: RotationRad::from_degrees(90.0),
        };
        let mid = from.interpolate(to, Fraction::HALF);
        assert_eq!(mid.region.min, Vec2::new(20.0, 40.0));
        assert_eq!(mid.region.size, Vec2::new(75.0, 75.0));
        assert!((mid.rotation.to_degrees() - 45.0).abs() < 1e-9);
    }

    /// `physical_zoom` converts dpi into pixels per millimeter.
    #[test]
    fn physical_zoom_scales_millimeters() {
        assert_eq!(physical_zoom(254.0), PixelsPerMm::new(10.0));
    }

    /// Serde round-trips a view.
    #[test]
    fn serde_round_trip() {
        let view = View::<Pixels> {
            region: Rect {
                min: Vec2::new(10.0, 20.0),
                size: Vec2::new(300.0, 200.0),
            },
            rotation: RotationRad::from_degrees(15.0),
        };
        let json = serde_json::to_string(&view).expect("serialize");
        assert_eq!(serde_json::from_str::<View<Pixels>>(&json).expect("deserialize"), view);
    }
}
