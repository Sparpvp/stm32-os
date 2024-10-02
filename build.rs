use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Re-run the build script if memory.x or boot.S changes
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=src/asm/boot.S");

    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tmemory.x");

    // Compile boot.S using the cc crate
    cc::Build::new().file("src/asm/boot.S").compile("boot");
}
