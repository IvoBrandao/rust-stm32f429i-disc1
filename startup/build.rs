use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Make link.x available on the linker search path.
    fs::write(out.join("link.x"), include_bytes!("link.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // --nmagic: disable page alignment of sections (required when Flash/RAM
    // origins are not aligned to 0x10000, as is the case for STM32 devices).
    println!("cargo:rustc-link-arg=--nmagic");

    // Select our linker script.
    println!("cargo:rustc-link-arg=-Tlink.x");

    println!("cargo:rerun-if-changed=link.x");
}
