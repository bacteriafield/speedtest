use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap_or_else(|_| panic!("failed to run git"));

    let commit = String::from_utf8(output.stdout).unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=GIT_COMMIT={}", commit.trim());
}
