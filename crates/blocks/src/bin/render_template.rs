use std::fs::File;
use color_eyre::eyre;
use askama::Template;
use blocks::block::BlocksProps;
use std::io::Write;

fn main() -> eyre::Result<()> {
    let blocks = BlocksProps::example_blocks(); // instantiate your struct

    let code = blocks.render()?;

    let mut file = File::create("./rendered.html")?;

    file.write(code.as_bytes())?;

    Ok(())

}
