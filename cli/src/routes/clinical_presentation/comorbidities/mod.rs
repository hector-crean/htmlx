use crate::rich_text_block;
use blocks::{
    block::{
        bar_chart::{BarChartDatum, BarChartProps, Margin, PtsdComborbiditiesLayout},
        html,
        placeholder_container::PlaceholderContainerProps,
        references::ReferencesProps,
        Block,
    },
    span::ref_note::RefNote,
};
use maud::{html, Render};

pub struct Page;

impl Page {
    fn bars(&self) -> Vec<BarChartDatum> {
        vec![
            BarChartDatum::new(
                0,
                "Substance use",
                46.,
                None,
                Some("#33ff57"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#33ff57".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "46".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#33ff57]" {"Substance use"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Substance use" }
                            p {
                            "The self-medication theory is the leading explanation for the connection between substance use and PTSD. "
                            "According to this theory, individuals with PTSD are more likely to turn to alcohol or drugs to cope with "
                            "the distressing symptoms and consequences of their condition, increasing their risk of substance use"(RefNote(13))". "
                            "Substance use is more common among men than women"(RefNote(14))". "
                            "Veterans with PTSD have an increased risk of substance use disorders"(RefNote(15))". "
                            }
                        },
                    },
                )),
            ),
            BarChartDatum::new(
                0,
                "Alcohol use",
                46.,
                None,
                Some("#ff8d33"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#ff8d33".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "10".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#ff8d33]" {"Alcohol use"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Alcohol use" }
                            p {
                                "The self-medication theory is the leading explanation for the connection between substance use and PTSD. "
                                "According to this theory, individuals with PTSD are more likely to turn to alcohol or drugs to cope with "
                                "the distressing symptoms and consequences of their condition, increasing their risk of substance use"(RefNote(13))". "
                                "Research indicates a reciprocal relationship between PTSD and alcohol use: PTSD may lead to alcohol use due "
                                "to self-medication, while alcohol use may increase vulnerability to traumatic events and subsequent PTSD"(RefNote(13))". "

                            }
                        },
                    },
                )),
            ),
            BarChartDatum::new(
                0,
                "Major Depressive Disorder",
                50.,
                None,
                Some("#33ff8d"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#33ff8d".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "50".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#33ff8d]" {"Major Depressive Disorder (MDD)"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Major Depressive Disorder" }
                            p {
                                "Approximately 50% of individuals with PTSD are reported to have MDD"(RefNote(3))(RefNote(9))". "
                                "Furthermore, comorbidity suggests an increased illness burden with a severe course of "
                                "impairments and delay in treatment response. It is thought that depression may increase "
                                "the risk of developing PTSD following a traumatic experience"(RefNote(17))"."
                           }
                        },
                    },
                )),
            ),
            BarChartDatum::new(
                0,
                "Anxiety",
                10.,
                None,
                Some("#ffc733"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#ffc733".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "10".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#ffc733]" {"Anxiety"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Anxiety" }
                            p {
                                "Individuals with PTSD are more likely to have an anxiety disorder, with odds ranging from 2.4 to 7.1 times higher"(RefNote(9))". "
                                "Anxiety is often triggered by reminders of the traumatic event and can result in pervasive feelings of worry and "
                                "apprehension"(RefNote(2))". "
                                "The most prevalent among these anxiety disorders include phobias such as simple, social, "
                                "and agoraphobia, as well as generalized anxiety disorder"(RefNote(2))(RefNote(9))". "
                                "Research also indicates that individuals with PTSD may be at a higher risk of developing OCD. "
                                "Experiences of trauma and violation may lead to heightened sensitivity to perceived mental contamination "
                                "and a feeling of 'dirtiness’ that may contribute to the development or exacerbation of OCD symptoms"(RefNote(18))". "

                           }
                        },
                    },
                )),
            ),
            BarChartDatum::new(
                0,
                "Self-harm",
                5.,
                Some(19.),
                Some("#ff5733"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#ff5733".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "5-19".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#ff5733]" {"Self-harm"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Self-harm" }
                            p {
                                "Symptoms of PTSD have been identified as predictors of deliberate self-harm (DSH)"(RefNote(4))". "
                                "Depression in patients with PTSD often co-exists with suicidal ideation"(RefNote(2))". "
                                "Patients with substance use disorders (SUD) are of special interest, "
                                "often showing elevated rates of suicide attempts and DSH"(RefNote(4))". "
                           }
                        },
                    },
                )),
            ),
            //
            BarChartDatum::new(
                1,
                "Sleep dysfunction",
                70.,
                Some(87.),
                Some("#338dff"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#338dff".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "5-19".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#338dff]" {"Sleep dysfunction"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Sleep dysfunction" }
                            p {
                                "70-87% of patients diagnosed with PTSD experience sleep disturbances, including insomnia, posttraumatic nightmares, "
                                "awakenings, periodic limb movement disorder (PLMD), and obstructive sleep apnea (OSA)"(RefNote(8))(RefNote(19))(RefNote(20))". "
                                "Posttraumatic nightmares often involve reliving the trauma and can lead to intense fear or anxiety upon waking, "
                                "making it difficult to return to sleep"(RefNote(21))"."
                                "Awakenings in PTSD may not always be linked to distressing dreams. "
                                "About 33% of PTSD patients also suffer from PLMD, which can cause awakenings"(RefNote(21))". "
                                "Insomnia symptoms are reported by "
                                "approximately 70% of patients and are often related to increased autonomic arousal and fear of sleep, including fear "
                                "of losing control or experiencing nightmares"(RefNote(20))". "
                                "OSA is also common among PTSD patients, affecting 40-90% of patients"(RefNote(20))"."

                           }
                        },
                    },
                )),
            ),
            BarChartDatum::new(
                1,
                "Chronic Pain",
                20.,
                None,
                Some("#e5ff33"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#e5ff33".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "5-19".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#e5ff33]" {"Chronic pain"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Chronic pain" }
                            p {
                                "PTSD and chronic pain co-occur through various mechanisms. "
                                "In PTSD, catastrophizing, characterized by magnifying pain or trauma severity, "
                                "feeling overwhelmed, and expecting the worst outcome, amplifies trauma perception and emotional distress. "
                                "Individuals experiencing this may exhibit less control over their pain and a greater emotional impact"(RefNote(22))". "
                                "Additionally, dysregulated cortisol levels may increase sensitivity to pain"(RefNote(23))". "
                                "Individuals experiencing both chronic pain and posttraumatic stress disorder (PTSD) "
                                "tend to suffer from more intense pain and reduced quality of life compared to those solely dealing with chronic pain. "
                                "Both conditions are closely linked, with high occurrences of chronic pain in PTSD patients and vice versa. "
                                "Moreover, individuals with this dual diagnosis are commonly prescribed opioid medications for pain relief, "
                                "putting them at a higher risk for opioid use"(RefNote(24))"."
                           }
                        },
                    },
                )),
            ),
            BarChartDatum::new(
                1,
                "Cardiometabolic disorders",
                25.,
                None,
                Some("#e5ff33"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#e5ff33".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "25".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#e5ff33]" {"Cardiometabolic disorders"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Cardiometabolic disorders" }
                            p {
                                "Research indicated that PTSD is associated with increased rates of physical comorbidities involving immune dysregulation, "
                                "such as metabolic syndromes, obesity, diabetes, hypertension, hyperlipidemia, atherosclerotic cardiovascular disease, "
                                "and autoimmune diseases"(RefNote(25))(RefNote(26))"."
                                "Studies show that female survivors of the 9/11 attacks with PTSD faced a higher risk of heart disease hospitalization"(RefNote(26))"."

                           }
                        },
                    },
                )),
            ),
            BarChartDatum::new(
                1,
                "Dementia",
                4.7,
                Some(7.8),
                Some("#e5ff33"),
                Some(Block::PtsdComborbiditiesLayoutBlock(
                    PtsdComborbiditiesLayout {
                        fill: "#e5ff33".to_string(),
                        increased_risk_scale: None,
                        increased_risk_percentage: Some((
                            "4.7 - 7.8".to_string(),
                            html! { p { "of patients with PTSD had " span class="bg-[#e5ff33]" {"Dementia"} " as a comorbidity"}},
                        )),
                        overview: html! {
                            h3 { "Dementia" }
                            p {
                                "Individuals with PTSD face a 1.55x higher risk of developing dementia"(RefNote(12))". "
                                "The exact mechanisms linking PTSD to cognitive decline and dementia remain unclear"(RefNote(28))". "
                                "However, research suggests that intrusive thoughts, a key symptom of PTSD, may predict cognitive impairment years later"(RefNote(28))". "


                           }
                        },
                    },
                )),
            ),
        ]
    }
}

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/fc9c5804-9b7a-420c-8e82-a28be4076d56.html"),
        // Block::BarChartBlock(BarChartProps {
        //     title: String::from("Psychiatric Comorbidities"),
        //     bars: psychiatric_comborbidities_bars()
        // }),
        h2 {
            "Comborbidities associated with PTSD"
        }
        div class="panel" {
            p {
                span class="text-bold"{"Filter"} " the graph by medical or psychiatric comorbidities. "
                span class="text-bold"{"Select"}" a comorbidity bar to see more info."
            }

        }

        (Block::PlaceholderContainerBlock(PlaceholderContainerProps {
            id: String::from("ptsd-comorbidities-bar-chart"),
            full_bleed: true,
            class: String::from("round-lg")
        }))

        (Block::BarChartBlock(BarChartProps {
            title: String::from("Medical Comorbidities"),
            bars: self.bars(),
            margin: Margin::new(40., 20., 150., 40.),
            aspect_ratio: 2.

        }))

        // rich_text_block!("../../../input/clinical_presentation_comorbidities/ef31ae84-3c15-4fe3-a4c2-fc5fc3bbba00.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/a9c28646-8a85-4451-b325-84f66b4bd3b0.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/205c7abf-3558-47df-8802-24b65d66e573.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/a635eebc-678b-49bd-b3c6-7f784c0b0b1a.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/a38fe64f-0afb-4b78-9c63-c3af002238ac.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/ec93bd2c-999e-4377-9a9f-ba2ff44e0479.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/78ec8944-fa18-455e-babe-c5fa6cd7fb69.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/e2a3348c-921d-4aee-ba5d-0d9d5ea0d12d.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/f49aea70-5faa-4d15-9f1f-c461a474f83c.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/89a0070f-52df-4323-926e-cf76f526ced9.html"),
        // rich_text_block!("../../../input/clinical_presentation_comorbidities/f9f9d6ac-a315-469a-b56c-d61606380c5b.html"),

        (Block::ReferencesBlock(Box::new(ReferencesProps {
            references: rich_text_block!("../../../input/clinical_presentation_comorbidities/e0c2fb34-6776-4a5e-bc6f-cbaacfc675b2.html")
        })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
