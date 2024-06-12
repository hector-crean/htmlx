use maud::html;

use super::{
    rich_text::{RichText, RichTextProps},
    Block,
};
use stringcase::{kebab_case, Caser};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PieChartDatum {
    pub id: uuid::Uuid,
    pub label: String,
    pub value: f32,
}

impl Default for PieChartDatum {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            label: String::from(""),
            value: 0.,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PieChartProps {
    pub title: String,
    pub slices: Vec<PieChartDatum>,
}

impl Default for PieChartProps {
    fn default() -> Self {
        PieChartProps {
            title: "default-title".to_string(),
            slices: vec![PieChartDatum::default()],
        }
    }
}

impl maud::Render for PieChartProps {
    fn render(&self) -> maud::Markup {
        let pie_data_str: String = serde_json::to_string(&self.slices).unwrap();

        html!(
            div class="panel" {
                h2 {(self.title)}
                pie-chart piedata=(pie_data_str) {}
            }
        )
    }
}
