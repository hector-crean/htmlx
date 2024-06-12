use crate::{rich_text_block};
use blocks::block::references::ReferencesProps;
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use blocks::block::{Block};
use std::vec;
pub fn blocks() -> Vec<Block> {
    let tabs = TabsProps {
        id: uuid::Uuid::new_v4(),
        representation: TabsRepresentation::Standard,
        tabs: vec![
            Tab {
                name: String::from("Civilan / General Population"),
                blocks: vec![
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/74df1483-d108-492c-8d35-5737ba519ac6.html"),
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/b182874e-54ed-4b26-b1e0-7df6b7c9ea96.html"),
                   
                
                ],
            },
            Tab {
                name: String::from("PTSD in Women"),
                blocks: vec![
                  
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/181d5b1a-c561-4439-bb06-767e305905e9.html"),
                  
                ],
            },
            Tab {
                name: String::from("Military Population"),
                blocks: vec![
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/32088244-08d2-416a-8135-c1b51b9a9d54.html"),

                  
                   
                ],
            },
            Tab {
                name: String::from("LGBTQ+ and marginalized groups"),
                blocks: vec![
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/56a8e5cf-321d-45a1-a190-1b5a0f72364c.html"),
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f83dd12-589d-4f98-a74a-ea8766b3e042.html"),
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f05cdb0-a001-4afc-8b3c-04fec47a501c.html"),
                ],
            },
        ],
    };

    vec![
        Block::TabsBlock(tabs),
        Block::ReferencesBlock(Box::new(ReferencesProps {
            references: rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/958c48d1-9ddb-4c8d-a301-d7ccd9c6c104.html"),
        })),
    ]
}
