use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    
    // Pass the directory containing linker_x86_64.ld to the linker search path
    println!("cargo:rustc-link-search={}", manifest_dir.display());
    println!("cargo:rerun-if-changed=linker_x86_64.ld");
}
