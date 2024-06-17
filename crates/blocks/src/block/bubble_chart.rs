use std::vec;

use maud::html;

use super::{
    rich_text::{RichText, RichTextProps},
    Block,
};
use stringcase::{kebab_case, Caser};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum BubbleChartNode {
    Branch {
        id: uuid::Uuid,
        label: String,
        fill: Option<String>,
        children: Vec<BubbleChartNode>,
    },
    Leaf {
        id: uuid::Uuid,
        label: String,
        fill: Option<String>,
        value: f32,
    },
}

impl Default for BubbleChartNode {
    fn default() -> Self {
        Self::Branch {
            id: uuid::Uuid::new_v4(),
            label: "root".to_string(),
            fill: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BubbleChartProps {
    pub title: String,
    pub data: Vec<BubbleChartNode>,
}

impl Default for BubbleChartProps {
    fn default() -> Self {
        BubbleChartProps {
            title: "default-title".to_string(),
            data: vec![BubbleChartNode::default()],
        }
    }
}

impl maud::Render for BubbleChartProps {
    fn render(&self) -> maud::Markup {
        let bubble_data_str: String = serde_json::to_string(&self.data).unwrap();

        html!(
            div class="panel" {
                h2 {(self.title)}
                bubble-chart bubbledata=(bubble_data_str) {}
            }
        )
    }
}
