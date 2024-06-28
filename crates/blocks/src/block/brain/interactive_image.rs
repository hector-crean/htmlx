use maud::{html, Markup, Render};

use super::{brain_region::BrainRegionName, BrainRegion};
use std::convert::AsRef;
use strum::AsRefStr;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveImageProps {
    pub regions: Vec<BrainRegion>,
    pub src: String,
    #[serde(skip_serializing)]
    pub descriptions: Vec<(BrainRegionName, Markup)>,
}

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

impl Render for InteractiveImageProps {
    fn render(&self) -> maud::Markup {
        let regions = {
            match serde_json::to_string(&self.regions) {
                Ok(regions) => regions,
                Err(err) => String::from("[]"),
            }
        };

        html! {
            interactive-image regions=(regions) src=(self.src) data-full-bleed="true" {
                @for (id, desc) in &self.descriptions {
                    section slot="panel" id=(id.as_ref()) {
                        (desc)
                    }
                }
            }
        }
    }
}
