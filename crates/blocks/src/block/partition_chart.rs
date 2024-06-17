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
pub enum PartitionChartNode {
    Branch {
        id: uuid::Uuid,
        label: String,
        fill: Option<String>,
        children: Vec<PartitionChartNode>,
    },
    Leaf {
        id: uuid::Uuid,
        label: String,
        fill: Option<String>,
        value: f32,
    },
}

impl Default for PartitionChartNode {
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
pub struct PartitionChartProps {
    pub title: String,
    pub data: Vec<PartitionChartNode>,
}

impl Default for PartitionChartProps {
    fn default() -> Self {
        PartitionChartProps {
            title: "default-title".to_string(),
            data: vec![PartitionChartNode::default()],
        }
    }
}

impl maud::Render for PartitionChartProps {
    fn render(&self) -> maud::Markup {
        let partition_data_str: String = serde_json::to_string(&self.data).unwrap();

        html!(
            div class="panel" {
                h2 {(self.title)}
                partition-chart partitiondata=(partition_data_str) {}
            }
        )
    }
}
