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
                name: String::from("Personal Stigma"),
                blocks: vec![rich_text_block!(
                    "../../../input/OTS126_PTSD_Stigma_Node/2b5a8617-b294-4342-9479-516ae0bbf52c.json"
                )],
            },
            Tab {
                name: String::from("Community and Societal Stigma"),
                blocks: vec![rich_text_block!(
                    "../../../input/OTS126_PTSD_Stigma_Node/e6af01ff-7a34-4bde-97da-67fa3d57bf28.json"
                )],
            },
            Tab {
                name: String::from("HCP Stigma"),
                
                blocks: vec![rich_text_block!(
                    "../../../input/OTS126_PTSD_Stigma_Node/b4ce6e3b-ac17-4c24-bf6d-78a796c2b4b2.json"
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
