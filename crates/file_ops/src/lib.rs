use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn js_to_ts<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    // Get the current directory
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();

        // Check if the entry is a file with a .js extension
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("js") {
            // Create a new path with the .ts extension
            let new_path = path.with_extension("ts");
            // Rename the file
            fs::rename(&path, &new_path)?;
            println!("Renamed {:?} to {:?}", path, new_path);
        }
    }
    Ok(())
}
