pub mod bar_chart;
pub mod brain;
pub mod bubble_chart;
pub mod definition;
pub mod disclosure;
pub mod grid_table;
pub mod html;
pub mod icon;
pub mod nav;
pub mod partition_chart;
pub mod pie_chart;
pub mod placeholder_container;
pub mod references;
pub mod rich_text;
pub mod suggested_node;
pub mod table;
pub mod tabs;
pub mod text_with_icon_list;
use crate::SvgName;

use self::{
    bar_chart::BarChartProps,
    definition::{DefinitionListProps, DefinitionProps},
    nav::NavProps,
};
use bar_chart::PtsdComborbiditiesLayout;
use brain::{interactive_brain::InteractiveBrainProps, interactive_image::InteractiveImageProps};
use bubble_chart::BubbleChartProps;
use disclosure::DisclosureProps;
use grid_table::GridTableProps;
use html::HtmlProps;
use icon::{IconProps, SvgProps};
use maud::{html, Markup};
use partition_chart::PartitionChartProps;
use pie_chart::PieChartProps;
use placeholder_container::PlaceholderContainerProps;
use references::ReferencesProps;
use rich_text::{RichText, RichTextProps};
use std::fmt::{write, Display, Write};
use suggested_node::SuggestedNodeProps;
use tabs::{Tab, TabsProps};
use uuid::Uuid;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
// #[serde(tag = "type", content = "props")]
pub enum Block {
    SuggestedNodeBlock(SuggestedNodeProps),
    RichTextBlock(RichTextProps),
    TabsBlock(TabsProps),
    InteractiveBrainBlock(InteractiveBrainProps),
    BarChartBlock(BarChartProps),
    NavBlock(NavProps),
    DefinitionBlock(DefinitionProps),
    DefinitionListBlock(DefinitionListProps),
    PlaceholderContainerBlock(PlaceholderContainerProps),
    ReferencesBlock(Box<ReferencesProps>),
    PieChartBlock(PieChartProps),
    InteractiveImage(InteractiveImageProps),
    IconBlock(IconProps),
    HtmlBlock(HtmlProps),
    SvgBlock(SvgProps),
    DisclosureBlock(DisclosureProps),
    Html(Markup),
    GridTableBlock(GridTableProps),
    BubbleChartBlock(BubbleChartProps),
    PartitionChartBlock(PartitionChartProps),
    PtsdComborbiditiesLayoutBlock(PtsdComborbiditiesLayout),
}

impl maud::Render for Block {
    fn render(&self) -> maud::Markup {
        html!(
            @match self {
                Block::PtsdComborbiditiesLayoutBlock(block) => {
                    (block)
                }
                Block::PartitionChartBlock(block) => {
                    (block)
                }
                Block::BubbleChartBlock(block) => {
                    (block)
                }
                Block::GridTableBlock(block) => {
                    (block)
                }
                Block::SuggestedNodeBlock(block) => {
                    (block)
                }
                Block::Html(block) => {
                    (block)
                }
                Block::DisclosureBlock(block) => {
                    (block)
                }
                Block::SvgBlock(block) => {
                    (block)
                }
                Block::HtmlBlock(block) => {
                    (block)
                }
                Block::IconBlock(block) => {
                    (block)
                }
                Block::ReferencesBlock(block) => {
                    (block)
                }
                Block::RichTextBlock(block) => {
                    (block)
                }
                Block::TabsBlock(block) => {
                    (block)
                }

                Block::InteractiveBrainBlock(block) => {
                   (block)
                }
                Block::BarChartBlock(block) => {
                    (block)
                }
                Block::NavBlock(block) => {
                    (block)
                }
                Block::DefinitionBlock(block) => {
                    (block)
                }
                Block::DefinitionListBlock(block) => {
                    (block)
                }
                Block::PlaceholderContainerBlock(block) => {
                    (block)
                }
                Block::PieChartBlock(block) => {
                    (block)
                }
                Block::InteractiveImage(block) => {
                    (block)
                }

            }
        )
    }
}
