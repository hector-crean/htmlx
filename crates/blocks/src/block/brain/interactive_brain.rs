use crate::block::{
    definition::DefinitionListProps,
    icon::IconProps,
    rich_text::{RichText, RichTextProps},
    tabs::{Tab, TabsProps, TabsRepresentation, TabsTheme},
    Block,
};
use maud::{html, Markup, PreEscaped};
use stringcase::Caser;
use strum::IntoStaticStr;

use super::brain_region::BrainRegionName;

#[derive(Debug, Clone)]
pub struct BrainComment {
    pub icon: IconProps,
    pub symptom: Markup,
    pub highlighted_regions: Vec<BrainRegionName>,
    pub overview: Option<Markup>,
    pub description: Markup,
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
                class="flex flex-col items-start w-full h-full symptom"
                data-symptom="true"
                data-regions=(self.highlighted_regions_str()) {
                    div class="grid items-start justify-center w-full grid-cols-1" {
                        (self.icon)
                        div class="flex-1 col-span-2 px-1 text-xs text-center text-gray-700 break-words hyphens-auto 2xl:block" { (self.symptom) }
                    }

                    div data-kind="description" {
                       (self.description)
                    }
                    @match &self.overview {
                        Some(overview) => {
                            div data-kind="overview" {
                                (overview)
                             }
                        }
                        None => {

                        }
                    }
            }
        }
    }
}

impl BrainComment {
    pub fn new<S: Into<String>>(
        icon: IconProps,
        symptom: Markup,
        highlighted_regions: Vec<BrainRegionName>,
        overview: Option<Markup>,
        description: Markup,
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
            symptom: Markup::default(),
            highlighted_regions: vec![],
            overview: None,
            description: Markup::default(),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct InteractiveBrainProps {
    pub id: String,
    pub description: Markup,
    pub definitionList: Option<DefinitionListProps>,
    pub groups: Vec<CommentGroup>,
}

impl Default for InteractiveBrainProps {
    fn default() -> Self {
        InteractiveBrainProps {
            id: uuid::Uuid::new_v4().to_string(),
            description: Markup::default(),
            definitionList: None,
            groups: vec![CommentGroup::default()],
        }
    }
}

impl maud::Render for InteractiveBrainProps {
    fn render(&self) -> Markup {
        html! {
        div id=(self.id)  class="@container" {
                div class="flex flex-col w-full gap-2 rounded-lg" {

                    (Block::TabsBlock(TabsProps {
                        id: uuid::Uuid::new_v4(),
                        tabs: vec![
                            Tab {
                                name: "Brain Regions".to_string(),
                                blocks: vec![
                                    Block::Html(html! {
                                        div class="w-full h-full py-2"{

                                            div class="@container w-full h-min p-2 rounded-lg" {

                                                div class="flex flex-col justify-center gap-2 align-center" {
                                                    div class="flex flex-col items-center col-span-1 p-2 transition duration-500 ease-in-out rounded-lg presentation_wrapper text-md jusify-center" {
                                                        h2  { "Brain Regions"}

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
                                    )
                                ]
                            },
                            Tab {
                                name: "Symptoms".to_string(),
                                blocks: vec![
                                    Block::Html(html! {
                                        div class="w-full h-full py-2"{
                                            div class="@container w-full h-min  p-2 rounded-lg" {
                                                div class="flex flex-col justify-center gap-2 align-center presentation_wrapper" {

                                                    div class="w-full" {
                                                        h2 class="px-2" { "Symptoms"}
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

                                    })
                                ]
                            }
                        ],
                        representation: TabsRepresentation::Internal { theme: TabsTheme::new("#d6e5ee", "#3b3e3f"), full_bleed: true }
                    }))

                    div class="flex flex-col items-center justify-center w-full col-span-2 p-2"{
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
