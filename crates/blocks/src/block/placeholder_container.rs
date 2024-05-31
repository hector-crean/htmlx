use maud::html;

use super::rich_text::RichText;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct PlaceholderContainerProps {
    pub id: String,
}

impl maud::Render for PlaceholderContainerProps {
    fn render(&self) -> maud::Markup {
        html! {
            div id=(self.id) {}
        }
    }
}
