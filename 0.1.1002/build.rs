use std::env;
use std::fs;
use std::path::PathBuf;

const VAR: &str = "LOCKED_TRIPWIRE_MESSAGE";

fn main() {
    println!("cargo::rerun-if-env-changed={VAR}");

    let message = match env::var(VAR) {
        Ok(custom) => custom,
        Err(_) => "\
This binary does not support being installed without --locked. To install, run:

cargo install --locked <binary>"
            .to_string(),
    };

    // NOTE: For failure we just give the message, otherwise it would be confusing.

    // cargo always sets `OUT_DIR`, so this shouldn't fail
    let out_dir = env::var("OUT_DIR").expect(&message);
    let out_path = PathBuf::from(out_dir).join("message.txt");

    fs::write(out_path, message.as_bytes()).expect(&message);
}
