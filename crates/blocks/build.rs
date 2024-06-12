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

    // Define the directory containing the SVG files
    let svg_dir = Path::new(r"C:\Users\Hector.C\rust\htmx\crates\blocks\src\block\icon\icons");

    // Path to the output file
    let dest_path = out_dir.join("generated_icons.rs");
    let mut file = File::create(&dest_path).expect("Unable to create generated_icons.rs");

    // Start generating the enum and impl block
    let mut enum_variants = Vec::new();
    let mut match_arms = Vec::new();

    // Iterate over the SVG files in the directory
    for entry in fs::read_dir(svg_dir).expect("Unable to read SVG directory") {
        let entry = entry.expect("Unable to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "svg") {
            let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
            let variant_name = normalize_variant_name(&file_name);
            let variant = format!("    {},\n", variant_name);
            let match_arm = format!(
                "            SvgName::{} => include_str!(r#\"{}\"#),\n",
                variant_name,
                path.to_string_lossy()
            );

            enum_variants.push(variant);
            match_arms.push(match_arm);
        }
    }

    // Add a default match arm
    let default_match_arm = r#"            _ => "",\n"#;

    // Write the generated code to the file
    let enum_def = format!(
        r#"
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
pub enum SvgName {{
    #[default]
{}}}

impl SvgName {{
    pub fn svg(&self) -> &str {{
        match self {{
{}
            
        }}
    }}
}}
"#,
        enum_variants.concat(),
        match_arms.concat(),
    );

    file.write_all(enum_def.as_bytes())
        .expect("Unable to write to generated_icons.rs");

    // Tell Cargo to rerun the build script if the SVG directory changes
    println!("cargo:rerun-if-changed={}", svg_dir.display());
}

fn normalize_variant_name(file_name: &str) -> String {
    file_name
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            c.next().unwrap().to_uppercase().collect::<String>() + c.as_str()
        })
        .collect()
}
