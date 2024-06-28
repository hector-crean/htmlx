use blocks::{
    block::{
        html,
        references::ReferencesProps,
        table::{Table, TableConfig},
        tabs::{Tab, TabsProps, TabsRepresentation},
        Block,
    },
    span::ref_note::{FootNote, RefNote},
};
use maud::{html, Render};
use specta::reference::Reference;

pub struct Page;

impl Page {
    fn guidelines_tab(&self) -> Tab {
        Tab {
            name: "Guidelines".to_string(),
            blocks: vec![Block::Html(html! {

              div class="panel"{
                h2 { "Summary of US Guidelines for Pharmacotherapy"(RefNote(1)) }

                (Table::new(
                    TableConfig::default(),
                    Some([
                        &html! { span { "Drug/Class"} },
                        &html! { span { "FDA Approved"} },
                        &html! { span { "VA/DoD Guidelines (2023)"(RefNote(2))} },
                        &html! { span { "APA Guidelines (2004)"(RefNote(3))} },
                        &html! { span { "Drug/Class"(RefNote(4))} },
                        ]),
                        [
                            [
                                &html!(span { "Setraline"}),
                                &html!(span { "Yes"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                            ],
                            [
                                &html!(span { "Paroxetine"}),
                                &html!(span { "Yes"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                            ],
                            [
                                &html!(span { "Fluoxetine"}),
                                &html!(span { "-"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                            ],
                            [
                                &html!(span { "Other SSRIs"(FootNote::new("a"))}),
                                &html!(span { "-"}),
                                &html!(span { "Insufficient evidence"}),
                                &html!(span { "NA"}),
                                &html!(span { "NA"}),
                            ],
                            [
                                &html!(span { "Venlafaxine"}),
                                &html!(span { "-"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                                &html!(span { "+"}),
                            ],
                            [
                                &html!(span { "Quetiapine"}),
                                &html!(span { "-"}),
                                &html!(span { "-"}),
                                &html!(span { "NA"}),
                                &html!(span { "NA"}),
                            ],
                            [
                                &html!(span { "Risperidone"}),
                                &html!(span { "-"}),
                                &html!(span { "-"}),
                                &html!(span { "Insufficient evidence"}),
                                &html!(span { "Insufficient evidence"}),
                            ],
                            [
                                &html!(span { "Trcyclic antidepressants"(FootNote::new("a"))}),
                                &html!(span { "-"}),
                                &html!(span { "Insufficient evidence + weak"(FootNote::new("c"))}),
                                &html!(span { "NA"}),
                                &html!(span { "NA"}),
                            ],
                            [
                                &html!(span { "Prazosin"(FootNote::new("b"))}),
                                &html!(span { "-"}),
                                &html!(span { "- weak"}),
                                &html!(span { "NA"}),
                                &html!(span { "NA"}),
                            ],
                            [
                                &html!(span { "Benzodiazepines"(FootNote::new("a"))}),
                                &html!(span { "-"}),
                                &html!(span { "--"}),
                                &html!(span { "NA"}),
                                &html!(span { "NA"}),
                            ]
                        ]
                    )
                )

                dl {
                    dt { "+" }
                    dd { "recommendation for" }
                    dt { "-" }
                    dd { "recommendation against" }
                    dt { "--" }
                    dd { "strong recommendation against" }
                    dt { "NA" }
                    dd { "not addressed" }
                    dt { "a" }
                    dd { "Class of medications" }
                    dt { "b" }
                    dd { "First-line treatment for nightmares in PTSD according to AASM and modified from Merians AN et al, 2023" }
                    dt { "c"}
                    dd { "Weak recommendation for imipramine and amitriptyline only (adapted from Merians AN et al, 2023)."}
                }
              }

            })],
        }
    }
    fn psychotherapy_tab(&self) -> Tab {
        Tab {
            name: "Psychotherapy".to_string(),
            blocks: vec![Block::Html(html! {
                div class="panel" {
                    p {
                        "Psychotherapy has demonstrated clinical benefits for PTSD, including reduced symptom severity and improved remission rates"(RefNote(2))(RefNote(3))(RefNote(4))". "
                        "Several considerations must be made to utilize psychotherapy effectively, including"(RefNote(5))":"
                    }
                    ul class="list" {
                        li {"Treatment cost"}
                        li {"Resource availability"}
                        li {"Patient preference"}
                        li {"Comorbidities"}
                    }
                }

                div class="panel" {
                    h3 { "Cognitive Behavioral Therapy (CBT) for PTSD"}

                p {
                    "International and US guidelines strongly recommend trauma-focused cognitive behavioral therapy (TF-CBT) "
                    "as the first-line treatment for PTSD"(RefNote(2))(RefNote(6))". "
                    "Research shows that TF-CBT has a significantly greater effect on individuals with PTSD compared to other "
                    "psychotherapies and pharmacotherapies"(RefNote(3))". "
                }
                p {
                    "Cognitive behavioral therapy (CBT) emphasizes the connection between thoughts, feelings, and behaviors, "
                    "highlighting how changes in one area can enhance functioning in others. "
                    "For instance, modifying negative thought patterns can lead to healthier behaviors and better emotional regulation. "
                    "CBT addresses current issues and symptoms and is typically administered over 12-16 sessions, "
                    "either individually or in group settings"(RefNote(7))"."
                }

                h3 {"Prolonged Exposure (PE) for PTSD"}

                p {
                    "Prolonged Exposure (PE) is a form of Cognitive Behavioral Therapy (CBT) that helps patients gradually face trauma-related memories, "
                    "feelings, and situations they have been avoiding. PE works by having patients confront their fears, "
                    "which reduces PTSD symptoms and helps them regain control in their daily lives"(RefNote(1))(RefNote(8))". "
                }

                h3 {"Cognitive Processing Therapy (CPT) for PTSD"}

                p {
                    "Cognitive Processing Therapy (CPT) is another type of CBT designed to help patients modify and challenge unhelpful beliefs related to their trauma. "
                    "This 12-session therapy focuses on evaluating and changing distressing thoughts that have persisted since the trauma. "
                    "By altering these thoughts, patients can change their emotional responses. "
                    "CPT helps patients develop a new understanding of the traumatic event, reducing its negative impact on their current life"(RefNote(1))(RefNote(9))(RefNote(10))". "
                }

                h3 { "Eye Movement Desensitization and Reprocessing (EMDR) for PTSD"}

                p {
                    "Eye Movement Desensitization and Reprocessing (EMDR) is a psychotherapy for PTSD that assists patients in processing upsetting memories, "
                    "thoughts, and feelings related to trauma. "
                    "During EMDR sessions, patients focus on an external stimulus, such as alternating tones through headphones or alternating hand taps, "
                    "while recalling the distressing memory. "
                    "This dual focus helps patients reprocess the traumatic memory, leading to changes in how they experience and understand it"(RefNote(1))(RefNote(11))". "
                }
                }

            })],
        }
    }
    fn pharmacotherapy_tab(&self) -> Tab {
        Tab {
            name: "Pharmacotherapy".to_string(),
            blocks: vec![Block::Html(html! {
                div class="panel" {
                    p {
                        "Most patients with PTSD are treated with a combination of psychotherapy and pharmacotherapy"(RefNote(5))". "
                        "Pharmacotherapy is crucial in managing PTSD, and starting treatment early can"(RefNote(12))(RefNote(13))(RefNote(14))": "
                    }
                    ul class="list" {
                        li { "Improve core symptoms"}
                        li { "Reduce associated disability"}
                        li { "Enhance long-term outcomes"}
                    }

                }

                div class="panel" {
                    h3 { "Recommended Medications"}

                p {
                    "Guidelines recommend SSRIs (Selective Serotonin Reuptake Inhibitors) as the first-line pharmacologic treatment, when necessary, "
                    "with " em { "sertraline"} " and " em { "paroxetine"} " being the only FDA-approved options"(RefNote(2))(RefNote(15))". "
                    "The SNRI (Serotonin-Norepinephrine Reuptake Inhibitor) venlafaxine is also recommended"(RefNote(15))". "
                    "Approximately " em { "52%"} " of patients undergoing pharmacotherapy are treated with either SSRIs or SNRIs"(RefNote(16))". "
                }
                p {
                    "Long-term pharmacotherapy (lasting more than 12 weeks) may be necessary to sustain improvements made during the initial treatment period"(RefNote(14))". "
                }

                h3 { "Polypharmacy in PTSD"}

                ul {
                    li {
                        "Due to the limited efficacy and options of FDA-approved treatments, many patients require multiple medications"(RefNote(17))". "
                    }
                    li {
                        "On average, patients with PTSD are prescribed 1.6 medications for their condition"(RefNote(18))". "
                    }
                    li {
                        "Common combinations include SSRIs, anxiolytics, and benzodiazepines to address depression and sleep disturbances"(RefNote(17))". "
                    }
                }
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
                tabs: vec![self.guidelines_tab(), self.psychotherapy_tab(), self.pharmacotherapy_tab()]
            })
            (ReferencesProps {
                references: Block::Html(html! {
                    ol class="list" {
                        li { "Merians AN, Spiller T, Harpaz-Rotem I, Krystal JH, Pietrzak RH. Post-traumatic Stress Disorder. Med Clin North Am. 2023;107(1):85-99. doi:10.1016/j.mcna.2022.04.003" }
                        li { "Yehuda R, Hoge CW, McFarlane AC, et al. Post-traumatic stress disorder. " em { "Nat Rev Dis Primers" } ". 2015;1:15057. Published 2015 Oct 8." }
                        li { "Lee DJ, Schnitzlein CW, Wolf JP, Vythilingam M, Rasmusson AM, Hoge CW. Psychotherapy Versus Pharmacotherapy for Posttraumatic Stress Disorder: Systemic Review and Meta-Analyses to Determine First-Line Treatments. " em { "Depress Anxiety" } ". 2016;33(9):792-806. doi:10.1002/da.22511" }
                        li { "Mavranezouli I, Megnin-Viggars O, Daly C, et al. Psychological treatments for post-traumatic stress disorder in adults: a network meta-analysis. " em { "Psychol Med" } ". 2020;50(4):542-555. doi:10.1017/S0033291720000070" }
                        li { "Martin A, Naunton M, Kosari S, Peterson G, Thomas J, Christenson JK. Treatment Guidelines for PTSD: A Systematic Review. " em { "J Clin Med" } ". 2021;10(18):4175. Published 2021 Sep 15. doi:10.3390/jcm10184175" }
                        li { "Bisson JI, Berliner L, Cloitre M, et al. The International Society for Traumatic Stress Studies New Guidelines for the Prevention and Treatment of Posttraumatic Stress Disorder: Methodology and Development Process. " em { "J Trauma Stress" } ". 2019;32(4):475-483. doi:10.1002/jts.22421" }
                        li { "American Psychological Association. Cognitive behavioral therapy (CBT) for treatment of PTSD. American Psychological Association. https://www.apa.org/ptsd-guideline/treatments/cognitive-behavioral-therapy. Published July 31, 2017." }
                        li { "U.S. Department of Veterans Affairs. Prolonged Exposure for PTSD - PTSD: National Center for PTSD. Va.gov. Published 2014. " a href="https://www.ptsd.va.gov/understand_tx/prolonged_exposure.asp" { "https://www.ptsd.va.gov/understand_tx/prolonged_exposure.asp" } }
                        li { "U.S. Department of Veterans Affairs. Cognitive Processing Therapy for PTSD - PTSD: National Center for PTSD. Va.gov. Published 2014. " a href="https://www.ptsd.va.gov/understand_tx/cognitive_processing.asp" { "https://www.ptsd.va.gov/understand_tx/cognitive_processing.asp" } }
                        li { "American Psychological Association. Cognitive processing therapy (CPT). American Psychological Association. https://www.apa.org/ptsd-guideline/treatments/cognitive-processing-therapy. Published July 31, 2017." }
                        li { "U.S. Department of Veterans Affairs. Eye Movement Desensitization and Reprocessing (EMDR) for PTSD - PTSD: National Center for PTSD. Va.gov. Published 2014. " a href="https://www.ptsd.va.gov/understand_tx/emdr.asp" { "https://www.ptsd.va.gov/understand_tx/emdr.asp" } }
                        li { "Stein DJ, Ipser JC, Seedat S. Pharmacotherapy for post traumatic stress disorder (PTSD). " em { "Cochrane Database Syst Rev" } ". 2006;2006(1):CD002795. Published 2006 Jan 25. doi:10.1002/14651858.CD002795.pub2" }
                        li { "Williams T, Phillips NJ, Stein DJ, Ipser JC. Pharmacotherapy for post traumatic stress disorder (PTSD). " em { "Cochrane Database Syst Rev" } ". 2022;3(3):CD002795. Published 2022 Mar 2. doi:10.1002/14651858.CD002795.pub3" }
                        li { "Davis LL, Frazier EC, Williford RB, Newell JM. Long-term pharmacotherapy for post-traumatic stress disorder. " em { "CNS Drugs" } ". 2006;20(6):465-476. doi:10.2165/00023210-200620060-00003" }
                        li { "American Psychological Association. Treatments for PTSD. https://www.apa.org. https://www.apa.org/ptsd-guideline/treatments. Published June 2020." }
                        li { "Cook JM, Zeber JE, Simiola V, et al. Comparisons Between Patients Diagnosed with PTSD in Primary Care Versus Mental Health Care in Five Large Civilian Health Care Systems" em { "J Clin Psychol Med Settings" } ". 2021;28(2):221-228. doi:10.1007/s10880-020-09706-8" }
                        li { "Krystal JH, Davis LL, Neylan TC, et al. It Is Time to Address the Crisis in the Pharmacotherapy of Posttraumatic Stress Disorder: A Consensus Statement of the PTSD Psychopharmacology Working Group [published correction appears in Biol Psychiatry. 2018 Feb 1;83(3):296. doi: 10.1016/j.biopsych.2017.11.003]. " em { "Biol Psychiatry" } ". 2017;82(7):e51-e59. doi:10.1016/j.biopsych.2017.03.007" }
                        li { "Otsuka data on file" }
                    }
                })
            })
        }
    }
}
