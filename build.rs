// Copies memory.x to OUT_DIR so the linker can find it when startup/link.x
// does `INCLUDE memory.x`.  The --nmagic and -Tlink.x flags are emitted by
// the startup crate's own build script.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
