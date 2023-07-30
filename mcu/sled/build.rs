use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ch32v003/memory.x");

    // cargo feature `ch32v003`
    if let Ok(_) = env::var("CARGO_FEATURE_CH32V003") {
        println!("cargo:rustc-link-arg=-Tch32v003/memory.x");
        println!("cargo:rustc-link-arg=-Tlink.x");
    }
}
