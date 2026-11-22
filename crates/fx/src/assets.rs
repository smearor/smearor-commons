//! `assets` - the optional texture channel for effects which sample
//! auxiliary imagery (BMW's `*.png` resources).
//!
//! Every asset effect has a deterministic **procedural fallback** -
//! the same role `ProceduralGlyphAtlas` plays for `Matrix`: no file
//! dependency, no loader, reproducible output. A compositor can pass
//! real textures through [`FxContext::assets`] to override them.

use smearor_commons_buffer::BufferView;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Simplex2DFractal;

/// Optional auxiliary textures an effect may sample - BMW's image
/// resources as a typed channel. `None` selects the effect's
/// procedural fallback.
#[derive(Debug, Clone, Copy, Default)]
pub struct FxAssets<'a> {
    /// Claw-scratch map for `trex` (BMW `claws.png`) - `r` is the
    /// scratch's local time field.
    pub claw: Option<BufferView<'a, Rgba>>,
    /// Dust particle map for `snap` (BMW `dust.png`) - `r` dissolve
    /// threshold, `g` per-particle layer id.
    pub dust: Option<BufferView<'a, Rgba>>,
    /// Brush sweep for `paint-brush` (BMW `brush.png`) - one channel
    /// per brush size, value = sweep time.
    pub brush: Option<BufferView<'a, Rgba>>,
    /// Shard map for `broken-glass` (BMW `shards.png`) - `r` per-shard
    /// random, `g` per-shard layer id.
    pub shard: Option<BufferView<'a, Rgba>>,
}

/// The procedural texture substitutes - one function per asset,
/// returning the sampled channels the shader semantics expect.
pub(crate) mod procedural {
    use super::*;

    /// Voronoi F1 helper: nearest jittered site wins, `r`/`g` are two
    /// independent site hashes - the shared base of BMW's `dust.png`
    /// and `shards.png`.
    fn voronoi_rg(uv: Vec2<()>, cells: f64) -> (f64, f64) {
        let p = Vec2::<()>::new(uv.x * cells, uv.y * cells);
        let base = Vec2::<()>::new(p.x.floor(), p.y.floor());
        let mut best = f64::INFINITY;
        let mut best_id = Vec2::<()>::new(0.0, 0.0);
        for j in -1..=1 {
            for i in -1..=1 {
                let cell = Vec2::<()>::new(base.x + i as f64, base.y + j as f64);
                let site = cell + Hash::<Vec2<()>>::hash(&cell);
                let dx = p.x - site.x;
                let dy = p.y - site.y;
                let d = dx * dx + dy * dy;
                if d < best {
                    best = d;
                    best_id = cell;
                }
            }
        }
        (Hash::<f64>::hash(&best_id), Hash::<f64>::hash(&(best_id + Vec2::<()>::new(19.19, 47.47))))
    }

    /// `dust.png` substitute - `(dissolve threshold, layer id)`.
    pub fn dust(uv: Vec2<()>) -> (f64, f64) {
        voronoi_rg(uv, 6.0)
    }

    /// `shards.png` substitute - `(per-shard random, layer id)`.
    pub fn shard(uv: Vec2<()>) -> (f64, f64) {
        voronoi_rg(uv, 8.0)
    }

    /// `claws.png` substitute - three curved scratch stripes; `r`
    /// runs 0->1 along the stripe so the scratch "draws" over time
    /// (`1.0` = untouched).
    pub fn claw(uv: Vec2<()>) -> f64 {
        let mut value: f64 = 1.0;
        for k in 0..3 {
            let kf = k as f64;
            // A diagonal stripe, curved by a low-frequency bend.
            let center = uv.y + 0.08 * (uv.y * std::f64::consts::PI).sin() + 0.16 + kf * 0.18;
            let dist = (uv.x - center).abs();
            let width = 0.028 - 0.012 * uv.y;
            if dist < width {
                // Along the stripe: earlier parts scratch first.
                let t = (1.0 - uv.y).clamp(0.0, 1.0);
                value = value.min(t.max(0.02));
            }
        }
        value
    }

    /// `brush.png` substitute - a left-to-right sweep with a ragged
    /// noise edge; `channel` 0..3 mimics the PNG's per-size planes
    /// (rougher/larger strokes cover earlier).
    pub fn brush(uv: Vec2<()>, channel: usize) -> f64 {
        let channel_f = channel as f64;
        let edge = Simplex2DFractal.at(Vec2::<()>::new(uv.y * 3.0 + channel_f * 17.0, channel_f * 5.0));
        let ragged = Simplex2DFractal.at(Vec2::<()>::new(uv.x * 12.0, uv.y * 12.0 + channel_f * 29.0)) - 0.5;
        // Larger brushes (higher channel) sweep faster + rougher.
        let t = uv.x * (1.05 - 0.08 * channel_f) + edge * 0.12 + ragged * 0.08;
        t.clamp(0.0, 1.0)
    }
}
