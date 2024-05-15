use askama::Template;

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum RichText {
    Html(String),
}

// impl std::fmt::Display for RichText {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             RichText::Html(content) => {
//                 write!(f, "{}", content)
//             }
//         }
//     }
// }

#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[template(path = "rich_text.html", ext = "html", escape = "none")]
pub struct RichTextProps {
    pub text: RichText,
}
