use maud::{html, Render};

use super::{brain_region::BrainRegionName, BrainRegion};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BrainGlossaryProps {
    pub region_names: Vec<BrainRegionName>,
}

impl Render for BrainGlossaryProps {
    fn render(&self) -> maud::Markup {
        let region_names_str = {
            match serde_json::to_string(&self.region_names) {
                Ok(region_names) => region_names,
                Err(err) => String::from("[]"),
            }
        };

        html! {
            interactive-brain regions=(region_names_str) {}
        }
    }
}
