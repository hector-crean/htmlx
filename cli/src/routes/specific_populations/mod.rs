use crate::rich_text_block;
use blocks::block::bar_chart::{BarChartDatum, BarChartProps};
use blocks::block::grid_table::GridTableProps;
use blocks::block::html::HtmlProps;
use blocks::block::icon::{IconProps, SvgProps};
use blocks::block::partition_chart::{PartitionChartNode, PartitionChartProps};
use blocks::block::references::ReferencesProps;
use blocks::block::suggested_node::SuggestedNodeProps;
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme};
use blocks::block::text_with_icon_list::TextWithIconList;
use blocks::block::{html, Block};
use blocks::span::ref_note::RefNote;
use blocks::SvgName;
use maud::{html, Render};
use std::vec;

pub struct Page;

impl Page {
    fn civilian_and_general_population_tab(&self) -> Vec<Block> {
        vec![
            Block::Html(html! {
                div class="flex flex-col gap-12 text-center panel" {
                    div class="grid w-full grid-cols-2 gap-x-2" {
                        div class="flex flex-col items-center justify-center col-span-2"{
                            (Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDGlobalUSInfographic}))
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span { "The " strong { "global" } " lifetime prevalence of PTSD is " span class="bg-[#005178] text-[#d2e3ee]" { "4-10%"}(RefNote::new(1))(RefNote::new(2))}
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span {"PTSD is one of the most common mental health disorders in the " strong { "US" } ", with " span class="bg-[#005178] text-[#d2e3ee]"{"7-8 out of every 100 people"} " experiencing PTSD at some point in their lifetime"(RefNote::new(3))(RefNote::new(4))}
                        }
                    }
                    div class="p-2 bg-[#accbdd]"{
                        (Block::SvgBlock(SvgProps { name: SvgName::OTS12649Infographic}))
                        div{
                            span class="bg-[#005178] text-[#d2e3ee]  rounded-sm px-2" {"~13 million"}
                            span { " adults in the US will experience PTSD during a given year "(RefNote::new(5))}
                        }
                    }
                    div class="p-2" {
                        (Block::SvgBlock(SvgProps { name: SvgName::OTS12620PercentMilitarty}))

                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span { span class="bg-[#005178] text-[#d2e3ee]"{">80%"} " of PTSD patients are in the general population rather than the military population"(RefNote::new(6))}
                        }
                    }
                    div class="p-2 bg-[#accbdd]" {
                        (Block::SvgBlock(SvgProps { name: SvgName::OTS12623Transparent}))

                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span {"Typically PTSD first appears during young and middle adulthood. Among adults in the United States, the " span class="bg-[#005178] text-[#d2e3ee]"{ "median age of onset is around 23 years old"}(RefNote::new(7))}
                        }
                    }
                }
            }),
            // Block::PartitionChartBlock(PartitionChartProps {
            //     title: "USA PTSD prevalence".to_string(),
            //     data: vec![PartitionChartNode::Branch {
            //         id: uuid::Uuid::new_v4(),
            //         label: "USA".to_string(),
            //         fill: None,
            //         children: vec![
            //             PartitionChartNode::Branch {
            //                 id: uuid::Uuid::new_v4(),
            //                 label: "PTSD".to_string(),
            //                 fill: None,
            //                 children: vec![
            //                     PartitionChartNode::Leaf {
            //                         id: uuid::Uuid::new_v4(),
            //                         label: "Civilian".to_string(),
            //                         fill: None,
            //                         value: 10.4,
            //                     },
            //                     PartitionChartNode::Leaf {
            //                         id: uuid::Uuid::new_v4(),
            //                         label: "Military".to_string(),
            //                         fill: None,
            //                         value: 2.6,
            //                     },
            //                 ],
            //             },
            //             PartitionChartNode::Leaf {
            //                 id: uuid::Uuid::new_v4(),
            //                 label: "No PTSD".to_string(),
            //                 fill: None,
            //                 value: 320.,
            //             },
            //         ],
            //     }],
            // }),
            Block::Html(html! {
                div class="panel"{
                   "The majority of individuals exposed to trauma do not develop PTSD"(RefNote::new(1))". However, the type of trauma experience influences the likelihood of developing PTSD."
                   (Block::SuggestedNodeBlock(SuggestedNodeProps::new("Trauma in PTSD", "trauma")))

                }

            }),
        ]
    }
    fn ptsd_in_women_tab(&self) -> Vec<Block> {
        vec![Block::Html(html! {
                div class="panel" {
                    h2 { "PTSD in Women "}

                div  {
                    div class="grid w-full grid-cols-2 gap-2" {
                        div class="flex flex-col items-center justify-center p-2 rounded-md " {
                            (Block::SvgBlock(SvgProps { name: SvgName::OTS126FemaleIcon}))
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span {
                                "In the general population, women are " span class="bg-[#005178] text-[#d2e3ee]" {"2x"} " as likely as men to develop PTSD, partly due to their increased exposure to traumatic events closely associated with PTSD, such as sexual abuse and rape"(RefNote::new(1))". Women account for " span class="bg-[#005178] text-[#d2e3ee]" {"66.4%"} " of the overall PTSD population in the US"(RefNote::new(1))"."
                            }
                        }
                    }
                }
                div  {
                    p class="py-2" {
                        "Even after adjusting for differences in trauma exposure and prior victimization or abuse history, women still exhibit a significantly elevated risk of developing PTSD compared to men, with a lifetime prevalence of PTSD of 13% in women and 6% in men, suggesting a higher susceptibility among women"(RefNote::new(6))". Generic research suggests a greater heritability risk in women, with genes like adenylate cyclase activating polypeptide 1 (pituitary) receptor (ADCYAP1R1) showing allelic variations linked to PTSD risk"(RefNote::new(1))". Ultimately, the heightened prevalence of PTSD among women likely arises from a combination of increased trauma exposure and inherent vulnerabilities"(RefNote::new(1))". Additionally, females in the general population experience PTSD for a longer duration than males"(RefNote::new(5))". "
                    }
                }
                }





        })]
    }
    fn military_population_tab(&self) -> Vec<Block> {
        vec![
            Block::Html(html! {
                div class="flex flex-col gap-2 panel" {
                    h2 { "PTSD prevalence within the military"(RefNote::new(4))}
                    div class="grid items-center justify-center grid-cols-2 overflow-hidden text-center rounded-md" {
                        div class="flex flex-col items-center justify-center w-full p-2 " {
                            (Block::SvgBlock(SvgProps {
                                name: SvgName::OTS126WarVeteranIcon,
                            }))
                        }
                        div class="text-xl" { "Male war veterans " span class="bg-[#005178] text-[#d2e3ee]" { "7.7%"}}
                        div class="flex flex-col items-center justify-center w-full p-2 " {
                            (Block::SvgBlock(SvgProps {
                                name: SvgName::OTS126FemaleWarVeteranIcon,
                            }))
                        }

                        div class="text-xl" { "Female war veterans " span class="bg-[#005178] text-[#d2e3ee]" { "13.4%"}}

                    }
                    (  Block::GridTableBlock(GridTableProps {
                        headers: vec![],
                        rows: vec![
                            vec![
                                Block::Html(html! { strong {  "Heightened exposure to trauma"}}),
                                Block::Html(html! {
                                p {
                                    "PTSD is a significant issue among U.S military veterans, often stemming from exposure to traumatic events during service"(RefNote::new(4))(RefNote::new(6))}
                                }),
                            ],
                            vec![
                                Block::Html(html! {  strong { "Gender differences"}}),
                                Block::Html(html! {
                                    p {
                                    "PTSD diagnoses among military personnel vary based on gender, with the prevalence of PTSD higher among female service members"(RefNote::new(4))". "
                                    "Male veterans reported higher levels of war zone exposure, while female veterans reported experiencing interpersonal violence and military sex trauma (MST)"(RefNote::new(4))(RefNote::new(6))". "
                                    }
                                }),
                            ],
                            vec![
                                Block::Html(
                                    html! { strong { "Peaks in military PTSD incidence over the years"}},
                                ),
                                Block::Html(html! {
                                   p {
                                    "Increase in PTSD cases align with times of heightened military engagement, such as deployments following the events of 9/11"(RefNote::new(6))". "
                                    "Among the 2.7 million personnel deployed to Iraq or Afghanistan since 2001, it's estimated that 5-20% have experienced PTSD"(RefNote::new(7))". "
                                   }
                                }),
                            ],
                        ],
                    }))
                }

            }),
            // Block::IconBlock(IconProps { name: SvgName::OTS126ExposureGenderPeaksTable}),
        ]
    }
    fn marginalized_groups_tab(&self) -> Vec<Block> {
        vec![
            Block::Html(html! {
                div class="panel" {
                    h2 { "LGBTQ+" }
                    div class="grid w-full grid-cols-2 grid-rows-2 gap-2" {
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            (Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDWithinLGBTQInfographicGAY}))
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span { "Up to " span class="bg-[#005178] text-[#d2e3ee]" { "48%" } " among lesbian, gay and bisexual individuals"(RefNote::new(9))}
                        }

                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            (Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDWithinLGBTQInfographicTRANS}))
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span {"Up to " span class="bg-[#005178] text-[#d2e3ee]" { "48%" } " among transgender and gender diverse individuals"(RefNote::new(9))}
                        }
                    }
                    ( Block::GridTableBlock(GridTableProps {
                        headers: vec![],
                        rows: vec![
                            vec![
                                Block::Html(html! { strong { "Heightened exposure to trauma"}}),
                                Block::Html(html! {
                                span {
                                "The LGBTQ+ community faces a disproportionately high risk of trauma and PTSD compared to cisgender/heterosexual individuals, primarily due to increased exposure to hate crimes, intimate partner violence, and sexual assaults"(RefNote::new(10))}
                                }),
                            ],
                            vec![
                                Block::Html(html! { strong { "Childhood abuse"}}),
                                Block::Html(html! {
                                span {
                                "Childhood abuse is more prevalent among sexual minority children, significantly contributing to mental health disparities, particularly in PTSD, and accounting for up to half of the disparities in PTSD based on sexual orientation"(RefNote::new(10))(RefNote::new(11))}
                                }),
                            ],
                            vec![
                                Block::Html(html! { strong { "Internalized stigma"}}),
                                Block::Html(html! {
                                    span {
                                        "Research suggests that internalized homophobia (IH) predicts the severity of PTSD symptoms in LGBTQ+ minorities with trauma histories"(RefNote::new(10))"."
                                        "IH in sexual minorities correlates with depression, substance use, and low self-esteem, and reduces the likelihood of recovery from traumatic events such as sexual assault and hate crimes"(RefNote::new(12))"."
                                    }
                                }),
                            ],
                            vec![
                                Block::Html(html! { strong { "Social stressors"}}),
                                Block::Html(html! {
                                    span {
                                        "LGBTQ individuals experience elevated levels of discrimination, victimization, and minority stress. These social stressors often prompt feelings of shame and the concealment of their minority identity, contributing to heightened mental and physical health challenges within the LGBTQ+ community"(RefNote::new(9))"."
                                    }
                                }),
                            ],
                        ],
                    }))
                }
            }),
            Block::Html(html! {
               div class="panel" {
                h2 { "Race" }
                // ( Block::BarChartBlock(BarChartProps {
                //     title: "Race-PTSD prevalence".to_string(),
                //     slices: vec![
                //         BarChartDatum {
                //             id: uuid::Uuid::new_v4(),
                //             label: "Black".to_string(),
                //             value: 8.7,
                //             fill: Some("#7A4D3B".to_string()),
                //         },
                //         BarChartDatum {
                //             id: uuid::Uuid::new_v4(),
                //             label: "White".to_string(),
                //             value: 7.4,
                //             fill: Some("#FFCEA7".to_string()),
                //         },
                //         BarChartDatum {
                //             id: uuid::Uuid::new_v4(),
                //             label: "Hispanic".to_string(),
                //             value: 7.0,
                //             fill: Some("#DB8C68".to_string()),
                //         },
                //         BarChartDatum {
                //             id: uuid::Uuid::new_v4(),
                //             label: "Asian".to_string(),
                //             value: 4.0,
                //             fill: Some("#F9DAA6".to_string()),
                //         },
                //     ],
                // }))
                h3 { "PTSD Prevalence"(RefNote::new(13))}
                (SvgProps { name: SvgName::OTS126PTSDRaceInfographicCopy})
                ul class="list" {
                    li {
                        "Black individuals experience more frequent, severe, and chronic PTSD symptoms compared to White individuals"(RefNote::new(14))"."
                    }
                    li {
                        "Despite higher rates of PTSD and trauma exposure among low-income Black individuals, racial disparities persist even when controlling for these factors"(RefNote::new(14))"."
                    }
                    li {
                        "Racial discrimination triggers negative emotional states and heightened stress responses, increasing vulnerability to mental health difficulties like depression, anxiety, and substance use"(RefNote::new(14))"."
                    }
                    li {
                        "Chronic exposure to racial discrimination exacerbates mental health disparities in Black populations and has been associated with increased severity of PTSD symptoms"(RefNote::new(14))(RefNote::new(15))"."
                    }
                   }
               }
            }),
            Block::Html(html! {
               div class="panel" {
                h2 {
                    "Increased risk of PTSD is associated with a number of demographic and behavioral factors"(RefNote::new(5))(RefNote::new(16))
                }

                (TextWithIconList { list: vec![
                    (IconProps { name: SvgName::OTS126FemaleSex}, "Female Sex".to_string()),
                    (IconProps { name: SvgName::OTS126YoungerAge65Years}, "Younger age (< 65 years)".to_string()),
                    (IconProps { name: SvgName::OTS126BeingDivorced}, "Being divorced".to_string()),
                    (IconProps { name: SvgName::OTS126BeingOfLowIncome}, "Being of low income".to_string()),
                    (IconProps { name: SvgName::OTS126DiagnosedMentalIllness}, "Diagnosed metnal illness".to_string()),
                    (IconProps { name: SvgName::OTS126SubstanceUseDisorder}, "Substance use disorder".to_string()),
                    (IconProps { name: SvgName::OTS126DrugUseDisorder}, "Drug use disorder".to_string()),
                    (IconProps { name: SvgName::OTS126AlcoholUseDisorder}, "Alcohol use disorder".to_string()),

                ]})
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
                        blocks: self.ptsd_in_women_tab(),
                    },
                    Tab {
                        name: String::from("Military Population"),
                        blocks: self.military_population_tab(),
                    },
                    Tab {
                        name: String::from("LGBTQ+ and marginalized groups"),
                        blocks: self.marginalized_groups_tab()
                    },
                ],
            }))
        (Block::ReferencesBlock(Box::new(ReferencesProps {
            references: Block::Html(
                html! {
                    ol class="list" {
                        li {
                            p {
                                "Yehuda R, Hoge CW, McFarlane AC, et al. Post-traumatic stress disorder. " em { "Nat Rev Dis Primers" }
                                ". 2015;1:15057. Published 2015 Oct 8. doi:10.1038/nrdp.2015.57"
                            }
                        }
                        li {
                            p {
                                "Kilpatrick DG, Resnick HS, Milanak ME, Miller MW, Keyes KM, Friedman MJ. National estimates of exposure to traumatic events and PTSD prevalence using DSM-IV and DSM-5 criteria. " em { "J Trauma Stress" }
                                ". 2013;26(5):537-547. doi:10.1002/jts.21848"
                            }
                        }
                        li {
                            p {
                                "Koenen KC, Sumner JA, Gilsanz P, et al. Post-traumatic stress disorder and cardiometabolic disease: improving causal inference to inform practice. " em { "Psychol Med" }
                                ". 2017;47(2):209-225. doi:10.1017/S0033291716002294"
                            }
                        }
                        li {
                            p {
                                "Lehavot K, Katon JG, Chen JA, Fortney JC, Simpson TL. Post-traumatic Stress Disorder by Gender and Veteran Status [published correction appears in Am J Prev Med. 2019 Oct;57(4):573]. " em { "Am J Prev Med" }
                                ". 2018;54(1):e1-e9. doi:10.1016/j.amepre.2017.09.008"
                            }
                        }
                        li {
                            p {
                                a href="http://5.Otsuka" target="_blank" rel="noopener noreferrer nofollow" { "Otsuka" }
                                " data on file, DSE Module 1"
                            }
                        }
                        li {
                            p {
                                "Judkins JL, Moore BA, Collette TL, Hale WJ, Peterson AL, Morissette SB. Incidence Rates of Posttraumatic Stress Disorder Over a 17-Year Period in Active Duty Military Service Members. " em { "J Trauma Stress" }
                                ". 2020;33(6):994-1006. doi:10.1002/jts.22558"
                            }
                        }
                        li {
                            p {
                                "What Predicts Persistent Posttraumatic Stress Disorder Among Military Personnel? PsychU. Published October 15, 2018. Accessed April 12, 2024. "
                                a href="https://psychu.org/predicts-persistent-posttraumatic-stress-disorder-among-military-personnel/" target="_blank" rel="noopener noreferrer nofollow" { "https://psychu.org/predicts-persistent-posttraumatic-stress-disorder-among-military-personnel/" }
                            }
                        }
                        li {
                            p {
                                "62% Of Military Discharged For Misconduct Were Diagnosed With Mental Illness Or Traumatic Brain Injury. PsychU. Published June 16, 2017. Accessed April 12, 2024. "
                                a href="https://psychu.org/62-military-discharged-misconduct-diagnosed-mental-illness-traumatic-brain-injury/" target="_blank" rel="noopener noreferrer nofollow" { "https://psychu.org/62-military-discharged-misconduct-diagnosed-mental-illness-traumatic-brain-injury/" }
                            }
                        }
                        li {
                            p {
                                "Livingston NA, Berke D, Scholl J, Ruben M, Shipherd JC. Addressing Diversity in PTSD Treatment: Clinical Considerations and Guidance for the Treatment of PTSD in LGBTQ Populations. " em { "Curr Treat Options Psychiatry" }
                                ". 2020;7(2):53-69. doi:10.1007/s40501-020-00204-0"
                            }
                        }
                        li {
                            p {
                                "Marchi M, Travascio A, Uberti D, et al. Post-traumatic stress disorder among LGBTQ people: a systematic review and meta-analysis. " em { "Epidemiol Psychiatr Sci" }
                                ". 2023;32:e44. Published 2023 Jul 11. doi:10.1017/S2045796023000586"
                            }
                        }
                        li {
                            p {
                                "Roberts AL, Rosario M, Corliss HL, Koenen KC, Austin SB. Elevated risk of posttraumatic stress in sexual minority youths: mediation by childhood abuse and gender nonconformity. " em { "Am J Public Health" }
                                ". 2012;102(8):1587-1593. doi:10.2105/AJPH.2011.300530"
                            }
                        }
                        li {
                            p {
                                a href="http://12.Gold" target="_blank" rel="noopener noreferrer nofollow" { "Gold" }
                                " SD, Feinstein BA, Skidmore WC, Marx BP. Childhood physical abuse, internalized homophobia, and experiential avoidance among lesbians and gay men. " em { "Psychological Trauma: Theory, Research, Practice, and Policy" }
                                ". 2011;3(1):50-60. doi: "
                                a href="https://doi.org/10.1037/a0020487" target="_blank" rel="noopener noreferrer nofollow" { "https://doi.org/10.1037/a0020487" }
                            }
                        }
                        li {
                            p {
                                "Roberts AL, Gilman SE, Breslau J, Breslau N, Koenen KC. Race/ethnic differences in exposure to traumatic events, development of post-traumatic stress disorder, and treatment-seeking for post-traumatic stress disorder in the United States. " em { "Psychol Med" }
                                ". 2011;41(1):71-83. doi:10.1017/S0033291710000401"
                            }
                        }
                        li {
                            p {
                                "Mekawi Y, Carter S, Brown B, et al. Interpersonal Trauma and Posttraumatic Stress Disorder among Black Women: Does Racial Discrimination Matter?. " em { "J Trauma Dissociation" }
                                ". 2021;22(2):154-169. doi:10.1080/15299732.2020.1869098"
                            }
                        }
                        li {
                            p {
                                "Brooks Holliday S, Dubowitz T, Haas A, Ghosh-Dastidar B, DeSantis A, Troxel WM. The association between discrimination and PTSD in African Americans: exploring the role of gender. " em { "Ethn Health" }
                                ". 2020;25(5):717-731. doi:10.1080/13557858.2018.1444150"
                            }
                        }
                        li {
                            p {
                                "Kessler RC, Chiu WT, Demler O, Merikangas KR, Walters EE. Prevalence, severity, and comorbidity of 12-month DSM-IV disorders in the National Comorbidity Survey Replication [published correction appears in Arch Gen Psychiatry. 2005 Jul;62(7):709. Merikangas, Kathleen R [added]]. " em { "Arch Gen Psychiatry" }
                                ". 2005;62(6):617-627. doi:10.1001/archpsyc.62.6.617"
                            }
                        }
                    }
                }

            )
        })))
        }
    }
}
