use askama::Template;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum RichText {
    Html(String),
}

impl std::fmt::Display for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RichText::Html(content) => {
                write!(f, "{}", content)
            }
        }
    }
}

#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[template(path = "rich_text.html")]

pub struct RichTextProps {
    pub text: RichText,
}
