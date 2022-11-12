extern crate bindgen;

use std::env;
use std::path::{Path};

fn main() {
    if !std::env::var("TARGET").unwrap().contains("-linux") {
        return;
    }

    generate_bindings(env::var("OUT_DIR").expect("Out Dir variable not set"));
}

#[cfg(not(feature = "buildtime_bindgen"))]
fn generate_bindings(out_path: impl AsRef<Path>) {
    let in_path = env::var("AVAHI_SYS_BINDINGS_FILE").expect(
        "AVAHI_SYS_BINDINGS_FILE should be populated if buildtime_bindgen feature is not enabled",
    );

    std::fs::copy(in_path, out_path.as_ref().join("bindings.rs"))
        .expect("Failed to copy bindings to desintation");
}

#[cfg(feature = "buildtime_bindgen")]
fn generate_bindings(out_path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed=avahi-sys");
    println!("cargo:rerun-if-changed=wrapper.h");

    println!("cargo:rustc-link-lib=avahi-client");
    println!("cargo:rustc-link-lib=avahi-common");

    let mut builder = bindgen::Builder::default();

    if cfg!(feature = "verbose_build") {
        builder = builder.clang_arg("-v");
    }

    builder
        .header("wrapper.h")
        .ctypes_prefix("::libc")
        .size_t_is_usize(true)
        .bitfield_enum("AvahiClientFlags")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(out_path.as_ref().join("bindings.rs"))
        .expect("failed to write bindings to file");
}
