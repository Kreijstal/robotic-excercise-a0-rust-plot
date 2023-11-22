extern crate bindgen;
use std::path::PathBuf;
use copy_to_output::copy_to_output;  
use std::env;
fn main() {
    // Tell Cargo to tell rustc to link the shared library.
    // Adjust the names as needed.
    println!("cargo:rustc-link-lib=dylib=SpringMassWrapper");
    println!("cargo:rustc-link-search=native=./res");  // specify the directory where the library resides
    // Re-runs script if any files in res are changed  
    println!("cargo:rerun-if-changed=res/*");  
    copy_to_output("res", &env::var("PROFILE").unwrap()).expect("Could not copy");  

    // The bindgen::Builder is the main entry point to bindgen, and lets you build up
    // options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("src/SpringMassWrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Since $OUT_DIR is not to be used, specify where you'd like to save the bindings.
    // You can adjust "target" and "bindings.rs" as needed.
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set!"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
