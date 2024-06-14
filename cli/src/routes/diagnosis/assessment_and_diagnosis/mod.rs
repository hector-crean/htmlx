
use blocks::block::brain::brain_glossary::BrainGlossaryProps;
use blocks::block::brain::brain_region::BrainRegionName;
use blocks::block::definition::{DefinitionListProps, DefinitionProps};
use blocks::block::disclosure::{DisclosureProps, DisclosureTheme};
use blocks::SvgName;
use blocks::block::icon::IconProps;
use blocks::block::brain::interactive_brain::{
    BrainComment, CommentGroup, InteractiveBrainProps,
};

use blocks::block::references::ReferencesProps;
use blocks::block::rich_text::{RichText, RichTextBlock, RichTextProps};
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use blocks::block::{Block};




use std::vec;


use crate::{rich_text, rich_text_block};

pub fn blocks() -> Vec<Block> {

  


    vec![
        rich_text_block!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/legend.html"),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#dae3f3", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/3_2a104e7e-0cba-4467-b4ae-be6ee99c3c9d.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/5_2bdd6b41-5f2c-47f6-9665-44f4dc1bcd81.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/6_e7121e88-a5ca-4255-86a4-a174e4bb7434.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/7_7fc5e344-e530-41b7-bbf3-c65cded60fae.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/8_c55cbab3-655a-44a6-83eb-38584d068ed2.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/9_af10b6a4-7a63-4ff9-9ed8-66086070a84d.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/10_d61002a0-659d-4b30-9485-d4eee1fccbe2.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/11_763dc90a-7d79-4db3-bc33-10cab3adffa4.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/12_3ddbec48-5328-4ca7-9177-051668d111de.html"),
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/13_0c47faa7-47d5-45d8-afbf-90f997f46ecb.html"),
            ]
        }),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#dae3f3", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/14_0e1d7b77-2216-403a-91ec-d115d46caebc.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/15_0e1d7b77-2216-403a-91ec-d115d46caebc.html")
            ]
        }),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#f8cbad", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/18_00117b2c-aae4-4558-b8ea-9b9140f1351d.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/19_00117b2c-aae4-4558-b8ea-9b9140f1351d.html")
            ]
        }),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#f8cbad", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/21_d2c3f7e4-c073-4b6e-bd18-4536c3652d1e.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/22_d2c3f7e4-c073-4b6e-bd18-4536c3652d1e.html")
            ]
        }),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#dae3f3", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/23_580b6a78-18f4-410a-a80a-26d5d15e79cb.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/24_580b6a78-18f4-410a-a80a-26d5d15e79cb.html")
            ]
        }),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#dae3f3", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/25_2397e180-f20b-472b-9d5c-a6c97def1fdd.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/26_2397e180-f20b-472b-9d5c-a6c97def1fdd.html")
            ]
        }),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#f8cbad", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/27_eaff2ce2-e3d6-4bba-a63c-3efb12cb796f.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/28_eaff2ce2-e3d6-4bba-a63c-3efb12cb796f.html")
            ]
        }),
        Block::DisclosureBlock(DisclosureProps { 
            theme: DisclosureTheme::new("#f8cbad", "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/29_e656ef91-dfe4-4377-81b1-07728febfaea.html"),
            longform: vec![
                rich_text!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/30_e656ef91-dfe4-4377-81b1-07728febfaea.html")
            ]
        }),
        
        

       
    
      
        //References
        Block::ReferencesBlock(Box::new(ReferencesProps {
            references:  rich_text_block!("../../../input/OTS126_PTSD_Assessment_Diagnosis_Screening_Node/31_9231d58a-7200-4125-9dba-2cc00f1e9012.html")
        })),
    ]
}
