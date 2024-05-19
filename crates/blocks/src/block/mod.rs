pub mod bar_chart;
pub mod interactive_brain;
pub mod nav;
pub mod rich_text;
pub mod tabs;

use self::{bar_chart::BarChartProps, interactive_brain::InteractiveBrainProps, nav::NavProps};
use core::fmt;
use maud::html;
use rich_text::{RichText, RichTextProps};
use std::fmt::{write, Display, Write};
use tabs::{Tab, TabsProps};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct BlocksProps {
    blocks: Vec<Block>,
}

impl maud::Render for BlocksProps {
    fn render(&self) -> maud::Markup {
        html!(
            @for block in &self.blocks {
                (block)
            }
        )
    }
}

impl BlocksProps {
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(tag = "type", content = "props")]
pub enum Block {
    RichTextBlock(RichTextProps),
    TabsBlock(TabsProps),
    InteractiveBrainBlock(InteractiveBrainProps),
    BarChartBlock(BarChartProps),
    NavBlock(NavProps),
}

impl maud::Render for Block {
    fn render(&self) -> maud::Markup {
        html!(
            @match self {
                Block::RichTextBlock(rich_text) => {
                    (rich_text)
                }
                Block::TabsBlock(tabs) => {
                    (tabs)
                }

                Block::InteractiveBrainBlock(interactive_brain) => {
                   (interactive_brain)
                }
                Block::BarChartBlock(bar_chart) => {
                    (bar_chart)
                }
                Block::NavBlock(nav) => {
                    (nav)
                }
            }
        )
    }
}
