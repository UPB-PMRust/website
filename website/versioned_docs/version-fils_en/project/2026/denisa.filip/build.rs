fn main() {
    // Tell Cargo to re-run this build script if the linker script changes.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
