fn main() {
    println!("cargo:rerun-if-changed=linker_x86_64.ld");
    println!("cargo:rerun-if-changed=linker_aarch64.ld");
    println!("cargo:rerun-if-changed=src/arch/x86_64/boot.s");
    println!("cargo:rerun-if-changed=src/arch/aarch64/boot.s");
}

