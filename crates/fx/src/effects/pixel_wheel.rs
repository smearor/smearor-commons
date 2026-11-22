use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::SpokeCount;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use std::f64::consts::PI;

/// The window pixelates progressively while angular spokes hide it
/// section by section - a wheel-wipe driven pixelation.
///
/// Behavioral spec (source shader: `pixel-wheel.frag`, re-implemented
/// from semantics): the smoothstepped direction-weighted progress
/// grows a pixel-block grid `ceil(pixel_size * p + 1)`; each cell
/// samples its center texel. The spoke mask is the angle to the
/// downward direction mirrored at x = 0; a spoke section vanishes
/// once progress passes `mod(angle * spoke_count, 1)` - sections die
/// staggered around the wheel.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PixelWheel {
    /// Angular segment count - schema `pixel-wheel-spoke-count`, default `12`.
    #[serde(default = "PixelWheel::default_spoke_count")]
    pub spoke_count: SpokeCount,
    /// Pixel-block size at full progress - schema `pixel-wheel-pixel-size`, default `10.0`.
    #[serde(default = "PixelWheel::default_pixel_size")]
    pub pixel_size: Pixels<f64>,
}

impl PixelWheel {
    const fn default_spoke_count() -> SpokeCount {
        SpokeCount::new(12)
    }
    const fn default_pixel_size() -> Pixels<f64> {
        Pixels::new(10.0)
    }
}

impl Default for PixelWheel {
    fn default() -> Self {
        Self {
            spoke_count: Self::default_spoke_count(),
            pixel_size: Self::default_pixel_size(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "spoke_count",
        label: "Spoke Count",
        kind: ParamKind::Int { min: 1, max: 64, default: 12 },
    },
    ParamSpec {
        key: "pixel_size",
        label: "Pixel Size",
        kind: ParamKind::Float {
            min: 1.0,
            max: 64.0,
            step: 1.0,
            default: 10.0,
        },
    },
];

impl WindowEffect for PixelWheel {
    fn nick(&self) -> &'static str {
        "pixel-wheel"
    }

    fn label(&self) -> &'static str {
        "Pixel Wheel"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Int(self.spoke_count.0 as i64), ParamValue::Float(self.pixel_size.0)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("pixel-wheel param {index}"));
        self.spoke_count = SpokeCount(values.first().and_then(ParamValue::as_i64).ok_or(mismatch(0))?.max(0) as u32);
        self.pixel_size = Pixels::new(values.get(1).and_then(ParamValue::as_f64).ok_or(mismatch(1))?);
        if values.len() > 2 {
            return Err(FxError::ParamMismatch(format!("pixel-wheel takes 2 parameters, got {}", values.len())));
        }
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        // Progress is smoothstepped and direction-weighted.
        let raw = match ctx.direction {
            crate::Direction::Open => 1.0 - ctx.progress.as_f64(),
            crate::Direction::Close => ctx.progress.as_f64(),
        };
        let progress = crate::smoothstep(0.0, 1.0, raw);
        let size = ctx.window.size();
        let pixel_size = (self.pixel_size.0 * progress + 1.0).ceil();
        let grid = Vec2::<()>::new(pixel_size / size.x as f64, pixel_size / size.y as f64);
        let (w, h) = (size.x as f64, size.y as f64);
        let spokes = self.spoke_count.0 as f64;
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            // Cell-center uv - the whole block samples one texel.
            let texcoord = Vec2::<()>::new(uv.x - (uv.x % grid.x) + grid.x * 0.5, uv.y - (uv.y % grid.y) + grid.y * 0.5);
            let mut c = ctx.window.sample_bilinear(texcoord);
            // Spoke mask: angle to the downward direction, mirrored at x = 0.
            let dir = (texcoord - Vec2::<()>::new(0.5, 0.5)).normalized();
            let mut angle = 0.5 * dir.y.clamp(-1.0, 1.0).acos() / PI;
            if dir.x < 0.0 {
                angle = 1.0 - angle;
            }
            let threshold = crate::fract(angle * spokes);
            if progress > threshold {
                c.a = 0.0;
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
    use smearor_commons_buffer::BufferView;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn ctx_for<'a>(window: BufferView<'a, Rgba>, progress: f64) -> FxContext<'a> {
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_millis(300),
            seed: Seed(1),
            direction: Direction::Close,
            layer: Layer::Window,
            window,
            surface: None,
            assets: crate::FxAssets::default(),
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, 32.0, 32.0),
        }
    }

    /// At full progress every spoke is gone; at 0 progress nothing is
    /// hidden (and the pixel grid is 1px - the identity sample).
    #[test]
    fn pixel_wheel_endpoints() {
        let data = [10, 20, 30, 255].repeat(32 * 32);
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(32, 32)).unwrap();
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));

        PixelWheel::default().apply(&ctx_for(window, 0.0), &mut out.view_mut());
        assert_eq!(out.view().data(), &data[..], "progress 0: identity");

        PixelWheel::default().apply(&ctx_for(window, 1.0), &mut out.view_mut());
        assert!(out.view().data().iter().skip(3).step_by(4).all(|a| *a == 0), "progress 1: all hidden");
    }

    /// At a mid progress, texels inside one pixel block sample the
    /// identical (cell-center) texel.
    #[test]
    fn pixel_wheel_pixelated() {
        let mut data = vec![0u8; 32 * 32 * 4];
        for y in 0..32 {
            for x in 0..32 {
                let o = (y * 32 + x) * 4;
                data[o] = x as u8;
                data[o + 1] = y as u8;
                data[o + 3] = 255;
            }
        }
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(32, 32)).unwrap();
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        PixelWheel::default().apply(&ctx_for(window, 0.5), &mut out.view_mut());
        // The pixel blocks are larger than one texel: neighbors inside
        // a block must be identical where the block is opaque.
        let d = out.view().data();
        let mut saw_opaque_block = false;
        for y in 0..31 {
            for x in 0..31 {
                let a = (y * 32 + x) * 4;
                let b = (y * 32 + x + 1) * 4;
                if d[a + 3] > 0 && d[b + 3] > 0 && d[a] == d[b] && d[a + 1] == d[b + 1] {
                    saw_opaque_block = true;
                }
            }
        }
        assert!(saw_opaque_block, "no equal opaque neighbor pairs found - pixelation missing");
    }

    /// `spoke_count` dark arc segments appear on a ring around the
    /// center at mid progress.
    #[test]
    fn pixel_wheel_spokes() {
        let data = [10, 20, 30, 255].repeat(64 * 64);
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(64, 64)).unwrap();
        let ctx = FxContext {
            window_rect: Rect::from_xywh(0.0, 0.0, 64.0, 64.0),
            ..ctx_for(window, 0.5)
        };
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        // Minimal pixelation so the ~15-degree spoke gaps stay resolvable.
        PixelWheel {
            pixel_size: Pixels::new(1.0),
            ..Default::default()
        }
        .apply(&ctx, &mut out.view_mut());
        let d = out.view().data();
        let alpha_at = |angle: f64| {
            let x = (32.0 + 20.0 * angle.cos()) as u32;
            let y = (32.0 + 20.0 * angle.sin()) as u32;
            d[((y * 64 + x) * 4 + 3) as usize]
        };
        // Count runs of dark samples around a full circle: each of the
        // 12 spokes contributes one dark arc.
        let n = 360;
        let mut dark_runs = 0;
        let mut in_dark = false;
        for i in 0..=n {
            let dark = alpha_at(i as f64 * 2.0 * PI / n as f64) == 0;
            if dark && !in_dark {
                dark_runs += 1;
            }
            in_dark = dark;
        }
        assert_eq!(dark_runs, 12, "expected 12 dark spoke arcs");
    }
}
