use crate::rich_text_block;

use blocks::block::references::ReferencesProps;

use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use blocks::block::{html, Block};
use blocks::span::ref_note::RefNote;
use maud::{html, Render};

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
                        name: String::from("Personal Stigma"),
                        blocks: vec![
                            Block::Html(
                                html! {
                                    p {
                                        "Some traumas are likely to be systematically under-reported because they are considered embarrassing or culturally sensitive"
                                        (RefNote::new(1))
                                        "."
                                    }

                                    p {
                                        strong { "Self-stigma" }
                                    }

                                    ul class="list" {
                                        li {
                                            p {
                                                "Defined as the internalization of others’ prejudices of mental illness and is a significant issue among patients with PTSD"
                                                (RefNote::new(2))
                                                (RefNote::new(3))
                                                "."
                                            }
                                        }
                                        li {
                                            p {
                                                "Symptoms such as negative thoughts about oneself, feelings of shame, and avoidance may explain co-occurrence and relevance of self-stigma"
                                                (RefNote::new(4))
                                                "."
                                            }
                                        }
                                        li {
                                            p {
                                                "Is associated with negative outcomes including depression, suicidal ideation, poor recovery, and lower treatment engagement"
                                                (RefNote::new(2))
                                                "."
                                            }
                                        }
                                        li {
                                            p {
                                                "Concerns about stigma, shame, and rejection are among the most significant barriers to seeking care for PTSD"
                                                (RefNote::new(5))
                                                "."
                                            }
                                        }
                                    }
                                }

                            )
                        ],
                    },
                    Tab {
                        name: String::from("Community and Societal Stigma"),
                        blocks: vec![
                            Block::Html(html! {
                                p {
                                    "Due to the stigma surrounding PTSD, individuals with the condition may be looked down upon by others. People may hold false beliefs about PTSD, causing them to treat those with the condition and their families differently. Unfortunately, stigma can also extend to other aspects of a person's identity, including race, sexuality, gender identity, religion, and disability status"
                                    (RefNote::new(2))
                                    (RefNote::new(4))
                                    "."
                                }
                                p {
                                    "Misconceptions about mental health symptoms and PTSD can lead to stigmatization by families and the broader community. Additionally, society often associates the condition with military veterans and violence, adding further stigma to PTSD, and reinforcing the assumption that it only affects military veterans or those who have experienced violence"
                                    (RefNote::new(2))
                                    (RefNote::new(6))
                                    "."
                                }
                                p {
                                    "Even well-intentioned people may feel uncomfortable upon learning someone has PTSD, making it difficult for that person to find a job or housing"
                                    (RefNote::new(4))
                                    "."
                                }
                                p {
                                    "Stigma occurs when others"
                                    (RefNote::new(4))
                                    ":"
                                }
                                ul class="list" {
                                    li {
                                        p { "Don't understand PTSD" }
                                    }
                                    li {
                                        p { "Don't realize PTSD is a treatable illness" }
                                    }
                                    li {
                                        p { "Think mental illness is \"your own fault\" or something you can \"get over\"" }
                                    }
                                    li {
                                        p { "Are afraid they might catch what you have" }
                                    }
                                    li {
                                        p { "Believe PTSD makes you dangerous" }
                                    }
                                }
                            }
                            )
                        ],
                    },
                    Tab {
                        name: String::from("HCP Stigma"),

                        blocks: vec![
                            Block::Html(html! {
                                p {
                                    "Negative attitudes toward mental illness, including PTSD, among healthcare professionals (HCP) are common globally and can negatively impact patients."
                                }
                                h3 {
                                      "Impact of HCP Stigma on PTSD Patients"
                                }

                                ul class="list" {
                                    li {
                                        p { "Delays in seeking help" }
                                    }
                                    li {
                                        p { "Decreased quality of care" }
                                    }
                                    li {
                                        p { "Longer waiting times" }
                                    }p
                                    li {
                                        p { "Substandard treatment" }
                                    }
                                    li {
                                        p { "Poor prognosis" }
                                    }
                                }
                                p {}

                                p {
                                    "Patients often find mental health professionals the most stigmatizing, which damages the patient-provider relationship and discourages help-seeking behavior"
                                    (RefNote::new(7))
                                    "."
                                }
                            }
                            )
                        ],
                    },
                ],
            }))
            (Block::ReferencesBlock(Box::new(ReferencesProps {
                references: Block::Html(html! {
                    ol class="list" {
                        li {
                            p {
                                "Kessler RC, Aguilar-Gaxiola S, Alonso J, et al. Trauma and PTSD in the WHO World Mental Health Surveys. "
                                em { "Eur J Psychotraumatol" }
                                ". 2017;8(sup5):1353383. Published 2017 Oct 27. doi:10.1080/20008198.2017.1353383"
                            }
                        }
                        li {
                            p {
                                "Benfer N, Howell MK, Lucksted A, Romero EG, Drapalski AL. Self-Stigma and PTSD: Conceptualization and Implications for Research and Treatment. "
                                em { "Psychiatr Serv." }
                                "2023;74(10):1081-1083. doi:10.1176/"
                                a target="_blank" rel="noopener noreferrer nofollow" href="http://appi.ps" { "appi.ps" }
                                ".20220397"
                            }
                        }
                        li {
                            p {
                                "Lewis C, Zammit S, Jones I, Bisson JI. Prevalence and correlates of self-stigma in Post-Traumatic Stress Disorder (PTSD). "
                                em { "Eur J Psychotraumatol." }
                                " 2022;13(1):2087967. Published 2022 Jul 22. doi:10.1080/20008198.2022.2087967"
                            }
                        }
                        li {
                            p {
                                a target="_blank" rel="noopener noreferrer nofollow" href="https://www.ptsd.va.gov/understand/related/ptsd_work_community.asp#:~:text=Because%20of%20stigma%20about%20PTSD%2C%20others%20may%20look,race%2C%20sexuality%2C%20gender%20identity%2C%20religion%20and%20disability%20status" {
                                    "https://www.ptsd.va.gov/understand/related/ptsd_work_community.asp#:~:text=Because%20of%20stigma%20about%20PTSD%2C%20others%20may%20look,race%2C%20sexuality%2C%20gender%20identity%2C%20religion%20and%20disability%20status"
                                }
                            }
                        }
                        li {
                            p {
                                "Kantor V, Knefel M, Lueger-Schuster B. Perceived barriers and facilitators of mental health service utilization in adult trauma survivors: A systematic review. "
                                em { "Clin Psychol Rev" }
                                ". 2017;52:52-68. doi:10.1016/j.cpr.2016.12.001"
                            }
                        }
                        li {
                            p {
                                "Parrott, S. (2023). PTSD in the News: Media Framing, Stigma, and Myths About Mental Illness. "
                                em { "Electronic News" }
                                ", 17(3), 181-197."
                            }
                        }
                        li {
                            p {
                                "Ghuloum S, Mahfoud ZR, Al-Amin H, Marji T, Kehyayan V. Healthcare Professionals' Attitudes Toward Patients With Mental Illness: A Cross-Sectional Study in Qatar. "
                                em { "Front Psychiatry." }
                                " 2022;13:884947. Published 2022 May 16. doi:10.3389/fpsyt.2022.884947"
                            }
                        }
                    }
                }
                ),
            })))
        }
    }
}
