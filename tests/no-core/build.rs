#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if is_nightly() {
        println!("cargo:rustc-cfg=nightly");
    }
}

fn is_nightly() -> bool {
    env::var_os("RUSTC")
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map_or(false, |version| version.contains("nightly") || version.contains("dev"))
}
