use std::path::Path;

fn main() {
    
    let tool = cc::windows_registry::find_tool("msvc", "devenv").unwrap();
    let p = tool.path().parent().unwrap().parent().unwrap().parent().unwrap();
    let mut b = p.to_path_buf();
    b.push("VC");
    b.push("Tools");
    b.push("MSVC");
    b.push("14.16.27023");
    b.push("atlmfc");
    
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=avcodec");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/*.h");
    println!("cargo:rerun-if-changed=src/*.cpp");

    // Link path and libs
    print!("cargo:rustc-link-search=");
    print!("{}", b.to_str().unwrap());
    println!("\\lib\\x64");
    
    println!(r"cargo:rustc-link-search=C:\Users\andyb\Downloads\ffmpeg-4.2.2-win64-dev\lib");
    println!(r"cargo:rustc-link-lib=D3D11");
    println!(r"cargo:rustc-link-lib=dxgi");
    println!(r"cargo:rustc-link-lib=gdiplus");
    println!(r"cargo:rustc-link-lib=uuid");
    println!(r"cargo:rustc-link-lib=mfplat");
    println!(r"cargo:rustc-link-lib=mfuuid");
    
    // Include path
    b.push("include");
    
    let atl_path = Path::new(b.to_str().unwrap());
    
    // Build Nebula
    cc::Build::new()
        .file("src/Nebula.cpp")
        .file("src/DXGIManager.cpp")
        .include(atl_path)
        .compile("nebula");
}
