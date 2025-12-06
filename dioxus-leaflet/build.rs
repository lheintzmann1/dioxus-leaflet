use std::path::PathBuf;

use dioxus_use_js::BunBuild;

fn main() {
    BunBuild::builder()
        .src_files(vec![PathBuf::from("js_utils/src/dioxus_leaflet.ts")])
        .output_dir(PathBuf::from("assets"))
        .skip_if_no_bun(true)
        .build()
        .run();
}