use crate::FxContext;
use crate::FxError;
use crate::Layer;
use crate::Layers;
use crate::MorphStyle;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::Phase;
use crate::WindowEffect;
use crate::effects::morph::frame::MorphFrame;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferView;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Simplex2D;

/// The desktop flows forward along a curl-noise field while the
/// window arrives from the opposite direction and condenses out of
/// it in noise-driven blotches - or, per `morph-style`, the layers
/// melt along their own luminance edges / coarsen into shared pixel
/// blocks that flip colour.
///
/// Behavioral spec (source shader: `morph.frag`, re-implemented from
/// semantics): `flowAt` builds a curl-noise displacement field
/// (`streamFn` = two simplex octaves, finite differences, soft-limited
/// to ~unit length) plus a radial push away from the frame centre,
/// masked to `CORNER_PX`-rounded `uFrameRect` with `SPILL_PX` feather.
/// Timeline: the surface's displacement grows over `T_SURFACE`, the
/// window's shrinks over `T_WINDOW`, the blotch dissolve sweeps
/// `T_FADE` (`DISSOLVE_SCALE` noise, `DISSOLVE_EDGE` softness); both
/// layers blur by `BLUR_PX * strength` while displaced (13-tap disc,
/// premultiplied). Style `melt`: `meltDir` derives direction from the
/// layer's own luminance gradient (`MELT_RADIUS` ring, `MELT_ALONG` /
/// `MELT_ACROSS` mix, `MELT_STEPS` advection), strong window edges
/// arriving `EDGE_FIRST` earlier. Style `pixelate`: both layers snap
/// to blocks of the same grid anchored at the frame centre
/// (`PIXEL_TAPS`²-tap block means); window blocks flip colour at
/// noise+random staggered times (`PIXEL_RANDOM`, `PIXEL_SPREAD`,
/// `PIXEL_DUR`) inside `P_FADE`, between the `P_COARSEN`/`P_SHARPEN`
/// phases. `uSeed` is the normalized `[0,1]` seed hash.
///
/// Two-pass effect: `Layer::Surface` morphs the backdrop,
/// `Layer::Window` the window - the consumer composites window over
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Morph {
    /// Morph style: `0` = liquid flow, `1` = edge-guided melt,
    /// `2` = pixelate - schema `morph-style`, default `0`.
    #[serde(default = "Morph::default_style")]
    pub style: MorphStyle,
    /// Displacement multiplier - schema `morph-strength`, default `1.0`.
    #[serde(default = "Morph::default_strength")]
    pub strength: Factor,
}

impl Morph {
    const fn default_style() -> MorphStyle {
        MorphStyle::LiquidFlow
    }
    const fn default_strength() -> Factor {
        Factor::new(1.0)
    }
}

impl Default for Morph {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            strength: Self::default_strength(),
        }
    }
}

const BLUR_PX: Pixels<f64> = Pixels::new(6.0);
const DISSOLVE_SCALE: f64 = 380.0;
const DISSOLVE_EDGE: f64 = 0.22;
const MELT_PX: Pixels<f64> = Pixels::new(40.0);
const MELT_RADIUS: Pixels<f64> = Pixels::new(8.0);
const MELT_ALONG: f64 = 0.7;
const MELT_ACROSS: f64 = 0.5;
const MELT_NOISE: f64 = 0.35;
const MELT_STEPS: usize = 3;
const EDGE_FIRST: f64 = 0.35;
const PIXEL_PX: Pixels<f64> = Pixels::new(24.0);
const PIXEL_TAPS: usize = 3;
const PIXEL_RANDOM: f64 = 0.75;
const PIXEL_SPREAD: f64 = 1.3;
const PIXEL_DUR: f64 = 0.5;
const PIXEL_RESIZE: f64 = 0.5;
const P_COARSEN: Phase = Phase::new(0.0, PIXEL_RESIZE);
const P_FADE: Phase = Phase::new(0.0, 1.0);
const P_SHARPEN: Phase = Phase::new(1.0 - PIXEL_RESIZE, 1.0);
const T_SURFACE: Phase = Phase::new(0.00, 0.60);
const T_WINDOW: Phase = Phase::new(0.40, 1.00);
const T_FADE: Phase = Phase::new(0.20, 0.85);

/// The layer's texture read - surface clamps, the window reads
/// transparent outside (`readLayer` in the shader).
fn read_layer(view: BufferView<'_, Rgba>, uv: Vec2<()>, surface: bool) -> Color<f64> {
    if !surface && (!(0.0..=1.0).contains(&uv.x) || !(0.0..=1.0).contains(&uv.y)) {
        return Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
    }
    view.sample_bilinear(uv)
}

/// `blurredRead` - 13-tap disc blur, premultiplied then straightened.
fn blurred_read(view: BufferView<'_, Rgba>, uv: Vec2<()>, radius: f64, texel: Vec2<()>, surface: bool) -> Color<f64> {
    if radius < 0.5 {
        return read_layer(view, uv, surface);
    }
    let mut acc = [0.0; 4];
    for i in 0..13usize {
        let off = if i == 0 {
            Vec2::<()>::new(0.0, 0.0)
        } else {
            let k = (i - 1) as f64;
            let ang = k * std::f64::consts::FRAC_PI_6 + if k < 6.0 { 0.0 } else { std::f64::consts::FRAC_PI_6 * 0.5 };
            let rad = if k < 6.0 { 0.5 * radius } else { radius };
            Vec2::<()>::new(ang.cos() * rad * texel.x, ang.sin() * rad * texel.y)
        };
        let s = read_layer(view, uv + off, surface);
        acc[0] += s.r * s.a;
        acc[1] += s.g * s.a;
        acc[2] += s.b * s.a;
        acc[3] += s.a;
    }
    let a = acc[3] / 13.0;
    if a <= 0.0 {
        return Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
    }
    Color::<f64>::new_rgba(acc[0] / 13.0 / a, acc[1] / 13.0 / a, acc[2] / 13.0 / a, a)
}

/// `luma` - transparent counts as mid-grey, so the window's outline
/// is an edge too. The alpha-over-midgrey composite is a sampling
/// policy of this edge detector, not a property of `Color`.
fn luma(c: Color<f64>) -> f64 {
    c.a * c.luminance_601().as_f64() + (1.0 - c.a) * 0.5
}

/// `meltDir` - direction from the layer's own luminance gradient.
/// Returns (direction, edge strength).
fn melt_dir(view: BufferView<'_, Rgba>, uv: Vec2<()>, texel: Vec2<()>, surface: bool) -> (Vec2<()>, f64) {
    let mut g = Vec2::<()>::new(0.0, 0.0);
    for k in 0..8 {
        let ang = k as f64 * std::f64::consts::FRAC_PI_4;
        let dir = Vec2::<()>::new(ang.cos(), ang.sin());
        let l = luma(read_layer(
            view,
            Vec2::<()>::new(uv.x + dir.x * MELT_RADIUS.0 * texel.x, uv.y + dir.y * MELT_RADIUS.0 * texel.y),
            surface,
        ));
        g += dir * l;
    }
    g = g * 0.25;
    let len = (g.x * g.x + g.y * g.y).sqrt();
    if len < 1e-4 {
        return (Vec2::<()>::new(0.0, 0.0), 0.0);
    }
    let strength = crate::smoothstep(0.02, 0.25, len);
    let n = Vec2::<()>::new(g.x / len, g.y / len);
    (
        Vec2::<()>::new(strength * (MELT_ALONG * -n.y + MELT_ACROSS * n.x), strength * (MELT_ALONG * n.x + MELT_ACROSS * n.y)),
        strength,
    )
}

/// The morph's per-frame geometry shared by all styles.
const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "style",
        label: "Style",
        kind: ParamKind::Int { min: 0, max: 2, default: 0 },
    },
    ParamSpec {
        key: "strength",
        label: "Strength",
        kind: ParamKind::Float {
            min: 0.0,
            max: 3.0,
            step: 0.05,
            default: 1.0,
        },
    },
];
impl WindowEffect for Morph {
    fn nick(&self) -> &'static str {
        "morph"
    }

    fn label(&self) -> &'static str {
        "Morph"
    }

    fn layers(&self) -> Layers {
        Layers::SurfaceAndWindow
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Int(i64::from(u32::from(self.style))), ParamValue::Float(self.strength.0)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("morph param {index}"));
        if values.len() != 2 {
            return Err(FxError::ParamMismatch(format!("morph takes 2 parameters, got {}", values.len())));
        }
        self.style = MorphStyle::try_from(values[0].as_i64().ok_or(mismatch(0))?.clamp(0, 2) as u32).map_err(|_| mismatch(0))?;
        self.strength = Factor::new(values[1].as_f64().ok_or(mismatch(1))?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = out.size();
        let size_px = Vec2::<()>::new(f64::from(size.x), f64::from(size.y));
        let opening = ctx.direction == crate::Direction::Open;
        let progress = if opening { ctx.progress.as_f64() } else { 1.0 - ctx.progress.as_f64() };
        let surface_pass = ctx.layer == Layer::Surface;
        let source = if surface_pass { ctx.surface } else { Some(ctx.window) };
        let Some(view) = source else {
            // No surface texture - the consumer's own backdrop shows
            // through; the pass renders transparent.
            crate::render::fill(out, |_, _| Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0));
            return;
        };
        let frame = ctx.window_rect;
        let mf = MorphFrame {
            frame: (Vec2::<()>::new(frame.min.x, frame.min.y), Vec2::<()>::new(frame.size.x, frame.size.y)),
            size_px,
            seed: Hash::<Vec2<()>>::hash(&ctx.seed.as_f64()),
            strength: self.strength.0,
        };
        let texel = Vec2::<()>::new(1.0 / size_px.x, 1.0 / size_px.y);

        let blur = BLUR_PX.0 * self.strength.0.min(1.0);
        let melt = self.style == MorphStyle::EdgeMelt;

        if self.style == MorphStyle::Pixelate {
            self.apply_pixelate(ctx, out, view, &mf, progress, surface_pass);
            return;
        }

        crate::render::fill(out, |x, y| {
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let uv0 = Vec2::<()>::new(px.x / size_px.x, px.y / size_px.y);
            let flow = mf.flow_at(px);

            if surface_pass {
                let s = T_SURFACE.at(progress);
                if s <= 0.0 || (flow.x == 0.0 && flow.y == 0.0) {
                    return view.sample_bilinear(uv0);
                }
                let mut tc = uv0;
                if melt {
                    for _ in 0..MELT_STEPS {
                        let (d, _) = melt_dir(view, tc, texel, true);
                        let step = mf.region_mask(px) * MELT_PX.0 * mf.strength;
                        tc += Vec2::<()>::new(
                            (d.x * step + flow.x * MELT_NOISE) * -s / MELT_STEPS as f64 * texel.x,
                            (d.y * step + flow.y * MELT_NOISE) * -s / MELT_STEPS as f64 * texel.y,
                        );
                    }
                } else {
                    tc -= Vec2::<()>::new(flow.x * s * texel.x, flow.y * s * texel.y);
                }
                let mut c = blurred_read(view, tc, blur * s * mf.region_mask(px), texel, true);
                c.a = 1.0;
                c
            } else {
                let w = 1.0 - T_WINDOW.at(progress);
                let f = T_FADE.at(progress);
                if w <= 0.0 && f >= 1.0 {
                    return read_layer(view, uv0, false);
                }
                let mut tc = uv0;
                let mut edge = 0.0;
                if melt {
                    for _ in 0..MELT_STEPS {
                        let (d, e) = melt_dir(view, tc, texel, false);
                        edge = e;
                        let step = mf.region_mask(px) * MELT_PX.0 * mf.strength;
                        tc += Vec2::<()>::new(
                            (d.x * step + flow.x * MELT_NOISE) * w / MELT_STEPS as f64 * texel.x,
                            (d.y * step + flow.y * MELT_NOISE) * w / MELT_STEPS as f64 * texel.y,
                        );
                    }
                } else {
                    tc += Vec2::<()>::new(flow.x * w * texel.x, flow.y * w * texel.y);
                }
                let mut c = blurred_read(view, tc, blur * w * mf.region_mask(px), texel, false);
                let mut n = Simplex2D.at(Vec2::<()>::new(
                    px.x / DISSOLVE_SCALE + mf.seed.x * 37.0 + 3.7,
                    px.y / DISSOLVE_SCALE + mf.seed.y * 37.0 + 3.7,
                ));
                n = ((n - 0.5) * 1.6 + 0.5).clamp(0.0, 1.0);
                let early = if melt { EDGE_FIRST } else { 0.0 };
                n -= early * edge;
                let a = crate::smoothstep(n - DISSOLVE_EDGE, n + DISSOLVE_EDGE, crate::mix(-DISSOLVE_EDGE - early, 1.0 + DISSOLVE_EDGE, f));
                c.a *= a;
                c
            }
        });
    }
}

impl Morph {
    /// The `pixelate` style - shared block grid, staggered colour
    /// flips between `P_COARSEN` and `P_SHARPEN`.
    fn apply_pixelate(
        &self,
        _ctx: &FxContext<'_>,
        out: &mut BufferViewMut<'_, Rgba>,
        view: BufferView<'_, Rgba>,
        mf: &MorphFrame,
        progress: f64,
        surface_pass: bool,
    ) {
        let size_px = mf.size_px;
        let texel = Vec2::<()>::new(1.0 / size_px.x, 1.0 / size_px.y);

        let coarse = P_COARSEN.at(progress) * (1.0 - P_SHARPEN.at(progress));
        let bs = crate::mix(1.0, (PIXEL_PX.0 * mf.strength).max(1.0), coarse).floor().max(1.0);
        let (fo, fs) = mf.frame;
        let centre = fo + fs * 0.5;
        let peak = (PIXEL_PX.0 * mf.strength + 0.5).floor().max(1.0);
        let f = P_FADE.at(progress);
        let seed_px = Vec2::<()>::new((mf.seed.x * 1000.0).floor(), (mf.seed.y * 1000.0).floor());

        crate::render::fill(out, |x, y| {
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let uv0 = Vec2::<()>::new(px.x / size_px.x, px.y / size_px.y);
            let cell = Vec2::<()>::new(((px.x - centre.x) / bs).floor(), ((px.y - centre.y) / bs).floor());
            let lo = centre + cell * bs;
            let block_c = lo + Vec2::<()>::new(0.5 * bs, 0.5 * bs);
            let in_region = lo.x < fo.x + fs.x && lo.y < fo.y + fs.y && lo.x + bs > fo.x && lo.y + bs > fo.y;

            let mut c = if in_region && bs > 1.0 {
                // `blockColor` - PIXEL_TAPS²-tap premultiplied mean.
                let mut acc = [0.0; 4];
                for j in 0..PIXEL_TAPS {
                    for i in 0..PIXEL_TAPS {
                        let p = Vec2::<()>::new(
                            block_c.x + ((i as f64 + 0.5) / PIXEL_TAPS as f64 - 0.5) * bs,
                            block_c.y + ((j as f64 + 0.5) / PIXEL_TAPS as f64 - 0.5) * bs,
                        );
                        let t = read_layer(view, Vec2::<()>::new(p.x * texel.x, p.y * texel.y), surface_pass);
                        acc[0] += t.r * t.a;
                        acc[1] += t.g * t.a;
                        acc[2] += t.b * t.a;
                        acc[3] += t.a;
                    }
                }
                let n = (PIXEL_TAPS * PIXEL_TAPS) as f64;
                let a = acc[3] / n;
                if a > 0.0 {
                    Color::<f64>::new_rgba(acc[0] / n / a, acc[1] / n / a, acc[2] / n / a, a)
                } else {
                    Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0)
                }
            } else {
                read_layer(view, uv0, surface_pass)
            };

            if surface_pass {
                c.a = 1.0;
            } else if f < 1.0 {
                let pcell = Vec2::<()>::new(((block_c.x - centre.x) / peak).floor(), ((block_c.y - centre.y) / peak).floor());
                let pc = centre + (pcell + Vec2::<()>::new(0.5, 0.5)) * peak;
                let mut n = Simplex2D.at(Vec2::<()>::new(
                    pc.x / DISSOLVE_SCALE + mf.seed.x * 37.0 + 3.7,
                    pc.y / DISSOLVE_SCALE + mf.seed.y * 37.0 + 3.7,
                ));
                n = ((n - 0.5) * 1.6 + 0.5).clamp(0.0, 1.0);
                n = crate::mix(n, Hash::<f64>::hash(&(pcell + seed_px)), PIXEL_RANDOM);
                n = ((n - 0.5) * PIXEL_SPREAD + 0.5).clamp(0.0, 1.0);
                let t0 = n * (1.0 - PIXEL_DUR);
                c.a *= crate::smoothstep(t0, t0 + PIXEL_DUR, f);
            }
            c
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn solid(size: u32) -> PixelBuffer<Rgba> {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(size, size));
        let mut view = buffer.view_mut();
        let c = Color::<f64>::new_rgba(0.5, 0.5, 0.5, 1.0);
        for y in 0..size {
            for x in 0..size {
                view.put_color(Pixels::new(x), Pixels::new(y), c);
            }
        }
        buffer
    }

    fn ctx_of<'a>(window: &'a PixelBuffer<Rgba>, surface: Option<&'a PixelBuffer<Rgba>>, layer: Layer, progress: f64, direction: Direction) -> FxContext<'a> {
        let size = window.size();
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_secs_f64(2.0),
            seed: Seed(0xABCD_EF01_2345_6789),
            direction,
            layer,
            window: window.view(),
            surface: surface.map(|s| s.view()),
            assets: crate::FxAssets::default(),
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(4.0, 4.0, f64::from(size.x) - 8.0, f64::from(size.y) - 8.0),
        }
    }

    /// Open end: the window dissolved fully in; close end: gone.
    #[test]
    fn window_pass_endpoints() {
        let window = solid(48);
        let surface = solid(48);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(48, 48));
        let fx = Morph::default();
        fx.apply(&ctx_of(&window, Some(&surface), Layer::Window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, Some(&surface), Layer::Window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// The dissolve is blotch-driven: mid-close some spots are in,
    /// some still out.
    #[test]
    fn dissolve_is_uneven() {
        let window = solid(96);
        let surface = solid(96);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(96, 96));
        Morph::default().apply(&ctx_of(&window, Some(&surface), Layer::Window, 0.5, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let mut lo = false;
        let mut hi = false;
        for y in 0..96 {
            for x in 0..96 {
                let a = view.sample_bilinear(Vec2::<()>::new((x as f64 + 0.5) / 96.0, (y as f64 + 0.5) / 96.0)).a;
                lo |= a < 0.5;
                hi |= a > 0.5;
            }
        }
        assert!(lo && hi, "dissolve should leave both states mid-run");
    }
}
