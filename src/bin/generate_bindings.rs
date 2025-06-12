use std::path::PathBuf;

/// Regenerates the bindings file `src/lo_sys.rs`.
fn main() {
    // Probe system liblo via pkg-config to find include paths.
    let library = pkg_config::Config::new()
        .atleast_version("0.32")
        .probe("liblo")
        .expect("Could not find system liblo via pkg-config. Please install liblo.");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-DHAVE_CONFIG_H=1")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .raw_line(
            "#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]",
        )
        .blocklist_item("IPPORT_RESERVED");

    // Add the include paths from pkg-config to bindgen's clang arguments.
    for path in &library.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    println!("Generating bindings for liblo...");
    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the `src` directory.
    let out_file = PathBuf::from("src/lo_sys.rs");
    bindings
        .write_to_file(&out_file)
        .expect("Couldn't write bindings!");

    println!(
        "✅ Successfully generated bindings at: {}",
        out_file.display()
    );
}
