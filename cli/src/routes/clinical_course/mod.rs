use blocks::{
    block::{
        html::HtmlProps,
        nav::NavProps,
        references::ReferencesProps,
        suggested_node::SuggestedNodeProps,
        table::Table,
        tabs::{Tab, TabsProps, TabsRepresentation},
        Block,
    },
    span::ref_note::RefNote,
};
use maud::{html, Render};

pub struct Page;

impl Page {
    fn delayed_onset_ptsd_tab(&self) -> Tab {
        Tab {
            name: "Delayed Onset PTSD".to_string(),
            blocks: vec![Block::Html(html! {
                div class="space-y-4 panel" {
                    p {
                        "PTSD symptoms may occur soon after index trauma or may be delayed"(RefNote(3))"."
                    }
                    p {
                        "Delayed onset PTSD is defined as meeting diagnostic criteria ≥6 months after the index event"(RefNote(1))". "
                        "PTSD onset is usually within 3 months of the trauma"(RefNote(2))"."
                    }
                    p {
                        "Individuals may not meet full criteria for PTSD for months after the trauma"(RefNote(2))". "
                        "Some symptoms may appear immediately but with a delay in meeting the full criteria (subsyndromal PTSD), or individuals can have a completely symptom free period"(RefNote(2))"."

                    }
                    p {
                        "At least 25% of individuals with PTSD experience delayed onset"(RefNote(3))
                        ", but this could be higher as one study found that 37.3% of individuals who had PTSD at 12 months post-trauma did not have PTSD at 3 months post-trauma"(RefNote(3))"."
                    }
                    p {
                        "The symptoms of PTSD and the relative predominance of different symptoms may vary over time,  as can duration of symptoms"(RefNote(2))"."
                    }
                    p {
                        "It is important to understand delayed-onset PTSD has consequences for follow-up and diagnosis of trauma victims"(RefNote(4))"."
                    }
                }

                div class="space-y-4 panel" {
                    h2 {
                        "Factors affecting delayed onset"
                    }
                    p {
                        "Military combat exposure, Western cultural background, and lower cumulative PTSD incidence (with initial subthreshold symptoms) have been associated with delayed-onset PTSD"(RefNote(5))
                    }
                    p {
                        "Multiple studies have found that there is a higher rate of delayed-onset PTSD in military populations compared to civilian populations"(RefNote(6))". "
                        "There are multiple hypotheses as to why, including emotional numbing and the effect of multiple tours"(RefNote(6))". "
                    }
                    p {
                        "Frequency of delayed-onset PTSD in civilians and military"(RefNote(6))
                    }
                }
            })],
        }
    }
    fn chronic_ptsd_tab(&self) -> Tab {
        Tab {
            name: "Chronic PTSD".to_string(),
            blocks: vec![Block::Html(html! {
                div class="space-y-4 panel" {
                    h3 {
                        "PTSD is considered chronic once symptoms have been present for more than 3 months"(RefNote(7))
                    }
                    p {
                        "1/3 of patients may recover by one-year follow-up"(RefNote(8))
                    }
                    p {
                       "More than 1/3 remain symptomatic 10 years after index trauma"(RefNote(8))
                    }
                    p {
                        "PTSD symptom recurrence and intensification may occur in response to reminders of the original trauma, ongoing life stressors, or newly experienced traumatic events"(RefNote(2))
                    }
                    p {
                        "Declining health, worsening cognitive functioning, and social isolation may exacerbate symptoms in the elderly"(RefNote(2))
                    }
                    p {
                        "The symptoms of PTSD and the relative predominance of different symptoms may vary over time,  as can duration of symptoms"(RefNote(2))
                    }
                }

            })],
        }
    }
    fn underdiagnosis_tab(&self) -> Tab {
        Tab {
            name: "Underdiagnosis".to_string(),
            blocks: vec![Block::Html(html! {
                div class="space-y-4 panel" {
                    h3 {
                        "Individuals with PTSD may experience significant delays in diagnosis"
                    }
                    p {
                        "≈50% of patients with PTSD seek treatment"(RefNote(9))(RefNote(10))(RefNote(11))
                    }
                    p {
                        "Average time from index trauma to symptom presentation: 2.2 years"(RefNote(12))
                    }
                    p {
                      "Average time from index trauma to PTSD diagnosis: 8.7 years"(RefNote(12))
                    }
                    p {
                        "Median time from onset of symptoms to treatment is 12 years"(RefNote(9))(RefNote(11))
                    }
                    p {
                        "Many patients and individuals with PTSD seek care for physical symptoms without mentioning psychiatric symptoms or trauma histories due to a lack of understanding regarding the relationship between trauma exposure and their own symptoms"(RefNote(13))
                    }
                }
                div class="space-y-4 panel" {
                    h3 {
                        "PTSD is often underdiagnosed or misdiagnosed as other mental health conditions"
                    }
                    h4 { "Most prominent barriers to care"(RefNote(14))}
                    ul class="list" {
                        li {
                            "Concerns related to stigma, shame/rejection"
                        }
                        li {
                            "Low mental health literacy"
                        }
                        li {
                            "Lack of knowledge and treatment-related doubts"
                        }
                        li {
                           "Fear of negative social consequences"
                        }
                        li {
                            "Limited resources"
                        }
                    }
                    p {
                        "Individuals with PTSD are 80% more likely than those without PTSD to have symptoms that meet diagnostic criteria for at least one other mental health disorder"
                        " (e.g., depressive, bipolar, anxiety, or substance use disorders)"(RefNote(2))"."
                    }
                    (Table::new(
                        Some([&html! { "Misdiagnosis"}, &html! { "Underdiagnosis"}]),
                        [
                            [
                                &html! {
                                        h4 {
                                            "In the primary care setting, of individuals meeting diagnostic criteria for PTSD"(RefNote(15))":"
                                        }
                                        ul class="list" {
                                            li {
                                                "50% were diagnosed with depression"
                                            }
                                            li {
                                                "23% were diagnosed with anxiety or panic attacks"
                                            }
                                            li {
                                                "11% received a diagnosis of PTSD"
                                            }
                                        }


                                },
                                &html! {
                                    p {
                                        "<50% of individuals who meet criteria for PTSD are correctly diagnosed in primary and secondary care settings"(RefNote(13))(RefNote(16))(RefNote(17))
                                    }
                                }
                            ]
                        ]
                    ))
                }
                div class="space-y-4 panel" {
                    h3 {
                        "Decreased awareness in the general population and underuse of existing diagnostic tools"
                    }
                       h4 {
                        "Misdiagnosis is associated with ineffective management, leading to negative impact on"(RefNote(20))":"
                       }
                       ul class="list" {
                        li {
                            "Treatment compliance"
                        }
                        li {
                            "Treatment response"
                        }
                        li {
                            "Patient satisfaction"
                        }
                       }

                    p {
                        "PTSD diagnostic screening tools are underused in practice"(RefNote(21))(RefNote(22))
                    }
                    (SuggestedNodeProps {
                        prompt: "Diagnostic screening tools".to_string(),
                        to_node_id: "diagnostic".to_string()
                    })
                    p {
                        "Only 7% trauma centres screen for PTSD"(RefNote(23))". "
                        "The gold-standard test for PTSD is considered to be the CAPS-5 test, but this is a time-consuming test that requires training"(RefNote(23))". "
                    }
                    p {
                        "PTSD screening tools can aid in provisional diagnosis, especially for physicians with minimal training in PTSD diagnosis, "
                        "and make it easier to diagnose PTSD without having to assess all PTSD symptoms, "
                        "which can be time consuming"(RefNote(21))(RefNote(22))". "
                        "Two screening tools that are psychometrically sound and aid in the probable diagnosis of PTSD: "
                        "Primary Care PTSD Screen for DSM-5 (PC-PTSD-5)"(RefNote(21))
                        "and PTSD Checklist for DSM-5 (PCL-5)"(RefNote(22))". "
                        "These tools are recommended in guidelines based on a systematic evidence-based review by the US Department of Veterans Affairs and the Department of Defense"(RefNote(24))
                    }
                    (Table::new(
                        Some(
                            [
                            &html!{ span {"PC-PTSD-5"(RefNote(24))}},
                            &html! { span { "PCL-5"(RefNote(21))}},
                            &html! { span {  "CAPS-5"(RefNote(22))}}
                        ]
                        ),
                        [
                            [
                                &html!{
                                    ul class="list" {
                                        li {
                                            "Shorter screening measure"
                                        }
                                        li {
                                            "Variability in best cut-off score in different populations"
                                        }
                                        li {
                                            "Primary care setting"
                                        }
                                    }
                                },
                                &html!{
                                    ul class="list" {
                                        li {
                                            "More accessible"
                                        }
                                        li {
                                            "Take time to complete"
                                        }
                                        li {
                                            "Require some expertise to interpret"
                                        }
                                    }
                                },
                                &html!{
                                    ul class="list" {
                                        li {
                                            "Detailed gold standard"
                                        }
                                        li {
                                            "Time-consuming"
                                        }
                                        li {
                                            "Requires training"
                                        }
                                    }
                                }
                            ]
                        ]
                    ))
                }
            })],
        }
    }
}

impl Render for Page {
    fn render(&self) -> maud::Markup {
        html! {
            (TabsProps {
                id: uuid::Uuid::new_v4(),
                representation: TabsRepresentation::TopLevel,
                tabs: vec![self.delayed_onset_ptsd_tab(), self.chronic_ptsd_tab(), self.underdiagnosis_tab()]
            })
            (ReferencesProps {
                references: Block::Html(html! {
                    ol class="list" {
                        li { "American Psychiatric Association. "; em { "Diagnostic and Statistical Manual of Mental Disorders" }; ". 1994. Accessed May 13, 2024. "; a href="https://dsm.psychiatryonline.org/doi/book/10.1176/appi.books.9780890420614.dsm-iv" { "link" } }
                        li { "American Psychiatric Association. "; em { "Diagnostic and Statistical Manual of Mental Disorders" }; " Fifth Edition. American Psychiatric Publishing; 2013. Accessed May 2, 2024. "; a href="https://dsm.psychiatryonline.org/doi/epub/10.1176/appi.books.9780890425596" { "link" } }
                        li { "Bryant RA, O'Donnell ML, Creamer M, McFarlane AC, Silove D. A Multisite Analysis of the Fluctuating Course of Posttraumatic Stress Disorder. "; em { "JAMA Psychiatry" }; " 2013;70(8):839-846. doi:10.1001/jamapsychiatry.2013.1137" }
                        li { "Utzon-Frank N, Breinegaard N, Bertelsen M, et al. Occurrence of delayed-onset post-traumatic stress disorder: a systematic review and meta-analysis of prospective studies. "; em { "Scandinavian Journal of Work, Environment & Health" }; " 2014;40(3):215-229. doi:10.5271/sjweh.3420" }
                        li { "PLACEHOLDER 1." }
                        li { "Andrews B, Brewin CR, Philpott R, Stewart L. Delayed-Onset Posttraumatic Stress Disorder: A Systematic Review of the Evidence. "; em { "AJP" }; " 2007;164(9):1319-1326. doi:10.1176/appi.ajp.2007.06091491" }
                        li { "Bisson JI, Roberts NP, Andrew M, Cooper R, Lewis C. Psychological therapies for chronic post-traumatic stress disorder (PTSD) in adults. "; em { "Cochrane Database of Systematic Reviews" }; " 2013;(12). doi:10.1002/14651858.CD003388.pub4" }
                        li { "Kessler RC, Sonnega A, Bromet E, Hughes M, Nelson CB. Posttraumatic Stress Disorder in the National Comorbidity Survey. "; em { "Archives of General Psychiatry" }; " 1995;52(12):1048-1060. doi:10.1001/archpsyc.1995.03950240066012" }
                        li { "Nobles CJ, Valentine SE, Gerber MW, Shtasel DL, Marques L. Predictors of treatment utilization and unmet treatment need among individuals with posttraumatic stress disorder from a national sample. "; em { "Gen Hosp Psychiatry" }; " 2016;43:38-45. doi:10.1016/j.genhosppsych.2016.09.001" }
                        li { "Koenen KC, Ratanatharathorn A, Ng L, et al. Posttraumatic stress disorder in the World Mental Health Surveys. "; em { "Psychol Med" }; " 2017;47(13):2260-2274. doi:10.1017/S0033291717000708" }
                        li { "Wang PS, Berglund P, Olfson M, Pincus HA, Wells KB, Kessler RC. Failure and Delay in Initial Treatment Contact After First Onset of Mental Disorders in the National Comorbidity Survey Replication. "; em { "Archives of General Psychiatry" }; " 2005;62(6):603-613. doi:10.1001/archpsyc.62.6.603" }
                        li { "PLACEHOLDER 2." }
                        li { "Greene T, Neria Y, Gross R. Prevalence, Detection and Correlates of PTSD in the Primary Care Setting: A Systematic Review. "; em { "J Clin Psychol Med Settings" }; " 2016;23(2):160-180. doi:10.1007/s10880-016-9449-8" }
                        li { "Kantor V, Knefel M, Lueger-Schuster B. Perceived barriers and facilitators of mental health service utilization in adult trauma survivors: A systematic review. "; em { "Clinical Psychology Review" }; " 2017;52:52-68. doi:10.1016/j.cpr.2016.12.001" }
                        li { "Meltzer EC, Averbuch T, Samet JH, et al. Discrepancy in diagnosis and treatment of post-traumatic stress disorder (PTSD): Treatment for the wrong reason. "; em { "J Behav Health Serv Res" }; " 2012;39(2):190-201. doi:10.1007/s11414-011-9263-x" }
                        li { "Liebschutz J, Saitz R, Brower V, et al. PTSD in Urban Primary Care: High Prevalence and Low Physician Recognition. "; em { "J Gen Intern Med" }; " 2007;22(6):719-726. doi:10.1007/s11606-007-0161-0" }
                        li { "Zammit S, Lewis C, Dawson S, et al. Undetected post-traumatic stress disorder in secondary-care mental health services: systematic review. "; em { "The British Journal of Psychiatry" }; " 2018;212(1):11-18. doi:10.1192/bjp.2017.8" }
                        li { "Institute of Medicine. "; em { "Treatment for Posttraumatic Stress Disorder in Military and Veteran Populations: Final Assessment" }; ". National Academies Press (US); 2014. Accessed May 13, 2024. "; a href="http://www.ncbi.nlm.nih.gov/books/NBK224878/" { "link" } }
                        li { "Parrott S. PTSD in the News: Media Framing, Stigma, and Myths About Mental Illness. "; em { "Electronic News" }; " 2023;17(3):181-197. doi:10.1177/19312431221146757" }
                        li { "Guess KF. Posttraumatic Stress Disorder: Early Detection is Key. "; em { "The Nurse Practitioner" }; " 2006;31(3):26." }
                        li { "Williamson MLC, Stickley MM, Armstrong TW, Jackson K, Console K. Diagnostic accuracy of the Primary Care PTSD Screen for DSM-5 (PC-PTSD-5) within a civilian primary care sample. "; em { "Journal of Clinical Psychology" }; " 2022;78(11):2299-2308. doi:10.1002/jclp.23405" }
                        li { "Geier TJ, Hunt JC, Nelson LD, Brasel KJ, deRoon-Cassini TA. Detecting PTSD in a traumatically injured population: The diagnostic utility of the PTSD Checklist for DSM-5. "; em { "Depress Anxiety" }; " 2019;36(2):170-178. doi:10.1002/da.22873" }
                        li { "Love J, Zatzick D. A Survey of Screening & Intervention for Comorbid Alcohol, Drugs, Suicidality, Depression & PTSD at Trauma Centers. "; em { "Psychiatr Serv" }; " 2014;65(7):918-923. doi:10.1176/appi.ps.201300399" }
                        li { "U.S Department of Veteran Affairs. "; em { "VA/DoD Clinical Practice Guideline for Management of Posttraumatic Stress Disorder and Acute Stress Disorder" }; ". 2023. "; a href="www.healthquality.va.gov/guidelines/MH/ptsd/VA-DoD-CPG-PTSD-Full-CPG.pdf" { "link" } }
                    }
                }
                )
            })
        }
    }
}
