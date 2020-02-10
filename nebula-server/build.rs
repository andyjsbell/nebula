
use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    
    // // Tell cargo to tell rustc to link the system bzip2
    // // shared library.
    // println!("cargo:rustc-link-lib=avcodec");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/*.h");
    println!("cargo:rerun-if-changed=src/*.cpp");

    // Link path and libs - TODO need to parametise this
    println!(r"cargo:rustc-link-search=C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\atlmfc\lib\x64");
    println!(r"cargo:rustc-link-search=C:\Users\andyb\Downloads\ffmpeg-4.2.2-win64-dev\lib");
    println!(r"cargo:rustc-link-lib=D3D11");
    println!(r"cargo:rustc-link-lib=dxgi");
    println!(r"cargo:rustc-link-lib=gdiplus");
    println!(r"cargo:rustc-link-lib=uuid");
    println!(r"cargo:rustc-link-lib=mfplat");
    println!(r"cargo:rustc-link-lib=mfuuid");
    
    // Include path - TODO need to parametise this
    let atl_path = Path::new(r"C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\atlmfc\include");
    
    // Build Nebula
    cc::Build::new()
        .file("src/Nebula.cpp")
        .file("src/DXGIManager.cpp")
        .include(atl_path)
        .compile("nebula");
}
