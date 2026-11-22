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
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Hash;

use super::backdrop;
use super::cake;
use super::grid;

/// What shows behind the turning panels - schema
/// `aperture-panels-backdrop`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum Backdrop {
    /// The desktop shows through the gaps (`uBlackVoid = 0`).
    #[default]
    Wallpaper,
    /// Panels flip over an opaque black void.
    BlackVoid,
    /// The real screen shows through (no wall at all).
    Transparent,
    /// Black void with twinkling stars.
    StarField,
    /// Star field plus a colourful nebula.
    Nebula,
}

/// The window splits into a grid of panels (the Portal wall panels)
/// that flip over one after another - each panel's front shows what
/// was behind the window, its back the window.
///
/// Behavioral spec (source shader: `aperture-panels.frag`,
/// re-implemented from semantics): the visible window (`uFrame` =
/// `ctx.window_rect`) is cut into a `cols × rows` grid, `uCell`
/// dividing it exactly (`aperture-panels-panel-size` is the target,
/// snapped so it fits). `cellTiming` squeezes every flip into
/// `[border_lead, 1 - border_trail]` of the timeline, staggered by
/// `wavePosition` (five `wave_shape`s, `5` = random picked from the
/// seed) - each panel turns `local * PI` around its centre axis
/// (`flip_axis` 0 = vertical squash axis per `FLIP_AXIS_VERTICAL`,
/// `2` = random), projected with `PERSPECTIVE` and inverted to
/// `panelUV`. Front faces (local ≤ 0.5) show the surface texture,
/// back faces the window; gaps stay transparent or take the
/// `backdrop` (black void / star field / nebula; `Transparent` also
/// skips painting the surface on front faces - the real backdrop
/// shows through). The `EDGE_SHADE` darkens turning panels, the
/// `border_style` runs an outline wave ahead of and/or behind the
/// flip wave (`BORDER_LEAD`/`BORDER_TRAIL`). In the shadow margin
/// (outside `uFrame`) the surface pass copies the texture and the
/// window pass fades the shadow in once the nearest edge panel
/// turned. `easter_egg` enables the cake at `CAKE_CHANCE` (the seed
/// decides, `uCakePos` hashes it inside the window).
///
/// Two-pass effect: `Layer::Surface` paints the windows-behind faces
/// and the backdrop, `Layer::Window` the window faces and the shadow
/// margin - the consumer composites window over surface.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct AperturePanels {
    /// Target panel size in px - schema `aperture-panels-panel-size`, default `88`.
    #[serde(default = "AperturePanels::default_panel_size")]
    pub panel_size: u32,
    /// Wave shape `0..=5` (`5` = random) - schema `aperture-panels-wave-shape`, default `4`.
    #[serde(default = "AperturePanels::default_wave_shape")]
    pub wave_shape: u32,
    /// Flip axis `0` = around a vertical axis, `1` = horizontal,
    /// `2` = random - schema `aperture-panels-flip-axis`, default `0`.
    #[serde(default = "AperturePanels::default_flip_axis")]
    pub flip_axis: u32,
    /// Border wave `0..=4` - schema `aperture-panels-border-style`, default `4`.
    #[serde(default = "AperturePanels::default_border_style")]
    pub border_style: u32,
    /// Border colour - schema `aperture-panels-border-color`, default `rgba(0, 0, 0, 0.4)`.
    #[serde(default = "AperturePanels::default_border_color")]
    pub border_color: Color<f64>,
    /// What shows behind the panels - schema `aperture-panels-backdrop`, default `wallpaper`.
    #[serde(default)]
    pub backdrop: Backdrop,
    /// Cake easter egg - schema `aperture-panels-easter-egg`, default `true`.
    #[serde(default = "AperturePanels::default_easter_egg")]
    pub easter_egg: bool,
}

impl AperturePanels {
    const fn default_panel_size() -> u32 {
        88
    }
    const fn default_wave_shape() -> u32 {
        4
    }
    const fn default_flip_axis() -> u32 {
        0
    }
    const fn default_border_style() -> u32 {
        4
    }
    const fn default_border_color() -> Color<f64> {
        Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.4)
    }
    const fn default_easter_egg() -> bool {
        true
    }
}

impl Default for AperturePanels {
    fn default() -> Self {
        Self {
            panel_size: Self::default_panel_size(),
            wave_shape: Self::default_wave_shape(),
            flip_axis: Self::default_flip_axis(),
            border_style: Self::default_border_style(),
            border_color: Self::default_border_color(),
            backdrop: Backdrop::default(),
            easter_egg: Self::default_easter_egg(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "panel_size",
        label: "Panel Size",
        kind: ParamKind::Int { min: 8, max: 512, default: 88 },
    },
    ParamSpec {
        key: "wave_shape",
        label: "Wave Shape",
        kind: ParamKind::Int { min: 0, max: 5, default: 4 },
    },
    ParamSpec {
        key: "flip_axis",
        label: "Flip Axis",
        kind: ParamKind::Int { min: 0, max: 2, default: 0 },
    },
    ParamSpec {
        key: "border_style",
        label: "Border Style",
        kind: ParamKind::Int { min: 0, max: 4, default: 4 },
    },
    ParamSpec {
        key: "border_color",
        label: "Border Color",
        kind: ParamKind::Color {
            default: AperturePanels::default_border_color(),
        },
    },
    ParamSpec {
        key: "easter_egg",
        label: "Easter Egg",
        kind: ParamKind::Toggle { default: true },
    },
];

impl WindowEffect for AperturePanels {
    fn nick(&self) -> &'static str {
        "aperture-panels"
    }

    fn label(&self) -> &'static str {
        "Aperture Panels"
    }

    fn layers(&self) -> Layers {
        Layers::SurfaceAndWindow
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Int(i64::from(self.panel_size)),
            ParamValue::Int(i64::from(self.wave_shape)),
            ParamValue::Int(i64::from(self.flip_axis)),
            ParamValue::Int(i64::from(self.border_style)),
            ParamValue::Color(self.border_color),
            ParamValue::Bool(self.easter_egg),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("aperture-panels param {index}"));
        if values.len() != 6 {
            return Err(FxError::ParamMismatch(format!("aperture-panels takes 6 parameters, got {}", values.len())));
        }
        self.panel_size = values[0].as_i64().ok_or(mismatch(0))?.clamp(8, 512) as u32;
        self.wave_shape = values[1].as_i64().ok_or(mismatch(1))?.clamp(0, 5) as u32;
        self.flip_axis = values[2].as_i64().ok_or(mismatch(2))?.clamp(0, 2) as u32;
        self.border_style = values[3].as_i64().ok_or(mismatch(3))?.clamp(0, 4) as u32;
        self.border_color = values[4].as_color().ok_or(mismatch(4))?;
        self.easter_egg = values[5].as_bool().ok_or(mismatch(5))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = out.size();
        let (w, h) = (f64::from(size.x), f64::from(size.y));
        let opening = ctx.direction == crate::Direction::Open;
        let progress = if opening { ctx.progress.as_f64() } else { 1.0 - ctx.progress.as_f64() };
        let surface_pass = ctx.layer == Layer::Surface;
        let source = if surface_pass { ctx.surface } else { Some(ctx.window) };

        // Resolve "random" picks from the seed.
        let seed_h = Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        let wave_shape = if self.wave_shape >= 5 {
            (seed_h.x * 5.0).floor().min(4.0) as u32
        } else {
            self.wave_shape
        };
        let vertical = match self.flip_axis {
            1 => false,
            2 => seed_h.y >= 0.5,
            _ => true,
        };

        // Panel grid over the visible window only (`uFrame`).
        let frame = ctx.window_rect;
        let fo = Vec2::<()>::new(frame.min.x, frame.min.y);
        let fs = Vec2::<()>::new(frame.size.x, frame.size.y);
        let cols = (fs.x / f64::from(self.panel_size.max(1))).round().max(1.0);
        let rows = (fs.y / f64::from(self.panel_size.max(1))).round().max(1.0);
        let ucell = Vec2::<()>::new(fs.x / cols, fs.y / rows);

        let style = self.border_style;
        let border_on = style > 0 && self.border_color.a > 0.0;
        let lead = if style == 2 || style == 4 { grid::BORDER_LEAD } else { 0.0 };
        let trail = if style == 3 || style == 4 { grid::BORDER_TRAIL } else { 0.0 };
        let dist = grid::PERSPECTIVE * fs.x.max(fs.y);
        let black_void = matches!(self.backdrop, Backdrop::BlackVoid | Backdrop::StarField | Backdrop::Nebula);
        let star_field = matches!(self.backdrop, Backdrop::StarField | Backdrop::Nebula);
        let nebula_on = self.backdrop == Backdrop::Nebula;
        let star_seed = seed_h.x;
        let t = progress * ctx.duration.as_secs_f64();
        // The cake: `easter_egg` gates `CAKE_CHANCE`, the seed decides
        // and places it (`uCakePos`).
        let cake_seed = Hash::<Vec2<()>>::hash(&(seed_h + Vec2::<()>::new(3.7, 9.1)));
        let show_cake =
            self.easter_egg && cake_seed.x < cake::CAKE_CHANCE && fs.x >= cake::CAKE_PX * cake::CAKE_MIN_SCALE && fs.y >= cake::CAKE_PX * cake::CAKE_MIN_SCALE;
        let cake_pos = Vec2::<()>::new(crate::fract(cake_seed.x * 13.1), crate::fract(cake_seed.y * 7.9));

        crate::render::fill(out, |x, y| {
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let uv0 = Vec2::<()>::new(px.x / w, px.y / h);
            let q = px - fo;
            let mut o = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);

            if q.x < 0.0 || q.y < 0.0 || q.x >= fs.x || q.y >= fs.y {
                // Shadow margin: the texture as-is; the window pass
                // fades the shadow in as the nearest edge panel turns.
                o = source.map_or(o, |view| view.sample_bilinear(uv0));
                if !surface_pass {
                    let nq = Vec2::<()>::new(q.x.clamp(0.0, fs.x - 0.001), q.y.clamp(0.0, fs.y - 0.001));
                    let nearest = Vec2::<()>::new((nq.x / ucell.x).floor(), (nq.y / ucell.y).floor());
                    let reveal = (grid::cell_progress(nearest, ucell, fs, wave_shape, lead, trail, progress) * 2.0 - 1.0).clamp(0.0, 1.0);
                    o.a *= reveal;
                }
                return o;
            }

            let cell = Vec2::<()>::new((q.x / ucell.x).floor(), (q.y / ucell.y).floor());
            let cell_uv = Vec2::<()>::new((q.x - cell.x * ucell.x) / ucell.x, (q.y - cell.y * ucell.y) / ucell.y);
            let local = grid::cell_progress(cell, ucell, fs, wave_shape, lead, trail, progress);
            let squash = (local * std::f64::consts::PI).cos().abs();

            // Inverse projection onto the turning panel.
            let centre = (cell + Vec2::<()>::new(0.5, 0.5)).component_mul(&ucell) - fs * 0.5;
            let screen = centre + (cell_uv - Vec2::<()>::new(0.5, 0.5)).component_mul(&ucell);
            let (centre, screen) = if vertical {
                (centre, screen)
            } else {
                (Vec2::<()>::new(centre.y, centre.x), Vec2::<()>::new(screen.y, screen.x))
            };
            let panel = grid::panel_uv(local, ucell, centre, screen, dist, vertical);

            let showing_face = if surface_pass { local <= 0.5 } else { local > 0.5 };
            // Front paints the surface on every on-panel pixel; the
            // window pass only once the panel is past 90 degrees.
            let draw = surface_pass || local > 0.5;
            if let (Some(puv), true) = (panel, draw) {
                if let Some(view) = source {
                    let src = fo + (cell + puv).component_mul(&ucell);
                    let mut c = view.sample_bilinear(Vec2::<()>::new(src.x / w, src.y / h));
                    c.r *= crate::mix(grid::EDGE_SHADE, 1.0, squash);
                    c.g *= crate::mix(grid::EDGE_SHADE, 1.0, squash);
                    c.b *= crate::mix(grid::EDGE_SHADE, 1.0, squash);

                    if showing_face && border_on {
                        let amount = grid::cell_border(cell, ucell, fs, wave_shape, lead, trail, progress);
                        if amount > 0.0 {
                            let rim = if vertical {
                                Vec2::<()>::new(ucell.x * squash, ucell.y)
                            } else {
                                Vec2::<()>::new(ucell.x, ucell.y * squash)
                            };
                            let edge = Vec2::<()>::new(puv.x.min(1.0 - puv.x) * rim.x, puv.y.min(1.0 - puv.y) * rim.y);
                            let line = 1.0 - crate::smoothstep(grid::BORDER_PX - 0.5, grid::BORDER_PX + 0.5, edge.x.min(edge.y));
                            let mut b = self.border_color;
                            b.a *= line * amount;
                            c = crate::alpha_over(c, b);
                        }
                    }
                    o = c;
                }
            } else if surface_pass {
                // A gap between turning panels.
                if black_void {
                    o = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 1.0);
                    if star_field {
                        let stars = backdrop::star_field(px, Vec2::<()>::new(0.0, 0.0), star_seed, t);
                        if nebula_on {
                            let neb = backdrop::nebula(px, Vec2::<()>::new(0.0, 0.0), star_seed, t);
                            let dim = 1.0 - 0.6 * neb.a;
                            o = Color::<f64>::new_rgba(
                                (neb.r + stars.0 * dim).min(1.0),
                                (neb.g + stars.1 * dim).min(1.0),
                                (neb.b + stars.2 * dim).min(1.0),
                                1.0,
                            );
                        } else {
                            o = Color::<f64>::new_rgba(stars.0, stars.1, stars.2, 1.0);
                        }
                    }
                }

                // The cake sits on the backdrop, peeking through gaps.
                if show_cake {
                    let ccentre = Vec2::<()>::new(cake::CAKE_PX * 0.5, cake::CAKE_PX * 0.5)
                        + cake_pos.component_mul(&(fs - Vec2::<()>::new(cake::CAKE_PX, cake::CAKE_PX)));
                    let mut c = (q - ccentre) * (0.8 / (cake::CAKE_PX * 0.5));
                    c.y = -c.y;
                    if c.x.abs() <= 0.82 && c.y.abs() <= 0.82 {
                        o = crate::alpha_over(o, cake::draw_cake(c, 0.8 / (cake::CAKE_PX * 0.5), t));
                    }
                }
            }
            o
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

    /// Open end: every panel shows its window face; close end: all
    /// fronts - nothing of the window left.
    #[test]
    fn window_pass_endpoints() {
        let window = solid(64);
        let surface = solid(64);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        let fx = AperturePanels::default();
        fx.apply(&ctx_of(&window, Some(&surface), Layer::Window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, Some(&surface), Layer::Window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// Mid-flip some panels are turned, some not - the window pass
    /// has both content and gaps.
    #[test]
    fn panels_flip_individually() {
        let window = solid(96);
        let surface = solid(96);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(96, 96));
        let fx = AperturePanels {
            panel_size: 12,
            ..AperturePanels::default()
        };
        fx.apply(&ctx_of(&window, Some(&surface), Layer::Window, 0.5, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let mut gaps = 0;
        let mut faces = 0;
        for y in 0..96 {
            for x in 0..96 {
                if view.sample_bilinear(Vec2::<()>::new((x as f64 + 0.5) / 96.0, (y as f64 + 0.5) / 96.0)).a < 0.5 {
                    gaps += 1
                } else {
                    faces += 1
                }
            }
        }
        assert!(gaps > 0 && faces > 0, "panels should flip individually: {gaps} gaps, {faces} faces");
    }
}
