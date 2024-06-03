use maud::html;

use super::rich_text::RichText;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct PlaceholderContainerProps {
    pub id: String,
    pub full_bleed: bool,
    pub class: String,
}

impl maud::Render for PlaceholderContainerProps {
    fn render(&self) -> maud::Markup {
        html! {
            div id=(self.id) class=(self.class) data-full-bleed=(self.full_bleed) {}
        }
    }
}
