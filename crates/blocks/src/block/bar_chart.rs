use maud::{html, Render};

use super::{
    rich_text::{RichText, RichTextProps},
    Block,
};
use crate::block::icon::IconProps;
use crate::SvgName;
use stringcase::{kebab_case, Caser};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarChartDatum {
    pub id: uuid::Uuid,
    pub label: String,
    pub start: f32,
    pub end: Option<f32>,
    pub fill: Option<String>,
}

impl BarChartDatum {
    pub fn new(label: &str, start: f32, end: Option<f32>, fill: Option<&str>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            label: label.into(),
            start,
            end,
            fill: fill.map(|s| s.into()),
        }
    }
}

impl Default for BarChartDatum {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            label: String::from(""),
            start: 0.,
            end: None,
            fill: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarChartProps {
    pub title: String,
    pub bars: Vec<BarChartDatum>,
}

impl Default for BarChartProps {
    fn default() -> Self {
        BarChartProps {
            title: "default-title".to_string(),
            bars: vec![BarChartDatum::default()],
        }
    }
}

impl maud::Render for BarChartProps {
    fn render(&self) -> maud::Markup {
        let bar_data_str: String = serde_json::to_string(&self.bars).unwrap();

        html!(
            div class="round-lg grid grid-cols-1 p-4 bg-black-900 items-start justify-between bg-[#d4e4ee] rounded-lg mt-2 mx-2" {

                h3 {(self.title)}
                div class="relative flex flex-col items-center justify-center flex-1 w-full"{
                    bar-chart bardata=(bar_data_str) {
                        @for bar in &self.bars {
                            section slot="panel" id=(bar.id) {
                                "Panel"(bar.id)
                            }
                        }
                    }
                }

                div class="presentation_wrapper"{
                    (BarChartInfo)
                }
            }
        )
    }
}

pub struct BarChartInfo;

impl Render for BarChartInfo {
    fn render(&self) -> maud::Markup {
        html! {
                div class="flex flex-col gap-4 rounded-lg bg-[#e6f0f6] p-8" {
                    div class="col-span-1" {
                        (IconProps {
                            name: SvgName::UpArrow
                        })
                    }
                    div class="col-span-1" {
                        (IconProps {
                            name: SvgName::Stats
                        })
                    }
                }

        }
    }
}
