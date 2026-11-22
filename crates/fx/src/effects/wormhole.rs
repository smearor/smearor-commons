use crate::FxContext;
use crate::FxError;
use crate::Layer;
use crate::Layers;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferView;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Vec2;

/// The desktop is a rubber sheet that gets sucked into the screen
/// around the window's centre, turning into a fog-darkened tube; a
/// small window lies on the sheet's rising floor and is pushed out
/// as the sheet flattens.
///
/// Behavioral spec (source shader: `wormhole.frag`, re-implemented
/// from semantics in `WINDOW_ON_SHEET` mode with `BOUNCE` and
/// `BOUNCE_ON_CLOSE`): geometry lives in "portal space"
/// (`v = (px - center) / axes`, distance = superellipse norm with
/// `SHAPE_POWER`, so the funnel follows the window's aspect). The
/// camera sits `CAMERA_DIST` above the sheet; a sheet point at depth
/// `z` is seen at `v * dist / (dist + z)` - `unproject` inverts that
/// by bisection (`SOLVE_STEPS`). Sheet depth
/// `amp * (1-u)^2 / (u - throat)^THROAT_EXP` flattens seamlessly at
/// `u = 1` and plunges to the rising floor near the throat; fog
/// `exp(-depth * fog)` darkens it into the void. The window rides on
/// the sheet (`gSheetScale`), fading in over `T_APPEAR`.
/// `wormhole-bounce` (> 0) replaces the eased landing with a
/// physically-timed ball bounce (`BOUNCE_COUNT` dips, restitution =
/// `bounce`) - opening bounces the flattening, closing the funnel's
/// collapse. `uAxes` = `frame_half * funnel_size + FUNNEL_EXTRA`,
/// clamped to the texture, matching `Wormhole.js`.
///
/// Two-pass effect: `Layer::Surface` warps the backdrop, `Layer::Window`
/// rides the sheet - the consumer composites window over surface.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Wormhole {
    /// Throat radius - schema `wormhole-void-size`, default `0.65`.
    #[serde(default = "Wormhole::default_void_size")]
    pub void_size: Factor,
    /// Visible tube depth - schema `wormhole-tunnel-depth`, default `1.0`.
    #[serde(default = "Wormhole::default_tunnel_depth")]
    pub tunnel_depth: Factor,
    /// Funnel reach - schema `wormhole-funnel-size`, default `1.35`.
    #[serde(default = "Wormhole::default_funnel_size")]
    pub funnel_size: Factor,
    /// Landing bounce restitution - schema `wormhole-bounce`, default `0.5`.
    #[serde(default = "Wormhole::default_bounce")]
    pub bounce: Factor,
}

impl Wormhole {
    const fn default_void_size() -> Factor {
        Factor::new(0.65)
    }
    const fn default_tunnel_depth() -> Factor {
        Factor::new(1.0)
    }
    const fn default_funnel_size() -> Factor {
        Factor::new(1.35)
    }
    const fn default_bounce() -> Factor {
        Factor::new(0.5)
    }
}

impl Default for Wormhole {
    fn default() -> Self {
        Self {
            void_size: Self::default_void_size(),
            tunnel_depth: Self::default_tunnel_depth(),
            funnel_size: Self::default_funnel_size(),
            bounce: Self::default_bounce(),
        }
    }
}

const SHAPE_POWER: f64 = 2.5;
const CAMERA_DIST: f64 = 1.5;
const AMPLITUDE: f64 = 0.5;
const THROAT_EXP: f64 = 1.0;
const AMP_START: f64 = 0.6;
const THROAT_START: f64 = 0.35;
const SPREAD_START: f64 = 0.55;
const FOG_BASE: f64 = 0.9;
const WALL_LIGHT: f64 = 0.35;
const LIGHT_TO: Vec2<()> = Vec2::<()>::new(std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2);
const WIN_SHEET_START: f64 = 0.12;
const FLOOR_START: f64 = 0.002;
const FLOOR_RISE: f64 = 1.0;
const T_SINK: (f64, f64) = (0.0, 0.30);
const T_WIDEN: (f64, f64) = (0.05, 0.50);
const T_APPEAR: (f64, f64) = (0.38, 0.48);
const T_FLAT: (f64, f64) = (0.45, 1.00);
const T_RISE: (f64, f64) = (0.40, 0.95);
const BOUNCE_COUNT: usize = 3;
const SOLVE_STEPS: usize = 24;
const FUNNEL_EXTRA: f64 = 80.0;

/// `shapeNorm` - superellipse distance, follows the window's aspect.
fn shape_norm(v: Vec2<()>) -> f64 {
    let a = v.x.abs();
    let b = v.y.abs();
    (a.powf(SHAPE_POWER) + b.powf(SHAPE_POWER)).powf(1.0 / SHAPE_POWER)
}

/// `bounceTime` - total air time of all dips, in units of the fall.
fn bounce_time(restitution: f64) -> f64 {
    let mut total = 0.0;
    let mut e = 1.0;
    for _ in 0..BOUNCE_COUNT {
        e *= restitution;
        total += 2.0 * e;
    }
    total
}

/// `bounceDip` - depth below "at rest", `s` in units of the fall's
/// duration (the fall covers 1 in time 1, so gravity = 2).
fn bounce_dip(s: f64, restitution: f64) -> f64 {
    let mut start = 0.0;
    let mut e = 1.0;
    for _ in 0..BOUNCE_COUNT {
        e *= restitution;
        let dur = 2.0 * e;
        if s < start + dur {
            let x = (s - start) / dur;
            return e * e * 4.0 * x * (1.0 - x);
        }
        start += dur;
    }
    0.0
}

/// Per-frame funnel state (the shader's `g*` uniforms).
struct Funnel {
    amp: f64,
    throat: f64,
    fog: f64,
    floor: f64,
    sheet_scale: f64,
    win_alpha: f64,
    axes: Vec2<()>,
    center: Vec2<()>,
}

impl Funnel {
    /// `surfaceDepth` - sheet depth at norm `u`, to the floor.
    fn depth(&self, u: f64) -> f64 {
        if u >= 1.0 || self.amp <= 0.0 {
            return 0.0;
        }
        let d = (u - self.throat).max(0.0) + self.floor;
        self.amp * (1.0 - u) * (1.0 - u) / d.powf(THROAT_EXP)
    }

    /// `project` - where a sheet point at norm `u` appears on screen.
    fn project(&self, u: f64) -> f64 {
        u * CAMERA_DIST / (CAMERA_DIST + self.depth(u))
    }

    /// `unproject` - the sheet point seen at screen norm `uq`
    /// (bisection; `project` rises monotonically).
    fn unproject(&self, uq: f64) -> f64 {
        if uq >= 1.0 || self.amp <= 0.0 {
            return uq;
        }
        let (mut lo, mut hi) = (0.0, 1.0);
        for _ in 0..SOLVE_STEPS {
            let mid = 0.5 * (lo + hi);
            if self.project(mid) < uq {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }

    /// `sheetAt` - the sheet pixel seen at layer pixel `px`, plus fog
    /// and slope lighting.
    fn sheet_at(&self, px: Vec2<()>, v: Vec2<()>, uq: f64) -> (Vec2<()>, f64) {
        if uq >= 1.0 || self.amp <= 0.0 {
            return (px, 1.0);
        }
        let u = self.unproject(uq);
        let k = u / uq.max(1e-6);
        let sheet_px = self.center + Vec2::<()>::new(v.x * k * self.axes.x, v.y * k * self.axes.y);
        let h = self.depth(u);
        let slope = ((self.depth(u + 1e-3) - h) / -1e-3).max(0.0);
        let slope = slope / (1.0 + slope);
        let dir = if uq > 1e-5 {
            let l = (v.x * v.x + v.y * v.y).sqrt();
            Vec2::<()>::new(v.x / l, v.y / l)
        } else {
            Vec2::<()>::new(0.0, 0.0)
        };
        let bright = ((-h * self.fog).exp() * (1.0 + WALL_LIGHT * (dir.x * LIGHT_TO.x + dir.y * LIGHT_TO.y) * slope)).max(0.0);
        (sheet_px, bright)
    }
}

/// The window's texture read - transparent outside `[0, 1]` (the
/// shader's bounds check), unlike the clamp-to-edge surface read.
fn read_window(view: BufferView<'_, Rgba>, uv: Vec2<()>) -> Color<f64> {
    if !(0.0..=1.0).contains(&uv.x) || !(0.0..=1.0).contains(&uv.y) {
        return Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
    }
    view.sample_bilinear(uv)
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "void_size",
        label: "Void Size",
        kind: ParamKind::Float {
            min: 0.05,
            max: 2.0,
            step: 0.05,
            default: 0.65,
        },
    },
    ParamSpec {
        key: "tunnel_depth",
        label: "Tunnel Depth",
        kind: ParamKind::Float {
            min: 0.05,
            max: 4.0,
            step: 0.05,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "funnel_size",
        label: "Funnel Size",
        kind: ParamKind::Float {
            min: 0.5,
            max: 4.0,
            step: 0.05,
            default: 1.35,
        },
    },
    ParamSpec {
        key: "bounce",
        label: "Bounce",
        kind: ParamKind::Float {
            min: 0.0,
            max: 0.95,
            step: 0.05,
            default: 0.5,
        },
    },
];

impl WindowEffect for Wormhole {
    fn nick(&self) -> &'static str {
        "wormhole"
    }

    fn label(&self) -> &'static str {
        "Wormhole"
    }

    fn layers(&self) -> Layers {
        Layers::SurfaceAndWindow
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.void_size.0),
            ParamValue::Float(self.tunnel_depth.0),
            ParamValue::Float(self.funnel_size.0),
            ParamValue::Float(self.bounce.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("wormhole param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("wormhole takes 4 parameters, got {}", values.len())));
        }
        self.void_size = Factor::new(values[0].as_f64().ok_or(mismatch(0))?);
        self.tunnel_depth = Factor::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.funnel_size = Factor::new(values[2].as_f64().ok_or(mismatch(2))?);
        self.bounce = Factor::new(values[3].as_f64().ok_or(mismatch(3))?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = out.size();
        let (w, h) = (f64::from(size.x), f64::from(size.y));
        let opening = ctx.direction == crate::Direction::Open;
        let progress = ctx.progress.as_f64();

        // Portal geometry: centre = window centre, axes clamp to the
        // texture (the JS clamps to the screen).
        let frame = ctx.window_rect;
        let center = Vec2::<()>::new(frame.min.x + frame.size.x * 0.5, frame.min.y + frame.size.y * 0.5);
        let ax = (frame.size.x * 0.5 * self.funnel_size.0 + FUNNEL_EXTRA)
            .min(center.x)
            .min(w - center.x)
            .max(8.0);
        let ay = (frame.size.y * 0.5 * self.funnel_size.0 + FUNNEL_EXTRA)
            .min(center.y)
            .min(h - center.y)
            .max(8.0);

        // Timeline + bounce (BOUNCE_KEEP_INTRO path: normal pace until
        // the knee, then fall + dips in the time left).
        let bounce_on = self.bounce.0 > 0.001;
        let open_bounce = bounce_on && opening;
        let close_bounce = bounce_on && !opening;
        let mut progress_eff = if opening { progress } else { 1.0 - progress };
        let mut dip = 0.0;
        let mut landed = false;
        if open_bounce || close_bounce {
            let knee = if open_bounce { T_FLAT.0 } else { 1.0 - T_SINK.1 };
            let speed = 1.0 + bounce_time(self.bounce.0);
            let fall_dur = (1.0 - knee) / speed;
            let land_at = knee + fall_dur;
            let p = if progress <= knee {
                progress
            } else {
                (knee + (progress - knee) * speed).min(1.0)
            };
            progress_eff = if opening { p } else { 1.0 - p };
            if progress > land_at {
                landed = true;
                dip = bounce_dip((progress - land_at) / fall_dur, self.bounce.0);
            }
        }

        let span = |t: (f64, f64)| crate::smoothstep(t.0, t.1, progress_eff);
        let mut sink = span(T_SINK);
        if close_bounce {
            let x = ((T_SINK.1 - progress_eff) / (T_SINK.1 - T_SINK.0)).clamp(0.0, 1.0);
            sink = 1.0 - x * x + dip;
        }
        let mut widen = span(T_WIDEN);
        let fall = |t: (f64, f64)| {
            let x = ((progress_eff - t.0) / (t.1 - t.0)).clamp(0.0, 1.0);
            x * x
        };
        let mut flat_t = if open_bounce { fall(T_FLAT) - dip } else { span(T_FLAT) };
        if close_bounce && landed {
            sink = 1.0;
            widen = 1.0;
            flat_t = 1.0 - dip;
        }
        let mut rise_t = span(T_RISE);
        if open_bounce {
            // Fall in real time to the landing so the speed has no
            // kink where the timeline speeds up (BOUNCE_KEEP_INTRO).
            let knee = T_FLAT.0;
            let speed = 1.0 + bounce_time(self.bounce.0);
            let land_at = knee + (1.0 - knee) / speed;
            let x = ((progress - T_RISE.0) / (land_at - T_RISE.0)).clamp(0.0, 1.0);
            rise_t = x * x;
        }
        let flatness = 1.0 - flat_t;
        let floor_t = if close_bounce { flat_t.max(1.0 - sink) } else { flat_t };

        let funnel = Funnel {
            amp: AMPLITUDE * sink * crate::mix(AMP_START, 1.0, widen) * flatness,
            throat: self.void_size.0 * sink * crate::mix(THROAT_START, 1.0, widen) * flatness,
            fog: FOG_BASE / self.tunnel_depth.0.max(0.05),
            floor: FLOOR_START + FLOOR_RISE * floor_t * floor_t,
            sheet_scale: WIN_SHEET_START.ln().mul_add(1.0 - rise_t, 0.0).exp(),
            win_alpha: span(T_APPEAR),
            axes: Vec2::<()>::new(ax * crate::mix(SPREAD_START, 1.0, widen), ay * crate::mix(SPREAD_START, 1.0, widen)),
            center,
        };

        let surface_pass = ctx.layer == Layer::Surface;
        let source = if surface_pass { ctx.surface } else { Some(ctx.window) };
        crate::render::fill(out, |x, y| {
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let uv0 = Vec2::<()>::new(px.x / w, px.y / h);
            let Some(view) = source else {
                return Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
            };
            let v = Vec2::<()>::new((px.x - center.x) / funnel.axes.x, (px.y - center.y) / funnel.axes.y);
            let uq = shape_norm(v);
            if surface_pass {
                if uq >= 1.0 || funnel.amp <= 0.0 {
                    return view.sample_bilinear(uv0);
                }
                let (sheet_px, bright) = funnel.sheet_at(px, v, uq);
                let mut c = view.sample_bilinear(Vec2::<()>::new(sheet_px.x / w, sheet_px.y / h));
                c.r *= bright;
                c.g *= bright;
                c.b *= bright;
                c.a = 1.0;
                c
            } else {
                if funnel.amp <= 0.0 && funnel.sheet_scale >= 0.9999 && funnel.win_alpha >= 1.0 {
                    return view.sample_bilinear(uv0);
                }
                let (sheet_px, bright) = funnel.sheet_at(px, v, uq);
                let src_px = Vec2::<()>::new(
                    center.x + (sheet_px.x - center.x) / funnel.sheet_scale.max(1e-4),
                    center.y + (sheet_px.y - center.y) / funnel.sheet_scale.max(1e-4),
                );
                let mut c = read_window(view, Vec2::<()>::new(src_px.x / w, src_px.y / h));
                c.r *= bright;
                c.g *= bright;
                c.b *= bright;
                c.a *= funnel.win_alpha;
                c
            }
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
    use smearor_commons_geometry::Pixels;
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

    /// Open end: window at rest; close end: gone into the void.
    #[test]
    fn window_pass_endpoints() {
        let window = solid(48);
        let surface = solid(48);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(48, 48));
        let fx = Wormhole::default();
        fx.apply(&ctx_of(&window, Some(&surface), Layer::Window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, Some(&surface), Layer::Window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// The surface pass leaves the sheet alone when the funnel is
    /// flat (close start / open end).
    #[test]
    fn surface_pass_flat_at_rest() {
        let window = solid(48);
        let surface = solid(48);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(48, 48));
        Wormhole::default().apply(&ctx_of(&window, Some(&surface), Layer::Surface, 0.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
    }
}
