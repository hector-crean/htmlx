use crate::rich_text_block;
use blocks::block::bar_chart::{BarChartDatum, BarChartProps};
use blocks::block::grid_table::GridTableProps;
use blocks::block::html::HtmlProps;
use blocks::block::icon::{IconProps, SvgProps};
use blocks::block::references::ReferencesProps;
use blocks::block::suggested_node::SuggestedNodeProps;
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme};
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
                div class="panel" {
                    (Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDGlobalUSInfographic}))
                    div class="grid w-full grid-cols-2 gap-2" {
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span { "The " strong { "global" } " lifetime prevalence of PTSD is " span class="bg-[#005178] text-[#d2e3ee]" { "4-10%"}(RefNote::new(1))(RefNote::new(2))}
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span {"PTSD is one of the most common mental health disorders in the " strong { "US" } ", with " span class="bg-[#005178] text-[#d2e3ee]"{"7-8 out of every 100 people"} " experiencing PTSD at some point in their lifetime"(RefNote::new(3))(RefNote::new(4))}
                        }
                    }

                }
            }),
            Block::Html(html! {
                div class="panel" {
                    div class="grid grid-cols-2"{
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span class="bg-[#005178] text-[#d2e3ee] text-4xl rounded-sm p-2"{"~13 million"}
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span { "adults in the US will experience PTSD during a given year "(RefNote::new(5))}
                        }
                    }
                }


            }),
            Block::Html(html! {
                div class="panel" {
                    (Block::Html(html! {
                        div class="grid grid-cols-10"{
                            @for i in 1..=10{
                                @match i {
                                    1..=2 => {
                                        (Block::SvgBlock(SvgProps { name: SvgName::Military}))
                                    }
                                    _ => {
                                        (Block::SvgBlock(SvgProps { name: SvgName::Hominid}))
                                    }
                                }


                            }
                        }
                    }))
                    div class="flex flex-col items-center justify-center p-2 rounded-md" {
                        span { span class="bg-[#005178] text-[#d2e3ee]"{">80%"} " of PTSD patients are in the general population rather than the military population"(RefNote::new(6))}
                    }
                }


            }),
            Block::Html(html! {
                div class="panel"{
                    ( Block::SvgBlock(SvgProps { name: SvgName::OTS126TypicalAgePTSDText}))
                    div class="flex flex-col items-center justify-center p-2 rounded-md" {
                        span {"The typical age at which PTSD first appears is during young and middle adulthood. Among adults in the United States, the " span class="bg-[#005178] text-[#d2e3ee]"{ "median age of onset is around 23 years old"}(RefNote::new(7))}
                    }
                }
            }),
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
                h3 { "PTSD in Women "}

                div class="panel" {
                    div class="grid w-full grid-cols-2 gap-2" {
                        div class="flex flex-col items-center justify-center p-2 rounded-md " {
                            (Block::SvgBlock(SvgProps { name: SvgName::Woman}))
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span {
                                "In the general population, women are " span class="bg-[#005178] text-[#d2e3ee]" {"2x"} " as likely as men to develop PTSD, partly due to their increased exposure to traumatic events closely associated with PTSD, such as sexual abuse and rape"(RefNote::new(1))". Women account for " span class="bg-[#005178] text-[#d2e3ee]" {"66.4%"} " of the overall PTSD population in the US"(RefNote::new(1))"."
                            }
                        }
                    }
                }
                div class="panel" {
                    p class="py-2" {
                        "Even after adjusting for differences in trauma exposure and prior victimization or abuse history, women still exhibit a significantly elevated risk of developing PTSD compared to men, with a lifetime prevalence of PTSD of 13% in women and 6% in men, suggesting a higher susceptibility among women"(RefNote::new(6))". Generic research suggests a greater heritability risk in women, with genes like adenylate cyclase activating polypeptide 1 (pituitary) receptor (ADCYAP1R1) showing allelic variations linked to PTSD risk"(RefNote::new(1))". Ultimately, the heightened prevalence of PTSD among women likely arises from a combination of increased trauma exposure and inherent vulnerabilities"(RefNote::new(1))". Additionally, females in the general population experience PTSD for a longer duration than males"(RefNote::new(5))". "
                    }
                }





        })]
    }
    fn military_population_tab(&self) -> Vec<Block> {
        vec![
            Block::Html(html! {
                div {
                    div class="grid items-center justify-center grid-cols-2" {
                        div class="flex flex-col items-center justify-center w-full p-2 bg-[#bbd5e7]" {
                            (Block::SvgBlock(SvgProps {
                                name: SvgName::Hominid,
                            }))
                        }
                        div class="flex flex-col items-center justify-center w-full  p-2 bg-[#bbd5e7]" {
                            (Block::SvgBlock(SvgProps {
                                name: SvgName::Woman,
                            }))
                        }
                        div { "Male war veterans " span class="bg-[#005178] text-[#d2e3ee]" { "7.7%"}}
                        div { "Female war veterans " span class="bg-[#005178] text-[#d2e3ee]" { "13.4%"}}

                    }
                }
            }),
            // Block::IconBlock(IconProps { name: SvgName::OTS126ExposureGenderPeaksTable}),
            Block::GridTableBlock(GridTableProps {
                rows: vec![
                    vec![
                        Block::Html(html! { "Heightened exposure to trauma"}),
                        Block::Html(
                            html! { "PTSD is a significant issue among U.S military veterans, often stemming from exposure to traumatic events during service"(RefNote::new(4))(RefNote::new(6))},
                        ),
                    ],
                    vec![
                        Block::Html(html! { "Gender differences"}),
                        Block::Html(html! {
                            "PTSD diagnoses among military personnel vary based on gender, with the prevalence of PTSD higher among female service members"(RefNote::new(4))". "
                            "Male veterans reported higher levels of war zone exposure, while female veterans reported experiencing interpersonal violence and military sex trauma (MST)"(RefNote::new(4))(RefNote::new(6))". "
                        }),
                    ],
                    vec![
                        Block::Html(html! { "Peaks in military PTSD incidence over the years"}),
                        Block::Html(html! {
                            "Increase in PTSD cases align with times of heightened military engagement, such as deployments following the events of 9/11"(RefNote::new(6))". "
                            "Among the 2.7 million personnel deployed to Iraq or Afghanistan since 2001, it's estimated that 5-20% have experienced PTSD"(RefNote::new(7))". "
                        }),
                    ],
                ],
            }),
        ]
    }
    fn marginalized_groups_tab(&self) -> Vec<Block> {
        vec![
            Block::Html(html! { h3 { "LGBTQ+" }}),
            Block::Html(html! {
                div class="panel" {
                    (Block::SvgBlock(SvgProps { name: SvgName::OTS126PTSDWithinLGBTQInfographic}))
                    div class="grid w-full grid-cols-2 gap-2" {
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span { "Up to " span class="bg-[#005178] text-[#d2e3ee]" { "48%" } " among lesbian, gay and bisexual individuals"(RefNote::new(9))}
                        }
                        div class="flex flex-col items-center justify-center p-2 rounded-md" {
                            span {"Up to " span class="bg-[#005178] text-[#d2e3ee]" { "48%" } " among transgender and gender diverse individuals"(RefNote::new(9))}
                        }
                    }
                }
            }),
            Block::GridTableBlock(GridTableProps {
                rows: vec![
                    vec![
                        Block::Html(html! { span { "Heightened exposure to trauma"}}),
                        Block::Html(html! {
                        span {
                        "The LGBTQ+ community faces a disproportionately high risk of trauma and PTSD compared to cisgender/heterosexual individuals, primarily due to increased exposure to hate crimes, intimate partner violence, and sexual assaults"(RefNote::new(10))}
                        }),
                    ],
                    vec![
                        Block::Html(html! { span { "Childhood abuse"}}),
                        Block::Html(html! {
                        span {
                        "Childhood abuse is more prevalent among sexual minority children, significantly contributing to mental health disparities, particularly in PTSD, and accounting for up to half of the disparities in PTSD based on sexual orientation"(RefNote::new(10))(RefNote::new(11))}
                        }),
                    ],
                    vec![
                        Block::Html(html! { span { "Internalized stigma"}}),
                        Block::Html(html! {
                            span {
                                "Research suggests that internalized homophobia (IH) predicts the severity of PTSD symptoms in LGBTQ+ minorities with trauma histories"(RefNote::new(10))"."
                                "IH in sexual minorities correlates with depression, substance use, and low self-esteem, and reduces the likelihood of recovery from traumatic events such as sexual assault and hate crimes"(RefNote::new(12))"."
                            }
                        }),
                    ],
                    vec![
                        Block::Html(html! { span { "Social stressors"}}),
                        Block::Html(html! {
                            span {
                                "LGBTQ individuals experience elevated levels of discrimination, victimization, and minority stress. These social stressors often prompt feelings of shame and the concealment of their minority identity, contributing to heightened mental and physical health challenges within the LGBTQ+ community"(RefNote::new(9))"."
                            }
                        }),
                    ],
                ],
            }),
            Block::Html(html! { h3 { "Race" }}),
            // Block::BarChartBlock(BarChartProps {
            //     title: "Race-PTSD prevalence".to_string(),
            //     bars: vec![],
            // }),
            Block::SvgBlock(SvgProps {
                name: SvgName::OTS126PTSDRaceInfographic,
            }),
            Block::BarChartBlock(BarChartProps {
                title: "Race-PTSD prevalence".to_string(),
                slices: vec![
                    BarChartDatum {
                        id: uuid::Uuid::new_v4(),
                        label: "Black".to_string(),
                        value: 8.7,
                    },
                    BarChartDatum {
                        id: uuid::Uuid::new_v4(),
                        label: "White".to_string(),
                        value: 7.4,
                    },
                    BarChartDatum {
                        id: uuid::Uuid::new_v4(),
                        label: "Hispanic".to_string(),
                        value: 7.0,
                    },
                    BarChartDatum {
                        id: uuid::Uuid::new_v4(),
                        label: "Asian".to_string(),
                        value: 4.0,
                    },
                ],
            }),
            Block::Html(html! {
               div class="panel" {
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
            Block::GridTableBlock(GridTableProps {
                rows: vec![
                    vec![
                        Block::Html(html! { span { "Female sex"}}),
                        Block::Html(html! { span { "Diagnosed mental illness"}}),
                    ],
                    vec![
                        Block::Html(html! { span { "Younger age (<65 years)"}}),
                        Block::Html(html! { span { "Substance use disorder"}}),
                    ],
                    vec![
                        Block::Html(html! { span { "Being divorced"}}),
                        Block::Html(html! { span { "Drug use disorder"}}),
                    ],
                    vec![
                        Block::Html(html! { span { "Being of low income"}}),
                        Block::Html(html! { span { "Alcohol use disorder"}}),
                    ],
                ],
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
            references: rich_text_block!("../../input/OTS126_PTSD_Specific_Populations_Node/958c48d1-9ddb-4c8d-a301-d7ccd9c6c104.html"),
        })))
        }
    }
}
