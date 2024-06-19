use crate::{rich_text, rich_text_block};
use blocks::block::grid_table::GridTableProps;
use blocks::block::html::HtmlProps;
use blocks::block::icon::{IconProps, SvgProps};
use blocks::block::pie_chart::{PieChartDatum, PieChartProps};
use blocks::block::references::ReferencesProps;
use blocks::block::rich_text::{self, RichTextProps};
use blocks::block::suggested_node::SuggestedNodeProps;
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
                                                    h2 { "Suicide"}
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
                                                    div class="flex flex-col gap-4" {
                                                        h2 { "PTSD symptoms can negatively affect the relationships between patients and their partners"(RefNote::new(7))"." }
                                                       (Block::GridTableBlock(GridTableProps {
                                                        headers: vec![],
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
                                                    })
                                                    )
                                                       (Block::Html(html! {
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
                                                        div class="flex flex-col  text-[#6e777e] gap-1" {
                                                            h1 {
                                                                "Effect of PTSD on partners and family"
                                                            }
                                                            div class="grid grid-cols-[1fr_30px_1fr] gap-y-2 overflow-hidden rounded-md" {
                                                                h3 { "Individuals with PTSD"}
                                                                div class="text-lg" {}
                                                                h3  { "Partners and family"}
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
                                                    })
        )
                                                    }

                                                }),

                                            ]
                                        },
                                        Tab {
                                            name: String::from("Work and Finance"),
                                            blocks: vec![

                                                Block::Html(html! {
                                                    h2 { "Homelessness"}
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
                                                    h2 { "Incarceration"}
                                                    p {
                                                        "Patients with PTSD have an increased likelihood of incarceration among US prisoners"(RefNote::new(4))"."
                                                    }
                                                    p {
                                                        "Experiencing ≥ 4 traumas was associated with elevated odds of arrest and being jailed and imprisoned"(RefNote::new(4))"."
                                                    }

                                                }),
                                                Block::Html(html! {
                                                    h2 { "Work"}
                                                    p {
                                                        "78% of the civilian population and 81% of the military population are unemployed in the US"(RefNote::new(5))"."
                                                    }
                                                    p {
                                                        "The likelihood of unemployment increases with symptom severity"(RefNote::new(6))"."
                                                        "Avoidance symptoms result in reluctance or refusal to take public transportation to and from work, which further exacerbates social isolation"(RefNote::new(2))"."
                                                    }

                                                }),

                                                Block::SvgBlock(SvgProps { name: SvgName::OTS126DiseaseBurdenPTSDAtWorkInfographic}),





                                            ]
                                        },
                                        Tab {
                                            name: String::from("Interpersonal"),
                                            blocks: vec![
                                                Block::Html(html! {
                                                    h2 {
                                                        "Interpersonal impact"
                                                    }
                                                    p {
                                                        "Recent research has found that the symptoms of PTSD frequently result in deleterious consequences for intimate relationships and make it difficult for individuals to interact with their friends and family"(RefNote::new(6))(RefNote::new(7))"."
                                                        "Individuals struggle to adapt to societal norms and undermine social support networks, placing a substantial burden on their interpersonal relationships"(RefNote::new(2))(RefNote::new(8))"."
                                                    }
                                                }),


                                                Block::SvgBlock(SvgProps { name: SvgName::PTSDHyperviliganceInfogrpahic}),





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
                              div class="panel" {
                                p {
                                    "The average PTSD patient costs $19,630, which is higher than other mental health conditions such as anxiety and major depressive disorder (MDD)"(RefNote::new(26))"."
                                   }
                                   (Block::SvgBlock(SvgProps { name: SvgName::OST126DiseaseBurdenInfographic011 }))
                              }
                              }),


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


                                                Block::Html(html! {

                                                    h3 {
                                                        "Figure 1. Excess Economic Burden of PTSD in the Overall US Population in 2018, Billion USD"
                                                    }
                                                    ( Block::SvgBlock(SvgProps { name: SvgName::OST126PTSDAtWorkInfographicFig1 }))
                                                    @let legend_items = [
                                                        ("#01072d", "Excess direct health care costs"),
                                                        ("#162065", "Excess direct non-health care costs"),
                                                        ("#24448e", "Excess costs of unemployment"),
                                                        ("#3d91be", "Excess costs of productivity loss"),
                                                        ("#62c4c4", "Excess costs due to caregiving"),
                                                        ("#5fffd1", "Excess costs of premature mortality"),
                                                        ];

                                                    div class="flex flex-wrap items-center justify-center space-x-4" {
                                                        @for (color, description) in &legend_items {
                                                            div class="flex items-center space-x-2" {
                                                                div class="w-4 h-4" style=(format!("background-color: {};", color)) {}
                                                                div { (description) }
                                                            }
                                                        }
                                                    }
                                                    p {
                                                        "In 2018, PTSD-related costs for the US civilian population were almost five times the cost for the military population"(RefNote::new(26))"."
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

                                                Block::Html(html! {
                                                    h3 { "Figure 2. Excess Economic Burden of PTSD in the US Civilian Population in 2018, Billion USD"}
                                                    ( Block::SvgBlock(SvgProps { name: SvgName::OST126PTSDAtWorkInfographicFig2 }))
                                                    @let legend_items = [
                                                        ("#01072d", "Excess direct health care costs"),
                                                        ("#162065", "Excess direct non-health care costs"),
                                                        ("#24448e", "Excess costs of unemployment"),
                                                        ("#3d91be", "Excess costs of productivity loss"),
                                                        ("#62c4c4", "Excess costs due to caregiving"),
                                                        ("#5fffd1", "Excess costs of premature mortality"),
                                                        ];
                                                    div class="flex flex-wrap items-center justify-center space-x-4" {
                                                        @for (color, description) in &legend_items {
                                                            div class="flex items-center space-x-2" {
                                                                div class="w-4 h-4" style=(format!("background-color: {};", color)) {}
                                                                div { (description) }
                                                            }
                                                        }
                                                    }
                                                    p {
                                                        "In 2018, PTSD-related costs for the US civilian population were almost five times the cost for the military population"(RefNote::new(26))"."
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

                                                Block::Html(html! {
                                                    h3 { "Figure 3. Excess Economic Burden of PTSD in the US Military Population in 2018, Billion USD"}
                                                    ( Block::SvgBlock(SvgProps { name: SvgName::OST126PTSDAtWorkInfographicFig3 }))
                                                    @let legend_items = [
                                                        ("#01072d", "Excess direct health care costs"),
                                                        ("#162065", "Excess direct non-health care costs"),
                                                        ("#24448e", "Excess costs of unemployment"),
                                                        ("#3d91be", "Excess costs of productivity loss"),
                                                        ("#62c4c4", "Excess costs due to caregiving"),
                                                        ("#5fffd1", "Excess costs of premature mortality"),
                                                        ];

                                                    div class="flex flex-wrap items-center justify-center space-x-4" {
                                                        @for (color, description) in &legend_items {
                                                            div class="flex items-center space-x-2" {
                                                                div class="w-4 h-4" style=(format!("background-color: {};", color)) {}
                                                                div { (description) }
                                                            }
                                                        }
                                                    }
                                                    p {
                                                        "In 2018, PTSD-related costs for the US civilian population were almost five times the cost for the military population"(RefNote::new(26))"."
                                                    }
                                                }),


                                            ]
                                        }
                                    ]
                            }),

                            Block::Html(
                                html! {
                                    div class="panel" {

                                        ( Block::GridTableBlock(GridTableProps {
                                            headers: vec![],
                                            rows: vec![
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126NonmedicalDirectCost}),
                                                    Block::Html(html! {
                                                        strong { "Nonmedical direct costs"(RefNote::new(26))}
                                                    }),
                                                    Block::Html(html! {
                                                       p {
                                                        "~$36 billion are spent on:"
                                                       }
                                                       ul class="list" {
                                                        li { "Disability"}
                                                        li { "Substance use disorder"}
                                                        li { "Homelessness"}
                                                        li { "Research and training"}
                                                        li { "Psychotherapy (for uninsured civilian patients)"}
                                                       }


                                                       }
                                                    )

                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126MedicalCosts}),
                                                    Block::Html(html! {
                                                        strong { "Medical costs"}
                                                    }),
                                                    Block::Html(html! {
                                                        ul class="list" {
                                                            li { "$2500 to $4000 per patient per year is the average excess medical cost associated with PTSD in the US"(RefNote::new(28))"."}
                                                            li { "PTSD has been shown to incur higher costs than coronary heart disease (CHD) and some psychiatric disorders, including anxiety and depression"(RefNote::new(26))"."}

                                                           }
                                                    })

                                                ],
                                                vec![
                                                    Block::IconBlock(IconProps { name: SvgName::OTS126UntreatedPatients}),
                                                    Block::Html(html! {
                                                        strong { "Untreated patients"(RefNote::new(27))}
                                                    }),
                                                    Block::Html(html! {

                                                       ul class="list" {
                                                        li { "The overall cost in untreated patients is substantial"}
                                                        li { "Result in much higher lifetime costs"}

                                                       }


                                                       }
                                                    )

                                                ],



                                            ],
                                        }))
                                    }
                                }
                               ),

                            ],
                        },
                    ],
                }))
                (Block::ReferencesBlock(Box::new(ReferencesProps {
                    references: Block::Html(
                        html! {
                            ol class="list" {
                                li {
                                    p class="bn-inline-content" {
                                        "Ayano G, Solomon M, Tsegay L, Yohannes K, Abraha M. A Systematic Review and
                                        Meta-Analysis of the Prevalence of Post-Traumatic Stress Disorder among Homeless People. "
                                        em { "Psychiatr Q" }
                                        ". 2020;91(4):949-963. doi:10.1007/s11126-020-09746-1"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Yehuda R, Hoge CW, McFarlane AC, et al. Post-traumatic stress disorder. "
                                        em { "Nat Rev Dis Primers" }
                                        ". 2015;1:15057. Published 2015 Oct 8. doi:10.1038/nrdp.2015.57"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Davis JP, Diguiseppi G, De Leon J, et al. Understanding pathways between PTSD,
                                        homelessness, and substance use among adolescents. "
                                        em { "Psychol Addict Behav" }
                                        ". 2019;33(5):467-476. doi:10.1037/adb0000488"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Jäggi LJ, Mezuk B, Watkins DC, Jackson JS. The Relationship between Trauma, Arrest,
                                        and Incarceration History among Black Americans: Findings from the National Survey of American Life. "
                                        em { "Soc Ment Health" }
                                        ". 2016;6(3):187-206. doi:10.1177/2156869316641730"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Zivin K, Bohnert AS, Mezuk B, et al. Employment status of patients in the VA health
                                        system: implications for mental health services. "
                                        em { "Psychiatr Serv" }
                                        ". 2011;62(1):35-38. doi:10.1176/ps.62.1.pss6201_0035"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Rodriguez P, Holowka DW, Marx BP. Assessment of posttraumatic stress
                                        disorder-related functional impairment: a review. "
                                        em { "J Rehabil Res Dev" }
                                        ". 2012;49(5):649-665. doi:10.1682/jrrd.2011.09.0162"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Suomi A, Evans L, Rodgers B, Taplin S, Cowlishaw S. Couple and family therapies for
                                        post-traumatic stress disorder (PTSD). "
                                        em { "Cochrane Database Syst Rev" }
                                        ". 2019;12(12):CD011257. Published 2019 Dec 4. doi:10.1002/"
                                        a target="_blank" rel="noopener noreferrer nofollow" href="http://14651858.CD011257.pub" {
                                            "14651858.CD011257.pub"
                                        }
                                        "2"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Koven SG. Veteran Treatments: PTSD Interventions. "
                                        em { "Healthcare (Basel)" }
                                        ". 2018;6(3):94. Published 2018 Aug 6. doi:10.3390/healthcare6030094"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Nepon J, Belik SL, Bolton J, Sareen J. The relationship between anxiety disorders
                                        and suicide attempts: findings from the National Epidemiologic Survey on Alcohol and Related Conditions. "
                                        em { "Depress Anxiety" }
                                        ". 2010;27(9):791-798. doi:10.1002/da.20674"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Wilcox HC, Storr CL, Breslau N. Posttraumatic stress disorder and suicide attempts
                                        in a community sample of urban american young adults. "
                                        em { "Arch Gen Psychiatry" }
                                        ". 2009;66(3):305-311. doi:10.1001/archgenpsychiatry.2008.557"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Cougle JR, Keough ME, Riccardi CJ, Sachs-Ericsson N. Anxiety disorders and
                                        suicidality in the National Comorbidity Survey-Replication. "
                                        em { "J Psychiatr Res" }
                                        ". 2009;43(9):825-829. doi:10.1016/j.jpsychires.2008.12.004"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Conner KR, Bossarte RM, He H, et al. Posttraumatic stress disorder and suicide in
                                        5.9 million individuals receiving care in the veterans health administration health system. "
                                        em { "J Affect Disord" }
                                        ". 2014;166:1-5. doi:10.1016/j.jad.2014.04.067"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Lambert JE, Engh R, Hasbun A, Holzer J. Impact of posttraumatic stress disorder on
                                        the relationship quality and psychological distress of intimate partners: a meta-analytic review. "
                                        em { "J Fam Psychol" }
                                        ". 2012;26(5):729-737. doi:10.1037/a0029341"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Taft CT, Monson CM, Schumm JA, Watkins LE, Panuzio J, Resick PA. Posttraumatic
                                    Stress Disorder Symptoms, Relationship Adjustment, and Relationship Aggression in a Sample of Female Flood
                                    Victims. "
                                        em { "J Fam Violence" }
                                        ". 2009;24(6):389-396. doi:10.1007/s10896-009-9241-8"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Krause ED, Kaltman S, Goodman L, Dutton MA. Role of distinct PTSD symptoms in
                                        intimate partner reabuse: a prospective study. "
                                        em { "J Trauma Stress" }
                                        ". 2006;19(4):507-516. doi:10.1002/jts.20136"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Davis JL, Petretic-Jackson PA. The impact of child sexual abuse on adult
                                        interpersonal functioning. "
                                        em { "Aggression and Violent Behavior" }
                                        ". 2000;5(3):291-328. doi:"
                                        a target="_blank" rel="noopener noreferrer nofollow" href="https://doi.org/10.1016/s1359-1789(99)00010-5" {
                                            "https://doi.org/10.1016/s1359-1789(99)00010-5"
                                        }
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Lamoureux BE, Palmieri PA, Jackson AP, Hobfoll SE. Child sexual abuse and
                                        adulthood-interpersonal outcomes: Examining pathways for intervention. "
                                        em { "Psychol Trauma" }
                                        ". 2012;4(6):605-613. doi:10.1037/a0026079"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Cloitre M, Stolbach BC, Herman JL, et al. A developmental approach to complex PTSD:
                                        childhood and adult cumulative trauma as predictors of symptom complexity. "
                                        em { "J Trauma Stress" }
                                        ". 2009;22(5):399-408. doi:10.1002/jts.20444"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Campbell SB, Renshaw KD. Posttraumatic stress disorder and relationship
                                        functioning: A comprehensive review and organizational framework. "
                                        em { "Clin Psychol Rev" }
                                        ". 2018;65:152-162. doi:10.1016/j.cpr.2018.08.003"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Jordan BK, Marmar CR, Fairbank JA, et al. Problems in families of male Vietnam
                                        veterans with posttraumatic stress disorder. "
                                        em { "J Consult Clin Psychol" }
                                        ". 1992;60(6):916-926. doi:10.1037//0022-006x.60.6.916"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Larsen SE. U.S. Department of Veterans Affairs. PTSD: National Center for PTSD.
                                        PTSD and the Family. Last updated 2023. "
                                        a target="_blank" rel="noopener noreferrer nofollow" href="https://www.ptsd.va.gov/professional/treat/specific/ptsd_family.asp" {
                                            "https://www.ptsd.va.gov/professional/treat/specific/ptsd_family.asp"
                                        }
                                        " (Accessed May 2024)."
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Gewirtz AH, Polusny MA, DeGarmo DS, Khaylis A, Erbes CR. Posttraumatic stress
                                    symptoms among National Guard soldiers deployed to Iraq: associations with parenting behaviors and couple
                                    adjustment. "
                                        em { "J Consult Clin Psychol" }
                                        ". 2010;78(5):599-610. doi:10.1037/a0020571"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Fredman SJ, Vorstenbosch V, Wagner AC, Macdonald A, Monson CM. Partner
                                    accommodation in posttraumatic stress disorder: initial testing of the Significant Others' Responses to
                                    Trauma Scale (SORTS). "
                                        em { "J Anxiety Disord" }
                                        ". 2014;28(4):372-381. doi:10.1016/j.janxdis.2014.04.001"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "McGaw, V. E., Reupert, A. E., & Maybery, D. (2019). Military posttraumatic
                                        stress disorder: A qualitative systematic review of the experience of families, parents and children. "
                                        em { "Journal of Child and Family Studies" }
                                        ", 28(11), 2942–2952."
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Nguyen AW, Chatters LM, Taylor RJ, Levine DS, Himle JA. Family, friends, and
                                        12-month PTSD among African Americans. "
                                        em { "Soc Psychiatry Psychiatr Epidemiol" }
                                        ". 2016;51(8):1149-1157. doi:10.1007/s00127-016-1239-y"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Davis LL, Schein J, Cloutier M, et al. The Economic Burden of Posttraumatic Stress
                                        Disorder in the United States From a Societal Perspective. "
                                        em { "J Clin Psychiatry" }
                                        ". 2022;83(3):21m14116. Published 2022 Apr 25. doi:10.4088/JCP.21m14116"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Gagnon-Sanschagrin P, Schein J, Urganus A, et al. Identifying individuals with
                                    undiagnosed post-traumatic stress disorder in a large United States civilian population - a machine learning
                                    approach. "
                                        em { "BMC Psychiatry" }
                                        ". 2022;22(1):630. Published 2022 Sep 29. doi:10.1186/s12888-022-04267-6"
                                    }
                                }
                                li {
                                    p class="bn-inline-content" {
                                        "Cohen GH, Tamrakar S, Lowe S, et al. Improved social services and the burden of
                                    post-traumatic stress disorder among economically vulnerable people after a natural disaster: a modelling
                                    study. "
                                        em { "Lancet Planet Health" }
                                        ". 2019;3(2):e93-e101. doi:10.1016/S2542-5196(19)30012-9"
                                    }
                                }
                            }
                        }

                    )
                })))
            }
    }
}
