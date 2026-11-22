use crate::AuraGlow;
use crate::Crossfade;
use crate::Fire;
use crate::FxError;
use crate::Glide;
use crate::Matrix;
use crate::ParamValue;
use crate::PixelWheel;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;

/// A configured effect - one variant per implemented effect.
///
/// Serializes as `{ "effect": "crossfade", ... }`: each variant's
/// payload **is** the effect struct (params as fields, serde derives
/// on the struct itself), so one definition serves runtime, JSON and
/// codegen.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "effect", rename_all = "kebab-case")]
pub enum EffectSpec {
    /// The reference effect - plain cross-fade.
    Crossfade(Crossfade),
    /// UV-transform glide - scale, squish, tilt, shift + dissolve.
    Glide(Glide),
    /// Pixelation + angular spoke wipe.
    PixelWheel(PixelWheel),
    /// Fractal-noise burn with a 5-stop flame ramp.
    Fire(Fire),
    /// Glyph rain + trail composite (Matrix effect).
    Matrix(Matrix),
    /// Radial blur + cos-palette glow + superellipse crop.
    AuraGlow(AuraGlow),
    // Doom, Incinerate, ... - appended by phase 4
}

impl EffectSpec {
    /// One default-configured instance of every shipped spec - the
    /// dropdown's source and `by_nick`'s search space.
    pub fn all() -> Vec<EffectSpec> {
        vec![
            EffectSpec::Crossfade(Crossfade),
            EffectSpec::Glide(Glide::default()),
            EffectSpec::PixelWheel(PixelWheel::default()),
            EffectSpec::Fire(Fire::default()),
            EffectSpec::Matrix(Matrix::default()),
            EffectSpec::AuraGlow(AuraGlow::default()),
        ]
    }

    /// Finds + returns the default spec named by `nick` - `Err` on
    /// unknown `"effect"` tags, giving consumers a typed error
    /// instead of serde's raw one.
    pub fn by_nick(nick: &str) -> Result<EffectSpec, FxError> {
        Self::all()
            .into_iter()
            .find(|spec| spec.nick() == nick)
            .ok_or_else(|| FxError::UnknownEffect(nick.to_string()))
    }

    /// The spec's stable identifier (`"effect"` tag).
    pub fn nick(&self) -> &'static str {
        match self {
            Self::Crossfade(effect) => effect.nick(),
            Self::Glide(effect) => effect.nick(),
            Self::PixelWheel(effect) => effect.nick(),
            Self::Fire(effect) => effect.nick(),
            Self::Matrix(effect) => effect.nick(),
            Self::AuraGlow(effect) => effect.nick(),
        }
    }

    /// The effect object carrying these parameters.
    pub fn build(&self) -> Box<dyn WindowEffect> {
        match self {
            Self::Crossfade(effect) => Box::new(*effect),
            Self::Glide(effect) => Box::new(*effect),
            Self::PixelWheel(effect) => Box::new(*effect),
            Self::Fire(effect) => Box::new(effect.clone()),
            Self::Matrix(effect) => Box::new(effect.clone()),
            Self::AuraGlow(effect) => Box::new(*effect),
        }
    }

    /// Applies live `values` (in `params()` order) onto the payload's
    /// typed fields - the runtime twin of [`WindowEffect::params`].
    pub fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        match self {
            Self::Crossfade(effect) => effect.set_values(values),
            Self::Glide(effect) => effect.set_values(values),
            Self::PixelWheel(effect) => effect.set_values(values),
            Self::Fire(effect) => effect.set_values(values),
            Self::Matrix(effect) => effect.set_values(values),
            Self::AuraGlow(effect) => effect.set_values(values),
        }
    }

    /// Default spec named by `nick` with `values` applied - what a
    /// parameter panel produces for serialization.
    pub fn from_params(nick: &str, values: &[ParamValue]) -> Result<EffectSpec, FxError> {
        let mut spec = Self::by_nick(nick)?;
        spec.set_values(values)?;
        Ok(spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_roundtrip_is_the_identity() {
        let spec = EffectSpec::by_nick("crossfade").unwrap();
        let json = serde_json::to_string(&spec).unwrap();
        assert_eq!(json, r#"{"effect":"crossfade"}"#);
        let back: EffectSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nick(), "crossfade");
    }

    #[test]
    fn by_nick_resolves_all_shipped_effects() {
        for spec in EffectSpec::all() {
            let resolved = EffectSpec::by_nick(spec.nick()).unwrap();
            assert_eq!(resolved.nick(), spec.nick());
        }
    }

    #[test]
    fn by_nick_rejects_unknown_tags() {
        match EffectSpec::by_nick("napalm") {
            Err(FxError::UnknownEffect(tag)) => assert_eq!(tag, "napalm"),
            other => panic!("expected UnknownEffect, got {other:?}"),
        }
    }

    #[test]
    fn params_match_the_serde_payload() {
        // The param-drift guard: every ParamSpec::key must be a field
        // name in the effect's serialized form.
        for spec in EffectSpec::all() {
            let effect = spec.build();
            let json = serde_json::to_value(&spec).unwrap();
            for param in effect.params() {
                // Dotted keys address nested objects - `colors.ember`
                // resolves `{"colors": {"ember": ...}}`.
                let mut node = &json;
                let mut found = true;
                for segment in param.key.split('.') {
                    match node.get(segment) {
                        Some(child) => node = child,
                        None => {
                            found = false;
                            break;
                        }
                    }
                }
                assert!(found, "{}: param `{}` does not resolve in the serde payload", spec.nick(), param.key);
            }
            let _ = effect.values(); // every effect reports values
        }
    }
}
