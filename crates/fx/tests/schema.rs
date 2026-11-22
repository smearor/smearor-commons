//! Schema export contract (phase 6 `schemars`): the emitted
//! `EffectSpec` schema covers every shipped nick.

use smearor_commons_fx::EffectSpec;

/// `schema_covers_all` from the phase-6 test plan: every registered
/// nick must appear in the exported schema document - catches a
/// variant that serializes but never landed in `all()`, or a renamed
/// nick that drifted from the schema.
#[test]
fn schema_covers_all() {
    let schema = serde_json::to_value(schemars::schema_for!(EffectSpec)).unwrap();
    let text = schema.to_string();
    for spec in EffectSpec::all() {
        let nick = spec.nick();
        assert!(text.contains(&format!("\"{nick}\"")), "nick {nick} missing from schema");
    }
}
