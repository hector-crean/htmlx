use oxc::span::SourceType;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use oxc::{
    allocator::Allocator,
    codegen::{Codegen, CodegenOptions},
    parser::Parser,
    transformer::{
        ArrowFunctionsOptions, ES2015Options, ReactOptions, TransformOptions, Transformer,
        TypeScriptOptions,
    },
};

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

pub fn ts_to_js<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    // Get the current directory
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();

        // Check if the entry is a file with a .js extension
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ts") {
            let ts_code = fs::read_to_string(&path).expect("Failed to read TypeScript file");
            // Parse the TypeScript code

            let source_type = SourceType::from_path(&path).unwrap();
            let allocator = oxc::allocator::Allocator::default();

            let parser = Parser::new(&allocator, &ts_code, source_type);
            let ret = parser.parse();

            let mut program = ret.program;
            let transform_options = TransformOptions {
                typescript: TypeScriptOptions::default(),
                es2015: ES2015Options {
                    arrow_function: Some(ArrowFunctionsOptions::default()),
                },
                ..Default::default()
            };
            let _ = Transformer::new(
                &allocator,
                &path,
                source_type,
                &ts_code,
                &ret.trivias,
                transform_options,
            )
            .build(&mut program);

            let js_code = Codegen::<false>::new("", &ts_code, CodegenOptions::default(), None)
                .build(&program)
                .source_text;

            // Transpile to JavaScript

            // Write the JavaScript code to a file
            fs::write(&path, js_code).expect("Failed to write JavaScript file");

            // Create a new path with the .ts extension
            let new_path = &path.with_extension("js");
            // Rename the file
            fs::rename(&path, &new_path)?;
        }
    }
    Ok(())
}
