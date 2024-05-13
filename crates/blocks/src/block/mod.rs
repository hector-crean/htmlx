use askama::Template;
use core::fmt;
use rich_text::{RichText, RichTextProps};
use std::fmt::{write, Display, Write};
use tabs::{Tab, TabsProps};
use uuid::Uuid;

use self::{bar_chart::BarChartProps, interactive_brain::InteractiveBrainProps, text_with_icon::TextWithIconProps};

pub mod rich_text;
pub mod tabs;
pub mod text_with_icon;
pub mod interactive_brain;
pub mod bar_chart;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct BlocksProps {
    blocks: Vec<Block>,
}

impl fmt::Display for BlocksProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.blocks {
            writeln!(f, "{:#}", block)?;
        }
        fmt::Result::Ok(())
    }
}

impl BlocksProps {
    pub fn render(&self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();

        write!(&mut buf, "{}", self)?;

        Ok(buf)                     
    }                                          
}                                                

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum Block {
    RichTextBlock(RichTextProps),
    TabsBlock(TabsProps),
    TextWithIconBlock(TextWithIconProps),
    InteractiveBrainBlock(InteractiveBrainProps),
    BarChartBlock(BarChartProps)
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::RichTextBlock(rich_text) => {
                let rendered = rich_text.render().map_err(|_| fmt::Error)?;
                write!(f, "{}", rendered)
            }
            Block::TabsBlock(tabs) => {
                let rendered = tabs.render().map_err(|_| fmt::Error)?;
                write!(f, "{}", rendered)
            }
            Block::TextWithIconBlock(text_with_icon) => {
                let rendered = text_with_icon.render().map_err(|_| fmt::Error)?;
                write!(f, "{}", rendered)
            }
            Block::InteractiveBrainBlock(interactive_brain) => {
                let rendered = interactive_brain.render().map_err(|_| fmt::Error)?;
                write!(f, "{}", rendered)
            }
            Block::BarChartBlock(bar_chart) => {
                let rendered = bar_chart.render().map_err(|_| fmt::Error)?;
                write!(f, "{}", rendered)
            }
        }
    }
}

impl Block {
    pub fn render(&self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();

        write!(&mut buf, "{}", self)?;

        Ok(buf)
    }
}


impl BlocksProps {
    pub fn example_blocks() -> Self {
        Self {
            blocks: vec![
                Block::InteractiveBrainBlock(InteractiveBrainProps::default()),
                Block::BarChartBlock(BarChartProps::default()),
                Block::TabsBlock(TabsProps {
                id: Uuid::new_v4(),
                tabs: vec![
                    Tab {
                    name: "Tab 1".to_string(),
                    blocks: vec![
                        Block::InteractiveBrainBlock(InteractiveBrainProps::default()),
                        Block::BarChartBlock(BarChartProps::default()),
                        Block::TabsBlock(TabsProps {
                        id: Uuid::new_v4(),
                        tabs: vec![
                            Tab {
                            name: "Tab 1".to_string(),
                            blocks: vec![
                                Block::TabsBlock(TabsProps {
                                    id: Uuid::new_v4(),
                                    tabs: vec![],
                                })],
                            }]
                        ,
                    })] ,
                    }]
                ,
            })],
        }
    }
}
