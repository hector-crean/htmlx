use super::{
    definition::DefinitionListProps,
    icon::IconProps,
    rich_text::{RichText, RichTextProps},
    Block,
};
use maud::{html, Markup, PreEscaped};
use stringcase::Caser;
use strum::IntoStaticStr;

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

#[derive(
    Debug,
    Default,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    specta::Type,
    IntoStaticStr,
)]

pub enum BrainRegionName {
    #[default]
    #[serde(rename = "fronto-subcortical")]
    Frontosubcortical,
    #[serde(rename = "orbitofrontal")]
    Orbitofrontal,
    #[serde(rename = "anterior-cingulated-gyrus")]
    AnteriorCingulatedGyrus,
    #[serde(rename = "bilateral-temporal-cortex")]
    BilateralTemporalCortex,
    #[serde(rename = "parietal-lobe")]
    ParietalLobe,
    #[serde(rename = "thalmus")]
    Thalamus,
    #[serde(rename = "hippocampus")]
    Hippocampus,
    #[serde(rename = "amygdala")]
    Amygdala,
    #[serde(rename = "hypothalamus")]
    Hypothalamus,
    #[serde(rename = "anterior-cingulate-cortex")]
    AnteriorCingulateCortex,
    #[serde(rename = "posterior-cingulate-cortex")]
    PosteriorCingulateCortex,
    #[serde(rename = "striatum")]
    Striatum,
    #[serde(rename = "prefrontal-cortex")]
    PrefrontalCortex,
    #[serde(rename = "ventral-frontal-cortex")]
    VentralFrontalCortex,
    #[serde(rename = "frontal-lobe")]
    FrontalLobe,
    #[serde(rename = "dlpfc")]
    Dlpfc,
    #[serde(rename = "vlpfc")]
    Vlpfc,
    #[serde(rename = "nucleus-accumbens")]
    NucleusAccumbens,
    #[serde(rename = "basal-forebrain")]
    BasalForebrain,
    #[serde(rename = "anterior-caudate")]
    AnteriorCaudate,
    #[serde(rename = "grey-matter")]
    GreyMatter,
    #[serde(rename = "lateral-ventricle")]
    LateralVentricle,
    #[serde(rename = "occipital-lobe")]
    OccipitalLobe,
    #[serde(rename = "auditory-cortex")]
    AuditoryCortex,
    #[serde(rename = "substantia-nigra")]
    SubstantiaNigra,
    #[serde(rename = "nucleus-accumbens-area")]
    NucleusAccumbensArea,
    #[serde(rename = "amyloid-stage-1-mild-region-1")]
    AmyloidStage1MildRegion1,
    #[serde(rename = "amyloid-stage-2-moderate-region-1")]
    AmyloidStage2ModerateRegion1,
    #[serde(rename = "amyloid-stage-2-mild-region-1")]
    AmyloidStage2MildRegion1,
    #[serde(rename = "amyloid-stage-3-severe-region-1")]
    AmyloidStage3SevereRegion1,
    #[serde(rename = "amyloid-stage-3-moderate-region-1")]
    AmyloidStage3ModerateRegion1,
    #[serde(rename = "amyloid-stage-3-moderate-region-2")]
    AmyloidStage3ModerateRegion2,
    #[serde(rename = "locus-coeruleus")]
    LocusCoeruleus,
    #[serde(rename = "insula")]
    Insula,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BrainComment {
    pub icon: IconProps,
    pub symptom: String,
    pub highlighted_regions: Vec<BrainRegionName>,
    pub overview: Vec<Block>,
    pub description: Vec<Block>,
}

impl BrainComment {
    fn highlighted_regions_str(&self) -> String {
        self.highlighted_regions
            .iter()
            .map(|region| {
                let region: &'static str = region.into();
                region.to_string().to_kebab_case()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl maud::Render for BrainComment {
    fn render(&self) -> Markup {
        html! {
            button
                class="flex flex-col items-start w-full symptom"
                data-symptom="true"
                data-regions=(self.highlighted_regions_str()) {
                    div class="w-full grid grid-cols-1 2xl:grid-cols-[min-content_1fr_1fr] grid-rows-1 items-center justify-center" {
                        div class="flex items-center justify-center w-full h-12 col-span-1 aspect-square" { (self.icon) }
                        div class="flex-1 col-span-2 px-1 text-xs text-center text-gray-700 break-words hyphens-auto 2xl:block" { (self.symptom) }
                    }

                    div data-kind="description" {
                        @for block in &self.description {
                            (block)
                        }
                    }
                    div data-kind="overview" {
                        @for block in &self.overview {
                            (block)
                        }
                    }
            }
        }
    }
}

impl BrainComment {
    pub fn new<S: Into<String>>(
        icon: IconProps,
        symptom: S,
        highlighted_regions: Vec<BrainRegionName>,
        overview: Vec<Block>,
        description: Vec<Block>,
    ) -> Self {
        Self {
            icon: icon.into(),
            symptom: symptom.into(),
            highlighted_regions,
            overview,
            description,
        }
    }
}

impl Default for BrainComment {
    fn default() -> Self {
        BrainComment {
            icon: IconProps::default(),
            symptom: String::from(""),
            highlighted_regions: vec![],
            overview: vec![],
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
    pub id: String,
    pub description: RichText,
    pub definitionList: Option<DefinitionListProps>,
    pub groups: Vec<CommentGroup>,
}

impl Default for InteractiveBrainProps {
    fn default() -> Self {
        InteractiveBrainProps {
            id: uuid::Uuid::new_v4().to_string(),
            description: RichText::default(),
            definitionList: None,
            groups: vec![CommentGroup::default()],
        }
    }
}

impl maud::Render for InteractiveBrainProps {
    fn render(&self) -> Markup {
        html! {
        div id=(self.id) {
                div class="flex flex-col w-full gap-2 xl:flex-row xl:h-full xl:flex-1" {

                    div class="w-full swiper" {
                        // Additional required wrapper
                        div class="swiper-wrapper" {

                            div class="mb-12 swiper-slide " {
                                div class="w-full h-full py-2"{

                                    div class="@container w-full h-min p-2 rounded-lg" {

                                        div class="flex flex-col justify-center gap-2 align-center" {
                                            div class="flex flex-col items-center col-span-1 p-2 transition duration-500 ease-in-out rounded-lg presentation_wrapper text-md jusify-center" {
                                                h3  { "Brain Regions"}

                                                (self.description)


                                                @match &self.definitionList {
                                                    Some(def_list) => {
                                                        (def_list)
                                                    }
                                                    None => {

                                                    }
                                                }
                                            }
                                        }

                                    }

                                }
                             }
                            div class="mb-12 swiper-slide" {
                                div class="w-full h-full py-2"{
                                    div class="@container w-full h-min  p-2 rounded-lg" {
                                        div class="flex flex-col justify-center gap-2 align-center presentation_wrapper" {

                                            div class="w-full" {
                                                h3 class="px-2" { "Symptoms"}
                                            }


                                            div class="flex flex-row w-full p-2 " data-full-bleed="true" {

                                                @for group in &self.groups {
                                                    div class="flex flex-row grid-cols-5 gap-2 2xl:grid" {
                                                        @for comment in &group.comments {
                                                            (comment)
                                                        }
                                                    }
                                                }
                                            }

                                            div class="flex flex-col items-center justify-center p-2 transition duration-500 ease-in-out" {
                                                p id="description-panel" class="text-md"  {
                                                    "Click on each symptom to learn more about its involvement"
                                                }
                                            }
                                        }
                                    }
                                }
                             }

                        }
                        // If we need pagination
                        div class="p-4 swiper-pagination" {}

                        // If we need navigation buttons
                        div class="swiper-button-prev" {}
                        div class="swiper-button-next" {}

                        // If we need scrollbar
                        div class="swiper-scrollbar" {}
                    }

                    div class="flex flex-col justify-center w-full col-span-2 p-2"{
                        svg id="interactive-svg" class="rounded-lg shadow" width="100%" viewBox="0 0 960 400" preserveAspectRatio="xMidYMid meet" {}
                    }











                }

            }


        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let region = BrainRegionName::Frontosubcortical;
        let serialized = serde_json::to_string(&region).unwrap();
        assert_eq!(serialized, "\"fronto-subcortical\"");

        let region = BrainRegionName::Orbitofrontal;
        let serialized = serde_json::to_string(&region).unwrap();
        assert_eq!(serialized, "\"orbitofrontal\"");

        let region = BrainRegionName::AnteriorCingulatedGyrus;
        let serialized = serde_json::to_string(&region).unwrap();
        assert_eq!(serialized, "\"anterior-cingulated-gyrus\"");

        // Add more tests for other variants...
    }

    #[test]
    fn test_deserialization() {
        let serialized = "\"fronto-subcortical\"";
        let deserialized: BrainRegionName = serde_json::from_str(serialized).unwrap();
        assert_eq!(deserialized, BrainRegionName::Frontosubcortical);

        let serialized = "\"orbitofrontal\"";
        let deserialized: BrainRegionName = serde_json::from_str(serialized).unwrap();
        assert_eq!(deserialized, BrainRegionName::Orbitofrontal);

        let serialized = "\"anterior-cingulated-gyrus\"";
        let deserialized: BrainRegionName = serde_json::from_str(serialized).unwrap();
        assert_eq!(deserialized, BrainRegionName::AnteriorCingulatedGyrus);

        // Add more tests for other variants...
    }
}
