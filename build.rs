fn main() {
    try_pkg_config();
}

/// Use pkg-config to find system liblo
fn try_pkg_config() {
    // Skip lib search if building docs for docs.rs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    // Probe system liblo via pkg-config
    let _ = pkg_config::Config::new()
        .atleast_version("0.32")
        .probe("liblo")
        .expect("Could not find system liblo via pkg-config. Please install liblo.");
}
