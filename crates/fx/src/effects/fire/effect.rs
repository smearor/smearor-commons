use super::FireColor;
use super::mask::EffectMask;
use super::ramp::DEFAULT_RAMP;
use super::ramp::fire_ramp;
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
use smearor_commons_color::Palette;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use smearor_commons_noise::Field;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Simplex2DFractal;
use smearor_commons_noise::Simplex3DFractal;

/// The window burns away behind a fire-colored fractal noise front.
///
/// Behavioral spec (source shader: `fire.frag`, re-implemented from
/// semantics): a vertical noise field (`Simplex2DFractal`, or the 3D
/// variant animated along the third axis) drifts upward over the run;
/// an [`EffectMask`] splits progress into the burn-down phase (the
/// window dissolves along the burn direction) and the after-burn phase
/// (the remaining flame fades). The noise modulates the 5-stop
/// [`FireColor`] ramp which is alpha-composited over the fading
/// window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Fire {
    /// Noise scale - schema `fire-scale`, default `1.0`.
    #[serde(default = "Fire::default_scale")]
    pub scale: ScaleFactor,
    /// Vertical noise drift speed - schema `fire-movement-speed`, default `0.5`.
    #[serde(default = "Fire::default_movement_speed")]
    pub movement_speed: Factor,
    /// Hash-derived palette instead of the gradient - schema `fire-random-color`, default `false`.
    #[serde(default)]
    pub random_color: bool,
    /// Animate through 3D noise instead of 2D - schema `fire-noise-3d`, default `false`.
    #[serde(default)]
    pub noise_3d: bool,
    /// Burn bottom-to-top on close - schema `fire-closing-bottom-to-top`, default `false`.
    #[serde(default)]
    pub closing_bottom_to_top: bool,
    /// Burn bottom-to-top on open - schema `fire-opening-bottom-to-top`, default `false`.
    #[serde(default)]
    pub opening_bottom_to_top: bool,
    /// The five-stop flame ramp - the named [`Palette`] equivalent of
    /// BMW's `fire-color-1` .. `fire-color-5` GSettings keys. Slots are
    /// [`FireColor`] in rising-temperature order; missing slots fall
    /// back to the classic fire defaults.
    #[serde(default = "Fire::default_colors")]
    #[cfg_attr(feature = "schemars", schemars(with = "std::collections::HashMap<String, Color<f64>>"))]
    pub colors: Palette<FireColor, f64>,
}

impl Fire {
    const fn default_scale() -> ScaleFactor {
        ScaleFactor::new(1.0)
    }
    const fn default_movement_speed() -> Factor {
        Factor::new(0.5)
    }

    /// The palette form of [`DEFAULT_RAMP`].
    fn default_colors() -> Palette<FireColor, f64> {
        Palette::from_iter(DEFAULT_RAMP)
    }

    /// The five ramp colors in slot order, slot-missing falls back to
    /// the classic default so an incomplete palette still renders.
    fn ramp_colors(&self) -> [Color<f64>; 5] {
        DEFAULT_RAMP.map(|(slot, fallback)| self.colors.get_or(&slot, fallback))
    }

    /// The `uRandomColor` palette: seed-hashed hue rotations of red at
    /// fixed alphas - BMW's `offsetHue` chain `red -> +0.1 -> +0.1`.
    fn random_colors(&self, ctx: &FxContext<'_>) -> [Color<f64>; 5] {
        let hue: f64 = ctx.seed.as_f64().hash();
        let base0 = crate::offset_hue(Color::<f64>::new(1.0, 0.0, 0.0), hue);
        let base1 = crate::offset_hue(base0, 0.1);
        let base2 = crate::offset_hue(base1, 0.1);
        [
            Color::<f64>::new_rgba(base0.r, base0.g, base0.b, 0.0),
            Color::<f64>::new_rgba(base0.r, base0.g, base0.b, 0.5),
            Color::<f64>::new_rgba(base1.r, base1.g, base1.b, 0.8),
            Color::<f64>::new_rgba(base1.r, base1.g, base1.b, 0.9),
            Color::<f64>::new_rgba(base2.r, base2.g, base2.b, 1.0),
        ]
    }

    /// The direction-aware burn direction handed to [`EffectMask::at`].
    fn bottom_to_top(&self, ctx: &FxContext<'_>) -> bool {
        match ctx.direction {
            crate::Direction::Open => self.opening_bottom_to_top,
            crate::Direction::Close => self.closing_bottom_to_top,
        }
    }
}

impl Default for Fire {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            movement_speed: Self::default_movement_speed(),
            random_color: false,
            noise_3d: false,
            closing_bottom_to_top: false,
            opening_bottom_to_top: false,
            colors: Self::default_colors(),
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
            default: 1.0,
        },
    },
    ParamSpec {
        key: "movement_speed",
        label: "Movement Speed",
        kind: ParamKind::Float {
            min: -2.0,
            max: 2.0,
            step: 0.1,
            default: 0.5,
        },
    },
    ParamSpec {
        key: "random_color",
        label: "Random Color",
        kind: ParamKind::Toggle { default: false },
    },
    ParamSpec {
        key: "noise_3d",
        label: "3D Noise",
        kind: ParamKind::Toggle { default: false },
    },
    ParamSpec {
        key: "closing_bottom_to_top",
        label: "Closing Bottom To Top",
        kind: ParamKind::Toggle { default: false },
    },
    ParamSpec {
        key: "opening_bottom_to_top",
        label: "Opening Bottom To Top",
        kind: ParamKind::Toggle { default: false },
    },
    ParamSpec {
        key: "colors.ember",
        label: "Ember",
        kind: ParamKind::Color { default: DEFAULT_RAMP[0].1 },
    },
    ParamSpec {
        key: "colors.red",
        label: "Red",
        kind: ParamKind::Color { default: DEFAULT_RAMP[1].1 },
    },
    ParamSpec {
        key: "colors.orange",
        label: "Orange",
        kind: ParamKind::Color { default: DEFAULT_RAMP[2].1 },
    },
    ParamSpec {
        key: "colors.yellow",
        label: "Yellow",
        kind: ParamKind::Color { default: DEFAULT_RAMP[3].1 },
    },
    ParamSpec {
        key: "colors.white",
        label: "White",
        kind: ParamKind::Color { default: DEFAULT_RAMP[4].1 },
    },
];

impl WindowEffect for Fire {
    fn nick(&self) -> &'static str {
        "fire"
    }

    fn label(&self) -> &'static str {
        "Fire"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        let mut values = vec![
            ParamValue::Float(self.scale.0),
            ParamValue::Float(self.movement_speed.0),
            ParamValue::Bool(self.random_color),
            ParamValue::Bool(self.noise_3d),
            ParamValue::Bool(self.closing_bottom_to_top),
            ParamValue::Bool(self.opening_bottom_to_top),
        ];
        values.extend(DEFAULT_RAMP.map(|(slot, fallback)| ParamValue::Color(self.colors.get_or(&slot, fallback))));
        values
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("fire param {index}"));
        let float = |index: usize| values.get(index).and_then(ParamValue::as_f64).ok_or(mismatch(index));
        let boolean = |index: usize| values.get(index).and_then(ParamValue::as_bool).ok_or(mismatch(index));
        let color = |index: usize| values.get(index).and_then(ParamValue::as_color).ok_or(mismatch(index));
        if values.len() != 11 {
            return Err(FxError::ParamMismatch(format!("fire takes 11 parameters, got {}", values.len())));
        }
        self.scale = ScaleFactor(float(0)?);
        self.movement_speed = Factor(float(1)?);
        self.random_color = boolean(2)?;
        self.noise_3d = boolean(3)?;
        self.closing_bottom_to_top = boolean(4)?;
        self.opening_bottom_to_top = boolean(5)?;
        for (index, (slot, _fallback)) in DEFAULT_RAMP.iter().enumerate() {
            self.colors.insert(*slot, color(6 + index)?);
        }
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let colors = if self.random_color { self.random_colors(ctx) } else { self.ramp_colors() };
        let drift = ctx.progress.as_f64() * ctx.duration.as_secs_f64() * self.movement_speed.0;
        for y in 0..size.y {
            for x in 0..size.x {
                let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
                let uv = px / size_px;
                // Noise domain: BMW's 400x600-normalized aspect-corrected uv.
                let mut n_uv = px / Vec2::<()>::new(400.0, 600.0) / self.scale.0;
                n_uv += ctx.seed.offset_2d();
                n_uv.y += drift;
                let mut noise = match self.noise_3d {
                    true => Simplex3DFractal.at(Vec3::<()>::new(n_uv.x * 4.0, n_uv.y * 4.0, drift * 1.5)),
                    false => Simplex2DFractal.at(n_uv * 4.0),
                };
                let mask = EffectMask::at(ctx, self.bottom_to_top(ctx), px, size_px);
                noise *= mask.effect;

                let fire = fire_ramp(noise, &colors);
                let mut window = ctx.window.sample_bilinear(uv);
                window.a *= mask.window;
                out.put_color(Pixels::new(x), Pixels::new(y), fire.alpha_over(&window));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::ramp::RAMP_STOPS;
    use super::super::ramp::fire_ramp;
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::BufferView;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn ctx_for<'a>(window: BufferView<'a, Rgba>, direction: Direction, progress: f64, seed: Seed) -> FxContext<'a> {
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_millis(1500),
            seed,
            direction,
            layer: Layer::Window,
            window,
            surface: None,
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, 32.0, 32.0),
        }
    }

    /// `fire_ramp` hits `colors[i]` exactly at each hard stop.
    #[test]
    fn fire_ramp_stops() {
        let colors = [
            Color::<f64>::new(0.1, 0.0, 0.0),
            Color::<f64>::new(0.3, 0.0, 0.0),
            Color::<f64>::new(0.5, 0.0, 0.0),
            Color::<f64>::new(0.7, 0.0, 0.0),
            Color::<f64>::new(0.9, 0.0, 0.0),
        ];
        for (i, &stop) in RAMP_STOPS.iter().enumerate() {
            assert_eq!(fire_ramp(stop, &colors), colors[i], "stop {i}");
        }
    }

    /// `window_mask` is monotonic along the burn direction during the
    /// burn phase; `effect_mask` is strongest near the burn edge. The
    /// 200x200 window keeps the sampled column inside the
    /// `EDGE_FADE = 70` interior.
    #[test]
    fn fire_mask_phases() {
        let data = vec![255u8; 200 * 200 * 4];
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(200, 200)).unwrap();
        let size_px = Vec2::<()>::new(200.0, 200.0);
        let ctx = ctx_for(window, Direction::Close, 0.2, Seed(1));
        // Burn runs top -> down: the window mask is non-decreasing
        // downward (top already burnt, bottom still intact).
        let mut prev = 0.0;
        let mut saw_transition = false;
        for y in 0..200 {
            let m = EffectMask::at(&ctx, false, Vec2::<()>::new(100.0, y as f64 + 0.5), size_px).window;
            assert!(m >= prev - 1e-9, "window mask must not decrease downward (y={y}, {prev} -> {m})");
            if m > 0.0 && m < 1.0 {
                saw_transition = true;
            }
            prev = m;
        }
        assert!(saw_transition, "burn front must produce a fade zone");
        // The effect mask peaks inside the burn region.
        let peak = (0..200)
            .map(|y| EffectMask::at(&ctx, false, Vec2::<()>::new(100.0, y as f64 + 0.5), size_px).effect)
            .fold(0.0, f64::max);
        assert!(peak > 0.5, "effect mask must peak at the burn edge, got {peak}");
    }

    /// Same seed + progress = identical frame; different seed differs
    /// (via the random-color path which hashes the seed).
    #[test]
    fn fire_deterministic() {
        let data = [200, 100, 50, 255].repeat(32 * 32);
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(32, 32)).unwrap();
        let mut out_a = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let mut out_b = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fire = Fire {
            random_color: true,
            ..Default::default()
        };

        fire.apply(&ctx_for(window, Direction::Close, 0.5, Seed(0xDEAD_BEEF_CAFE_0042)), &mut out_a.view_mut());
        fire.apply(&ctx_for(window, Direction::Close, 0.5, Seed(0xDEAD_BEEF_CAFE_0042)), &mut out_b.view_mut());
        assert_eq!(out_a.view().data(), out_b.view().data(), "same seed = same frame");

        fire.apply(&ctx_for(window, Direction::Close, 0.5, Seed(0x0123_4567_89AB_CDEF)), &mut out_b.view_mut());
        assert_ne!(out_a.view().data(), out_b.view().data(), "different seed differs");
    }
}
