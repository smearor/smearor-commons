use super::GlyphSource;
use super::ProceduralGlyphAtlas;
use super::rain;
use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Hash;

/// Green rain glyphs fall down the window while it dissolves beneath
/// them - the Matrix effect.
///
/// Behavioral spec (source shader: `matrix.frag`, re-implemented from
/// semantics): the texture is vertically overscanned by `overshoot`
/// (`uv.y` mapped to `[-overshoot/2, 1+overshoot/2]`). Per `scale`-px
/// column a deterministic delay and speed place a "raindrop" whose
/// exponential tail (`TRAIL_LENGTH`) fades the glyph mask; the window
/// alpha follows the drop with a `FADE_WIDTH`-px gradient. Glyph
/// coverage comes from a [`GlyphSource`] - the bundled
/// [`ProceduralGlyphAtlas`] replaces BMW's `matrixFont.png` bitmap.
/// A `FINAL_FADE_START_TIME` tail-fade extinguishes the trails at the
/// end.
///
/// Note on the column RNG: BMW's rain-column hash is the *sinus* hash
/// (`fract(sin(column) * 78.233)`), not the Hoskins family - kept as a
/// private [`column_hash`] for deliberate parity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Matrix {
    /// Glyph cell size in px - schema `matrix-scale`, default `20`.
    #[serde(default = "Matrix::default_scale")]
    pub scale: Pixels<u32>,
    /// Column delay/speed jitter `0..=1` - schema `matrix-randomness`, default `1.0`.
    #[serde(default = "Matrix::default_randomness")]
    pub randomness: Fraction,
    /// Head glyph color - schema `matrix-tip-color`, default `rgb(255, 255, 255)`.
    #[serde(default = "Matrix::default_tip_color")]
    pub tip_color: Color<f64>,
    /// Trail color - schema `matrix-trail-color`, default `rgb(25, 255, 89)`.
    #[serde(default = "Matrix::default_trail_color")]
    pub trail_color: Color<f64>,
    /// Vertical overscan `0..=1` - schema `matrix-overshoot`, default `0.25`.
    #[serde(default = "Matrix::default_overshoot")]
    pub overshoot: Fraction,
}

/// Atlas grid size - `matrix.frag`'s `LETTER_TILES`.
const LETTER_TILES: f64 = 16.0;
/// Tile flickers per animation second - `LETTER_FLICKER_SPEED`.
const LETTER_FLICKER_SPEED: f64 = 2.0;

impl Matrix {
    const fn default_scale() -> Pixels<u32> {
        Pixels::new(20)
    }
    const fn default_randomness() -> Fraction {
        Fraction::new(1.0)
    }
    const fn default_tip_color() -> Color<f64> {
        Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0)
    }
    const fn default_trail_color() -> Color<f64> {
        Color::<f64>::new_rgba(25.0 / 255.0, 1.0, 89.0 / 255.0, 1.0)
    }
    const fn default_overshoot() -> Fraction {
        Fraction::new(0.25)
    }

    /// Flickering glyph coverage at overscanned pixel coords - the
    /// `getText` helper: each `scale`-px block picks a `hash22`-chained
    /// tile that flips `LETTER_FLICKER_SPEED * duration` times per run.
    fn text_mask(&self, ctx: &FxContext<'_>, px: Vec2<()>, glyphs: &impl GlyphSource) -> f64 {
        let letter = self.scale.0.max(1) as f64;
        let cell_uv = Vec2::<()>::new(crate::fract(px.x / letter), crate::fract(px.y / letter));
        let block = Vec2::<()>::new((px.x / letter).floor(), (px.y / letter).floor());
        // Tile index: hash the block through BMW's magic constants,
        // add the flicker-time hash, mod into the 16x16 atlas.
        let chained: Vec2<()> = Hash::<Vec2<()>>::hash(&block);
        let mixed = chained
            .component_mul(&Vec2::<()>::new(12.9898, 78.233))
            .map(|c| c.floor() + LETTER_FLICKER_SPEED * ctx.progress.as_f64() * ctx.duration.as_secs_f64() + 42.254);
        let flicker: Vec2<()> = Hash::<Vec2<()>>::hash(&mixed);
        let tile = flicker.map(|c| (c * LETTER_TILES).floor().min(LETTER_TILES - 1.0));
        glyphs.glyph(tile, cell_uv)
    }

    /// The paired rain/window masks - `getRain`'s semantics: per-column
    /// delay/speed derive a "drop position" over the *un-overscanned*
    /// `uv.y` (BMW's `getRain` ignores `fragCoord` and uses
    /// `iTexCoord`); the rain alpha is the drop's exponential tail, the
    /// window alpha a `FADE_WIDTH`-px gradient trailing the drop. Both
    /// ends of the column are shortened by a per-column `shorten`
    /// factor so trails don't touch the edges.
    fn rain(&self, ctx: &FxContext<'_>, uv: Vec2<()>, px: Vec2<()>, size_px: Vec2<()>) -> (f64, f64) {
        let letter = self.scale.0.max(1) as f64;
        let column = px.x - (px.x % letter);
        let randomness = self.randomness.0;

        let delay = rain::column_hash(column) * randomness;
        let speed = rain::column_speed_hash(column) * randomness * 0.3 + 1.5;
        let dist = (ctx.progress.as_f64() * 2.0 - delay) * speed - uv.y;

        let mut rain_alpha = if dist >= 0.0 { (-dist / rain::TRAIL_LENGTH).exp() } else { 0.0 };
        let mut window_alpha = 1.0 - (size_px.y * dist / rain::FADE_WIDTH).clamp(0.0, 1.0);

        rain_alpha *= crate::absolute_edge_mask(px, size_px, ctx.padding, Pixels::new(rain::EDGE_FADE), 0.5, ctx.fullscreen);

        let shorten = crate::fract((column + 42.0).sin() * 33.423) * self.overshoot.0 * 0.25 * randomness;
        rain_alpha *= crate::smoothstep(0.0, 1.0, (uv.y / shorten).clamp(0.0, 1.0));
        rain_alpha *= crate::smoothstep(0.0, 1.0, ((1.0 - uv.y) / shorten).clamp(0.0, 1.0));

        if ctx.direction == crate::Direction::Open {
            window_alpha = 1.0 - window_alpha;
        }

        (rain_alpha, window_alpha)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            randomness: Self::default_randomness(),
            tip_color: Self::default_tip_color(),
            trail_color: Self::default_trail_color(),
            overshoot: Self::default_overshoot(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "scale",
        label: "Scale",
        kind: ParamKind::Int { min: 8, max: 64, default: 20 },
    },
    ParamSpec {
        key: "randomness",
        label: "Randomness",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "tip_color",
        label: "Tip Color",
        kind: ParamKind::Color {
            default: Matrix::default_tip_color(),
        },
    },
    ParamSpec {
        key: "trail_color",
        label: "Trail Color",
        kind: ParamKind::Color {
            default: Matrix::default_trail_color(),
        },
    },
    ParamSpec {
        key: "overshoot",
        label: "Overshoot",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.25,
        },
    },
];

impl WindowEffect for Matrix {
    fn nick(&self) -> &'static str {
        "matrix"
    }

    fn label(&self) -> &'static str {
        "Matrix"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Int(self.scale.0 as i64),
            ParamValue::Float(self.randomness.0),
            ParamValue::Color(self.tip_color),
            ParamValue::Color(self.trail_color),
            ParamValue::Float(self.overshoot.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("matrix param {index}"));
        if values.len() != 5 {
            return Err(FxError::ParamMismatch(format!("matrix takes 5 parameters, got {}", values.len())));
        }
        self.scale = Pixels::new(values[0].as_i64().ok_or(mismatch(0))?.max(0) as u32);
        self.randomness = Fraction::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.tip_color = values[2].as_color().ok_or(mismatch(2))?;
        self.trail_color = values[3].as_color().ok_or(mismatch(3))?;
        self.overshoot = Fraction::new(values[4].as_f64().ok_or(mismatch(4))?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        self.apply_with_glyphs(ctx, out, &ProceduralGlyphAtlas)
    }
}

impl Matrix {
    /// `apply` against a custom [`GlyphSource`] - the point where a
    /// consumer plugs a real bitmap font atlas. `Sync` because the
    /// render driver shares the source across row-band workers.
    pub fn apply_with_glyphs(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>, glyphs: &(impl GlyphSource + Sync)) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let final_fade = 1.0 - ((ctx.progress.as_f64() - rain::FINAL_FADE_START_TIME) / (1.0 - rain::FINAL_FADE_START_TIME)).clamp(0.0, 1.0);
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let mut coords = uv;
            coords.y = coords.y * (self.overshoot.0 + 1.0) - self.overshoot.0 * 0.5;
            let px = Vec2::<()>::new(coords.x * size_px.x, coords.y * size_px.y);
            let px_orig = Vec2::<()>::new(uv.x * size_px.x, uv.y * size_px.y);

            let (rain_alpha, window_alpha) = self.rain(ctx, uv, px_orig, size_px);
            let text_alpha = self.text_mask(ctx, px, glyphs);
            let rain = final_fade * rain_alpha;

            let mut window = ctx.window.sample_bilinear(coords);
            window.a *= window_alpha;

            let blend = (rain + 0.1).powi(4).min(1.0);
            let text = Color::<f64>::new_rgba(
                crate::mix(self.trail_color.r, self.tip_color.r, blend),
                crate::mix(self.trail_color.g, self.tip_color.g, blend),
                crate::mix(self.trail_color.b, self.tip_color.b, blend),
                rain * text_alpha,
            );
            text.alpha_over(&window)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::BufferView;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn ctx_for<'a>(window: BufferView<'a, Rgba>, progress: f64) -> FxContext<'a> {
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_millis(1500),
            seed: Seed(1),
            direction: Direction::Close,
            layer: Layer::Window,
            window,
            surface: None,
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, 64.0, 64.0),
        }
    }

    /// Rain alpha is constant within a column and varies across
    /// columns (column hash decorrelates). The window is 200px wide so
    /// the sampled columns sit inside the `EDGE_FADE = 30` interior
    /// where the edge mask is 1 and cannot disturb the comparison.
    #[test]
    fn matrix_columns() {
        let data = [30, 30, 30, 255].repeat(200 * 64);
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(200, 64)).unwrap();
        let m = Matrix::default();
        let ctx = ctx_for(window, 0.5);
        let size_px = Vec2::<()>::new(200.0, 64.0);
        let rain_at = |x: f64| {
            let uv = Vec2::<()>::new(x / 200.0, 0.4);
            m.rain(&ctx, uv, Vec2::<()>::new(x, 0.4 * 64.0), size_px).0
        };
        // x = 50.5 and 55.5 share column block 40 (20px letters).
        assert_eq!(rain_at(50.5), rain_at(55.5), "same column block = same rain alpha");
        // Columns 40 and 60 decorrelate: their trails differ.
        assert_ne!(rain_at(50.5), rain_at(70.5), "different columns differ");
    }

    /// The bundled atlas is deterministic per tile and covers cells.
    #[test]
    fn matrix_atlas_parity() {
        let atlas = ProceduralGlyphAtlas;
        let tile = Vec2::<()>::new(3.0, 7.0);
        let mut filled = 0;
        // Scan the 5x7 font grid (in its 7x7 cell frame) at centers.
        for cy in 0..7 {
            for cx in 0..7 {
                let uv = Vec2::<()>::new((cx as f64 + 0.5) / 7.0, (cy as f64 + 0.5) / 7.0);
                let a = atlas.glyph(tile, uv);
                let b = atlas.glyph(tile, uv);
                assert_eq!(a, b, "deterministic per tile");
                if a > 0.0 {
                    filled += 1;
                }
            }
        }
        assert!(filled > 3, "a glyph tile should cover several cells, got {filled}");
    }
}
