use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/Users/zak/repos/oxidised-fusion-360-addin/adsk/lib");
    // println!("cargo:rustc-link-search=/Users/zak/Library/Application Support/Autodesk/Autodesk Fusion 360/API/CPP/lib/core");

    println!("cargo:rustc-link-lib=core");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header("wrapper.h")
        .header("/Users/zak/repos/oxidised-fusion-360-addin/adsk/include/Core/CoreAll.h")
        .clang_arg("-I/Users/zak/repos/oxidised-fusion-360-addin/adsk/include")
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        // .opaque_type("(::)?std::.*")
        .opaque_type("std::.*")
        .opaque_type("adsk::core::Iterator")
        .blocklist_type("rep")
        .blocklist_type("char_type")
        .blocklist_item("std::value")
        .generate_inline_functions(true)
        .derive_default(true)
        // .allowlist_type("adsk::core::Application")
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
}
