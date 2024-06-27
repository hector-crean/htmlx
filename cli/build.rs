use blocks::block::brain::BrainRegion;
use include_dir::{include_dir, Dir};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // Normalize paths
    let out_dir = PathBuf::from(out_dir);
    let manifest_dir = PathBuf::from(manifest_dir);

    let dest_path = out_dir.join("project_root.rs");

    let mut f = File::create(&dest_path).expect("Unable to create project_root.rs");
    writeln!(
        f,
        "pub const PROJECT_ROOT: &'static str = {:?};",
        manifest_dir.display()
    )
    .expect("Unable to write to project_root.rs");
}
