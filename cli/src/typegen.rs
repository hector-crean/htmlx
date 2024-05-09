use std::path::Path;


use specta::{export, ts::ExportError};

use crate::types::Block;

// Import types and modules to generate types from

// Hack to enable specta exporter to recognise types in our repo
#[cfg_attr(feature = "typings", derive(specta::Type))]
struct SpectaExport {
    block: Block,
}

pub fn typegen<P: AsRef<Path>>(output_path: P) -> Result<(), ExportError> {
    export::ts(output_path)?;

    Ok(())
}
