use std::ops::Range;

use blocks::{
    block::{
        bar_chart::{BarChartDatum, BarChartProps},
        grid_table::GridTableProps,
        references::ReferencesProps,
        table::Table,
        Block,
    },
    layout::tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme},
    span::ref_note::RefNote,
};
use maud::{html, Render};

pub struct Page;

impl Page {
    fn traumatic_events_drivers_tab(&self) -> Tab {
        Tab {
            name: "Information surrounding different traumatic events as key drivers".to_string(),
            renderable: Box::new(html! {
               div class="panel" {
               p {
                r#"Diagnosing Post-traumatic Stress Disorder (PTSD) hinges on identifying the "index trauma""#
                " – the specific event(s) an individual experienced or witnessed that triggered the onset of symptoms"(RefNote(1))"."
                "This trauma serves as a crucial reference point, guiding clinicians in assessing the severity of the condition"(RefNote(1))"."
                "However, the journey from trauma exposure to PTSD is not always straightforward, and the risk of developing the disorder can vary significantly depending on trauma type"(RefNote(2))(RefNote(3))(RefNote(5))"."
               }
               }
            }),
        }
    }
    fn risk_factors_tab(&self) -> Tab {
        Tab {
            name: "Risk Factors".to_string(),
            renderable: Box::new(TabsProps {
                id: uuid::Uuid::new_v4(),
                tabs: vec![
                    Tab {
                        name: "Trauma Type and PTD risk".to_string(),
                        renderable: Box::new(html! {
                            p { "Several types of traumatic events can be considered key drivers"(RefNote(5)) }
                            (Table::new(
                                Some([
                                    &"PTSD Key Drivers".to_string(), &"Odds ratio".to_string()
                                ]),
                                [
                                [&"Sexual relationship violence".to_string(), &"5.6".to_string()],
                                [&"Child abuse".to_string(), &"2.6 - 4.9".to_string()],
                                [&"Life theatening illness/injury".to_string(), &"1.9 - 3.2".to_string()],
                                [&"Interpersonal violence".to_string(), &"1.7 - 4.0".to_string()],
                                [&"Combat/war zone exposure".to_string(), &"1.6 - 5.1".to_string()],
                                [&"Traumatic brain injury".to_string(), &"1.5 - 1.8".to_string()]
                            ]
                            ))
                            p {
                                "Research indicates that the type and nature of the traumatic experience play a significant role in determining the likelihood of developing PTSD"
                                (RefNote(2))(RefNote(3))(RefNote(4))". "
                                "A World Health Organization analysis of trauma exposure revealed that witnessing death or serious injury, and the unexpected death of a loved one, were linked to the highest proportion of PTSD cases"
                                (RefNote(3))". "
                                "While the average risk of developing PTSD after trauma exposure is around 4%, it can reach up to 30% depending on the specific type of trauma"
                                (RefNote(2))(RefNote(3))(RefNote(4))(RefNote(6))". "
                            }
                            (BarChartProps {
                                title: "Highest proportion of PTSD cases (%)".to_string(),
                                bars: vec! [
                                    BarChartDatum::new("Unexpected death of a loved one", 31.4, None,None),
                                    BarChartDatum::new("Direct exposure to death or serious injury", 23.7, None,None),
                                ]
                            })
                            (BarChartProps {
                                title: "Risk of developing PTSD after trauma exposure (%)".to_string(),
                                bars: vec! [
                                    BarChartDatum::new("Rape", 19., None,  None),
                                    BarChartDatum::new("Physical abuse by a partner", 11.7,Some(13.), None),
                                    BarChartDatum::new("Physical abuse by a partner", 11.7, None,None),
                                    BarChartDatum::new("Being kidnapped", 11., None,None),
                                    BarChartDatum::new("Sexual assault", 10.5, None, None),
                                    BarChartDatum::new("Saw war-related atrocities", 5.4, None,None),
                                    BarChartDatum::new("Childhood physical abuse", 5., None,None),
                                    BarChartDatum::new("Combat experience", 3.6, None,None),
                                    BarChartDatum::new("Natural disasters", 0.3, None,None),


                                ]
                            })

                        }),
                    },
                    Tab {
                        name: "Other trauma types in PTSD".to_string(),
                        renderable: Box::new(html! {
                            p {
                                "While combat and violence are often associated with PTSD, it's crucial to recognize that trauma can stem from various sources"
                            (RefNote(7))(RefNote(8))(RefNote(9))(RefNote(10))". "
                            "Major health conditions, motor vehicle accidents, health pandemics (e.g., COVID-19), and acting as a first responder in emergencies are just a few examples of potentially traumatic experiences that may lead to PTSD"
                            (RefNote(7))(RefNote(8))(RefNote(9))(RefNote(10))(RefNote(11))". "
                            "A meta-analysis of the COVID-19 pandemic, for instance, reported a global PTSD prevalence of 17.52% among the general population"
                            (RefNote(12))". "
                            "Notably, healthcare professionals faced a significantly higher risk, with 30.98% experiencing PTSD compared to 15.45% of COVID-19 patients/survivors"
                            (RefNote(12))". "
                            }
                        }),
                    },
                    Tab {
                        name: "Individual Risk Factors".to_string(),
                        renderable: Box::new(html! {
                            (Table::new(
                                None,
                                [
                                    [&html! {"Sociodemographic factors" },&html!{ ul class="w-full list" { li { "Female"} li { "Indigenous American"}}}],
                                    [&html! {"Pre-trauma factors" },&html!{ ul class="w-full list" { li { "History of physical disease"} li { "Family history of psychiatric disorder"}}}],
                                    [&html! {"Peri-trauma factors" },&html!{ ul class="w-full list" { li { "Cumulative exposure to potentially truamatic experiences"} li { "Trauma severity"}}}]
                                ]
                            ))
                            p {
                                "Beyond the nature of the traumatic event itself, certain individual characteristics can also influence the likelihood of developing PTSD"
                                (RefNote(5))(RefNote(6))(RefNote(13))". "
                                "This includes:"
                            }
                            h3 { "Sexual Orientation and Gender Identity"}
                            p {
                                "Research suggests that LGBTQ+ individuals are at an increased risk of developing PTSD compared to heterosexual individuals"(RefNote(14))". "
                                "This may be due to experiences of discrimination, violence, and social stigma"(RefNote(14))". "

                            }
                            h3 { "Race"}
                            p {
                                "Studies have shown that Black individuals face a higher risk of developing PTSD compared to White individuals"(RefNote(15))". "
                                "This disparity may be linked to systemic racism, exposure to community violence, and historical trauma"(RefNote(15))". "

                            }

                        }),
                    },
                ],
                representation: TabsRepresentation::Internal {
                    theme: TabsTheme::default(),
                    full_bleed: false,
                },
            }),
        }
    }
}

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (TabsProps {
                id: uuid::Uuid::new_v4(),
                representation: TabsRepresentation::TopLevel,
                tabs: vec![self.traumatic_events_drivers_tab(), self.risk_factors_tab()]
            })
            (ReferencesProps {
                references: Block::Html(html! {
                    ol class="list" {
                        li {
                            "Priebe K, Kleindienst N, Schropp A, et al. Defining the index trauma in post-traumatic stress disorder patients with multiple trauma exposure: impact on severity scores and treatment effects of using worst single incident versus multiple traumatic events. " i { "Eur J Psychotraumatol." } " 2018;9(1):1486124. Published 2018 Jul 9."
                        }
                        li {
                            "Liu H, Petukhova MV, Sampson NA, et al. Association of DSM-IV Posttraumatic Stress Disorder With Traumatic Experience Type and History in the World Health Organization World Mental Health Surveys [published correction appears in " i { "JAMA Psychiatry." } " 2017 Jul 1;74(7):764]. " i { "JAMA Psychiatry." } " 2017;74(3):270-281."
                        }
                        li {
                            "Kessler RC, Aguilar-Gaxiola S, Alonso J, Benjet C, Bromet EJ, Cardoso G, Degenhardt L, de Girolamo G, Dinolova RV, Ferry F, Florescu S, Gureje O, Haro JM, Huang Y, Karam EG, Kawakami N, Lee S, Lepine JP, Levinson D, Navarro-Mateu F, Pennell BE, Piazza M, Posada-Villa J, Scott KM, Stein DJ, Ten Have M, Torres Y, Viana MC, Petukhova MV, Sampson NA, Zaslavsky AM, Koenen KC. Trauma and PTSD in the WHO World Mental Health Surveys. " i { "Eur J Psychotraumatol." } " 2017 Oct 27;8(sup5):1353383. doi: 10.1080/20008198.2017.1353383"
                        }
                        li {
                            "Luz MP, Coutinho ES, Berger W, et al. Conditional risk for posttraumatic stress disorder in an epidemiological study of a Brazilian urban population. " i { "J Psychiatr Res." } " 2016;72:51-57."
                        }
                        li {
                            "Analysis Group. (2020). Prevalence and Risk Factors of Post-traumatic Stress Disorder in the United States. (Data on file)."
                        }
                        li {
                            "Tortella-Feliu M, Fullana MA, Pérez-Vigil A, et al. Risk factors for posttraumatic stress disorder: An umbrella review of systematic reviews and meta-analyses. " i { "Neurosci Biobehav Rev." } " 2019;107:154-165."
                        }
                        li {
                            "Parker AM, Sricharoenchai T, Raparla S, Schneck KW, Bienvenu OJ, Needham DM. Posttraumatic stress disorder in critical illness survivors: a meta-analysis. " i { "Crit Care Med." } " 2015;43(5):1121-1129."
                        }
                        li {
                            "Cordova MJ, Riba MB, Spiegel D. Post-traumatic stress disorder and cancer. " i { "Lancet Psychiatry." } " 2017;4(4):330-338."
                        }
                        li {
                            "Fekadu W, Mekonen T, Belete H, Belete A, Yohannes K. Incidence of Post-Traumatic Stress Disorder After Road Traffic Accident. " i { "Front Psychiatry." } " 2019;10:519. Published 2019 Jul 19."
                        }
                        li {
                            "Nagarajan R, Krishnamoorthy Y, Basavarachar V, Dakshinamoorthy R. Prevalence of post-traumatic stress disorder among survivors of severe COVID-19 infections: A systematic review and meta-analysis. " i { "J Affect Disord." } " 2022;299:52-59."
                        }
                        li {
                            "Prioux C, Marillier M, Vuillermoz C, Vandentorren S, Rabet G, Petitclerc M, Baubet T, Stene LE, Pirard P, Motreff Y. PTSD and Partial PTSD among First Responders One and Five Years after the Paris Terror Attacks in November 2015. " i { "Int J Environ Res Public Health." } " 2023 Feb 25;20(5):4160."
                        }
                        li {
                            "Yunitri N, Chu H, Kang XL, et al. Global prevalence and associated risk factors of posttraumatic stress disorder during COVID-19 pandemic: A meta-analysis. " i { "Int J Nurs Stud." } " 2022;126:104136."
                        }
                        li {
                            "Schein J, Houle C, Urganus A, et al. Prevalence of post-traumatic stress disorder in the United States: a systematic literature review. " i { "Curr Med Res Opin." } " 2021;37(12):2151-2161."
                        }
                        li {
                            "Roberts AL, Austin SB, Corliss HL, Vandermorris AK, Koenen KC. Pervasive trauma exposure among US sexual orientation minority adults and risk of posttraumatic stress disorder. " i { "Am J Public Health." } " 2010;100(12):2433-2441."
                        }
                        li {
                            "Roberts AL, Gilman SE, Breslau J, Breslau N, Koenen KC. Race/ethnic differences in exposure to traumatic events, development of post-traumatic stress disorder, and treatment-seeking for post-traumatic stress disorder in the United States. " i { "Psychol Med." } " 2011 Jan;41(1):71-83."
                        }
                    }
                }
                )
            })
        }
    }
}

pub struct PtsdTraumaTypeRow {
    pub driver: String,
    pub odds_ratio: Range<f32>,
}
impl PtsdTraumaTypeRow {
    pub fn new(driver: &str, odds_ratio: Range<f32>) -> Self {
        Self {
            driver: driver.to_string(),
            odds_ratio,
        }
    }
}

pub struct PtsdTraumaTypeTable<const I: usize> {
    rows: [PtsdTraumaTypeRow; I],
}
impl<const I: usize> PtsdTraumaTypeTable<I> {
    fn new(rows: [PtsdTraumaTypeRow; I]) -> Self {
        Self { rows }
    }
}

impl<const I: usize> Render for PtsdTraumaTypeTable<I> {
    fn render(&self) -> maud::Markup {
        let rows: Vec<Vec<Block>> = self
            .rows
            .iter()
            .map(|PtsdTraumaTypeRow { driver, odds_ratio }| {
                vec![
                    Block::Html(html! { (driver)}),
                    Block::Html(html! { (odds_ratio.start)"-"(odds_ratio.end)}),
                ]
            })
            .collect();

        GridTableProps {
            headers: vec![
                Block::Html(html! { "PTSD Key Drivers"}),
                Block::Html(html! { "Odds ratio"}),
            ],
            rows,
        }
        .render()
    }
}
