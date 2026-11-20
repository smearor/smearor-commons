use crate::Pixels;
use crate::RotationRad;
use crate::ScaleFactor;
use crate::Vec2;
use crate::Vec3;
use serde::Deserialize;
use serde::Serialize;

/// An orthographic orbit camera: a `yaw`/`pitch` view rotation plus
/// a world-to-screen `scale`, projecting [`Vec3`] world points to
/// screen pixels together with a painter-sort depth.
///
/// The camera carries no viewport - pass the viewport extent per
/// call to [`Self::project`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Camera {
    /// Azimuth around the `y` axis, applied first.
    pub yaw: RotationRad,
    /// Elevation around the `x` axis, applied after `yaw`.
    pub pitch: RotationRad,
    /// Screen pixels per world unit.
    pub scale: ScaleFactor,
}

impl Camera {
    /// The unrotated camera at unit scale.
    pub const IDENTITY: Self = Self {
        yaw: RotationRad::ZERO,
        pitch: RotationRad::ZERO,
        scale: ScaleFactor::ONE,
    };

    /// The point in camera space: rotated by `yaw`, then `pitch`.
    /// `+z` points toward the viewer.
    pub fn view<U>(&self, point: Vec3<U>) -> Vec3<U> {
        point.rotated_y(self.yaw).rotated_x(self.pitch)
    }

    /// The screen position (in `Pixels`, `x` rightward and `y`
    /// downward from the viewport center) and the painter depth -
    /// larger means further away - of `point` inside a `viewport`.
    pub fn project<U>(&self, point: Vec3<U>, viewport: Vec2<Pixels>) -> (Vec2<Pixels>, f64) {
        let view = self.view(point);
        let screen = Vec2::<Pixels>::new(viewport.x / 2.0 + view.x * self.scale.as_f64(), viewport.y / 2.0 - view.y * self.scale.as_f64());
        (screen, -view.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn identity_maps_center_to_viewport_center() {
        let (screen, depth) = Camera::IDENTITY.project(Vec3::<()>::new(0.0, 0.0, 0.0), Vec2::<Pixels>::new(100.0, 80.0));
        assert_eq!(screen, Vec2::new(50.0, 40.0));
        assert_eq!(depth, 0.0);
    }

    #[test]
    fn project_scales_and_flips_y() {
        let camera = Camera {
            scale: ScaleFactor::new(2.0),
            ..Camera::IDENTITY
        };
        let (screen, _) = camera.project(Vec3::<()>::new(10.0, 5.0, 0.0), Vec2::<Pixels>::new(100.0, 100.0));
        assert_eq!(screen, Vec2::new(70.0, 40.0), "x+20, y-10 from center");
    }

    #[test]
    fn yaw_90_turns_z_into_view_x() {
        let camera = Camera {
            yaw: RotationRad::new(FRAC_PI_2),
            ..Camera::IDENTITY
        };
        let view = camera.view(Vec3::<()>::new(0.0, 0.0, 1.0));
        assert!((view.x - 1.0).abs() < 1e-12 && view.z.abs() < 1e-12);
    }

    #[test]
    fn depth_grows_away_from_the_viewer() {
        let (_, near) = Camera::IDENTITY.project(Vec3::<()>::new(0.0, 0.0, 1.0), Vec2::<Pixels>::new(10.0, 10.0));
        let (_, far) = Camera::IDENTITY.project(Vec3::<()>::new(0.0, 0.0, -1.0), Vec2::<Pixels>::new(10.0, 10.0));
        assert!(far > near);
    }
}
