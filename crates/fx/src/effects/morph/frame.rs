use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2D;

const FLOW_PX: Pixels<f64> = Pixels::new(30.0);
const FLOW_SCALE: f64 = 160.0;
const RADIAL_PX: Pixels<f64> = Pixels::new(30.0);
const SPILL_PX: Pixels<f64> = Pixels::new(5.0);
const CORNER_PX: Pixels<f64> = Pixels::new(24.0);

/// `streamFn` - two-octave curl source.
fn stream_fn(p: Vec2<()>, seed: Vec2<()>) -> f64 {
    Simplex2D.at(p + seed * 100.0) + 0.5 * Simplex2D.at(p * 2.03 + Vec2::<()>::new(17.1, 5.3) + seed * 100.0)
}

pub(super) struct MorphFrame {
    /// `uFrameRect` in layer pixels.
    pub(super) frame: (Vec2<()>, Vec2<()>),
    /// Texture size in pixels.
    pub(super) size_px: Vec2<()>,
    /// Normalized `[0,1]` seed (`uSeed`).
    pub(super) seed: Vec2<()>,
    /// `uStrength`.
    pub(super) strength: f64,
}

impl MorphFrame {
    /// `frameDist` - signed distance to the rounded frame rect.
    fn frame_dist(&self, px: Vec2<()>) -> f64 {
        let (o, s) = self.frame;
        let c = o + s * 0.5;
        let h = Vec2::<()>::new((0.5 * s.x - CORNER_PX.0).max(0.0), (0.5 * s.y - CORNER_PX.0).max(0.0));
        let q = Vec2::<()>::new((px.x - c.x).abs() - h.x, (px.y - c.y).abs() - h.y);
        let mx = q.x.max(0.0);
        let my = q.y.max(0.0);
        (mx * mx + my * my).sqrt() + q.x.max(q.y).min(0.0) - CORNER_PX.0
    }

    /// `regionMask` - 1 inside, fading to 0 `SPILL_PX.0` outside.
    pub(super) fn region_mask(&self, px: Vec2<()>) -> f64 {
        1.0 - crate::smoothstep(0.0, SPILL_PX.0, self.frame_dist(px))
    }

    /// `flowAt` - the full displacement field in pixels.
    pub(super) fn flow_at(&self, px: Vec2<()>) -> Vec2<()> {
        let m = self.region_mask(px);
        if m <= 0.0 {
            return Vec2::<()>::new(0.0, 0.0);
        }
        let p = Vec2::<()>::new(px.x / FLOW_SCALE, px.y / FLOW_SCALE);
        const E: f64 = 0.01;
        let dx = (stream_fn(p + Vec2::<()>::new(E, 0.0), self.seed) - stream_fn(p - Vec2::<()>::new(E, 0.0), self.seed)) / (2.0 * E);
        let dy = (stream_fn(p + Vec2::<()>::new(0.0, E), self.seed) - stream_fn(p - Vec2::<()>::new(0.0, E), self.seed)) / (2.0 * E);
        let mut c = Vec2::<()>::new(dy, -dx);
        let cl = (c.x * c.x + c.y * c.y).sqrt();
        c = c / (2.3 + 0.3 * cl);

        let (o, s) = self.frame;
        let centre = o + s * 0.5;
        let rel = Vec2::<()>::new((px.x - centre.x) / (0.5 * s.x).max(1.0), (px.y - centre.y) / (0.5 * s.y).max(1.0));
        let r = (rel.x * rel.x + rel.y * rel.y).sqrt();
        let radial = if r > 1e-4 {
            rel / r * (std::f64::consts::PI * r.min(1.0)).sin()
        } else {
            Vec2::<()>::new(0.0, 0.0)
        };

        Vec2::<()>::new(
            m * self.strength * (c.x * FLOW_PX.0 + radial.x * RADIAL_PX.0),
            m * self.strength * (c.y * FLOW_PX.0 + radial.y * RADIAL_PX.0),
        )
    }
}
