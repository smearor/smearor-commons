use crate::Apparition;
use crate::AuraGlow;
use crate::BrokenGlass;
use crate::Crossfade;
use crate::Doom;
use crate::EnergizeA;
use crate::EnergizeB;
use crate::Fire;
use crate::FxError;
use crate::Glide;
use crate::Glitch;
use crate::Hexagon;
use crate::Incinerate;
use crate::Materialize;
use crate::Matrix;
use crate::Mushroom;
use crate::PaintBrush;
use crate::ParamValue;
use crate::PixelWheel;
use crate::PixelWipe;
use crate::Pixelate;
use crate::Portal;
use crate::RgbWarp;
use crate::Shredder;
use crate::Snap;
use crate::TeamRocket;
use crate::Trex;
use crate::Tv;
use crate::TvGlitch;
use crate::VenetianBlinds;
use crate::WindowEffect;
use crate::Wisps;
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
    /// Pixelated column melt (DOOM screen melt).
    Doom(Doom),
    /// Shake + suction + whirl dematerialization.
    Apparition(Apparition),
    /// Portal swirl + window travel.
    Portal(Portal),
    /// CRT collapse to the center line.
    Tv(Tv),
    /// CRT collapse overlaid with glitch rows.
    TvGlitch(TvGlitch),
    /// Row shear + interference + grain + RGB split.
    Glitch(Glitch),
    /// Turning/folding horizontal slats.
    VenetianBlinds(VenetianBlinds),
    /// Falling paper strips with torn edges.
    Shredder(Shredder),
    /// Quantized panel growth + tint reveal.
    Materialize(Materialize),
    /// Radial atom burst dissolve.
    EnergizeA(EnergizeA),
    /// Top-down shower + streak + atom dissolve.
    EnergizeB(EnergizeB),
    /// Vertical channel-split peel + brightness pulse. BMW's nick
    /// is the odd unhyphenated `rgbwarp`.
    #[serde(rename = "rgbwarp")]
    RgbWarp(RgbWarp),
    /// 8-bit scale + sparks + rays + star rings.
    Mushroom(Mushroom),
    /// Cell-dropout pixel dissolve.
    Pixelate(Pixelate),
    /// Radial pixel wipe from the pointer.
    PixelWipe(PixelWipe),
    /// Shrinking hexagonal tiles + glow lines.
    Hexagon(Hexagon),
    /// Fiery-edge burn with smoke + embers.
    Incinerate(Incinerate),
    /// Orbiting light dots + noise dissolve.
    Wisps(Wisps),
    /// Sparkle flare + off-center warp.
    TeamRocket(TeamRocket),
    /// Timed claw scratches + wound flash.
    Trex(Trex),
    /// Wind-blown dust disintegration.
    Snap(Snap),
    /// Brush-stroke sweep + drop shadow.
    PaintBrush(PaintBrush),
    /// Shattered shards + gravity drift.
    BrokenGlass(BrokenGlass),
    // Incinerate, Wisps, ... - appended by phase 4
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
            EffectSpec::Doom(Doom::default()),
            EffectSpec::Apparition(Apparition::default()),
            EffectSpec::Portal(Portal::default()),
            EffectSpec::Tv(Tv::default()),
            EffectSpec::TvGlitch(TvGlitch::default()),
            EffectSpec::Glitch(Glitch::default()),
            EffectSpec::VenetianBlinds(VenetianBlinds::default()),
            EffectSpec::Shredder(Shredder::default()),
            EffectSpec::Materialize(Materialize::default()),
            EffectSpec::EnergizeA(EnergizeA::default()),
            EffectSpec::EnergizeB(EnergizeB::default()),
            EffectSpec::RgbWarp(RgbWarp::default()),
            EffectSpec::Mushroom(Mushroom::default()),
            EffectSpec::Pixelate(Pixelate::default()),
            EffectSpec::PixelWipe(PixelWipe::default()),
            EffectSpec::Hexagon(Hexagon::default()),
            EffectSpec::Incinerate(Incinerate::default()),
            EffectSpec::Wisps(Wisps::default()),
            EffectSpec::TeamRocket(TeamRocket::default()),
            EffectSpec::Trex(Trex::default()),
            EffectSpec::Snap(Snap::default()),
            EffectSpec::PaintBrush(PaintBrush::default()),
            EffectSpec::BrokenGlass(BrokenGlass::default()),
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
            Self::Doom(effect) => effect.nick(),
            Self::Apparition(effect) => effect.nick(),
            Self::Portal(effect) => effect.nick(),
            Self::Tv(effect) => effect.nick(),
            Self::TvGlitch(effect) => effect.nick(),
            Self::Glitch(effect) => effect.nick(),
            Self::VenetianBlinds(effect) => effect.nick(),
            Self::Shredder(effect) => effect.nick(),
            Self::Materialize(effect) => effect.nick(),
            Self::EnergizeA(effect) => effect.nick(),
            Self::EnergizeB(effect) => effect.nick(),
            Self::RgbWarp(effect) => effect.nick(),
            Self::Mushroom(effect) => effect.nick(),
            Self::Pixelate(effect) => effect.nick(),
            Self::PixelWipe(effect) => effect.nick(),
            Self::Hexagon(effect) => effect.nick(),
            Self::Incinerate(effect) => effect.nick(),
            Self::Wisps(effect) => effect.nick(),
            Self::TeamRocket(effect) => effect.nick(),
            Self::Trex(effect) => effect.nick(),
            Self::Snap(effect) => effect.nick(),
            Self::PaintBrush(effect) => effect.nick(),
            Self::BrokenGlass(effect) => effect.nick(),
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
            Self::Doom(effect) => Box::new(*effect),
            Self::Apparition(effect) => Box::new(*effect),
            Self::Portal(effect) => Box::new(*effect),
            Self::Tv(effect) => Box::new(*effect),
            Self::TvGlitch(effect) => Box::new(*effect),
            Self::Glitch(effect) => Box::new(*effect),
            Self::VenetianBlinds(effect) => Box::new(*effect),
            Self::Shredder(effect) => Box::new(*effect),
            Self::Materialize(effect) => Box::new(*effect),
            Self::EnergizeA(effect) => Box::new(*effect),
            Self::EnergizeB(effect) => Box::new(*effect),
            Self::RgbWarp(effect) => Box::new(*effect),
            Self::Mushroom(effect) => Box::new(effect.clone()),
            Self::Pixelate(effect) => Box::new(*effect),
            Self::PixelWipe(effect) => Box::new(*effect),
            Self::Hexagon(effect) => Box::new(effect.clone()),
            Self::Incinerate(effect) => Box::new(effect.clone()),
            Self::Wisps(effect) => Box::new(effect.clone()),
            Self::TeamRocket(effect) => Box::new(*effect),
            Self::Trex(effect) => Box::new(*effect),
            Self::Snap(effect) => Box::new(*effect),
            Self::PaintBrush(effect) => Box::new(*effect),
            Self::BrokenGlass(effect) => Box::new(*effect),
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
            Self::Doom(effect) => effect.set_values(values),
            Self::Apparition(effect) => effect.set_values(values),
            Self::Portal(effect) => effect.set_values(values),
            Self::Tv(effect) => effect.set_values(values),
            Self::TvGlitch(effect) => effect.set_values(values),
            Self::Glitch(effect) => effect.set_values(values),
            Self::VenetianBlinds(effect) => effect.set_values(values),
            Self::Shredder(effect) => effect.set_values(values),
            Self::Materialize(effect) => effect.set_values(values),
            Self::EnergizeA(effect) => effect.set_values(values),
            Self::EnergizeB(effect) => effect.set_values(values),
            Self::RgbWarp(effect) => effect.set_values(values),
            Self::Mushroom(effect) => effect.set_values(values),
            Self::Pixelate(effect) => effect.set_values(values),
            Self::PixelWipe(effect) => effect.set_values(values),
            Self::Hexagon(effect) => effect.set_values(values),
            Self::Incinerate(effect) => effect.set_values(values),
            Self::Wisps(effect) => effect.set_values(values),
            Self::TeamRocket(effect) => effect.set_values(values),
            Self::Trex(effect) => effect.set_values(values),
            Self::Snap(effect) => effect.set_values(values),
            Self::PaintBrush(effect) => effect.set_values(values),
            Self::BrokenGlass(effect) => effect.set_values(values),
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

    /// `values()` ↔ `params()` length parity + `set_values(values())`
    /// is a no-op for every shipped spec.
    #[test]
    fn values_roundtrip_through_set_values() {
        for spec in EffectSpec::all() {
            let effect = spec.build();
            let values = effect.values();
            assert_eq!(values.len(), effect.params().len(), "{}: values/params length mismatch", spec.nick());
            let mut restored = spec.clone();
            restored
                .set_values(&values)
                .unwrap_or_else(|err| panic!("{}: set_values failed: {err}", spec.nick()));
            let after = serde_json::to_value(&restored).unwrap();
            let before = serde_json::to_value(&spec).unwrap();
            assert_eq!(before, after, "{}: set_values(values()) drifted", spec.nick());
        }
    }
}
