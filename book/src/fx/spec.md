# Spec

`EffectSpec` is the serde-tagged dispatch root - one variant per
shipped effect, the variant payload **is** the effect struct:

```rust,ignore
#[serde(tag = "effect", rename_all = "kebab-case")]
pub enum EffectSpec {
    Crossfade(Crossfade),
    // Fire(Fire), Glide(Glide), ...
}
```

So `{"effect":"crossfade"}` is the complete serialized form, and one
struct definition serves runtime, JSON and `emit` codegen.

```rust,ignore
let spec = EffectSpec::by_nick("crossfade")?;   // default config
let mut effect = spec.build();                  // Box<dyn WindowEffect>
effect.check(&ctx)?;                            // MissingSurface pre-flight
effect.apply(&ctx, &mut out.view_mut());        // renders one frame
```

- `EffectSpec::all()` - one default-configured instance of every
  shipped spec (the dropdown's source)
- `EffectSpec::by_nick(nick)` - `Err(FxError::UnknownEffect)` on
  unknown `"effect"` tags, a typed error instead of serde's raw one
- `EffectSpec::from_params(nick, values)` - spec construction from a
  parameter panel's row output
