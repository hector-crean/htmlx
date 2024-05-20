use maud::html;

use super::{
    rich_text::{RichText, RichTextProps},
    Block,
};
use stringcase::{kebab_case, Caser};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarDatum {
    pub id: uuid::Uuid,
    pub short_title: String,
    pub full_title: String,
    pub icon: String,
    pub content: Block,
    pub percent: f32,
}

impl Default for BarDatum {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            short_title: "default-title".to_string(),
            full_title: "".to_string(),
            icon: "icon.png".to_string(),
            content: Block::RichTextBlock(RichTextProps::default()),
            percent: 0.,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BarChartProps {
    pub title: String,
    pub bars: Vec<BarDatum>,
}

impl Default for BarChartProps {
    fn default() -> Self {
        BarChartProps {
            title: "default-title".to_string(),
            bars: vec![BarDatum::default()],
        }
    }
}

impl maud::Render for BarChartProps {
    fn render(&self) -> maud::Markup {
        html!(
            div class="panel" {
                h2 {(self.title)}
                svg id=(self.title.to_kebab_case()) width="100%" preserveAspectRatio="xMidYMid meet" {}
                p class="panel" id=(format!("info-{}", self.title.to_kebab_case())) style="margin-bottom: 0; width: 100%;"{
                    // @for bar in &self.bars {
                    //     p data-bar-id=(bar.id) {
                    //         (bar.content)
                    //     }
                    // }
                }
            }
        )
    }
}
