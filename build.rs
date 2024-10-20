use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Re-run the build script if these file change
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=src/asm/boot.S");
    println!("cargo:rerun-if-changed=src/asm/trap.S");
    println!("cargo:rerun-if-changed=src/asm/context_switch.S");

    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tmemory.x");

    cc::Build::new()
        .file("src/asm/boot.S")
        .file("src/asm/trap.S")
        .file("src/asm/context_switch.S")
        .compile("asm-boot-trap");
}
