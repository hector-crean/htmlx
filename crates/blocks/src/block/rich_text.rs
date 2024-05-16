use askama::Template;
use std::fmt::Write;
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

impl Default for RichText {
    fn default() -> Self {
        Self::Html(String::default())
    }
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

impl RichText {
    // pub fn render(&self) -> Result<String, std::fmt::Error> {
    //     let mut buf = String::new();

    //     write!(&mut buf, "{}", self)?;

    //     Ok(buf)
    // }
   
}

#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[template(path = "rich_text.html", escape="none")]
pub struct RichTextProps {
    pub text: RichText,
}

impl Default for RichTextProps { 
    fn default() -> Self {
        Self { text: RichText::default()}
    }
}

// impl std::fmt::Display for RichTextProps {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.text)
//     }
// }

impl RichTextProps { 
    pub fn escaped_html(&self) -> String {
        match &self.text {
            RichText::Html(html) => html_escape::encode_safe(&html).into(),
            _ => String::new(),
        }
    }
}