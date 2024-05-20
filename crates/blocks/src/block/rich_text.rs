use maud::{html, PreEscaped};
use maud::{Markup, Render};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum RichText {
    Html(String),
    Tiptap(JSONContent),
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
                },
                RichText::Tiptap(json) => (json)
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Attrs {
    #[serde(flatten)]
    map: HashMap<String, Value>,
}
impl specta::Type for Attrs {
    fn inline(type_map: &mut specta::TypeMap, generics: specta::Generics) -> specta::DataType {
        specta::DataType::Any
    }
}
impl Attrs {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct JSONContent {
    pub content_type: Option<String>,
    pub attrs: Option<Attrs>,
    pub content: Option<Vec<JSONContent>>,
    pub marks: Option<Vec<Mark>>,
    pub text: Option<String>,
    pub other: Attrs,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Mark {
    pub mark_type: String,
    pub attrs: Option<Attrs>,
    pub other: Attrs,
}

impl JSONContent {
    pub fn new() -> Self {
        JSONContent {
            content_type: None,
            attrs: None,
            content: None,
            marks: None,
            text: None,
            other: Attrs::new(),
        }
    }
}

impl Mark {
    pub fn new(mark_type: String) -> Self {
        Mark {
            mark_type,
            attrs: None,
            other: Attrs::new(),
        }
    }
}

impl Render for JSONContent {
    fn render(&self) -> Markup {
        let tag = self
            .content_type
            .clone()
            .unwrap_or_else(|| "div".to_string());

        html! {
            (maud::PreEscaped(format!("<{} ", tag)))

            @match &self.content {
                Some(blocks) => {
                    @for block in blocks {
                        (block)
                    }
                },
                None => ("")
            }
            (maud::PreEscaped(format!("</{}>", tag)))
        }
    }
}
