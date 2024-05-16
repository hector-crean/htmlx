use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("project_root.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut f = File::create(&dest_path).unwrap();
    writeln!(
        f,
        "pub const PROJECT_ROOT: &'static str = {:?};",
        manifest_dir
    )
    .unwrap();
}
