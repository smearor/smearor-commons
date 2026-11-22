use crate::FxContext;
use crate::FxError;
use crate::Layers;
use crate::ParamSpec;
use crate::ParamValue;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;

/// A per-pixel effect over frozen textures - the analogue of one
/// Burn-My-Windows shader.
///
/// Implementors are plain serde structs (the `EffectSpec` variant
/// payloads); the trait adds identity, parameter metadata and the
/// frame application.
pub trait WindowEffect {
    /// Stable serialized identifier, e.g. `"fire"` - the spec's
    /// `"effect"` tag.
    fn nick(&self) -> &'static str;

    /// Human label, e.g. `"Fire"`.
    fn label(&self) -> &'static str;

    /// The layers this effect renders - [`Layers::SurfaceAndWindow`]
    /// makes the consumer run two passes and supply `ctx.surface`.
    fn layers(&self) -> Layers {
        Layers::WindowOnly
    }

    /// Parameter metadata - drives UI rows, serde defaults, codegen.
    fn params(&self) -> &'static [ParamSpec];

    /// The live parameter values in `params()` order - the
    /// serialization and UI currency.
    fn values(&self) -> Vec<ParamValue>;

    /// Applies `values` (in `params()` order) onto the typed fields;
    /// length/kind mismatches yield [`FxError::ParamMismatch`].
    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError>;

    /// Pre-flight check for a pass: `Err(MissingSurface)` when this
    /// is a two-pass effect and `ctx.surface` is `None`.
    fn check(&self, ctx: &FxContext<'_>) -> Result<(), FxError> {
        match (self.layers(), ctx.surface.is_some()) {
            (Layers::SurfaceAndWindow, false) => Err(FxError::MissingSurface),
            _ => Ok(()),
        }
    }

    /// Render one frame: read `ctx`, write `out` fully (every pixel).
    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::BufferView;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    /// A minimal two-pass effect for the context check.
    struct DummyLayered;

    impl WindowEffect for DummyLayered {
        fn nick(&self) -> &'static str {
            "dummy-layered"
        }

        fn label(&self) -> &'static str {
            "Dummy Layered"
        }

        fn layers(&self) -> Layers {
            Layers::SurfaceAndWindow
        }

        fn params(&self) -> &'static [ParamSpec] {
            &[]
        }

        fn values(&self) -> Vec<ParamValue> {
            Vec::new()
        }

        fn set_values(&mut self, _values: &[ParamValue]) -> Result<(), FxError> {
            Ok(())
        }

        fn apply(&self, _ctx: &FxContext<'_>, _out: &mut BufferViewMut<'_, Rgba>) {}
    }

    /// A `SurfaceAndWindow` effect surfaces `MissingSurface` when
    /// `ctx.surface` is `None`, and passes `check` when it is `Some`.
    #[test]
    fn layered_context_requires_surface() {
        let data = [0u8; 4];
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(1, 1)).unwrap();
        let ctx = FxContext {
            progress: Fraction::ZERO,
            duration: Duration::ZERO,
            seed: Seed(0),
            direction: Direction::Open,
            layer: Layer::Window,
            window,
            surface: None,
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, 1.0, 1.0),
        };
        assert!(matches!(DummyLayered.check(&ctx), Err(FxError::MissingSurface)));

        let ctx = FxContext { surface: Some(window), ..ctx };
        assert!(DummyLayered.check(&ctx).is_ok());
    }
}
