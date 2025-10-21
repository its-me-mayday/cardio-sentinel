// build.rs
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Prova a leggere la short git SHA; fallback a "unknown"
    let git_sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=GIT_SHA={}", git_sha);

    // Epoch seconds al build-time (UTC)
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("cargo:rustc-env=BUILD_TIME_UNIX={}", secs);

    // Rirun se cambia HEAD
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");
}