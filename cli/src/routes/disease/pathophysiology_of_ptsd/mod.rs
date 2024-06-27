use blocks::{
    block::{
        brain::{
            brain_region::BrainRegionName, interactive_brain::InteractiveBrainProps,
            interactive_image::InteractiveImageProps, BrainRegion,
        },
        tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme},
        Block,
    },
    span::ref_note::RefNote,
};
use maud::{html, Render};
pub mod brain_bg;
use brain_bg::BRAIN_BG_HREF;

pub struct BrainModelTab;

impl BrainModelTab {
    fn intro(&self) -> maud::Markup {
        html! {}
    }
    fn brain_regions(&self) -> Result<Vec<BrainRegion>, serde_json::Error> {
        let json = include_str!(r#"C:\Users\Hector.C\rust\htmx\cli\data\brain_regions.json"#);
        let brain_regions: Vec<BrainRegion> = serde_json::from_str(json)?;

        let filtered = brain_regions
            .iter()
            .cloned()
            .filter(|region| match region.id {
                BrainRegionName::Striatum
                | BrainRegionName::Hippocampus
                | BrainRegionName::Amygdala
                | BrainRegionName::PrefrontalCortex
                | BrainRegionName::Hypothalamus => true,
                _ => false,
            })
            .collect::<Vec<BrainRegion>>();

        Ok(filtered)
    }
}

impl Render for BrainModelTab {
    fn render(&self) -> maud::Markup {
        let encoded_href = base64::encode(BRAIN_BG_HREF);

        html! {
            p {
                "Post-traumatic stress disorder (PTSD) is a dynamic disorder involving fluctuations between contrasting forms of emotional dysregulation"(RefNote(1))"."
            }
            p {
                "Emotional dysregulation may be due to a complex interplay of key brain regions—namely, the amygdala, hippocampus, prefrontal cortex (PFC), and striatum"(RefNote(2))(RefNote(3))(RefNote(4))". "
                "PTSD often presents with symptoms that develop following traumatic experiences and are associated with structural deficits in stress-regulating brain regions"(RefNote(1))(RefNote(5))". "
            }
            (InteractiveImageProps {
                regions: self.brain_regions().unwrap(),
                src: encoded_href.into(),
                descriptions: vec![
                    (BrainRegionName::Striatum, html!{
                        h3 { "Striatum" }

                        p {
                            "The striatum is a crucial brain component involved in reward processing, decision-making, "
                            "and the reinforcement of behaviors through dopaminergic pathways"(RefNote(1))(RefNote(2))"."
                            "In the context of PTSD, the striatum exhibits significant dysfunction"(RefNote(3))(RefNote(4))". "
                            "Research has shown decreased striatal activation in individuals with PTSD during reward processing compared to healthy controls"(RefNote(5))". "
                            "This hypoactivity disrupts the reward circuits, contributing to the negative alterations in mood and cognition observed in PTSD"(RefNote(6))". "
                            "Dysfunctional dopamine levels in the striatum enhance the processing of aversive stimuli and disrupt reward processing, leading to avoidant behavior"(RefNote(6))". "
                        }

                        p {
                            "Consequently, striatal dysfunction plays a key role in the development of PTSD symptoms related to anhedonia and emotional numbing, "
                            "underscoring the importance of reward processing abnormalities in the disorder"(RefNote(7))(RefNote(8))". "
                        }
                    }),
                    (BrainRegionName::Hippocampus, html!{
                        h3  { "Hippocampus" }

                        p {
                            "The hippocampus plays a vital role in memory formation, spatial navigation, and context processing"
                            (RefNote(1))(RefNote(2))(RefNote(3))(RefNote(4))". "
                        }
                        p {
                            "In PTSD, the hippocampus often exhibits hypoactivity, impairing the process of fear extinction—an essential mechanism for overcoming trauma-related responses"
                            (RefNote(4))(RefNote(5))(RefNote(6))". "
                            "This hypoactivity, along with deficits in the prefrontal cortex and heightened activity in the amygdala, contributes to symptoms of intrusion and arousal"
                            (RefNote(4))". "
                        }
                        p {
                            "Serotonin projections from the raphe nuclei to the hippocampus play a significant role in stress response and mood regulation"
                            (RefNote(7))(RefNote(8))(RefNote(9))". "
                            "Stress can alter serotonin 5-HT1A receptor activity in the hippocampus, impacting mood and contributing to PTSD symptoms"
                            (RefNote(7))(RefNote(8))(RefNote(9))". "
                        }
                        p {
                            "Impaired hippocampal function also leads to deficits in context processing, making it difficult for individuals to distinguish between safe and dangerous "
                            "environments"(RefNote(3))(RefNote(4))(RefNote(10))". "
                            "This inability to contextualize fear responses perpetuates the hypervigilance and anxiety characteristic of PTSD"(RefNote(2))(RefNote(7))(RefNote(10))". "
                        }
                    }),
                    (BrainRegionName::Amygdala, html!{
                        h3 { "Amygdala"}
                        p {
                            "The amygdala plays a central role in fear conditioning and fear extinction"
                            (RefNote(1)) (RefNote(2))". "
                            "In the amygdala, fear processing involves two pathways: fear conditioning influenced by sensory input and fear extinction, "
                            "which depends on executive control from the prefrontal cortex (PFC) and contextual processing in the hippocampus"
                            (RefNote(3)) (RefNote(4))". "
                        }
                        p {
                            "In PTSD, the amygdala typically exhibits hyperactivity, which is associated with heightened fear responses and hyperarousal"
                            (RefNote(3)) (RefNote(5))". "
                            "This heightened sensitivity is significantly influenced by dysregulation in monoamine neurotransmitter systems, particularly noradrenaline and serotonin"
                            (RefNote(2)) (RefNote(6)) (RefNote(7)) (RefNote(8)) (RefNote(9))". "
                        }
                        p {
                            "Noradrenaline hyperactivity leads to increased amygdala activity and impaired PFC function, which diminishes executive control over "
                            "emotions and enhances fear conditioning"
                            (RefNote(6)) (RefNote(7)) (RefNote(8))". "
                            "This hyperactivity combined with low 5-HT levels may increase activation of the fear conditioning pathway "
                            (RefNote(7))
                            " and contribute to symptoms like hyperarousal and re-experiencing traumatic events"
                            (RefNote(10)) (RefNote(11))". "
                            "Inhibitory 5-HT1A receptors in the amygdala reduce fear behaviors, "
                            (RefNote(9)) (RefNote(12))
                            " and excitatory 5-HT2A receptors increase fear behaviors "
                            (RefNote(13))
                            " affecting emotional regulation and stress responses, and contributes to mood disturbances and arousal symptoms in PTSD"
                            (RefNote(2)) (RefNote(14))". "
                        }
                    }),
                    (BrainRegionName::PrefrontalCortex,
                    html!{
                        h3 { "Prefrontal cortex"}

                        p {
                            "The prefrontal cortex (PFC) provides executive regulation of emotion and can inhibit fear responses produced by the amygdala. "
                            (RefNote(1)) ". It plays a key role in decision-making, social behavior, and personality expression. "
                            (RefNote(2)) "."
                        }
                        p {
                            "In PTSD, hypoactivity of the PFC, often observed alongside a similarly hypoactive hippocampus, may contribute to impaired fear extinction in PTSD. "
                            (RefNote(1))(RefNote(3))(RefNote(4)) " This PFC and hippocampus hypoactivity, coupled with amygdala hyperactivity through activation of a1-adrenoceptors "
                            (RefNote(2))(RefNote(5))(RefNote(6))(RefNote(7)) ", contributes to the heightened symptoms of intrusion and arousal characteristic of PTSD "
                            (RefNote(3))(RefNote(8)) "."
                        }
                        p {
                            "Noradrenaline hyperactivity further impairs PFC function, restricting executive control over emotions and exacerbating fear conditioning. "
                            (RefNote(2)) ". Additionally, dopamine modulates PFC activity, with dysfunctional dopamine levels potentially enhancing the processing of aversive stimuli and promoting avoidant behavior. "
                            (RefNote(9)) "."
                        }
                        p {
                            "Studies have shown that PTSD was associated with impaired extinction memory, as well as decreased PFC activation and that re-experiencing symptom severity was negatively correlated with inferior PFC activity. "
                            (RefNote(10))(RefNote(11)) ". Disruption to the PFC in PTSD can also lead to altered reward processing and an imbalance of approach-avoidance systems. "
                            (RefNote(12))(RefNote(13)) ". This dysfunction of the PFC in PTSD may bias processing toward aversive stimuli and promote avoidant behavior. "
                            (RefNote(1)) "."
                        }
                    }),
                    (
                        BrainRegionName::Hypothalamus,
                        html!{
                            h3 {
                                "Hypothalamic-pituitary-adrenal axis"
                            }

                            p {
                                "The hypothalamic-pituitary-adrenal (HPA) axis is crucial for the body's response to stress, regulating cortisol levels and maintaining homeostasis. "
                                (RefNote(1)) "."
                            }
                            p {
                                "In PTSD, there is significant dysregulation of the HPA axis, characterized by abnormal feedback regulation and altered cortisol levels. This dysregulation exacerbates stress responses and contributes to the chronic stress and anxiety seen in PTSD. "
                                (RefNote(1))(RefNote(2))". HPA axis dysfunction plays a role in the persistence of PTSD symptoms by maintaining a heightened state of physiological arousal."
                            }
                            p {
                                "The chronic dysregulation of the HPA axis underscores the physiological basis of PTSD and its impact on stress-related symptoms. "
                                (RefNote(1))(RefNote(3))"."
                            }
                        }
                    )
                ]
            })














        }
    }
}

pub struct Page;

impl Page {
    fn monoamine_dysfunction_tab(&self) -> Tab {
        Tab {
            name: "Monoamine Dysfunction in PTSD".to_string(),
            blocks: vec![Block::Html(html! {
                div class="panel" {
                    p {
                        "Monoamine dysfunction in PTSD involves abnormalities in the regulation of neurotransmitters like serotonin, norepinephrine, and dopamine"(RefNote(1))"."
                        "These imbalances disrupt the normal functioning of brain circuits involved in fear processing, stress response, and emotional regulation, "
                        "contributing to the symptoms of PTSD"(RefNote(1))(RefNote(2))(RefNote(3))"."

                    }
                }
            })],
        }
    }
    fn pathophysiology_tab(&self) -> Tab {
        let brain_regions = vec![
            BrainRegionName::Frontosubcortical,
            BrainRegionName::Orbitofrontal,
        ];
        Tab {
            name: "Pathophysiology of PTSD".to_string(),
            blocks: vec![Block::Html(html! {
                (BrainModelTab)
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
                tabs: vec![self.pathophysiology_tab(), self.monoamine_dysfunction_tab()]
            })
        }
    }
}
