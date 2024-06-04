use maud::html;

use super::{rich_text::RichText, Block};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct ReferencesProps {
    pub references: Block,
}

impl maud::Render for ReferencesProps {
    fn render(&self) -> maud::Markup {
        html! {
           div class="bg-[#b0cfe7] rounded-lg text-sm p-4 mt-4" {
            h3 class="uppercase" { "References "}
            div class="pl-3" {
                (self.references)
            }
           }


        }
    }
}
