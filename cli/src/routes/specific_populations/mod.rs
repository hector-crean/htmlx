use crate::{rich_text_block};
use blocks::block::icon::{IconProps, SvgProps};
use blocks::span::ref_note::RefNote;
use blocks::SvgName;
use blocks::block::references::ReferencesProps;
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme};
use blocks::block::{Block};
use maud::{html, Render};
use std::vec;





pub struct Page;

impl Page {
    fn civilian_and_general_population_tab(&self) -> Vec<Block> {
        vec![
            Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDGlobalUSInfographic}),
            Block::Html( html! {
                p {
                    "The global lifetime prevalence of PTSD is 4-10%"(RefNote::new(1))(RefNote::new(2))
                }
                p {
                    "PTSD is one of the most common mental health disorders in the US, with 7-8 out of every 100 people experiencing PTSD at some point in their lifetime"(RefNote::new(3))(RefNote::new(4))
                }
            }),
            Block::SvgBlock(SvgProps { name: SvgName::OTS12613AdultsPTSDInfographic}),
            Block::Html( html! {
                p {
                    "~13 million adults in the US will experience PTSD during a given year"(RefNote::new(5))
                }
               
            }),
            Block::SvgBlock(SvgProps { name: SvgName::OTS126TypicalAgePTSDText}),
            Block::Html( html! {
                p {
                    ">80% of PTSD patients are in the general population rather than the military population"(RefNote::new(6))
                }
               
            }),

           
           
        
        ]
    }
}

impl maud::Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (Block::TabsBlock(TabsProps {
                id: uuid::Uuid::new_v4(),
                representation: TabsRepresentation::TopLevel,
                tabs: vec![
                    Tab {
                        name: String::from("Civilan / General Population"),
                        blocks: self.civilian_and_general_population_tab(),
                    },
                    Tab {
                        name: String::from("PTSD in Women"),
                        blocks: vec![
                            Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDWarVeteransInfographic}),
        
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/181d5b1a-c561-4439-bb06-767e305905e9.html"),
                          
                        ],
                    },
                    Tab {
                        name: String::from("Military Population"),
                        blocks: vec![
                            Block::SvgBlock(SvgProps { name: SvgName::OTS12680MilitaryPopulationInfographic}),
                            // Block::IconBlock(IconProps { name: SvgName::OTS126ExposureGenderPeaksTable}),
        
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/32088244-08d2-416a-8135-c1b51b9a9d54.html"),
        
                          
                           
                        ],
                    },
                    Tab {
                        name: String::from("LGBTQ+ and marginalized groups"),
                        blocks: vec![
        
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/56a8e5cf-321d-45a1-a190-1b5a0f72364c-1.html"),
                            Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDWithinLGBTQInfographic}),
        
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/56a8e5cf-321d-45a1-a190-1b5a0f72364c-2.html"),
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/56a8e5cf-321d-45a1-a190-1b5a0f72364c-3.html"),
        
        
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f83dd12-589d-4f98-a74a-ea8766b3e042.html"),
        
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f05cdb0-a001-4afc-8b3c-04fec47a501c-1.html"),
                            Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDRaceInfographic}),
        
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f05cdb0-a001-4afc-8b3c-04fec47a501c-2.html"),
                            rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f05cdb0-a001-4afc-8b3c-04fec47a501c-3.html"),
        
        
        
        
                        ],
                    },
                ],
            }))
        }
    }
}


pub fn blocks() -> Vec<Block> {
    let tabs = TabsProps {
        id: uuid::Uuid::new_v4(),
        representation: TabsRepresentation::TopLevel,
        tabs: vec![
            Tab {
                name: String::from("Civilan / General Population"),
                blocks: vec![
                    Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDGlobalUSInfographic}),
                    Block::SvgBlock(SvgProps { name: SvgName::OTS12613AdultsPTSDInfographic}),
                    Block::SvgBlock(SvgProps { name: SvgName::OTS126TypicalAgePTSDText}),

                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/74df1483-d108-492c-8d35-5737ba519ac6.html"),
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/b182874e-54ed-4b26-b1e0-7df6b7c9ea96.html"),
                   
                
                ],
            },
            Tab {
                name: String::from("PTSD in Women"),
                blocks: vec![
                    Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDWarVeteransInfographic}),

                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/181d5b1a-c561-4439-bb06-767e305905e9.html"),
                  
                ],
            },
            Tab {
                name: String::from("Military Population"),
                blocks: vec![
                    Block::SvgBlock(SvgProps { name: SvgName::OTS12680MilitaryPopulationInfographic}),
                    // Block::IconBlock(IconProps { name: SvgName::OTS126ExposureGenderPeaksTable}),

                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/32088244-08d2-416a-8135-c1b51b9a9d54.html"),

                  
                   
                ],
            },
            Tab {
                name: String::from("LGBTQ+ and marginalized groups"),
                blocks: vec![

                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/56a8e5cf-321d-45a1-a190-1b5a0f72364c-1.html"),
                    Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDWithinLGBTQInfographic}),

                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/56a8e5cf-321d-45a1-a190-1b5a0f72364c-2.html"),
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/56a8e5cf-321d-45a1-a190-1b5a0f72364c-3.html"),


                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f83dd12-589d-4f98-a74a-ea8766b3e042.html"),

                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f05cdb0-a001-4afc-8b3c-04fec47a501c-1.html"),
                    Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDRaceInfographic}),

                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f05cdb0-a001-4afc-8b3c-04fec47a501c-2.html"),
                    rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/9f05cdb0-a001-4afc-8b3c-04fec47a501c-3.html"),




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
