use maud::{html, Markup, PreEscaped, Render};

use super::{
    rich_text::{RichText, RichTextProps},
    Block,
};
use crate::block::icon::IconProps;
use crate::SvgName;
use stringcase::{kebab_case, Caser};

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarChartDatum {
    pub id: uuid::Uuid,
    pub group_id: u32,
    pub label: String,
    pub start: f32,
    pub end: Option<f32>,
    pub fill: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<Block>,
}

impl BarChartDatum {
    pub fn new(
        group_id: u32,
        label: &str,
        start: f32,
        end: Option<f32>,
        fill: Option<&str>,
        description: Option<Block>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            group_id: group_id,
            label: label.into(),
            start,
            end,
            fill: fill.map(|s| s.into()),
            description,
        }
    }
    pub fn fill(&self) -> String {
        self.fill.clone().unwrap_or("RED".to_string())
    }
}

impl Default for BarChartDatum {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            group_id: 0,
            label: "DEFAULT_LABEL".to_string(),
            start: 0.,
            end: None,
            fill: None,
            description: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Margin {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}
impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 0.,
            left: 0.,
            bottom: 0.,
            right: 0.,
        }
    }
}

impl Margin {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarChartProps {
    pub title: String,
    pub margin: Margin,
    pub bars: Vec<BarChartDatum>,
    pub aspect_ratio: f32,
}

impl Default for BarChartProps {
    fn default() -> Self {
        BarChartProps {
            title: "default-title".to_string(),
            bars: vec![BarChartDatum::default()],
            margin: Margin::default(),
            aspect_ratio: 2.0,
        }
    }
}

impl maud::Render for BarChartProps {
    fn render(&self) -> maud::Markup {
        let bar_data_str: String = serde_json::to_string(&self.bars).unwrap();
        let margin: String = serde_json::to_string(&self.margin).unwrap();

        html!(
            div data-full-bleed="true" class="round-lg grid grid-cols-1 p-4 bg-black-900 items-start justify-between bg-[#d4e4ee] rounded-lg mt-2 mx-2" {

                h3 {(self.title)}
                div class="relative flex flex-col items-center justify-center flex-1 w-full"{
                    bar-chart bardata=(bar_data_str) margin=(margin) aspect-ratio=(self.aspect_ratio) {
                        @for bar in &self.bars {
                            @match &bar.description {
                                Some(desc) => {
                                    section slot="panel" id=(bar.id) {
                                        (desc)
                                    }
                                }
                                None => {

                                }
                            }
                        }
                    }
                }


            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct PtsdComborbiditiesLayout {
    pub increased_risk_scale: Option<(String, Markup)>,
    pub increased_risk_percentage: Option<(String, Markup)>,
    pub overview: Markup,
    pub fill: String,
}

impl Render for PtsdComborbiditiesLayout {
    fn render(&self) -> maud::Markup {
        html! {
                div class="flex flex-col gap-4 rounded-lg bg-[#e6f0f6] p-8" {
                    @match &self.increased_risk_scale {
                        Some((scale, markup)) => {
                            div class="flex flex-row items-center justify-start" {
                                div class="w-12 text-black" {
                                    (IconProps {
                                        name: SvgName::UpArrow
                                    })
                                }
                                span class="px-1 text-3xl" {(scale) }
                                div {
                                        span class="w-8 h-8" {
                                            (IconProps {
                                                name: SvgName::Cross
                                            })
                                        }
                                    }
                            }
                            div class="mt-4" {
                                (markup)
                            }
                        }
                        None => {}
                    }
                    @match &self.increased_risk_percentage {
                        Some((scale, markup)) => {
                            div class="flex flex-row items-center justify-start" {
                                div class="w-12 text-black" {
                                    (IconProps {
                                        name: SvgName::Stats
                                    })}
                                    span class="px-1 text-3xl" {(scale)"%" }
                            }
                            div class="mt-4" {
                                (markup)
                            }

                        }
                        None => {}
                    }

                    div class="flex flex-col items-center justify-center col-span-1" {
                        div class=(format!("pl-2 border-l-4 border-[{}]", self.fill)) {
                            (self.overview)
                        }
                    }


                }

        }
    }
}
