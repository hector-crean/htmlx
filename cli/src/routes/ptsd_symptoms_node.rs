use crate::{rich_text, rich_text_block, PROJECT_ROOT};
use blocks::block::bar_chart::BarChartProps;
use blocks::block::interactive_brain::{
    BrainComment, BrainRegion, BrainRegionName, CommentGroup, InteractiveBrainProps,
};
use blocks::block::rich_text::{RichText, RichTextProps};
use blocks::block::tabs::{Tab, TabsProps};
use blocks::block::{Block, BlocksProps};
use color_eyre::eyre;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::vec;
use uuid::Uuid;



pub fn ptsd_symptoms_blocks() -> Vec<Block> {
    let interactive_brain_block =  Block::InteractiveBrainBlock(InteractiveBrainProps {
        groups: vec![
            CommentGroup {
                name: "Intrusion".into(),
                comments: vec![
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/99bdf413-a7d9-4cc9-a7dc-ca3101696d19.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/85779108-f51a-4d79-9bfb-d40f46dad0d1.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/99bdf413-a7d9-4cc9-a7dc-ca3101696d19.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/112acedf-8aa3-46b3-b32e-ee4fe8a70a98.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/99bdf413-a7d9-4cc9-a7dc-ca3101696d19.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/ab3f8c75-a25a-472d-8d6a-1db141a55163.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/99bdf413-a7d9-4cc9-a7dc-ca3101696d19.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/e1c5016f-c507-44ba-8932-ed1860706449.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/99bdf413-a7d9-4cc9-a7dc-ca3101696d19.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/68dce148-2f2b-4e6a-9e39-3474ef4e2127.html")
                        ]
                    }
                ]
            },
            CommentGroup {
                name: "Avoidance".into(),
                comments: vec![
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/18939c45-8c7e-4c26-a1bc-b04e84b2773f.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/1a186e05-3910-4cf3-86f5-57617c393015.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/18939c45-8c7e-4c26-a1bc-b04e84b2773f.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/458822ca-1961-4686-9005-27be498d8db9.html")
                        ]
                    }
                ]
            },
            CommentGroup {
                name: "Negative".into(),
                comments: vec![
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/c4ea547c-d129-4bf9-9399-b540bf1746ac.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/b7abcdb9-56d9-49fc-ae68-978bc7c1a57a.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/c4ea547c-d129-4bf9-9399-b540bf1746ac.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/d3ce123a-ab6e-4e04-afdb-2ae590e83e7a.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/c4ea547c-d129-4bf9-9399-b540bf1746ac.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/525d13dd-a761-4443-a3cb-6f3485766310.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/c4ea547c-d129-4bf9-9399-b540bf1746ac.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/623cbc85-497a-4619-a348-bfae909e5b4b.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/c4ea547c-d129-4bf9-9399-b540bf1746ac.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/03444c89-bdd6-4ba2-bf7c-790701c3ad29.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/c4ea547c-d129-4bf9-9399-b540bf1746ac.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/8c6d5dc6-ddac-4e65-a7d6-4c20dfeea18d.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/c4ea547c-d129-4bf9-9399-b540bf1746ac.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/f7f8a21b-05f8-431d-8d53-4b5e2d69e7a3.html")
                        ]
                    },
                   
                ]
            },
            CommentGroup {
                name: "Arousal".into(),
                comments: vec![
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/294fe3f1-093a-4699-bed3-3ae3b35a7512.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/1994f39c-b9ac-41e4-9672-2cbb6b243cfb.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/294fe3f1-093a-4699-bed3-3ae3b35a7512.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/6f52f3e7-2f5c-4f9b-b50e-a7ecfa058b1f.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/294fe3f1-093a-4699-bed3-3ae3b35a7512.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/b60e70ed-a2f3-4002-ae16-b4b970214a25.html")
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/294fe3f1-093a-4699-bed3-3ae3b35a7512.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/f1c46da1-827f-4ffe-a1f8-41215bef7893.html"),
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/294fe3f1-093a-4699-bed3-3ae3b35a7512.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/176f8394-f50a-4ea8-ba06-91d296aa5792.html"),
                        ]
                    },
                    BrainComment {
                        icon: "".into(),
                        symptom: RichTextProps::default(),
                        highlighted_regions: vec![],
                        description: vec![
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/294fe3f1-093a-4699-bed3-3ae3b35a7512.html"),
                            rich_text_block!("../input/OTS126_PTSD_Symptoms_Node/2cf015f8-c45d-4796-8d9f-552cff4a231f.html"),
                        ]
                    },
                ]
            }                                    
        ]
    });

    vec![
        //Clinical Presentation
        rich_text_block!(
            "../input/OTS126_PTSD_Symptoms_Node/6bd35d11-fd3c-4907-a376-841dfb3d6c94.html"
        ),
        //Brain regions involved in PTSD
        rich_text_block!(
            "../input/OTS126_PTSD_Symptoms_Node/0b6a292c-bcde-466e-9419-bd620555ed6b.html"
        ),
        rich_text_block!(
            "../input/OTS126_PTSD_Symptoms_Node/5e6a4b9c-e6e3-48f9-932b-99f6e6377908.html"
        ),
        // Symptoms of PTSD
        rich_text_block!(
            "../input/OTS126_PTSD_Symptoms_Node/89201c17-fefc-4470-997d-494c226ad740.html"
        ),
        interactive_brain_block,
        //References
        rich_text_block!(
            "../input/OTS126_PTSD_Symptoms_Node/38c6fe92-2267-4814-ac2a-f4313f037a44.html"
        ),
    ]
}

