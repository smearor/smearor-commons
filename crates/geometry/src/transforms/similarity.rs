use crate::Fraction;
use crate::RotationRad;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;

/// Least-squares similarity transform - rotation, uniform scale and
/// translation via centroids - between two corresponding point sets.
///
/// Multi-touch gestures and marker tracking produce pairs of point
/// sets in known point-wise correspondence: [`Self::fit`] computes
/// the optimal angle and spread ratio in closed form, and
/// [`Self::apply`] then maps any baseline-space point - e.g. the
/// center of a dragged object - into the current space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimilarityTransform<U> {
    /// Rotation mapping the baseline points onto the current points,
    /// in the coordinate handedness of the input.
    pub rotation: RotationRad,
    /// Uniform scale ratio (current spread / baseline spread); `1.0`
    /// when the baseline has no spread to compare against.
    pub scale: Fraction,
    /// Centroid of the baseline point set.
    pub baseline_centroid: Vec2<U>,
    /// Centroid of the current point set.
    pub current_centroid: Vec2<U>,
}

impl<U> SimilarityTransform<U> {
    /// Fits the least-squares rotation and uniform scale between two
    /// corresponding point sets in closed form.
    ///
    /// Each corresponding pair votes for the angle with its squared
    /// distance from the centroid as weight (`atan2` of the summed
    /// cross- and dot-products, in [`Vec2::rotated`] sign convention);
    /// the scale is the ratio of the spreads' root-mean-square radii.
    ///
    /// `baseline` and `current` must have the same length in
    /// correspondence order (e.g. same touch index); `None` is
    /// returned for length mismatches and empty inputs - the cases
    /// where no correspondence exists at all. A baseline without
    /// spread (all points coincident) underdetermines rotation and
    /// scale; [`RotationRad::ZERO`] and [`Fraction::ONE`] are
    /// substituted so [`Self::apply`] still reproduces the centroid
    /// translation.
    pub fn fit(baseline: &[Vec2<U>], current: &[Vec2<U>]) -> Option<Self> {
        if baseline.len() != current.len() || baseline.is_empty() {
            return None;
        }
        let baseline_centroid = Vec2::centroid(baseline);
        let current_centroid = Vec2::centroid(current);
        let (mut numerator, mut denominator, mut radius_current, mut radius_baseline) = (0.0, 0.0, 0.0, 0.0);
        for (current_point, baseline_point) in current.iter().zip(baseline) {
            let current_offset = *current_point - current_centroid;
            let baseline_offset = *baseline_point - baseline_centroid;
            numerator += baseline_offset.cross::<f64>(&current_offset);
            denominator += current_offset.dot::<f64>(&baseline_offset);
            radius_current += current_offset.dot::<f64>(&current_offset);
            radius_baseline += baseline_offset.dot::<f64>(&baseline_offset);
        }
        let scale = if radius_baseline > f64::EPSILON {
            (radius_current / radius_baseline).sqrt()
        } else {
            1.0
        };
        Some(Self {
            rotation: RotationRad::new(numerator.atan2(denominator)),
            scale: Fraction::new(scale),
            baseline_centroid,
            current_centroid,
        })
    }

    /// A declared similarity around a fixed anchor - rotation and
    /// scale about `center`. The [`Self::fit`] counterpart for
    /// author-specified transforms where no point correspondence
    /// exists: both centroids coincide at the anchor, so
    /// [`Self::apply`] maps `center` onto itself.
    pub fn about(center: Vec2<U>, rotation: RotationRad, scale: Fraction) -> Self {
        Self {
            rotation,
            scale,
            baseline_centroid: center,
            current_centroid: center,
        }
    }

    /// Maps a baseline-space point into the current space: shifted to
    /// the baseline centroid, rotated and scaled, then re-anchored at
    /// the current centroid.
    pub fn apply(&self, point: Vec2<U>) -> Vec2<U> {
        self.current_centroid + (point - self.baseline_centroid).rotated(self.rotation) * self.scale.as_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pixels;

    fn baseline_square() -> Vec<Vec2<Pixels>> {
        vec![Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)]
    }

    fn assert_vec2_eq(actual: Vec2<Pixels>, expected: Vec2<Pixels>) {
        assert!((actual.x - expected.x).abs() < 1e-9 && (actual.y - expected.y).abs() < 1e-9, "{actual:?} != {expected:?}");
    }

    /// `about` anchors both centroids: the center stays fixed and
    /// other points rotate and scale around it.
    #[test]
    fn about_rotates_and_scales_around_center() {
        let center = Vec2::<Pixels>::new(5.0, 5.0);
        let transform = SimilarityTransform::about(center, RotationRad::QUARTER_TURN, Fraction::new(2.0));
        assert_vec2_eq(transform.apply(center), center);
        let point = Vec2::<Pixels>::new(10.0, 5.0);
        let mapped = transform.apply(point);
        assert_vec2_eq(mapped, Vec2::new(5.0, 15.0));
    }

    #[test]
    fn fit_same_points_is_identity() {
        let points = baseline_square();
        let transform = SimilarityTransform::fit(&points, &points).unwrap();
        assert_eq!(transform.rotation, RotationRad::ZERO);
        assert_eq!(transform.scale, Fraction::ONE);
        for point in &points {
            assert_vec2_eq(transform.apply(*point), *point);
        }
    }

    #[test]
    fn fit_recovers_rotation() {
        let baseline = baseline_square();
        let rotation = RotationRad::from_degrees(90.0);
        let centroid = Vec2::centroid(&baseline);
        let current: Vec<_> = baseline.iter().map(|point| centroid + (*point - centroid).rotated(rotation)).collect();
        let transform = SimilarityTransform::fit(&baseline, &current).unwrap();
        assert!((transform.rotation.to_degrees() - 90.0).abs() < 1e-9);
        assert!((transform.scale.as_f64() - 1.0).abs() < 1e-9);
        for (point, expected) in baseline.iter().zip(&current) {
            assert_vec2_eq(transform.apply(*point), *expected);
        }
    }

    #[test]
    fn fit_recovers_uniform_scale() {
        let baseline = baseline_square();
        let centroid = Vec2::centroid(&baseline);
        let current: Vec<_> = baseline.iter().map(|point| centroid + (*point - centroid) * 2.0).collect();
        let transform = SimilarityTransform::fit(&baseline, &current).unwrap();
        assert_eq!(transform.rotation, RotationRad::ZERO);
        assert!((transform.scale.as_f64() - 2.0).abs() < 1e-9);
        for (point, expected) in baseline.iter().zip(&current) {
            assert_vec2_eq(transform.apply(*point), *expected);
        }
    }

    #[test]
    fn fit_translation_moves_centroid() {
        let baseline = baseline_square();
        let offset = Vec2::<Pixels>::new(30.0, -20.0);
        let current: Vec<_> = baseline.iter().map(|point| *point + offset).collect();
        let transform = SimilarityTransform::fit(&baseline, &current).unwrap();
        assert_eq!(transform.rotation, RotationRad::ZERO);
        assert_eq!(transform.scale, Fraction::ONE);
        assert_vec2_eq(transform.current_centroid - transform.baseline_centroid, offset);
    }

    #[test]
    fn fit_rejects_mismatched_lengths() {
        assert_eq!(SimilarityTransform::<Pixels>::fit(&baseline_square(), &baseline_square()[..2]), None);
    }

    #[test]
    fn fit_rejects_empty_inputs() {
        let empty: Vec<Vec2<Pixels>> = vec![];
        assert_eq!(SimilarityTransform::fit(&empty, &empty), None);
    }

    #[test]
    fn fit_degenerate_baseline_yields_translation() {
        let baseline = vec![Vec2::<Pixels>::new(5.0, 5.0); 4];
        let offset = Vec2::<Pixels>::new(10.0, 0.0);
        let current: Vec<_> = baseline.iter().map(|point| *point + offset).collect();
        let transform = SimilarityTransform::fit(&baseline, &current).unwrap();
        assert_eq!(transform.rotation, RotationRad::ZERO);
        assert_eq!(transform.scale, Fraction::ONE);
        assert_vec2_eq(transform.apply(transform.baseline_centroid), transform.current_centroid);
    }
}
