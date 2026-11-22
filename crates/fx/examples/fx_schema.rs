//! Emits the JSON Schema of the effect model:
//!
//! ```sh
//! cargo run -p smearor-commons-fx --features schemars \
//!     --example fx_schema > effect.schema.json
//! ```
//!
//! The exported document is the `EffectSpec` root schema - every
//! shipped effect struct lands in `$defs`, so the file validates
//! `{ "effect": "<nick>", ... }` payloads and drives editors,
//! config checks, and docs generation.

use smearor_commons_fx::EffectSpec;

fn main() {
    let schema = schemars::schema_for!(EffectSpec);
    match serde_json::to_string_pretty(&schema) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("schema serialization failed: {e}"),
    }
}
