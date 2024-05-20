use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use svg::parser::Event;
use svg::Document;

fn make_svg_responsive<P: AsRef<Path>>(path: P) -> io::Result<()> {
    // Open the SVG file
    let mut contents = String::new();
    let parser = svg::open(path, &mut contents)?;

    // Parse the SVG
    let mut document = svg::Document::new()

    // Get the original width and height
    let (original_width, original_height) =
        if let (Some(width), Some(height)) = (document.get_width(), document.get_height()) {
            (width, height)
        } else {
            eprintln!("SVG does not have width or height attributes");
            return Ok(());
        };

    // Set the viewBox attribute
    document = document.set(
        "viewBox",
        format!("0 0 {} {}", original_width, original_height),
    );

    // Set width and height to 100%
    document = document.set("width", "100%").set("height", "100%");

    // Write the modified SVG to a new file
    let mut output = File::create("output.svg")?;
    write!(output, "{}", document)?;

    Ok(())
}
