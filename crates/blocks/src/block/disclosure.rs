use maud::html;

use super::{rich_text::RichText, Block};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct DisclosureTheme {
    bg_color: String,
    text_color: String,
}
impl DisclosureTheme {
    pub fn new(bg_color: &str, text_color: &str) -> Self {
        Self {
            bg_color: bg_color.into(),
            text_color: text_color.into(),
        }
    }
}
impl Default for DisclosureTheme {
    fn default() -> Self {
        Self {
            bg_color: "#dbeafe".to_string(),
            text_color: "#374155".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisclosureProps {
    pub theme: DisclosureTheme,
    pub id: String,
    pub summary: Vec<Block>,
    pub longform: Vec<Block>,
}

impl Default for DisclosureProps {
    fn default() -> Self {
        Self {
            theme: DisclosureTheme::default(),
            id: String::default(),
            summary: vec![],
            longform: vec![],
        }
    }
}

impl maud::Render for DisclosureProps {
    fn render(&self) -> maud::Markup {
        let container_class = format!("w-full overflow-hidden bg-[{bg_color}] text-[{text_color}] border border-[{bg_color}] rounded-lg shadow-lg", bg_color=&self.theme.bg_color, text_color=&self.theme.text_color);
        let details_class = format!("flex items-center justify-between p-3 font-bold text-white transition-colors cursor-pointer brightness-125 hover:brightness-150");
        let longform_class = format!(
            "p-3 border-t border-[{bg_color}]",
            bg_color = &self.theme.bg_color,
        );

        html! {
            details class=(container_class) {
                summary class=(details_class) {
                    @for text in &self.summary {
                        (text)
                    }
                }
                div class=(longform_class) {
                    @for text in &self.longform {
                        (text)
                    }
                }
            }
        }
    }
}
