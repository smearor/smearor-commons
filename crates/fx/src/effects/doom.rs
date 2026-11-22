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
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2DFractal;

/// The window melts like the classic DOOM screen melt - pixelated
/// columns slide downward with noise-staggered offsets while the
/// top/bottom edges fade.
///
/// Behavioral spec (source shader: `doom.frag`, re-implemented from
/// semantics): `pixelateProgress` is `progress` (`Close`) or `0.9 -
/// progress` (`Open`); the pixel grid snaps uv to `pixelSize` cells
/// (`uv.y` first stretched by `actor_scale` around the center).
/// `noise = simplex2DFractal(uv.x * hScale) * 2 - 0.5` staggers the
/// columns; `shift = noise * vScale + shiftProgress` slides them
/// (`Close`: `-vScale -> 1 + vScale`, `Open`: `-vScale - 1 -> 0`);
/// only the positive half (`Close`) or negative half (`Open`) applies
/// (`0.5 * actor_scale` factor). Top/bottom fade via a `0.1`-relative
/// edge mask. `actor_scale` is not a parameter - it comes from
/// `2 * max(1, layer_h / window_h)`, matching `Doom.js`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Doom {
    /// Column noise frequency - schema `doom-horizontal_scale`, default `10.0`.
    #[serde(default = "Doom::default_horizontal_scale")]
    pub horizontal_scale: Factor,
    /// Column fall distance - schema `doom-vertical_scale`, default `10.0`.
    #[serde(default = "Doom::default_vertical_scale")]
    pub vertical_scale: Factor,
    /// Melt pixel size - schema `doom-pixel_size`, default `10`.
    #[serde(default = "Doom::default_pixel_size")]
    pub pixel_size: Pixels<u32>,
}

impl Doom {
    const fn default_horizontal_scale() -> Factor {
        Factor::new(10.0)
    }
    const fn default_vertical_scale() -> Factor {
        Factor::new(10.0)
    }
    const fn default_pixel_size() -> Pixels<u32> {
        Pixels::new(10)
    }
}

impl Default for Doom {
    fn default() -> Self {
        Self {
            horizontal_scale: Self::default_horizontal_scale(),
            vertical_scale: Self::default_vertical_scale(),
            pixel_size: Self::default_pixel_size(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "horizontal_scale",
        label: "Horizontal Scale",
        kind: ParamKind::Float {
            min: 0.1,
            max: 50.0,
            step: 1.0,
            default: 10.0,
        },
    },
    ParamSpec {
        key: "vertical_scale",
        label: "Vertical Scale",
        kind: ParamKind::Float {
            min: 0.1,
            max: 50.0,
            step: 1.0,
            default: 10.0,
        },
    },
    ParamSpec {
        key: "pixel_size",
        label: "Pixel Size",
        kind: ParamKind::Int { min: 1, max: 50, default: 10 },
    },
];

impl WindowEffect for Doom {
    fn nick(&self) -> &'static str {
        "doom"
    }

    fn label(&self) -> &'static str {
        "Doom"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.horizontal_scale.0),
            ParamValue::Float(self.vertical_scale.0),
            ParamValue::Int(self.pixel_size.0 as i64),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("doom param {index}"));
        if values.len() != 3 {
            return Err(FxError::ParamMismatch(format!("doom takes 3 parameters, got {}", values.len())));
        }
        self.horizontal_scale = Factor::new(values[0].as_f64().ok_or(mismatch(0))?);
        self.vertical_scale = Factor::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.pixel_size = Pixels::new(values[2].as_i64().ok_or(mismatch(2))?.max(1) as u32);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let p = ctx.progress.as_f64();
        let (pixelate_progress, opening) = match ctx.direction {
            crate::Direction::Open => (0.9 - p, true),
            crate::Direction::Close => (p, false),
        };
        // Frame constants.
        let pixel_size = (self.pixel_size.0 as f64 * pixelate_progress + 1.0).ceil().max(1.0);
        let grid = Vec2::<()>::new(pixel_size / size_px.x, pixel_size / size_px.y);
        // How much the melt may fall below the window rect - Doom.js'
        // 2 * max(1, stage_h / actor_h), with the layer as the stage.
        let actor_scale = 2.0 * (size_px.y / ctx.window_rect.size.y).max(1.0);
        let h_scale = self.horizontal_scale.0 * size_px.x * 0.001;
        let v_scale = self.vertical_scale.0 * size_px.y * 0.00002 * actor_scale;
        let shift_progress = if opening {
            crate::mix(-v_scale - 1.0, 0.0, p)
        } else {
            crate::mix(-v_scale, 1.0 + v_scale, p)
        };
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            // Stretch around the center, then snap to the pixel grid.
            let mut texcoord = uv;
            texcoord.y = texcoord.y * actor_scale + 0.5 - actor_scale * 0.5;
            texcoord -= Vec2::<()>::new(crate::fract(uv.x / grid.x) * grid.x, crate::fract(uv.y / grid.y) * grid.y);
            texcoord += grid * 0.5;
            // Column noise staggers the melt.
            let noise = Simplex2DFractal.at(Vec2::<()>::new(texcoord.x * h_scale, texcoord.x * h_scale)) * 2.0 - 0.5;
            let shift = noise * v_scale + shift_progress;
            texcoord.y -= 0.5 * actor_scale * if opening { shift.min(0.0) } else { shift.max(0.0) };
            let mut c = ctx.window.sample_bilinear(texcoord);
            // Fade at the actor boundary over 10% of the window size.
            c.a *= crate::relative_edge_mask(uv, 0.1);
            c
        });
    }
}
