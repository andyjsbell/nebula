
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    
    println!(r"cargo:rustc-link-search=C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\atlmfc\lib\x64");
    println!(r"cargo:rustc-link-search=C:\Users\andyb\Downloads\ffmpeg-4.2.2-win64-dev\lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=avcodec");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(r"-IC:\Users\andyb\Downloads\ffmpeg-4.2.2-win64-dev\include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let atl_path = Path::new(r"C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\atlmfc\include");
    
    // Build Nebula
    cc::Build::new()
        .file("../Nebula.cpp")
        .include(atl_path)
        .compile("nebula");
}
