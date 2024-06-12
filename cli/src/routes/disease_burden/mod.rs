use crate::{rich_text_block};
use blocks::block::pie_chart::{PieChartDatum, PieChartProps};
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
                    name: String::from("Personal Burden"),
                    blocks: vec![
                        Block::TabsBlock( TabsProps { 
                            id: uuid::Uuid::new_v4(), 
                            tabs: vec![
                                Tab {
                                    name: String::from("Health"),
                                    blocks: vec![
                                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/c15e1e82-6b73-4acb-b146-bbccb5cdc4ce.html"),
                                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/5b200e5b-9a50-4d19-976c-9f0f38132b85.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/b7101e4f-a8ca-4165-af53-bc96183a0364.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/0cdc8891-6de9-48b6-92f4-2293f0579622.html"),
                                  
                                    ]
                                },
                                Tab {
                                    name: String::from("Family / Caregiver"),
                                    blocks: vec![
                                 
                                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/475fe6d4-5fe5-4d6a-b9e0-a823b48caf42.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/2e76714b-8d07-4557-849f-502525326146.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/a80701fb-893f-42b9-9cee-ece46f9ea204.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/cd6c8d9b-1f82-4cfd-9462-772585bc83d8.html"),
                                       
                                      

                                    ]
                                },
                                Tab {
                                    name: String::from("Work and Finance"),
                                    blocks: vec![
                                 
                                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/e6094d26-1207-478e-85e4-04cd070a73cf.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/c40fb323-fed0-494d-9a0e-974c0e2ed032.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/f54b078d-8dde-4ebe-97f8-f8e4e97a8e4b.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/c8a00e01-90a2-4a80-97a0-4415b831aa17.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/40b13af6-fdb1-4242-8330-1e651d103947.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/718cc37f-4463-4116-ae02-a965372736ec.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/2cc53ac7-26f8-4771-b696-fae065dae509.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/1bd244ce-d2c4-4ed2-8dc4-94a3641ae56a.html"),
                                        rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/a0bdcb3b-be14-4dd6-bbb6-78a8cc8b1a61.html"),
                                      
                                      

                                    ]
                                }
                            ], 
                            representation: TabsRepresentation::Standard })
                    ],
                },
                Tab {
                    name: String::from("Societal Burden"),
                    blocks: vec![
                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/08cff1ca-fb62-4bef-83be-136fb27d9204.html"),
                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/4311e0a5-f350-46eb-89e6-82bd4cf6e02b.html"),
                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/845c4825-c252-46ef-ae28-14fee999e820.html"),
                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/e89191c1-76c7-49bf-b03d-7ecdb2df1f7a.html"),
                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/eadd4628-57cf-4638-9031-0e18d4b825a8.html"),
                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/1e60c89c-1dca-4e61-b5c2-62f3c763962b.html"),
                        // rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/2c16e8e2-9200-46f4-9d71-8e386e425975.html"),
                        Block::TabsBlock(TabsProps {
                            id: uuid::Uuid::new_v4(),
                            representation: TabsRepresentation::Swiper,
                            tabs: vec![
                                Tab {
                                    name: String::from("Overal Economic Burden"),
                                    blocks: vec![
                                        Block::PieChartBlock(PieChartProps {
                                            title: String::from("Overall"),
                                            slices: vec![
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct health care costs"), value: 32.8 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct non-health care costs"), value: 15.4 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of unemployment"), value: 19.9 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of productivity loss"), value: 15. },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs due to caregiving"), value: 15.8 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of premature mortality"), value: 1.1 }
                                                ]
                                        })
                                    ]
                                },
                                Tab {
                                    name: String::from("Overal Economic Burden"),
                                    blocks: vec![
                                        Block::PieChartBlock(PieChartProps {
                                            title: String::from("Overall"),
                                            slices: vec![
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct health care costs"), value: 34.8 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct non-health care costs"), value: 8.9 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of unemployment"), value: 22.5 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of productivity loss"), value: 15.4 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs due to caregiving"), value: 17.6 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of premature mortality"), value: 0.7 }
                                                ]
                                        })
                                    ]
                                },
                                Tab {
                                    name: String::from("Overal Economic Burden"),
                                    blocks: vec![
                                        Block::PieChartBlock(PieChartProps {
                                            title: String::from("Overall"),
                                            slices: vec![
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct health care costs"), value: 23.6 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess direct non-health care costs"), value: 44.0 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of unemployment"), value: 8.3 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of productivity loss"), value: 13.1 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs due to caregiving"), value: 8.0 },
                                                PieChartDatum { id: uuid::Uuid::new_v4(), label: String::from("Excess costs of premature mortality"), value: 2.9 }
                                                ]
                                        })
                                    ]
                                }
                            ]
                        })
                    ],
                },
            ],
        }),
        Block::ReferencesBlock(Box::new(ReferencesProps {
            references:rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/571974e7-bc2e-450f-98bf-8e6a79c0ca7b.html")                                       
        })),
    ]
}
