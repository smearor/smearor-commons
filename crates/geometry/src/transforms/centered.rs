use crate::Position;
use crate::RotationRad;
use crate::ScaleFactor;
use crate::Size;

/// Placement of a child extent centered at a point, rotated and
/// scaled around that point.
///
/// The whole spec uses only commons-geometry types; the conversion
/// into [`gtk4::gsk::Transform`] composes
/// `translate(center) . rotate . scale . translate(-size/2)` - the
/// trailing offset moves the child's center to the origin first, so
/// rotation and scale act around the child's center, and `center`
/// then places the rotated child in parent coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CenteredTransform {
    /// The point (in parent coordinates) the child is centered on.
    pub center: Position<f32>,
    /// The child's unrotated, unscaled extent.
    pub child_size: Size<f32>,
    /// The rotation around `center`.
    pub rotation: RotationRad,
    /// The uniform scale around `center`.
    pub scale: ScaleFactor,
}

impl CenteredTransform {
    /// Creates a centered placement spec.
    pub const fn new(center: Position<f32>, child_size: Size<f32>, rotation: RotationRad, scale: ScaleFactor) -> Self {
        Self {
            center,
            child_size,
            rotation,
            scale,
        }
    }
}

impl From<CenteredTransform> for gtk4::gsk::Transform {
    fn from(spec: CenteredTransform) -> Self {
        let scale = spec.scale.as_f32();
        Self::new()
            .translate(&spec.center.into())
            .rotate(spec.rotation.to_degrees_f32())
            .scale(scale, scale)
            .translate(&spec.child_size.map(|v| v * -0.5).into())
    }
}

#[cfg(test)]
mod tests {
    use super::CenteredTransform;
    use crate::Position;
    use crate::RotationRad;
    use crate::ScaleFactor;
    use crate::Size;

    /// Identity spec: the child's top-left corner lands at
    /// `center - size / 2`.
    #[test]
    fn identity_places_child_top_left() {
        let spec = CenteredTransform::new(Position::new(100.0, 50.0), Size::new(40.0, 20.0), RotationRad::ZERO, ScaleFactor::ONE);
        let (xx, _yx, _xy, yy, dx, dy) = gtk4::gsk::Transform::from(spec).to_2d();
        assert!((xx - 1.0).abs() < 1e-6 && (yy - 1.0).abs() < 1e-6);
        assert!((dx - 80.0).abs() < 1e-4 && (dy - 40.0).abs() < 1e-4);
    }

    /// A 180-degree rotation around the center flips the child
    /// offset: the top-left corner maps to `center + size / 2`.
    #[test]
    fn rotates_around_child_center() {
        let spec = CenteredTransform::new(Position::new(100.0, 50.0), Size::new(40.0, 20.0), RotationRad::from_degrees(180.0), ScaleFactor::ONE);
        let (xx, _yx, _xy, yy, dx, dy) = gtk4::gsk::Transform::from(spec).to_2d();
        assert!((xx + 1.0).abs() < 1e-4 && (yy + 1.0).abs() < 1e-4);
        assert!((dx - 120.0).abs() < 1e-3 && (dy - 60.0).abs() < 1e-3);
    }

    /// The scale applies to the centering offset, too: with scale `2`
    /// the child keeps its center and doubles its extent.
    #[test]
    fn scales_around_child_center() {
        let spec = CenteredTransform::new(Position::new(100.0, 50.0), Size::new(40.0, 20.0), RotationRad::ZERO, ScaleFactor::new(2.0));
        let (xx, _yx, _xy, yy, dx, dy) = gtk4::gsk::Transform::from(spec).to_2d();
        assert!((xx - 2.0).abs() < 1e-6 && (yy - 2.0).abs() < 1e-6);
        assert!((dx - 60.0).abs() < 1e-4 && (dy - 30.0).abs() < 1e-4);
    }
}
