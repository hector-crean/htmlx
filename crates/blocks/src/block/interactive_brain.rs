use maud::{html, Markup, PreEscaped};
use strum::IntoStaticStr;
use super::{rich_text::{RichText, RichTextProps}, Block};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
struct BrainRegionLabel {
    position: Vec2,
    alt_draw_mode: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrainRegion {
    name: BrainRegionName,
    fill_color: String,
    label: BrainRegionLabel,
    centroid: Vec2,
    points: Vec<Vec2>,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, IntoStaticStr)]
#[serde(rename_all = "camelCase")]
pub enum BrainRegionName {
    #[default]
    Frontosubcortical,
    Orbitofrontal,
    AnteriorCingulatedGyrus,
    BilateralTemporalCortex,
    ParietalLobe,
    Thalamus,
    Hippocampus,
    Amygdala,
    Hypothalamus,
    AnteriorCingulateCortex,
    PosteriorCingulateCortex,
    Striatum,
    PrefrontalCortex,
    VentralFrontalCortex,
    FrontalLobe,
    Dlpfc,
    Vlpfc,
    NucleusAccumbens,
    BasalForebrain,
    AnteriorCaudate,
    GreyMatter,
    LateralVentricle,
    OccipitalLobe,
    AuditoryCortex,
    SubstantiaNigra,
    NucleusAccumbensArea,
    AmyloidStage1MildRegion1,
    AmyloidStage2ModerateRegion1,
    AmyloidStage2MildRegion1,
    AmyloidStage3SevereRegion1,
    AmyloidStage3ModerateRegion1,
    AmyloidStage3ModerateRegion2,
    LocusCoeruleus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BrainComment {
    pub icon: String,
    pub symptom: RichTextProps,
    pub highlighted_regions: Vec<BrainRegionName>,
    pub description: Vec<Block>,
}

impl BrainComment {
    fn highlighted_regions_str(&self) -> String {
        self.highlighted_regions
            .iter()
            .map(|region| {
                let region: &'static str = region.into();
                region.to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl maud::Render for BrainComment { 
    fn render(&self) -> Markup {
        html! {
            button class="symptom text-with-icon"
                data-regions=(self.highlighted_regions_str()) {
                img src=(self.icon) {}
                span { (self.symptom) }
                div class="info" {
                    h3 class="symptom-heading" { (self.icon) }
    
                    @for block in &self.description {
                        (block)
                    }
                }
            }
        }
    }
}

impl BrainComment {
    pub fn new<S: Into<String>>(icon: S, symptom: RichTextProps, highlighted_regions: Vec<BrainRegionName>, description: Vec<Block>) -> Self {
        Self {
            icon: icon.into(),
            symptom,
            highlighted_regions,
            description,
        }
    }
}

impl Default for BrainComment {
    fn default() -> Self {
        BrainComment {
            icon: String::from("🧠"),
            symptom: RichTextProps::default(),
            highlighted_regions: vec![],
            description: vec![],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CommentGroup {
    pub name: String,
    pub comments: Vec<BrainComment>,
}

impl CommentGroup {
    pub fn new<S: Into<String>>(name: S, comments: Vec<BrainComment>) -> Self {
        Self {
            name: name.into(),
            comments,
        }
    }
}

impl Default for CommentGroup {
    fn default() -> Self {
        CommentGroup {
            name: String::from("Default Group"),
            comments: vec![BrainComment::default()],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct InteractiveBrainProps {
    pub groups: Vec<CommentGroup>,
}



impl Default for InteractiveBrainProps {
    fn default() -> Self {
        InteractiveBrainProps {
            groups: vec![CommentGroup::default()],
        }
    }
}

impl maud::Render for InteractiveBrainProps {
    fn render(&self) -> Markup {
        html! {
            div class="flex space-x-2" {
                @for group in &self.groups {
                    div class="flex-1 space-y-2" {
                        h2 class="text-center" { (group.name) }
                        @for comment in &group.comments {
                            (comment)
                        }
                    }
                }
            }

            p class="panel text-xs transition duration-500 ease-in-out" id="info-panel" {
                "Click on each symptom to learn more about its involvement"
            }

            svg id="interactive-svg" class="rounded-lg shadow mt-4" width="100%" viewBox="0 0 960 400" preserveAspectRatio="xMidYMid meet" {}

            // Embedded JavaScript
            script type="text/javascript" {
                (PreEscaped(r#"
                document.addEventListener('DOMContentLoaded', function() {
                    const buttons = document.querySelectorAll('.symptom');
                    buttons.forEach(button => {
                        button.addEventListener('click', function() {
                            alert('Symptom clicked: ' + this.querySelector('span').textContent);
                        });
                    });
                });
                "#))
            }
        }
    }
}

