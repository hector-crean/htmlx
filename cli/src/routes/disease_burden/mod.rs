use crate::{rich_text_block, rich_text};
use blocks::block::html::HtmlProps;
use blocks::block::icon::{IconProps, SvgProps};
use blocks::block::pie_chart::{PieChartDatum, PieChartProps};
use blocks::block::references::ReferencesProps;
use blocks::block::rich_text::{self, RichTextProps};
use blocks::block::table::TableProps;
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme};
use blocks::block::Block;
use maud::{html, Render};
use std::vec;
use blocks::SvgName;


pub fn blocks() -> Vec<Block> {



    vec![
        Block::TabsBlock(TabsProps {
            id: uuid::Uuid::new_v4(),
            representation: TabsRepresentation::TopLevel,
            tabs: vec![
                Tab {
                    name: String::from("Personal Burden"),
                    blocks: vec![
                        Block::TabsBlock( TabsProps { 
                            id: uuid::Uuid::new_v4(), 
                            tabs: vec![
                                Tab {
                                    name: String::from("Health"),
                                    blocks: vec![
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/6_b7101e4f-a8ca-4165-af53-bc96183a0364.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/7_0cdc8891-6de9-48b6-92f4-2293f0579622.html"),
                                  
                                    ]
                                },
                                Tab {
                                    name: String::from("Family / Caregiver"),
                                    blocks: vec![
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/11_475fe6d4-5fe5-4d6a-b9e0-a823b48caf42.html"),
                                        Block::TableBlock(TableProps {
                                            dimension: [7,3],
                                            headers: vec![],
                                            rows: vec![
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126TimingOfIndexTrauma}),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/12_2f336f06-da9c-40be-85c7-aedfbce5e0df.html"),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/13_64b9e654-95dd-4e15-a9e2-f606130e9eae.html"),

                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126NaturalDisasters}),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/14_23b39c37-891a-4557-aabb-d997f7b19918.html"),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/15_94fd1036-f6c5-4120-a8d8-20b1d388e237.html"),
                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126InterpersonalVictimization}),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/16_4463242c-a47a-4e90-8fbb-934280eb9ade.html"),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/17_e0575462-3e34-4d78-93d8-d1af97c36e17.html"),
                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126ChildhoodSexualAbuse}),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/18_688e8c86-e679-4500-9400-7b746d2462a9.html"),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/19_efff67b7-83fc-4ca8-9cfb-e02622963606.html"),
                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126CiviliansWithPTSD}),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/20_0b5efa9f-b88c-4779-b265-099b299a2e16.html"),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/21_f33d8dc5-2f12-4745-89da-ea4873315cae.html"),
                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126PartnersOfIndividualWithPTSD}),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/22_c7c6043c-abbd-4dc6-83e0-b03dbfa72527.html"),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/23_51989c7c-6bfc-4b06-87cd-e9f258558644.html"),
                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126PartnersOfVeteransWithPTSD}),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/25_693ad3f9-35cd-406f-9895-d0ea04646acd.html"),
                                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/26_524d760e-72b0-4a4f-b18f-bebd35105757.html"),

                                                ],
                                               
                                            ] 
                                        }),
                                       
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/29_a80701fb-893f-42b9-9cee-ece46f9ea204.html"),

                                       
                                      

                                    ]
                                },
                                Tab {
                                    name: String::from("Work and Finance"),
                                    blocks: vec![
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/34_c40fb323-fed0-494d-9a0e-974c0e2ed032.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/35_f54b078d-8dde-4ebe-97f8-f8e4e97a8e4b.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/37_c8a00e01-90a2-4a80-97a0-4415b831aa17.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/38_40b13af6-fdb1-4242-8330-1e651d103947.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/40_718cc37f-4463-4116-ae02-a965372736ec.html"),
                                    
                                        Block::SvgBlock(SvgProps { name: SvgName::OTS126Icons02MNPTSDAtWorkInfographic}),

                                      
                                      
                                      

                                    ]
                                },
                                Tab {
                                    name: String::from("Interpersonal"),
                                    blocks: vec![
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/42_2cc53ac7-26f8-4771-b696-fae065dae509.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/44_1bd244ce-d2c4-4ed2-8dc4-94a3641ae56a.html"),

                                      
                                        Block::SvgBlock(SvgProps { name: SvgName::OTS126Icons02MNPTSDHyperviliganceInfogrpahic}),

                                      
                                      
                                      

                                    ]
                                }
                            ], 
                            representation: TabsRepresentation::Internal { theme: TabsTheme::new("#d6e5ee", "#3b3e3f"), full_bleed: false } })
                    ],
                },
                Tab {
                    name: String::from("Societal Burden"),
                    blocks: vec![

                   
                      rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/48_08cff1ca-fb62-4bef-83be-136fb27d9204.html"),
                      Block::SvgBlock(SvgProps { name: SvgName::OST126DiseaseBurdenInfographic011 }),

                        Block::TabsBlock(TabsProps {
                            id: uuid::Uuid::new_v4(),
                            representation: TabsRepresentation::Swiper,
                            tabs: vec![
                                Tab {
                                    name: String::from("Overal Economic Burden"),
                                    blocks: vec![
                                        // Block::PieChartBlock(PieChartProps {
                                        //     title: String::from("Figure 1. Excess Economic Burden of PTSD in the Overall US Population in 2018, Billion USD"),
                                        //     slices: vec![
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct health care costs"), value: 32.8 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct non-health care costs"), value: 15.4 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of unemployment"), value: 19.9 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of productivity loss"), value: 15. },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs due to caregiving"), value: 15.8 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of premature mortality"), value: 1.1 }
                                        //         ]
                                        // })
                                        Block::SvgBlock(SvgProps { name: SvgName::OST126PTSDAtWorkInfographic0 }),
                                        
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/48_08cff1ca-fb62-4bef-83be-136fb27d9204-legend.html"),


                                    ]
                                },
                                Tab {
                                    name: String::from("Overal Economic Burden"),
                                    blocks: vec![
                                        // Block::PieChartBlock(PieChartProps {
                                        //     title: String::from("Figure 2. Excess Economic Burden of PTSD in the US Civilian Population in 2018, Billion USD"),
                                        //     slices: vec![
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct health care costs"), value: 34.8 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct non-health care costs"), value: 8.9 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of unemployment"), value: 22.5 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of productivity loss"), value: 15.4 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs due to caregiving"), value: 17.6 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of premature mortality"), value: 0.7 }
                                        //         ]
                                        // })
                                        Block::SvgBlock(SvgProps { name: SvgName::OST126PTSDAtWorkInfographic1 }),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/48_08cff1ca-fb62-4bef-83be-136fb27d9204-legend.html"),

                                    ]
                                },
                                Tab {
                                    name: String::from("Overal Economic Burden"),
                                    blocks: vec![
                                        // Block::PieChartBlock(PieChartProps {
                                        //     title: String::from("Figure 3. Excess Economic Burden of PTSD in the US Military Population in 2018, Billion USD "),
                                        //     slices: vec![
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct health care costs"), value: 23.6 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct non-health care costs"), value: 44.0 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of unemployment"), value: 8.3 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of productivity loss"), value: 13.1 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs due to caregiving"), value: 8.0 },
                                        //         PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of premature mortality"), value: 2.9 }
                                        //         ]
                                        // })
                                        Block::SvgBlock(SvgProps { name: SvgName::OST126PTSDAtWorkInfographic2 }),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/48_08cff1ca-fb62-4bef-83be-136fb27d9204-legend.html"),


                                    ]
                                }
                            ]
                        }),

                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/50_4311e0a5-f350-46eb-89e6-82bd4cf6e02b.html"),

                        
                        Block::TableBlock(TableProps {
                            dimension: [3,2],
                            headers: vec![],
                            rows: vec![
                                vec![
                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126TimingOfIndexTrauma}),
                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/53_e89191c1-76c7-49bf-b03d-7ecdb2df1f7a.html"),

                                ],
                                vec![
                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126TimingOfIndexTrauma}),
                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/55_eadd4628-57cf-4638-9031-0e18d4b825a8.html"),

                                ],
                                vec![
                                    Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126TimingOfIndexTrauma}),
                                    rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/57_1e60c89c-1dca-4e61-b5c2-62f3c763962b.html"),

                                ],


        
                            ],
                        }),
                    ],
                },
            ],
        }),
        Block::ReferencesBlock(Box::new(ReferencesProps {
            references:rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/60_571974e7-bc2e-450f-98bf-8e6a79c0ca7b.html")                                       
        })),
    ]
}
