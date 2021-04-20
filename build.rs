extern crate bindgen;
use std::path::PathBuf;

fn main() {
    let src = [
        "src/raft_log.c",
        "src/raft_node.c",
        "src/raft_server.c",
        "src/raft_server_properties.c",
    ];

    let mut builder = cc::Build::new();

    let build = builder
        .files(src.iter())
        .include("include")
        .flag("-Wno-unused-parameter");

    build.compile("raft");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=include/wrapper.h");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/wrapper.h")
        // Tell cargo to invalidate the built crate whenever
        // any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    //Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
