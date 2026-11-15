use crate::RotationRad;
use crate::SimilarityTransform;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;

/// Least-squares rigid 2D transform - rotation and translation via
/// centroids - between two corresponding point sets; the
/// scale-forced-to-`1` counterpart of [`SimilarityTransform`].
///
/// Use [`Self::fit`] where both point sets share a physical unit
/// (e.g. a declared pin layout against measured touches, both in
/// millimeters): a fitted scale would absorb measurement noise as a
/// bogus size change, while rotation and translation are the actual
/// degrees of freedom of the placement.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RigidTransform<U> {
    /// Rotation mapping the baseline points onto the current points,
    /// in the coordinate handedness of the input.
    pub rotation: RotationRad,
    /// Centroid of the baseline point set.
    pub baseline_centroid: Vec2<U>,
    /// Centroid of the current point set.
    pub current_centroid: Vec2<U>,
}

impl<U> RigidTransform<U> {
    /// Fits the least-squares rotation and translation - scale fixed
    /// at `1` - between two corresponding point sets in closed form,
    /// via [`SimilarityTransform::fit`] with the spread ratio dropped.
    ///
    /// `baseline` and `current` must have the same length in
    /// correspondence order; `None` is returned for length mismatches
    /// and empty inputs. A baseline without spread (all points
    /// coincident) underdetermines rotation; [`RotationRad::ZERO`] is
    /// substituted so [`Self::apply`] still reproduces the centroid
    /// translation.
    pub fn fit(baseline: &[Vec2<U>], current: &[Vec2<U>]) -> Option<Self> {
        let fit = SimilarityTransform::fit(baseline, current)?;
        Some(Self {
            rotation: fit.rotation,
            baseline_centroid: fit.baseline_centroid,
            current_centroid: fit.current_centroid,
        })
    }

    /// Maps a baseline-space point into the current space:
    /// `current_centroid + R(rotation) · (point - baseline_centroid)`.
    pub fn apply(&self, point: Vec2<U>) -> Vec2<U> {
        self.current_centroid + (point - self.baseline_centroid).rotated(self.rotation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pixels;

    /// A quarter-turn baseline maps each point around the centroid -
    /// `apply` reproduces the rotated set exactly.
    #[test]
    fn fit_recovers_rotation() {
        let baseline = [Vec2::<Pixels>::new(0.0, 0.0), Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)];
        let centroid = Vec2::centroid(&baseline);
        let current: Vec<_> = baseline
            .iter()
            .map(|point| Vec2::new(100.0, 50.0) + (*point - centroid).rotated(RotationRad::QUARTER_TURN))
            .collect();
        let transform = RigidTransform::fit(&baseline, &current).expect("fit");
        assert!((transform.rotation - RotationRad::QUARTER_TURN).abs().as_f64() < 1e-9);
        for (point, expected) in baseline.iter().zip(&current) {
            let mapped = transform.apply(*point);
            assert!((mapped.x - expected.x).abs() < 1e-9 && (mapped.y - expected.y).abs() < 1e-9);
        }
    }

    /// Mismatched and empty inputs admit no correspondence.
    #[test]
    fn fit_rejects_mismatched_inputs() {
        let baseline = [Vec2::<()>::new(0.0, 0.0), Vec2::new(1.0, 0.0)];
        assert!(RigidTransform::fit(&baseline, &[]).is_none());
        assert!(RigidTransform::fit(&baseline, &baseline[..1]).is_none());
    }
}
