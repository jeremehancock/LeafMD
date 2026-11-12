// Packs the files in data/resources (the view and formatting icons) into the LeafMD program,
// so they're always available no matter which icon theme is installed.
fn main() {
    glib_build_tools::compile_resources(
        &["data/resources"],
        "data/resources/resources.gresource.xml",
        "leafmd.gresource",
    );

    // The Rust version LeafMD is compiled with, shown in the About window.
    // `rustc --version` prints something like "rustc 1.91.0 (f8297e351 2025-10-28)".
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let version = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|text| text.split_whitespace().nth(1).map(str::to_string))
        .unwrap_or_default();
    println!("cargo:rustc-env=LEAFMD_RUST_VERSION={version}");

    // The libspelling version, shown in the About window. libspelling has no
    // way to ask for its version while running, so it's read here instead.
    // The Flatpak builds its own copy just before LeafMD, so this is the one
    // LeafMD runs with. Empty if pkg-config can't say.
    let spelling = std::process::Command::new("pkg-config")
        .args(["--modversion", "libspelling-1"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|text| text.trim().to_string())
        .unwrap_or_default();
    println!("cargo:rustc-env=LEAFMD_LIBSPELLING_VERSION={spelling}");
}
