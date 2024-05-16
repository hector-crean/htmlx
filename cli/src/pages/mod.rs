use blocks::block::{Block, BlocksProps};
use color_eyre::eyre;
use std::io::Write;
use std::{fs::File, path::PathBuf};
pub mod ptsd_symptoms_node;
use maud::{html, Markup, Render};


pub trait Pagelike {
    fn blocks() -> Vec<Block>;
    fn output_path(&self) -> PathBuf;
    fn render_html(&self) -> eyre::Result<()> {
        let blocks = BlocksProps::new(Self::blocks());

        let code = blocks.render();


        let mut file = File::create(self.output_path())?;


        file.write(code.0.as_bytes())?;

        Ok(())
    }
}
