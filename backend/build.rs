use std::path::Path;
use std::process::{Command, Stdio};

const UI_DIR: &str = "../ui";

fn main() {
    println!("cargo:rerun-if-changed={UI_DIR}/src");
    println!("cargo:rerun-if-changed={UI_DIR}/Cargo.toml");
    println!("cargo:rerun-if-changed={UI_DIR}/index.html");
    build_frontend(UI_DIR);
}

fn build_frontend<P: AsRef<Path>>(source: P) {
    Command::new("trunk")
        .args(["build", "--dist", "../backend/dist"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(source.as_ref())
        .status()
        .expect("Failed to build Frontend");
}
