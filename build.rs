// Packs the files in data/resources (the view icons) into the LeafMD program,
// so they're always available no matter which icon theme is installed.
fn main() {
    glib_build_tools::compile_resources(
        &["data/resources"],
        "data/resources/resources.gresource.xml",
        "leafmd.gresource",
    );
}
