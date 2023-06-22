use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn get_commit_hash() -> String {
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    git_hash
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("$OUT_DIR enn var is not set! Please set it.");
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    let git_hash = get_commit_hash();

    let generated = format!(
        r#"
            pub const CARGO_PKG_VERSION: &'static str = "{version}";
            pub const GIT_HASH: &'static str = "{git_hash}";
        "#
    );

    let out_path = PathBuf::from(format!("{}/generated.rs", out_dir));

    // Write the parsed grid to $OUT_DIR/generated.rs:
    fs::write(out_path.join("generated.rs"), &generated).expect("Couldn't write generated.rs file!");
}
