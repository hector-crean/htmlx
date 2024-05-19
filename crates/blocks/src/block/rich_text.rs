use maud::{html, PreEscaped};
use std::fmt::Write;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum RichText {
    Html(String),
}

impl Default for RichText {
    fn default() -> Self {
        Self::Html(String::default())
    }
}

impl maud::Render for RichText {
    fn render(&self) -> maud::Markup {
        html! {
            @match &self {
                RichText::Html(html) => {
                    (PreEscaped(html))
                }
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct RichTextProps {
    pub text: RichText,
}

impl Default for RichTextProps {
    fn default() -> Self {
        Self {
            text: RichText::default(),
        }
    }
}

impl maud::Render for RichTextProps {
    fn render(&self) -> maud::Markup {
        html! {
            (self.text)
        }
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Document {
//     // pub type: String,
//     pub content: Vec<Node>,
// }
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type")]
// pub enum Node {
//     Paragraph(Paragraph),
//     Heading(Heading),
//     BulletList(BulletList),
//     OrderedList(OrderedList),
//     ListItem(ListItem),
//     Text(Text),
//     // Add other node types as needed
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Paragraph {
//     pub content: Vec<Node>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Heading {
//     pub attrs: HeadingAttrs,
//     pub content: Vec<Node>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct HeadingAttrs {
//     pub level: u8,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct BulletList {
//     pub content: Vec<Node>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OrderedList {
//     pub content: Vec<Node>,
//     pub attrs: OrderedListAttrs,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OrderedListAttrs {
//     pub order: u32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ListItem {
//     pub content: Vec<Node>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Text {
//     pub text: String,
//     pub marks: Option<Vec<Mark>>,
// }
