use super::grid::hex_cell;
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
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2D;

/// The window breaks into glowing hexagonal tiles that shrink away.
///
/// Behavioral spec (source shader: `hexagon.frag`, re-implemented
/// from semantics): `progress` inverts for `Open`, then noise-spreads
/// per pixel (`clamp(mix(noise - 1, noise + 1, progress))`) so tiles
/// pop individually. `glowProgress` fades the lines in during the
/// first half, `tileProgress` shrinks tiles in the second half by
/// offsetting the texture lookup toward the cell edge; tiles vanish
/// once `tileProgress >= hex.edge`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Hexagon {
    /// Tile scale - schema `hexagon-scale`, default `3.0`.
    #[serde(default = "Hexagon::default_scale")]
    pub scale: Factor,
    /// Glow-line width - schema `hexagon-line_width`, default `2.0`.
    #[serde(default = "Hexagon::default_line_width")]
    pub line_width: Factor,
    /// Sharp line color - schema `hexagon-line_color`, default `rgba(200, 255, 255, 0.5)`.
    #[serde(default = "Hexagon::default_line_color")]
    pub line_color: Color<f64>,
    /// Soft center glow color - schema `hexagon-glow_color`, default `rgba(20, 80, 255, 0.5)`.
    #[serde(default = "Hexagon::default_glow_color")]
    pub glow_color: Color<f64>,
    /// Additive line blending - schema `hexagon-additive_blending`, default `true`.
    #[serde(default = "Hexagon::default_additive_blending")]
    pub additive_blending: bool,
}

impl Hexagon {
    const fn default_scale() -> Factor {
        Factor::new(3.0)
    }
    const fn default_line_width() -> Factor {
        Factor::new(2.0)
    }
    const fn default_line_color() -> Color<f64> {
        Color::<f64>::new_rgba(200.0 / 255.0, 1.0, 1.0, 0.5)
    }
    const fn default_glow_color() -> Color<f64> {
        Color::<f64>::new_rgba(20.0 / 255.0, 80.0 / 255.0, 1.0, 0.5)
    }
    const fn default_additive_blending() -> bool {
        true
    }
}

impl Default for Hexagon {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            line_width: Self::default_line_width(),
            line_color: Self::default_line_color(),
            glow_color: Self::default_glow_color(),
            additive_blending: Self::default_additive_blending(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "scale",
        label: "Scale",
        kind: ParamKind::Float {
            min: 0.1,
            max: 10.0,
            step: 0.1,
            default: 3.0,
        },
    },
    ParamSpec {
        key: "line_width",
        label: "Line Width",
        kind: ParamKind::Float {
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default: 2.0,
        },
    },
    ParamSpec {
        key: "line_color",
        label: "Line Color",
        kind: ParamKind::Color {
            default: Hexagon::default_line_color(),
        },
    },
    ParamSpec {
        key: "glow_color",
        label: "Glow Color",
        kind: ParamKind::Color {
            default: Hexagon::default_glow_color(),
        },
    },
    ParamSpec {
        key: "additive_blending",
        label: "Additive Blending",
        kind: ParamKind::Toggle { default: true },
    },
];

impl WindowEffect for Hexagon {
    fn nick(&self) -> &'static str {
        "hexagon"
    }

    fn label(&self) -> &'static str {
        "Hexagon"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.scale.0),
            ParamValue::Float(self.line_width.0),
            ParamValue::Color(self.line_color),
            ParamValue::Color(self.glow_color),
            ParamValue::Bool(self.additive_blending),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("hexagon param {index}"));
        if values.len() != 5 {
            return Err(FxError::ParamMismatch(format!("hexagon takes 5 parameters, got {}", values.len())));
        }
        self.scale = Factor::new(values[0].as_f64().ok_or(mismatch(0))?.max(0.01));
        self.line_width = Factor::new(values[1].as_f64().ok_or(mismatch(1))?.max(0.0));
        self.line_color = values[2].as_color().ok_or(mismatch(2))?;
        self.glow_color = values[3].as_color().ok_or(mismatch(3))?;
        self.additive_blending = values[4].as_bool().ok_or(mismatch(4))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let opening = matches!(ctx.direction, crate::Direction::Open);
        // uSeed is a [0,1] uv shift, not a noise-coordinate offset.
        let seed = smearor_commons_noise::Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        // Frame constants.
        let raw_progress = ctx.progress.as_f64();
        let tex_scale = Vec2::<()>::new(0.1 * size_px.x / self.scale.0, 0.1 * size_px.y / self.scale.0);
        let line_half = self.line_width.0 * 0.02 * 0.5;
        let line_full = self.line_width.0 * 0.02;
        let line = self.line_color;
        let glow = self.glow_color;
        let additive = self.additive_blending;
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let progress = if opening { 1.0 - raw_progress } else { raw_progress };
            // Per-pixel noise spread so tiles pop individually.
            let noise = Simplex2D.at(uv + seed);
            let progress = crate::mix(noise - 1.0, noise + 1.0, progress).clamp(0.0, 1.0);
            let glow_progress = crate::smoothstep(0.0, 1.0, (progress / 0.5).clamp(0.0, 1.0));
            let tile_progress = crate::smoothstep(0.0, 1.0, ((progress - 0.5) / 0.5).clamp(0.0, 1.0));
            let hex = hex_cell(Vec2::<()>::new(uv.x * tex_scale.x, uv.y * tex_scale.y));
            // Crop outer parts of the shrinking tiles.
            if tile_progress < hex.edge {
                // Shrink tiles by offsetting the lookup toward the cell edge.
                let shrink = tile_progress / (1.0 - tile_progress);
                let offset = Vec2::<()>::new(hex.coords.x / tex_scale.x * shrink, hex.coords.y / tex_scale.y * shrink);
                let mut c = ctx.window.sample_bilinear(uv + offset);
                let mut glow_a = glow.a * (hex.glow.powf(20.0) * 10.0 + hex.glow.powf(10.0) * 5.0 + hex.glow.powf(2.0) * 0.5);
                let mut line_a = line.a * (1.0 - crate::smoothstep(line_half, line_full, hex.edge));
                glow_a *= glow_progress * c.a;
                line_a *= glow_progress * c.a;
                if additive {
                    c.r += glow.r * glow_a + line.r * line_a;
                    c.g += glow.g * glow_a + line.g * line_a;
                    c.b += glow.b * glow_a + line.b * line_a;
                } else {
                    c.r = crate::mix(c.r, glow.r, glow_a);
                    c.g = crate::mix(c.g, glow.g, glow_a);
                    c.b = crate::mix(c.b, glow.b, glow_a);
                    c.r = crate::mix(c.r, line.r, line_a);
                    c.g = crate::mix(c.g, line.g, line_a);
                    c.b = crate::mix(c.b, line.b, line_a);
                }
                c
            } else {
                Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0)
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

    fn ctx_of<'a>(window: &'a PixelBuffer<Rgba>, progress: f64, direction: Direction) -> FxContext<'a> {
        let size = window.size();
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_secs_f64(2.0),
            seed: Seed(0xABCD_EF01_2345_6789),
            direction,
            layer: Layer::Window,
            window: window.view(),
            surface: None,
            assets: crate::FxAssets::default(),
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),
        }
    }

    /// Full open untouched, full close gone.
    #[test]
    fn hexagon_endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = Hexagon::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// Late in the close the noise spread leaves some cells gone and
    /// some alive - alpha is no longer uniform.
    #[test]
    fn tiles_pop_individually() {
        let window = solid(64);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        Hexagon::default().apply(&ctx_of(&window, 0.65, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let mut gone = 0;
        let mut lit = 0;
        for y in 0..64 {
            for x in 0..64 {
                let a = view.sample_bilinear(Vec2::<()>::new((x as f64 + 0.5) / 64.0, (y as f64 + 0.5) / 64.0)).a;
                if a < 0.5 { gone += 1 } else { lit += 1 }
            }
        }
        assert!(gone > 0 && lit > 0, "tiles should pop individually: {gone} gone, {lit} lit");
    }
}
