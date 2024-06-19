use maud::html;

use super::{
    rich_text::{RichText, RichTextProps},
    Block,
};
use stringcase::{kebab_case, Caser};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarChartDatum {
    pub id: uuid::Uuid,
    pub label: String,
    pub value: f32,
    pub fill: Option<String>,
}

impl BarChartDatum {
    pub fn new(label: &str, value: f32, fill: Option<&str>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            label: label.into(),
            value,
            fill: fill.map(|s| s.into()),
        }
    }
}

impl Default for BarChartDatum {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            label: String::from(""),
            value: 0.,
            fill: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarChartProps {
    pub title: String,
    pub slices: Vec<BarChartDatum>,
}

impl Default for BarChartProps {
    fn default() -> Self {
        BarChartProps {
            title: "default-title".to_string(),
            slices: vec![BarChartDatum::default()],
        }
    }
}

impl maud::Render for BarChartProps {
    fn render(&self) -> maud::Markup {
        let bar_data_str: String = serde_json::to_string(&self.slices).unwrap();

        html!(
            div class="panel" {
                h3 {(self.title)}
                bar-chart bardata=(bar_data_str) {}
            }
        )
    }
}
