use blocks::block::brain::brain_glossary::BrainGlossaryProps;
use blocks::block::brain::brain_region::BrainRegionName;
use blocks::block::brain::interactive_brain::{BrainComment, CommentGroup, InteractiveBrainProps};
use blocks::block::definition::{DefinitionListProps, DefinitionProps};
use blocks::block::disclosure::{DisclosureProps, DisclosureTheme};
use blocks::block::icon::IconProps;
use blocks::span::ref_note::RefNote;
use blocks::SvgName;

use blocks::block::references::ReferencesProps;
use blocks::block::rich_text::{RichText, RichTextBlock, RichTextProps};
use blocks::block::tabs::{Tab, TabsProps, TabsRepresentation};
use blocks::block::{html, Block};
use maud::{html, Render};

use std::vec;

const BLUES: [&str; 7] = [
    "#bcdbff", "#eaeded", "#CC91D3", "#AFE2DD", "#C0E9DD", "#D2F0E2", "#E4F6EA",
];

use crate::{rich_text, rich_text_block};

pub struct Page;

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (Block::DisclosureBlock(DisclosureProps {
            theme: DisclosureTheme::new(BLUES[0], "#000000"),
            id: uuid::Uuid::new_v4().to_string(),
            summary: vec![
                Block::Html(
                    html! {
                        em { "Clinician Related Scale"}
                        strong { "DSM-5" }
                    }
                )
            ],
            longform: vec![
                Block::Html(html! {
                    p { "PTSD is diagnosed using the Diagnostic and Statistical Manual of Mental Disorders, Fifth Edition (DSM-5) criteria1." }

                    h2 { "Criterion A: Exposure to trauma" }
                    p { "This can occur either by direct or indirect confrontation with extreme trauma." }
                    p { "Specific examples include:" }
                    ul class="list" {
                        li { "Direct exposure" }
                        li { "Witnessing trauma" }
                        li { "Learning of a trauma" }
                        li { "Repeat or extreme indirect exposure to aversive details" }
                    }

                    h2 { "Criterion B: Intrusion symptoms" }
                    p { "At least one of the following must be present:" }
                    ul class="list" {
                        li { "Recurrent, involuntary, and intrusive distressing memories" }
                        li { "Recurrent distressing dreams" }
                        li { "Dissociative reactions (e.g., flashbacks)" }
                        li { "Intense or prolonged psychological distress to external/internal cues" }
                        li { "Marked physiological reactions to external/internal cues" }
                    }

                    h2 { "Criterion C: Persistent avoidance" }
                    p { "At least one of the following must be present:" }
                    ul class="list" {
                        li { "Avoidance of or efforts to avoid distressing memories, thoughts, or feelings" }
                        li { "Avoidance of or efforts to avoid external reminders that arouse distressing memories, thoughts, or feelings" }
                    }

                    h2 { "Criterion D: Negative alterations in cognition and mood" }
                    p { "At least two of the following must be present:" }
                    ul class="list" {
                        li { "Inability to remember" }
                        li { "Persistent or exaggerated bad feelings" }
                        li { "Persistent, distorted cognitions" }
                        li { "Persistent negative emotional state" }
                        li { "Marked diminished interest" }
                        li { "Feelings of detachment or estrangement from others" }
                        li { "Persistent inability to experience positive emotions" }
                    }

                    h2 { "Criterion E: Marked alterations in arousal and reactivity" }
                    p { "At least two of the following must be present:" }
                    ul class="list" {
                        li { "Irritable behavior and angry outbursts" }
                        li { "Reckless or self-destructive behavior" }
                        li { "Hypervigilance" }
                        li { "Exaggerated startle response" }
                        li { "Problems with concentration" }
                        li { "Sleep disturbance" }
                    }

                    h2 { "Criterion F: Duration" }
                    p { "The patient must experience criteria B, C, D and E for at least one month." }

                    h2 { "Criterion G: Functional significance" }
                    p { "Symptoms cause clinically significant distress or functional impairment in social, occupational or other domains." }
                    p { "Disability in at least one of these domains is required." }

                    h2 { "Criterion H: Exclusion" }
                    p { "Symptoms are not attributable to another medical condition, substance use or medication." }
                }
                )
            ]
            }))
            (Block::DisclosureBlock(DisclosureProps {
                theme: DisclosureTheme::new(BLUES[0], "#000000"),
                id: uuid::Uuid::new_v4().to_string(),
                summary: vec![
                    Block::Html(
                        html! {
                            em { "Clinician Related Scale"}
                            strong { "CAPS-5"(RefNote::new(2))(RefNote::new(3))(RefNote::new(4))(RefNote::new(5)) }
                        }
                    )
                ],
                longform: vec![
                    Block::Html(html! {
                        p { "The Clinician-Administered PTSD Scale for DSM-5 (CAPS-5) is the gold standard used to both diagnose PTSD and measure the severity of its symptoms." }

                        h2 { "Structured Interview" }
                        p { "CAPS-5 is a structured set of 30 questions assessing the 20 DSM-5 PTSD symptoms, ensuring consistency in how PTSD symptoms are assessed." }

                        h2 { "Diagnosis and Severity" }
                        p { "It serves two purposes: making a categorical diagnosis of PTSD and providing a measure of PTSD symptom severity, rating PTSD symptoms on a 5-point scale, ranging from 0 (absent) to 4 (extreme/incapacitating)." }

                        h2 { "Comprehensive Evaluation" }
                        p { "Beyond just identifying symptoms, it assesses when symptoms started, how long they have lasted, the distress they cause, their impact on daily life, any changes since the last assessment, and the overall severity of the condition." }

                        h2 { "Dissociative Subtype" }
                        p { "The tool also checks for specific features of a dissociative subtype of PTSD, such as depersonalization (feeling detached from oneself) and derealization (feeling detached from reality)." }

                        h2 { "Process" }
                        p { "The CAPS was designed to be administered by clinicians and clinical researchers who have a working knowledge of PTSD but can also be administered by appropriately trained paraprofessionals. The full interview takes 45-60 minutes to administer." }
                    }
                    )
                ]
            }))
            (Block::DisclosureBlock(DisclosureProps {
                theme: DisclosureTheme::new(BLUES[1], "#000000"),
                id: uuid::Uuid::new_v4().to_string(),
                summary: vec![
                    Block::Html(
                        html! {
                            em { "Self Reported Scale"}
                            strong { "PCL-5"(RefNote::new(5))(RefNote::new(6))(RefNote::new(7)) }
                        }
                    )
                ],
                longform: vec![
                    Block::Html(html! {
                        p { "PTSD Checklist for DSM-5 (PCL-5) is a 20-item self-report measure that assesses the 20 DSM-5 symptoms of PTSD." }

                        h2 { "Purpose" }
                        ul class="list" {
                            li { "Monitoring symptom change during and after treatment" }
                            li { "Screening individuals for PTSD" }
                            li { "Making a provisional PTSD diagnosis" }
                        }

                        p { "There are three formats of the PCL-5:" }
                        ul class="list" {
                            li { "Without potentially traumatic event (PTE) assessment (i.e., without Criterion A from the DSM-5)" }
                            li { "With potentially traumatic event (PTE) assessment" }
                            li { "With Life Events Checklist for DSM-5 (LEC-5) and PTE assessment" }
                        }

                        h2 { "Usage" }
                        ul class="list" {
                            li { "The PCL-5 is not intended to be used as a stand-alone diagnostic tool." }
                            li { "Intended to assess patient symptoms in the past month" }
                            li { "For a comprehensive diagnosis, clinical interviewing skills and a structured interview (e.g., CAPS-5) are recommended." }
                            li { "The checklist can be self-administered by the patient and be completed in approximately 5-10 minutes." }
                            li { "A decrease in PCL-5 score ≤28 indicates a clinically significant change." }
                        }
                    }

                    )
                ]
            }))
            (Block::DisclosureBlock(DisclosureProps {
                theme: DisclosureTheme::new(BLUES[1], "#000000"),
                id: uuid::Uuid::new_v4().to_string(),
                summary: vec![
                    Block::Html(
                        html! {
                            em { "Clinician Related Scale"}
                            strong { "PC-PTSD-5"(RefNote::new(5))(RefNote::new(8)) }
                        }
                    )
                ],
                longform: vec![
                    Block::Html(html! {
                        p { "Primary Care PTSD screen for DSM-5 (PC-PTSD-5) is a self-report screening tool used in primary care settings to identify individuals with probable PTSD." }

                        h2 { "Purpose" }
                        p { "Designed for quick screening of trauma exposure and PTSD symptoms in time-limited primary care environments." }

                        h2 { "Format" }
                        ul class="list" {
                            li { "Consists of 6 yes/no items" }
                            li { "If an individual screens positive for PTSD, further assessment is necessary. Ideally, this follow-up assessment should be done using a structured interview, such as the Clinician-Administered PTSD Scale for DSM-5 (CAPS-5). However, if this is unavailable, additional assessment using a validated self-report measure, such as the PTSD Checklist for DSM-5 (PCL-5), is recommended." }
                        }
                    }

                    )
                ]
            }))
            (Block::DisclosureBlock(DisclosureProps {
                theme: DisclosureTheme::new(BLUES[0], "#000000"),
                id: uuid::Uuid::new_v4().to_string(),
                summary: vec![
                    Block::Html(
                        html! {
                            em { "Clinician Related Scale"}
                            strong { "CGI"(RefNote::new(9))}
                        }
                    )
                ],
                longform: vec![
                    Block::Html(html! {
                        p { "The Clinical Global Impression (CGI) Scale (CSI) is a tool used by clinicians to evaluate and track patients' progress and treatment responses over time. It is composed of two components and is used for monitoring purposes." }

                        h2 { "CGI-Severity (CGI-S)" }
                        p { "This component uses a 7-point scale to rate the current severity of the patient's illness. The clinician evaluates how severe the illness is at the time of assessment." }

                        h2 { "CGI-Improvement (CGI-I)" }
                        p { "This component rates the change in the patient's condition since the beginning of treatment. It helps in determining how much the patient's condition has improved or worsened over time." }
                    }

                    )
                ]
            }))
            (Block::DisclosureBlock(DisclosureProps {
                theme: DisclosureTheme::new(BLUES[0], "#000000"),
                id: uuid::Uuid::new_v4().to_string(),
                summary: vec![
                    Block::Html(
                        html! {
                            em { "Clinician Related Scale"}
                            strong { "SOTS"(RefNote::new(10))}
                        }
                    )
                ],
                longform: vec![
                    Block::Html(html! {
                        p { "Symptoms of Trauma Scale (SOTS) is a 12-item, interview-based, clinician-rating tool designed to assess the severity of trauma-related symptoms." }

                        h2 { "Severity over time" }
                        p { "While it does not diagnose PTSD, it measures symptom severity, making it valuable for tracking changes over time and screening for PTSD symptoms." }

                        h2 { "Scoring" }
                        p { "SOTS uses a 7-point scale based on specific behavioral criteria to evaluate symptom severity. This approach ensures consistent and accurate assessments, providing a clear understanding of the impact of symptoms on an individual's functioning." }
                    }
                    )
                ]
            }))
            (Block::DisclosureBlock(DisclosureProps {
                theme: DisclosureTheme::new(BLUES[1], "#000000"),
                id: uuid::Uuid::new_v4().to_string(),
                summary: vec![
                    Block::Html(
                        html! {
                            em { "Clinician Related Scale"}
                            strong { "B-IPF"(RefNote::new(11)) }
                        }
                    )
                ],
                longform: vec![
                    Block::Html(html! {
                        p { "The Brief Inventory of Psychosocial Functioning (B-IPF) is a concise, 7-item self-report questionnaire designed to assess an individual's functioning in various PTSD-related psychosocial domains." }

                        h2 { "Purpose" }
                        p { "To assess PTSD-related functional impairment in the prior 30 days across several functional domains, including romantic relationships, family relationships, work, friendships and socializing, parenting, education, and self-care." }

                        h2 { "Format" }
                        p { "Consists of 7 questions that cover each of the functional domains." }
                    }

                    )
                ]
            }))
            (Block::DisclosureBlock(DisclosureProps {
                theme: DisclosureTheme::new(BLUES[1], "#000000"),
                id: uuid::Uuid::new_v4().to_string(),
                summary: vec![
                    Block::Html(
                        html! {
                            em { "Clinician Related Scale"}
                            strong { "SF-36"(RefNote::new(12)) }
                        }
                    )
                ],
                longform: vec![
                    Block::Html(html! {
                        p { "The 36-Item Short-Form Health Survey (SF-36) is a self-report for evaluating health-related quality of life (HRQoL)." }

                        h2 { "Format" }
                        p { "A 36-item questionnaire that measures across 8 domains:" }
                        ul class="list" {
                            li { "Physical functioning" }
                            li { "Role physical" }
                            li { "Bodily pain" }
                            li { "General health" }
                            li { "Vitality" }
                            li { "Social functioning" }
                            li { "Role emotional" }
                            li { "Mental health" }
                        }
                    }

                    )
                ]
            }))
            (Block::ReferencesBlock(
                Box::new(ReferencesProps {
                    references: Block::Html(html! {
                        ol class="list" {
                            li {
                                p class="bn-inline-content" {
                                    "Yehuda R, Hoge CW, McFarlane AC, et al. Post-traumatic stress disorder. "
                                    em { "Nat Rev Dis Primers" }
                                    ". 2015;1:15057. Published 2015 Oct 8. doi:10.1038/nrdp.2015.57"
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "ISTSS - clinician administered PTSD scale (CAPS). "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="http://Istss.org" { "Istss.org" }
                                    ". Accessed May 30, 2024. "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="https://istss.org/clinical-resources/assessing-trauma/clinician-administered-ptsd-scale-(caps-5)" {
                                        "https://istss.org/clinical-resources/assessing-trauma/clinician-administered-ptsd-scale-(caps-5)"
                                    }
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "PTSD: National center for PTSD. "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="http://Ptsd.va.gov" { "Ptsd.va.gov" }
                                    ". Accessed May 30, 2024. "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="https://www.ptsd.va.gov/professional/assessment/adult-int/caps.asp" {
                                        "https://www.ptsd.va.gov/professional/assessment/adult-int/caps.asp"
                                    }
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "Weathers FW, Bovin MJ, Lee DJ, et al. The Clinician-Administered PTSD Scale for DSM-5 (CAPS-5): Development and initial psychometric evaluation in military veterans. "
                                    em { "Psychol Assess" }
                                    ". 2018;30(3):383-395. doi:10.1037/pas0000486"
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "Merians AN, Spiller T, Harpaz-Rotem I, Krystal JH, Pietrzak RH. Post-traumatic Stress Disorder. "
                                    em { "Med Clin North Am" }
                                    ". 2023;107(1):85-99. doi:10.1016/j.mcna.2022.04.003"
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "PTSD: National center for PTSD. "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="http://Ptsd.va.gov" { "Ptsd.va.gov" }
                                    ". Accessed May 30, 2024. "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="https://www.ptsd.va.gov/professional/assessment/adult-sr/ptsd-checklist.asp" {
                                        "https://www.ptsd.va.gov/professional/assessment/adult-sr/ptsd-checklist.asp"
                                    }
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "Marx BP, Lee DJ, Norman SB, et al. Reliable and clinically significant change in the clinician-administered PTSD Scale for DSM-5 and PTSD Checklist for DSM-5 among male veterans. "
                                    em { "Psychol Assess" }
                                    ". 2022;34(2):197-203. doi:10.1037/pas0001098"
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "PTSD: National center for PTSD. "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="http://Ptsd.va.gov" { "Ptsd.va.gov" }
                                    ". Accessed May 30, 2024. "
                                    a target="_blank" rel="noopener noreferrer nofollow" href="https://www.ptsd.va.gov/professional/assessment/screens/pc-ptsd.asp" {
                                        "https://www.ptsd.va.gov/professional/assessment/screens/pc-ptsd.asp"
                                    }
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "Busner J, Targum SD. The clinical global impressions scale: applying a research tool in clinical practice. "
                                    em { "Psychiatry" }
                                    em { "(Edgmont)" }
                                    ". 2007;4(7):28-37."
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    a target="_blank" rel="noopener noreferrer nofollow" href="http://10.Ford" { "Ford" }
                                    " JD, Mendelsohn M, Opler LA, et al. The Symptoms of Trauma Scale (SOTS): An Initial Psychometric Study. "
                                    em { "J Psychiatr Pract" }
                                    ". 2015;21(6):474-483. doi:10.1097/PRA.0000000000000107"
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    a target="_blank" rel="noopener noreferrer nofollow" href="https://ptsd.va.gov/PTSD/professional/assessment/functioning-other/ipf_psychosocial_function.asp" {
                                        "https://ptsd.va.gov/PTSD/professional/assessment/functioning-other/ipf_psychosocial_function.asp"
                                    }
                                }
                            }
                            li {
                                p class="bn-inline-content" {
                                    "Lins L, Carvalho FM. SF-36 total score as a single measure of health-related quality of life: Scoping review. "
                                    em { "SAGE Open Med" }
                                    ". 2016;4:2050312116671725. Published 2016 Oct 4. doi:10.1177/2050312116671725"
                                }
                            }
                        }
                    }
                    )
                })))

        }
    }
}
