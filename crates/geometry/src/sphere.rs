use crate::Degrees;
use crate::Vec3;
use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;

/// A sphere - the 3D analog of [`crate::Circle`] - tagged with its
/// coordinate unit `U` and numeric component type `T`.
///
/// `T` defaults to `f64` for domain math. The type is a technical
/// primitive without validation: `radius` should be non-negative,
/// stricter bounds belong to consumer wrappers.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere<U = (), T = f64> {
    /// The center point.
    pub center: Vec3<U, T>,
    /// The radius.
    pub radius: T,
}

impl<U, T: Copy> Copy for Sphere<U, T> {}

impl<U, T: Clone> Clone for Sphere<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.radius.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Sphere<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

impl<U, T: Eq> Eq for Sphere<U, T> {}

impl<U, T: Hash> Hash for Sphere<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.radius.hash(state);
    }
}

impl<U, T> Sphere<U, T> {
    /// Creates a sphere from its center and radius.
    pub const fn new(center: Vec3<U, T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<U> Sphere<U, f64> {
    /// Whether `point` lies inside the ball - the surface counts
    /// as inside (`distance <= radius`).
    pub fn contains(&self, point: &Vec3<U>) -> bool {
        self.center.distance(point) <= self.radius
    }

    /// The surface point at `latitude` (elevation `-90..90` degrees
    /// above the equator plane `xz`) and `longitude` (azimuth in
    /// degrees around `y`, counterclockwise seen from `+y`).
    ///
    /// `y` points up: `latitude = +90` is the north pole, `0`/`0` is
    /// `+x` on the equator. The inverse is [`Self::angles_of`].
    pub fn point_at(&self, latitude: Degrees, longitude: Degrees) -> Vec3<U> {
        let lat = latitude.to_radians().as_f64();
        let lon = longitude.to_radians().as_f64();
        self.center + Vec3::<U>::new(lat.cos() * lon.cos(), lat.sin(), lat.cos() * lon.sin()) * self.radius
    }

    /// The `(latitude, longitude)` of a direction from the center -
    /// the inverse of [`Self::point_at`]. Longitude is normalized to
    /// `-180..180` degrees; the poles return longitude `0`.
    pub fn angles_of(&self, point: &Vec3<U>) -> (Degrees, Degrees) {
        let offset = *point - self.center;
        let radius = offset.magnitude();
        if radius <= f64::EPSILON {
            return (Degrees::ZERO, Degrees::ZERO);
        }
        let lat = (offset.y / radius).clamp(-1.0, 1.0).asin().to_degrees();
        let lon = if (offset.y / radius).abs() >= 1.0 - 1e-12 {
            0.0
        } else {
            offset.z.atan2(offset.x).to_degrees()
        };
        (Degrees::new(lat), Degrees::new(lon))
    }

    /// The diameter.
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_equator_prime_meridian_is_x_axis() {
        let sphere = Sphere::new(Vec3::<()>::new(0.0, 0.0, 0.0), 2.0);
        let point = sphere.point_at(Degrees::ZERO, Degrees::ZERO);
        assert!((point - Vec3::new(2.0, 0.0, 0.0)).magnitude() < 1e-12);
    }

    #[test]
    fn point_at_north_and_south_poles() {
        let sphere = Sphere::new(Vec3::<()>::new(1.0, 1.0, 1.0), 3.0);
        let north = sphere.point_at(Degrees::new(90.0), Degrees::new(45.0));
        let south = sphere.point_at(Degrees::new(-90.0), Degrees::new(200.0));
        assert!((north - Vec3::new(1.0, 4.0, 1.0)).magnitude() < 1e-12);
        assert!((south - Vec3::new(1.0, -2.0, 1.0)).magnitude() < 1e-12);
    }

    #[test]
    fn angles_of_inverts_point_at() {
        let sphere = Sphere::new(Vec3::<()>::new(0.0, 0.0, 0.0), 1.0);
        let (lat, lon) = (Degrees::new(35.0), Degrees::new(-70.0));
        let (back_lat, back_lon) = sphere.angles_of(&sphere.point_at(lat, lon));
        assert!((back_lat - lat).abs().as_f64() < 1e-9);
        assert!((back_lon - lon).abs().as_f64() < 1e-9);
    }

    #[test]
    fn contains_boundary_and_outside() {
        let sphere = Sphere::new(Vec3::<()>::new(0.0, 0.0, 0.0), 1.0);
        assert!(sphere.contains(&Vec3::new(0.5, 0.0, 0.0)));
        assert!(sphere.contains(&Vec3::new(1.0, 0.0, 0.0)));
        assert!(!sphere.contains(&Vec3::new(1.0, 0.5, 0.0)));
    }
}
