use std::path::PathBuf;

fn main() {
    let mut include_paths: Vec<PathBuf> = Vec::new();

    try_pkg_config(&mut include_paths);

    generate_bindings(include_paths);
}

/// Use pkg-config to find system liblo
fn try_pkg_config(include_paths: &mut Vec<PathBuf>) {
    // Probe system liblo via pkg-config
    let library = pkg_config::Config::new()
        .atleast_version("0.31")
        .probe("liblo")
        .expect("Could not find system liblo via pkg-config. Please install liblo.");

    // Add include paths from pkg-config for bindgen
    include_paths.extend(library.include_paths);
}

/// Generate bindings using bindgen
fn generate_bindings(include_paths: Vec<PathBuf>) {
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-DHAVE_CONFIG_H=1")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .raw_line(
            "#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]",
        )
        .blocklist_item("IPPORT_RESERVED");

    // Provide the necessary include paths to bindgen's internal clang instance
    for path in include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_file = PathBuf::from("src/lo_sys.rs");
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
