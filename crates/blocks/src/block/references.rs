use maud::html;

use super::{rich_text::RichText, Block};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct ReferencesProps {
    pub references: Block,
}

impl maud::Render for ReferencesProps {
    fn render(&self) -> maud::Markup {
        html! {
           div class="references panel" {
            h4 class="uppercase" { "References "}
            div class="pl-3" {
                (self.references)
            }
           }


        }
    }
}
