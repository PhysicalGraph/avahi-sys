extern crate bindgen;

use std::{env, path::Path};

#[cfg(feature = "buildtime_bindgen")]
#[derive(Debug)]
enum Error {
    Bindgen(bindgen::BindgenError),
    IO(std::io::Error),
}

fn main() {
    if !env::var("TARGET").unwrap().contains("-linux") {
        return;
    }

    println!("cargo:rustc-link-lib=avahi-client");
    println!("cargo:rustc-link-lib=avahi-common");

    let out_path =
        Path::new(&env::var("OUT_DIR").expect("Out Dir variable not set")).join("bindings.rs");

    if let Ok(in_path) = env::var("AVAHI_SYS_BINDINGS_FILE") {
        if let Err(e) = std::fs::copy(in_path, &out_path) {
            eprintln!("Failed to copy bindings to destination: {e:?}");
        } else {
            return;
        }
    }

    #[cfg(feature = "buildtime_bindgen")]
    if let Err(e) = generate_bindings(out_path) {
        eprintln!("Failed to generate bindings. Error: {e:?}");
    } else {
        return;
    }

    panic!("Failed to find bindings. Set AVAHI_SYS_BINDINGS_FILE to override binding generation");
}

#[cfg(feature = "buildtime_bindgen")]
fn generate_bindings(out_path: impl AsRef<Path>) -> Result<(), Error> {
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default();

    if cfg!(feature = "verbose_build") {
        builder = builder.clang_arg("-v");
    }

    let bindings = builder
        .header("wrapper.h")
        .ctypes_prefix("::libc")
        .size_t_is_usize(true)
        .bitfield_enum("AvahiClientFlags")
        .generate()
        .map_err(Error::Bindgen)?;

    bindings.write_to_file(out_path).map_err(Error::IO)?;

    Ok(())
}
