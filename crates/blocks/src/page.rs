use std::borrow::Cow;

use maud::{html, Render};

use crate::block::Block;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct Page {
    pub blocks: Vec<Block>,
}

impl Page {
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }
}

impl maud::Render for Page {
    fn render(&self) -> maud::Markup {
        html!(
            @for block in &self.blocks {
                (block)
            }
        )
    }
}
