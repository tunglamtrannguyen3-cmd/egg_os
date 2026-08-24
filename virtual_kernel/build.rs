use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-search={}", manifest_dir.display());
    println!("cargo:rerun-if-changed=linker_x86_64.ld");
}
