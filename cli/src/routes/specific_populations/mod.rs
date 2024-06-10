use crate::{rich_text, rich_text_block};
use blocks::block::bar_chart::BarChartProps;
use blocks::block::definition::{DefinitionListProps, DefinitionProps};
use blocks::block::icon::IconName;
use blocks::block::icon::IconProps;
use blocks::block::interactive_brain::{
    BrainComment, BrainRegion, BrainRegionName, CommentGroup, InteractiveBrainProps,
};
use blocks::block::references::ReferencesProps;
use blocks::block::rich_text::{self, BranchNode, RichText, RichTextProps};
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use blocks::block::{Block, BlocksProps};
use color_eyre::eyre;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::vec;
use uuid::Uuid;

pub fn blocks() -> Vec<Block> {
    let tabs = TabsProps {
        id: uuid::Uuid::new_v4(),
        representation: TabsRepresentation::Standard,
        tabs: vec![
            Tab {
                name: String::from("Civilan / General Population"),
                blocks: vec![rich_text_block!(
                    "../../input/OTS126_PTSD_Specific_Populations_Node/b182874e-54ed-4b26-b1e0-7df6b7c9ea96.json"
                )],
            },
            Tab {
                name: String::from("PTSD in Women"),
                blocks: vec![rich_text_block!(
                    "../../input/OTS126_PTSD_Specific_Populations_Node/b182874e-54ed-4b26-b1e0-7df6b7c9ea96.json"
                )],
            },
            Tab {
                name: String::from("Military Population"),
                blocks: vec![rich_text_block!(
                    "../../input/OTS126_PTSD_Specific_Populations_Node/b182874e-54ed-4b26-b1e0-7df6b7c9ea96.json"
                )],
            },
            Tab {
                name: String::from("LGBTQ+ and marginalized groups"),
                blocks: vec![rich_text_block!(
                    "../../input/OTS126_PTSD_Specific_Populations_Node/b182874e-54ed-4b26-b1e0-7df6b7c9ea96.json"
                )],
            },
        ],
    };

    vec![
        Block::TabsBlock(tabs),
        //References
        // Block::ReferencesBlock(Box::new(ReferencesProps {
        //     references: rich_text_block!(
        //         "../../../input/OTS126_PTSD_Stigma_Node/1b940d14-983a-4c19-be83-2b13819180e9.json"
        //     ),
        // })),
    ]
}
