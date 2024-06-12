use crate::rich_text_block;

use blocks::block::references::ReferencesProps;

use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use blocks::block::Block;

use std::vec;

pub fn blocks() -> Vec<Block> {
    vec![
        Block::TabsBlock(TabsProps {
            id: uuid::Uuid::new_v4(),
            representation: TabsRepresentation::Standard,
            tabs: vec![
                Tab {
                    name: String::from("Personal Stigma"),
                    blocks: vec![rich_text_block!(
                        "../../../input/OTS126_PTSD_Stigma_Node/2b5a8617-b294-4342-9479-516ae0bbf52c.html"
                    )],
                },
                Tab {
                    name: String::from("Community and Societal Stigma"),
                    blocks: vec![rich_text_block!(
                        "../../../input/OTS126_PTSD_Stigma_Node/e6af01ff-7a34-4bde-97da-67fa3d57bf28.html"
                    )],
                },
                Tab {
                    name: String::from("HCP Stigma"),

                    blocks: vec![rich_text_block!(
                        "../../../input/OTS126_PTSD_Stigma_Node/b4ce6e3b-ac17-4c24-bf6d-78a796c2b4b2.html"
                    )],
                },
            ],
        }),
        Block::ReferencesBlock(Box::new(ReferencesProps {
            references: rich_text_block!(
                "../../../input/OTS126_PTSD_Stigma_Node/1b940d14-983a-4c19-be83-2b13819180e9.html"
            ),
        })),
    ]
}
