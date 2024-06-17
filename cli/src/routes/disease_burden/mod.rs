use crate::{rich_text, rich_text_block};
use blocks::block::grid_table::GridTableProps;
use blocks::block::html::HtmlProps;
use blocks::block::icon::{IconProps, SvgProps};
use blocks::block::pie_chart::{PieChartDatum, PieChartProps};
use blocks::block::references::ReferencesProps;
use blocks::block::rich_text::{self, RichTextProps};
use blocks::block::suggested_node::SuggestedNodeProps;
use blocks::block::table::TableProps;
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme};
use blocks::block::Block;
use blocks::span::ref_note::RefNote;
use blocks::SvgName;
use maud::{html, PreEscaped, Render};
use std::vec;

pub struct Page;

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (Block::TabsBlock(TabsProps {
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
                                            Block::Html(html! {
                                                h3 { "Suicide"}
                                                ul class="list" {
                                                    li { "PTSD is associated with an increased risk of suicidal ideation, attempted suicide, and completed suicide" }
                                                    li { "Civilians who develop PTSD are at an increased risk (relative risk = 2.7) of attempted suicide compared to individuals who have never experienced a traumatic event"(RefNote::new(9))(RefNote::new(10))"." }
                                                    li { "In the US, PTSD is significantly associated with increased rates of suicide and attempted suicide"(RefNote::new(11))"." }
                                                    li { "A cross-sectional analysis of 5.9 million US veterans reported that PTSD was associated with increased odds of suicide"(RefNote::new(12))"." }
                                                }
                                                (Block::SuggestedNodeBlock(SuggestedNodeProps {
                                                    prompt: "health impacts of PTSD".to_string(),
                                                    to_node_id: "comorbidities".to_string()
                                                }))

                                            }),


                                        ]
                                    },
                                    Tab {
                                        name: String::from("Family / Caregiver"),
                                        blocks: vec![
                                            Block::Html(html! {
                                                p { "PTSD symptoms can negatively affect the relationships between patients and their partners"(RefNote::new(7))"." }
                                            }),
                                            Block::GridTableBlock(GridTableProps {
                                                rows: vec![
                                                    vec![
                                                        Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126TimingOfIndexTrauma}),
                                                        Block::Html(html! {
                                                            strong { "Timing of index trauma" }
                                                        }),
                                                        Block::Html(html! {
                                                            p { "Studies show impact on relationship quality was stronger among survivors who experienced traumatic events in the distant past compared to those who experienced more recent events"(RefNote::new(13))"." }
                                                        }),
                                                    ],
                                                    vec![
                                                        Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126NaturalDisasters}),
                                                        Block::Html(html! {
                                                            strong { "Natural Disasters" }
                                                        }),
                                                        Block::Html(html! {
                                                            p {
                                                                "Natural disasters can severely affect trauma survivors' mental health, with PTSD being the most common issue. It significantly impairs their ability to form and maintain healthy intimate relationships, leading to poor relationship adjustment"(RefNote::new(7))(RefNote::new(14))"."
                                                            }
                                                        }),
                                                    ],
                                                    vec![
                                                        Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126InterpersonalVictimization}),
                                                        Block::Html(html! {
                                                            strong { "Interpersonal victimization" }
                                                        }),
                                                        Block::Html(html! {
                                                            p {
                                                                "PTSD is linked to a higher probability of ongoing partner abuse, indicating its predictive role in family violence"(RefNote::new(7))(RefNote::new(15))"."
                                                            }
                                                        }),
                                                    ],
                                                    vec![
                                                        Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126ChildhoodSexualAbuse}),
                                                        Block::Html(html! {
                                                            strong { "Childhood sexual abuse" }
                                                        }),
                                                        Block::Html(html! {
                                                            p {
                                                                "Survivors of childhood sexual abuse predict problems with intimate relationships in adulthood and difficulties with intimacy and sexual dysfunction"(RefNote::new(7))(RefNote::new(16))(RefNote::new(17))(RefNote::new(18))"."
                                                            }
                                                        }),
                                                    ],
                                                    vec![
                                                        Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126CiviliansWithPTSD}),
                                                        Block::Html(html! {
                                                            strong { "Civilians with PTSD" }
                                                        }),
                                                        Block::Html(html! {
                                                            p {
                                                                "Avoidance symptoms can contribute to withdrawal from family members, detachment from others, and strengthening feelings of uncertainty and loneliness. Hyperarousal symptoms can create further difficulties with intimate relationships"(RefNote::new(6))"."
                                                            }
                                                        }),
                                                    ],
                                                    vec![
                                                        Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126PartnersOfIndividualWithPTSD}),
                                                        Block::Html(html! {
                                                            strong { "Partners of individuals with PTSD" }
                                                        }),
                                                        Block::Html(html! {
                                                            p {
                                                                "Research has extensively explored the concept of partner burden in relation to living with someone with PTSD, encompassing increased household responsibilities, financial strain, and social challenges. "
                                                                "Additionally, partners' efforts to accommodate survivors' symptoms, such as restricting noise in the house to avoid provoking a startle response or limiting social engagements, negatively affect relationships"(RefNote::new(19))"."
                                                            }
                                                        }),
                                                    ],
                                                    vec![
                                                        Block::IconBlock(IconProps { name: SvgName::OTS126Icons02MNOTS126PartnersOfVeteransWithPTSD}),
                                                        Block::Html(html! {
                                                            strong { "Partners of veterans with PTSD" }
                                                        }),
                                                        Block::Html(html! {
                                                            p {
                                                                "The partners of veterans coping with PTSD face heightened chances of encountering their own mental health challenges. They often express decreased levels of happiness, diminished life satisfaction, and heightened feelings of demoralization"(RefNote::new(20))"."
                                                            }
                                                        }),

                                                    ],

                                                ]
                                            }),
                                            Block::Html(html! {
                                                @let footer_text = "Emotional support is important, and negative family interactions may precipitate PTSD development by 1-year post-trauma<a aria-describedby=\"ref-marker\">25</a>";

                                                @let data = [
                                                    (
                                                        "Nightmares, insomnia, and recurrent disturbing dreams<a aria-describedby=\"ref-marker\">2</a><a aria-describedby=\"ref-marker\">21</a>.",
                                                        "Makes cohabitation difficult<a aria-describedby=\"ref-marker\">21</a>",
                                                    ),
                                                    (
                                                        "Emotional numbing, lack of emotional or physical intimacy, difficulty receiving and giving affection, increased isolation<a aria-describedby=\"ref-marker\">2</a><a aria-describedby=\"ref-marker\">21</a>",
                                                        "Physically present but emotionally absent<a aria-describedby=\"ref-marker\">21</a>",
                                                    ),
                                                    (
                                                        "Interpersonal victimization, hyperarousal and irritable or angry behavior<a aria-describedby=\"ref-marker\">23</a>",
                                                        "Feel like “walking on eggshells”<a aria-describedby=\"ref-marker\">23</a><a aria-describedby=\"ref-marker\">24</a>",
                                                    ),
                                                    (
                                                        "Changes to cognition and mood<a aria-describedby=\"ref-marker\">21</a>",
                                                        "Feel emotionally “cut-off”<a aria-describedby=\"ref-marker\">21</a>",
                                                    ),
                                                ];
                                                div class="flex flex-col [&>*:nth-child(odd)]:bg-[#c1daed] text-[#6e777e] [&>*:nth-child(even)]:bg-[#ededed] gap-1" {
                                                    div class="grid grid-cols-[1fr_30px_1fr] gap-y-2 overflow-hidden rounded-md" {
                                                        div { "Individuals with PTSD"}
                                                        div {}
                                                        div { "Partners and family"}
                                                    }
                                                    div class="grid grid-cols-[1fr_30px_1fr] gap-y-2 overflow-hidden rounded-md" {
                                                        @for (left_text, right_text) in &data {
                                                            div class="relative flex items-center px-4 py-2 text-white bg-[#005178]" {
                                                                span { (maud::PreEscaped(left_text)) }
                                                            }
                                                            div class="flex items-center space-x-2 text-gray-700 bg-[#c1daed]" {
                                                                svg width="100%" height="100%" fill="none" stroke="#005178" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" {
                                                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" {}
                                                                }
                                                            }
                                                            div class="flex items-center px-4 py-2 text-black bg-[#c1daed]" {
                                                                span { (maud::PreEscaped(right_text)) }
                                                            }
                                                        }
                                                    }
                                                    div class="col-span-3 bg-[#343e4d] text-[#ffffff]" {
                                                        span { (maud::PreEscaped(footer_text)) }
                                                    }
                                                }
                                            }),

                                        ]
                                    },
                                    Tab {
                                        name: String::from("Work and Finance"),
                                        blocks: vec![

                                            Block::Html(html! {
                                                h3 { "Homelessness"}
                                                p {
                                                    "27% average prevalence in the homeless population"(RefNote::new(1))"."
                                                }
                                                p {
                                                    "PTSD can often result in the loss of personal, social, and material resources"(RefNote::new(2))"."
                                                }
                                                p {
                                                    "PTSD is a key predictor of relapse and increased days of homelessness in a US study of adolescents receiving treatment for substance use disorder"(RefNote::new(3))"."
                                                }
                                                p {
                                                    "PTSD in the homeless population in linked to an increased risk of mortality from suicide, medical issues and drug-related problems"(RefNote::new(3))"."
                                                }
                                            }),
                                            Block::Html(html! {
                                                h3 { "Incarceration"}
                                                p {
                                                    "Patients with PTSD have an increased likelihood of incarceration among US prisoners"(RefNote::new(4))"."
                                                }
                                                p {
                                                    "Experiencing ≥ 4 traumas was associated with elevated odds of arrest and being jailed and imprisoned"(RefNote::new(4))"."
                                                }

                                            }),
                                            Block::Html(html! {
                                                h3 { "Work"}
                                                p {
                                                    "78% of the civilian population and 81% of the military population are unemployed in the US"(RefNote::new(5))"."
                                                }
                                                p {
                                                    "The likelihood of unemployment increases with symptom severity"(RefNote::new(6))"."
                                                    "Avoidance symptoms result in reluctance or refusal to take public transportation to and from work, which further exacerbates social isolation"(RefNote::new(2))"."
                                                }

                                            }),

                                            Block::SvgBlock(SvgProps { name: SvgName::OTS126Icons02MNPTSDAtWorkInfographic}),





                                        ]
                                    },
                                    Tab {
                                        name: String::from("Interpersonal"),
                                        blocks: vec![
                                            Block::Html(html! {
                                                h3 {
                                                    "Interpersonal impact"
                                                }
                                                p {
                                                    "Recent research has found that the symptoms of PTSD frequently result in deleterious consequences for intimate relationships and make it difficult for individuals to interact with their friends and family"(RefNote::new(6))(RefNote::new(7))"."
                                                    "Individuals struggle to adapt to societal norms and undermine social support networks, placing a substantial burden on their interpersonal relationships"(RefNote::new(2))(RefNote::new(8))"."
                                                }
                                            }),


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


                          Block::Html(html!{
                           p {
                            "The average PTSD patient costs $19,630, which is higher than other mental health conditions such as anxiety and major depressive disorder (MDD)"(RefNote::new(26))"."
                           }
                          }),
                         Block::SvgBlock(SvgProps { name: SvgName::OST126DiseaseBurdenInfographic011 }),

                        Block::TabsBlock(TabsProps {
                                id: uuid::Uuid::new_v4(),
                                representation: TabsRepresentation::Internal { theme: TabsTheme::new("#d6e5ee", "#3b3e3f"), full_bleed: false },
                                tabs: vec![
                                    Tab {
                                        name: String::from("Overall Economic Burden"),
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

                                            Block::Html(html! {
                                                @let legend_items = [
                                                    ("#005178", "Excess direct health care costs"),
                                                    ("#1178a0", "Excess direct non-health care costs"),
                                                    ("#184ca1", "Excess costs of unemployment"),
                                                    ("#4e69b1", "Excess costs of productivity loss"),
                                                    ("#5b97ca", "Excess costs due to caregiving"),
                                                    ("#678bc7", "Excess costs of premature mortality"),
                                                    ];

                                                div class="flex flex-wrap items-center justify-center space-x-4" {
                                                    @for (color, description) in &legend_items {
                                                        div class="flex items-center space-x-2" {
                                                            div class="w-4 h-4" style=(format!("background-color: {};", color)) {}
                                                            div { (description) }
                                                        }
                                                    }
                                                }
                                            }),



                                        ]
                                    },
                                    Tab {
                                        name: String::from("Civilian Economic Burden"),
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
                                            Block::Html(html! {
                                                @let legend_items = [
                                                    ("#005178", "Excess direct health care costs"),
                                                    ("#1178a0", "Excess direct non-health care costs"),
                                                    ("#184ca1", "Excess costs of unemployment"),
                                                    ("#4e69b1", "Excess costs of productivity loss"),
                                                    ("#5b97ca", "Excess costs due to caregiving"),
                                                    ("#678bc7", "Excess costs of premature mortality"),
                                                    ];

                                                div class="flex flex-wrap items-center justify-center space-x-4" {
                                                    @for (color, description) in &legend_items {
                                                        div class="flex items-center space-x-2" {
                                                            div class="w-4 h-4" style=(format!("background-color: {};", color)) {}
                                                            div { (description) }
                                                        }
                                                    }
                                                }
                                            }),

                                        ]
                                    },
                                    Tab {
                                        name: String::from("Military Economic Burden"),
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
                                            Block::Html(html! {
                                                @let legend_items = [
                                                    ("#005178", "Excess direct health care costs"),
                                                    ("#1178a0", "Excess direct non-health care costs"),
                                                    ("#184ca1", "Excess costs of unemployment"),
                                                    ("#4e69b1", "Excess costs of productivity loss"),
                                                    ("#5b97ca", "Excess costs due to caregiving"),
                                                    ("#678bc7", "Excess costs of premature mortality"),
                                                    ];

                                                div class="flex flex-wrap items-center justify-center space-x-4" {
                                                    @for (color, description) in &legend_items {
                                                        div class="flex items-center space-x-2" {
                                                            div class="w-4 h-4" style=(format!("background-color: {};", color)) {}
                                                            div { (description) }
                                                        }
                                                    }
                                                }
                                            }),


                                        ]
                                    }
                                ]
                        }),

                        Block::Html(
                            html! {
                                p {
                                    "In 2018, PTSD-related costs for the US civilian population were almost five times the cost for the military population"(RefNote::new(26))"."
                                }
                            }
                           ),






                            Block::GridTableBlock(GridTableProps {

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
            }))
            (Block::ReferencesBlock(Box::new(ReferencesProps {
                references:rich_text_block!("../../input/OTS126_PTSD_Disease_Burden_Node/60_571974e7-bc2e-450f-98bf-8e6a79c0ca7b.html")
            })))
        }
    }
}
