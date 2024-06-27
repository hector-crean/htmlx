use blocks::block::brain::brain_region::BrainRegionName;
use blocks::block::brain::interactive_brain::{BrainComment, CommentGroup, InteractiveBrainProps};
use blocks::block::definition::{DefinitionListProps, DefinitionProps};
use blocks::block::icon::IconProps;
use blocks::span::ref_note::RefNote;
use blocks::SvgName;

use blocks::block::references::ReferencesProps;
use blocks::block::rich_text::{RichText, RichTextBlock};
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use blocks::block::Block;
use maud::{html, Render};

use std::vec;

use crate::{rich_text, rich_text_block};

pub struct Page;

impl Page {
    fn intrusion_tab(&self) -> Tab {
        Tab {
            name: String::from("Intrusion"),
            blocks: vec![Block::InteractiveBrainBlock(InteractiveBrainProps {
                id: String::from("intrusion-interactive-brain"),
                description: html! {
                    span  {
                        "Intrusive symptoms in PTSD have been linked to the " span class="text-white bg-[#ff3636]"{ "hippocampal"} " and " span class="text-white bg-[#3283a8]"{ "amygdala" } " regions of the brain"(RefNote(3))
                    }
                },
                definitionList: Some(DefinitionListProps {
                    definitions: self.brain_definitins().clone(),
                }),
                groups: vec![CommentGroup {
                    name: "Intrusion".into(),
                    comments: vec![
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::Disassociation,
                            },
                            symptom: html!(span {"Dissociative reactions (e.g., flashbacks)"(RefNote(4))}),
                            highlighted_regions: vec![
                                BrainRegionName::Hippocampus,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                "Flashbacks, a hallmark symptom of posttraumatic stress disorder (PTSD), "
                                "involve the involuntary re-experiencing of intrusive memories from a past traumatic event. "
                                "These memories are characterized by a temporal distortion, making them feel as if they are happening in the present moment, "
                                "leading individuals to react as if the trauma were occurring again. "
                                "Flashbacks are described as brief, vivid, and rich in sensory details by PTSD patients"(RefNote(5))". "
                                "It's suggested that heightened activity in the anterior insula contributes to these reliving experiences, "
                                "implying its role in the time distortion reported during flashbacks"(RefNote(6))". "
                                }

                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::DistressingDreams,
                            },
                            symptom: html! { span { "Recurrent distressing dreams"(RefNote(4))}},
                            highlighted_regions: vec![
                                BrainRegionName::Hippocampus,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                "Individuals with PTSD commonly experience sleep disturbances, including posttraumatic nightmares"(RefNote(11))". "
                                "These nightmares often involve re-enactments of the trauma. Awakenings from these nightmares are accompanied by intense fear or anxiety, "
                                "leading to difficulty returning to sleep, contributing to excessive fatigue, "
                                "concentration difficulties, irritability, and feelings of helplessness, negatively impacting quality of life. "
                                "Despite existing therapies, nightmares can persist for decades and may prompt avoidance behaviors before bedtime, "
                                "exacerbating insomnia and daytime dysfunction"(RefNote(12))". "
                                "Additionally, research has linked nightmares to a significant increase in suicidality"(RefNote(13))". "
                                }

                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::PrologedPsychologicalStress,
                            },
                            symptom: html!(span { "Intense or prolonged psychological distress to external/internal cues"}),
                            highlighted_regions: vec![
                                BrainRegionName::Hippocampus,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                "In posttraumatic stress disorder (PTSD), individuals may experience intense or prolonged psychological distress triggered by external or internal cues related to their traumatic experiences. "
                                "These cues can be anything that reminds the individual of the traumatic event, such as sights, sounds, smells, or even certain thoughts or emotions"(RefNote(7))". "
                                "These triggers can involuntarily bring back unwanted memories of the event, leading to repetitive and distressing recollections that disrupt mental well-being"(RefNote(8))". "
                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::DistressingMemories,
                            },
                            symptom: html!(span { "Recurrent, involuntary, and intrusive distressing memories"}),
                            highlighted_regions: vec![
                                BrainRegionName::Hippocampus,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                "In PTSD, distressing memories of the traumatic event often recur involuntarily, despite efforts to avoid them. "
                                "These memories, including flashbacks, nightmares, and intrusive thoughts, can significantly disrupt daily life. "
                                "In children, repetitive play may occur in which themes or aspects of the traumatic events are re-enacted"(RefNote(10))". "
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::PsychologicalReactions,
                            },
                            symptom: html!(span { "Marked physiological reactions to external/internal cues"}),
                            highlighted_regions: vec![
                                BrainRegionName::Hippocampus,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "In response to external and internal triggers, individuals with PTSD can also experience physical sensations, and panic attacks, "
                                    "which can present with increased heart rate and blood pressure"(RefNote(9))(RefNote(10))". "

                                }
                            },
                        },
                    ],
                }],
            })],
        }
    }
    fn avoidance_tab(&self) -> Tab {
        Tab {
            name: String::from("Avoidance"),
            blocks: vec![Block::InteractiveBrainBlock(InteractiveBrainProps {
                id: String::from("avoidance-interactive-brain"),
                description: html!(span { "Greater avoidance symptomatology has been associated with greater activation in " span class="text-white bg-[#609fbb]"{ "amygdala"}", " span class="text-white bg-[#ff3636]"{ "hippocampus"}", " span class="text-white bg-[#9c548c]"{ "prefrontal cortex"} ", and dysfunction in the " span class="text-white bg-[#00ff00]" { "striatum" } (RefNote(14))(RefNote(15))(RefNote(16))}),
                definitionList: Some(DefinitionListProps {
                    definitions: self.brain_definitins().clone(),
                }),
                groups: vec![CommentGroup {
                    name: "Avoidance".into(),
                    comments: vec![
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::AvoidDistressingThoughts,
                            },
                            symptom: html!(span { "Avoidance of or efforts to avoid distressing memories, thoughts, or feelings"(RefNote(4))}),
                            highlighted_regions: vec![
                                BrainRegionName::Hippocampus,
                                BrainRegionName::Amygdala,
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::Striatum,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "Research consistently shows a link between avoidant coping and the severity of PTSD symptoms. "
                                    "Avoidance impedes the processing of traumatic memories and the reduction of associated negative emotions, "
                                    "hindering natural recovery and reinforcing the perception of danger associated with those memories"(RefNote(17))". "
                                    "Additionally, studies suggest that females tend to exhibit more pronounced avoidance, highlighting gender differences in PTSD symptomatology"(RefNote(18))". "
                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::AvoidReminders,
                            },
                            symptom: html!( span { "Avoidance of or efforts to avoid external reminders that arouse distressing memories, thoughts, or feelings"(RefNote(4)) }),
                            highlighted_regions: vec![
                                BrainRegionName::Hippocampus,
                                BrainRegionName::Amygdala,
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::Striatum,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "Research consistently indicates that individuals with PTSD tend to avoid external reminders of their trauma, such as specific places and activities. "
                                    "This avoidant coping strategy is closely tied to the severity of PTSD symptoms. By steering clear of external triggers, "
                                    "individuals hinder the natural processing of traumatic memories and the subsequent reduction of associated negative emotions, "
                                    "reinforcing the perception of danger associated with those memories"(RefNote(17))". "
                                    "Additionally, studies suggest that females tend to exhibit more pronounced avoidance, highlighting gender differences in PTSD symptomatology"(RefNote(18))". "

                                }
                            },
                        },
                    ],
                }],
            })],
        }
    }
    fn arousal_tab(&self) -> Tab {
        Tab {
            name: String::from("Arousal"),
            blocks: vec![Block::InteractiveBrainBlock(InteractiveBrainProps {
                id: String::from("arousal-interactive-brain"),
                description: html! {
                    span {
                        "Arousal symptoms are associated with increased activity in the " span class="text-white bg-[#3283a8]"{ "amygdala" } " and decreased activity in the medial " span class="text-white bg-[#9c548c]"{ "prefrontal cortex" } (RefNote(4))". "
                    "In healthy individuals, prefrontal brain regions regulate anger and aggression, while subcortical areas like the " span class="text-white bg-[#3283a8]"{ "amygdala" } " and ventral " span class="text-white bg-[#00ff00]"{ "striatum" } " may heighten these emotions. "
                    "It is hypothesized that reduced prefrontal control over subcortical regions in PTSD could predispose individuals to angry and aggressive behavior"(RefNote(25))". "
                    "Heightened sensory reactivity may arise from chronic increases in dopaminergic and noradrenergic levels in PTSD, which suppress sensory gating and enhance sensory cortical activity"(RefNote(26))". "
                    }
                },
                definitionList: Some(DefinitionListProps {
                    definitions: self.brain_definitins().clone(),
                }),
                groups: vec![CommentGroup {
                    name: "Arousal".into(),
                    comments: vec![
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::SleepDisturbance,
                            },
                            symptom: html! { span {"Sleep disturbance"(RefNote(4))}},
                            highlighted_regions: vec![
                                BrainRegionName::Amygdala,
                                BrainRegionName::Striatum,
                                BrainRegionName::PrefrontalCortex,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "Emerging evidence highlights insomnia as a core element of PTSD, mechanistically linked to its onset and persistence. "
                                    "Various mechanisms, such as neurobiological changes, impaired fear extinction, disrupted emotional processing during sleep, "
                                    "and repeated exposure to trauma cues during nightmares, contribute to this connection. "
                                    "These mechanisms elucidate how insomnia obstructs natural recovery from trauma, "
                                    "fosters PTSD development, and hampers response to treatments"(RefNote(27))". "

                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::AvoidDistressingThoughts,
                            },
                            symptom: html! { span {"Problems with concentration"(RefNote(4))}},
                            highlighted_regions: vec![
                                BrainRegionName::Amygdala,
                                BrainRegionName::Striatum,
                                BrainRegionName::PrefrontalCortex,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "Patients diagnosed with PTSD commonly report challenges related to concentration, attention, and memory. "
                                    "Studies have revealed poorer cognitive performance in individuals with PTSD on tasks assessing attention, "
                                    "declarative memory, and other cognitive functions"(RefNote(28))". "
                                    "Diminished cognitive performance in PTSD may be associated with reductions in hippocampal volume"(RefNote(29))(RefNote(30))". "

                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::AvoidDistressingThoughts,
                            },
                            symptom: html! { span {"Reckless or self-destructive behavior"}},
                            highlighted_regions: vec![
                                BrainRegionName::Amygdala,
                                BrainRegionName::Striatum,
                                BrainRegionName::PrefrontalCortex,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "The severity of PTSD correlates with challenges in impulse control, often leading to impulsive, reckless, and potentially self-destructive behaviors"(RefNote(32))(RefNote(32))". "
                                    "These actions can range from substance use and risky sexual behaviors to intentional self-harm"(RefNote(33))(RefNote(34))". "
                                    "A study on Veterans receiving treatment for PTSD revealed that a significant number of deaths were related to substance use, suicide, and homicide"(RefNote(35))". "

                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::Hypervigilance,
                            },
                            symptom: html! { span {"Hypervigilance"(RefNote(4))}},
                            highlighted_regions: vec![
                                BrainRegionName::Amygdala,
                                BrainRegionName::Striatum,
                                BrainRegionName::PrefrontalCortex,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "Hypervigilance, characterized by excessive and constant scanning of the environment for potential threats, "
                                    "may exacerbate PTSD symptoms by focusing attention on perceived dangers"(RefNote(36))". "
                                    "Individuals with PTSD often exhibit hypersensitivity to a wide range of sensory stimuli, not just trauma-related cues"(RefNote(26))". "

                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::IrritbaleBehaviour,
                            },
                            symptom: html! { span {"Irritable behavior and  angry outbursts"(RefNote(4))}},
                            highlighted_regions: vec![
                                BrainRegionName::Amygdala,
                                BrainRegionName::Striatum,
                                BrainRegionName::PrefrontalCortex,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "Emerging evidence suggests that anger escalates in individuals with PTSD following a traumatic event, "
                                    "with several studies indicating that anger in PTSD sufferers correlates with the severity and persistence of symptoms over time"(RefNote(37))". "

                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::ExageratedStartleResponse,
                            },
                            symptom: html! { span {"Exaggerated startle response"}},
                            highlighted_regions: vec![
                                BrainRegionName::Amygdala,
                                BrainRegionName::Striatum,
                                BrainRegionName::PrefrontalCortex,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "The acoustic startle response (ASR) is a subcortical reflex modulated by neural systems implicated in PTSD. "
                                    "Research indicates that individuals with a history of childhood abuse, whether it involves significant levels of physical or sexual abuse, "
                                    "exhibit a heightened startle response compared to those who experienced lower levels of abuse. "
                                    "The long-lasting effects of early life trauma contribute to an elevated risk of developing psychological issues in adulthood"(RefNote(38))". "

                                }
                            },
                        },
                    ],
                }],
            })],
        }
    }
    fn congition_mood_tab(&self) -> Tab {
        Tab {
            name: String::from("Cognition/Mood"),
            blocks: vec![Block::InteractiveBrainBlock(InteractiveBrainProps {
                id: String::from("negative-interactive-brain"),
                description: html! {
                    span {
                        "Negative emotional symptoms, also known as hypoarousal, are associated with increased activation of the medial prefrontal cortex and rostral anterior cingulate regions "
                        "and heightened inhibition of limbic region"(RefNote(4))(RefNote(19))
                        ", and decreased activity in the " span class="bg-[#00ff00] text-white"{ "stiatum" } " and " span class="text-white bg-[#3283a8]"{ "amygdala" }(RefNote(4))(RefNote(16))(RefNote(20))". "
                    }
                },
                definitionList: Some(DefinitionListProps {
                    definitions: self.brain_definitins().clone(),
                }),
                groups: vec![CommentGroup {
                    name: "Negative".into(),
                    comments: vec![
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::Disassociation,
                            },
                            symptom: html! { span {"Feelings of detachment or estrangement from others"}},
                            highlighted_regions: vec![
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::AnteriorCingulateCortex,
                                BrainRegionName::Striatum,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "Emotional detachment, a common reaction to trauma that helps people escape painful negative emotions, is often observed in PTSD patients, "
                                    "characterized by dissociative symptoms such as depersonalization, derealization, and emotional numbing"(RefNote(4))". "
                                    "These symptoms manifest as a sense of detachment from surroundings, distorted reality, and disconnected thoughts, emotions, and sensations, "
                                    "often emerging during or after traumatic events"(RefNote(21))". "
                                }
                                ul class="list" {
                                    li { "Dissociative symptoms have been associated with complex trauma in children, such as experiences of physical, sexual, and emotional abuse and neglect"(RefNote(22))". "}
                                    li {
                                        "Dissociative symptoms are prevalent in military personnel and veterans diagnosed with PTSD, with studies indicating higher levels of dissociation "
                                        "in those with combat-related PTSD compared to those without"(RefNote(23))(RefNote(24))". "
                                    }
                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::AvoidReminders,
                            },
                            symptom: html! { span {"Inability to remember"}},
                            highlighted_regions: vec![
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::AnteriorCingulateCortex,
                                BrainRegionName::Striatum,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "The inability to recall important aspects of the traumatic event in PTSD can be due to dissociative amnesia, drugs, or alcohol use"(RefNote(10))". "
                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::Hypervigilance,
                            },
                            symptom: html! { span {"Persistent, distorted cognitions"}},
                            highlighted_regions: vec![
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::AnteriorCingulateCortex,
                                BrainRegionName::Striatum,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "In PTSD, individuals often harbor persistent, exaggerated, and distorted perceptions of themselves, others, and the world, leading to pervasive feelings of self-blame and mistrust"(RefNote(10))". "
                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::DistressingMemories,
                            },
                            symptom: html! { span {"Persistent negative emotional state"}},
                            highlighted_regions: vec![
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::AnteriorCingulateCortex,
                                BrainRegionName::Striatum,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "In PTSD, persistent negative emotions include fear, guilt, anger, or shame"(RefNote(10))". "
                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::IrritbaleBehaviour,
                            },
                            symptom: html! { span {"Persistent or exaggerated bad feelings"}},
                            highlighted_regions: vec![
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::AnteriorCingulateCortex,
                                BrainRegionName::Striatum,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    r#"In PTSD, common beliefs, such as "I am inherently flawed" or "The world is dangerous, "#
                                    "further reinforce the sense of emotional turmoil and contribute to the enduring distress characteristic of PTSD"(RefNote(10))". "
                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::AvoidDistressingThoughts,
                            },
                            symptom: html! { span {"Persistent inability to experience positive emotions"}},
                            highlighted_regions: vec![
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::AnteriorCingulateCortex,
                                BrainRegionName::Striatum,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "In PTSD, persistent negative emotions, which may have intensified following the trauma, "
                                    "overshadow any potential for positive emotions, leaving individuals trapped in a cycle of despair and hopelessness"(RefNote(10))". "

                                }
                            },
                        },
                        BrainComment {
                            icon: IconProps {
                                name: SvgName::PsychologicalReactions,
                            },
                            symptom: html! { span {"Marked diminished interest"}},
                            highlighted_regions: vec![
                                BrainRegionName::PrefrontalCortex,
                                BrainRegionName::AnteriorCingulateCortex,
                                BrainRegionName::Striatum,
                                BrainRegionName::Amygdala,
                            ],
                            overview: None,
                            description: html! {
                                p {
                                    "PTSD is associated with diminished interest in significant activities that used to be enjoyable"(RefNote(10))". "

                                }
                            },
                        },
                    ],
                }],
            })],
        }
    }
    fn brain_definitins(&self) -> Vec<DefinitionProps> {
        vec![
            DefinitionProps {
                id: String::from("hippocampus"),
                color: Some(String::from("red")),
                term: html!("Hippocampus"),
                definition: html! {"involved in memory consolidation and maintenance of long-term memory"(RefNote(1))},
            },
            DefinitionProps {
                id: String::from("prefrontal-cortex"),
                color: Some(String::from("#9C548C")),
                term: html! {"Prefrontal Cortex"},
                definition: html! {
                    "involved in working memory, thinking, cognitive processes, and attention"(RefNote(1))
                },
            },
            DefinitionProps {
                id: String::from("amygdala"),
                color: Some(String::from("#3283a8")),
                term: html!("Amygdala"),
                definition: html! { "involved in emotional and behavioral regulation, regulates fear"(RefNote(1))},
            },
            DefinitionProps {
                id: String::from("striatum"),
                color: Some(String::from("green")),
                term: html!("Striatum"),
                definition: html! { "plays a central role in the motor and reward systems"(RefNote(2))},
            },
            DefinitionProps {
                id: String::from("anterior-cingulate-cortex"),
                color: Some(String::from("#ed0aff")),
                term: html! { "Anterior cingulate cortex"},
                definition: html!("involved in decision-making and impulse control"(RefNote(
                    1
                ))),
            },
        ]
    }
}

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (Block::TabsBlock(TabsProps {
                id: uuid::Uuid::new_v4(),
                representation: TabsRepresentation::TopLevel,
                tabs: vec![
                    self.intrusion_tab(),
                    self.avoidance_tab(),
                    self.congition_mood_tab(),
                    self.arousal_tab(),

                    ]
            }))
            //References
            (Block::ReferencesBlock(Box::new(ReferencesProps {
                references: rich_text_block!(
                    "../../../input/OTS126_PTSD_Symptoms_Node/38c6fe92-2267-4814-ac2a-f4313f037a44.html"
                ),
            })))
        }
    }
}
