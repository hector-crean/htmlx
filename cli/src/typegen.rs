use std::{fs::File, io::Write, path::Path};

use specta::{
    ts::{ExportConfig, ExportError},
    TypeCollection,
};

use blocks::block::Block;
use blocks::node_map::NodeMapData;

// Import types and modules to generate types from

pub fn typegen<P: AsRef<Path>>(output_path: P) -> Result<(), ExportError> {
    // let code = TypeCollection::default()
    //     .register::<Block>()
    //     .register::<NodeMapData>()
    //     .export_ts(&ExportConfig::default())?;

    // let mut file = File::create(output_path)?;

    // file.write(code.as_bytes())?;

    Ok(())
}
