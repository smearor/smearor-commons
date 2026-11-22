//! Effect code generator: reads an effect JSON file - saved by the
//! `geometry`, `color` or `palette` examples - and writes the
//! equivalent Rust constructor code to stdout (or a second file
//! argument).
//!
//! `cargo run -p smearor-commons-animation --example effectgen -- effect.json > effect.rs`

use std::env;
use std::fs;
use std::process::ExitCode;

mod common;

use common::emit_effect_rust;
use common::load_effect;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(input) = args.next() else {
        eprintln!("usage: effectgen <effect.json> [out.rs]");
        return ExitCode::FAILURE;
    };
    let Some(effect) = load_effect(&input) else {
        return ExitCode::FAILURE;
    };
    let rust = emit_effect_rust(&effect);
    match args.next() {
        Some(output) => match fs::write(&output, &rust) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("write {output} failed: {error}");
                ExitCode::FAILURE
            }
        },
        None => {
            print!("{rust}");
            ExitCode::SUCCESS
        }
    }
}
