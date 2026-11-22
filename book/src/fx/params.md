# Params

`ParamSpec`/`ParamKind`/`ParamValue` form the parameter metadata
model - one declaration drives GTK control rows, serde defaults and
codegen:

```rust,ignore
pub struct ParamSpec {
    pub key: &'static str,      // serde/JSON key, e.g. "scale"
    pub label: &'static str,    // control label, e.g. "Scale"
    pub kind: ParamKind,
}
```

| `ParamKind` | Widget (phase-3 experimenter) | GSettings analogue |
|---|---|---|
| `Float { min, max, step, default }` | `Scale` | `d` |
| `Int { min, max, default }` | `SpinButton` | `i` |
| `Toggle { default }` | `CheckButton` | `b` |
| `Color { default }` | `ColorDialogButton` | `s` `rgba(...)` |
| `Choice { options, default }` | `DropDown` | - |

`Color<f64>` serializes as a string (`"rgba(76, 51, 25, 0.0)"`,
`#fc0`) - byte-identical to BMW's schema color keys.

`ParamValue` is the untagged runtime twin: `values()` reads the
effect's live parameters in `params()` order, `set_values()` applies
them back, and `EffectSpec::from_params(nick, values)` builds a typed
spec from a panel's row output. Kind/length mismatches surface as
`FxError::ParamMismatch` - never a panic.
