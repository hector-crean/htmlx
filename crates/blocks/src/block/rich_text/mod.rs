pub mod css;

use css::css_to_tailwind;
use maud::{html, PreEscaped};
use maud::{Markup, Render};
use serde::Serialize;
use serde::{self, Deserialize, Deserializer};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::{format, Write};
use strum::{AsRefStr, Display};
use uuid::Uuid;

fn deserialize_content<'de, D>(deserializer: D) -> Result<Vec<RichTextInline>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ContentVisitor;

    impl<'de> serde::de::Visitor<'de> for ContentVisitor {
        type Value = Vec<RichTextInline>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a RichTextInline or a sequence of RichTextInline")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(elem) = seq.next_element()? {
                vec.push(elem);
            }
            Ok(vec)
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            Ok(vec![RichTextInline::deserialize(
                serde::de::value::MapAccessDeserializer::new(map),
            )?])
        }
    }

    deserializer.deserialize_any(ContentVisitor)
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Attrs {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, AsRefStr)]
#[serde(tag = "type", content = "props", rename_all = "camelCase")]
pub enum RichTextBlockNode {
    #[strum(serialize = "div")]
    Doc(Attrs),
    #[strum(serialize = "div")]
    Div(Attrs),
    #[strum(serialize = "p")]
    Paragraph(Attrs),
    #[strum(serialize = "div")]
    Heading(Attrs),
    #[strum(serialize = "li")]
    NumberedListItem(Attrs),
    #[strum(serialize = "li")]
    BulletListItem(Attrs),
    #[strum(serialize = "table")]
    Table(Attrs),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct TableRow {
    cells: Vec<Vec<RichTextInline>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, AsRefStr)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RichTextInlineNode {
    #[strum(serialize = "a")]
    Link {
        content: Vec<RichTextInline>,
        href: String,
    },
    #[strum(serialize = "text")]
    Text { text: String, styles: Attrs },
    #[strum(serialize = "table_content")]
    TableContent { rows: Vec<TableRow> },
}

impl Default for RichTextInlineNode {
    fn default() -> Self {
        RichTextInlineNode::Text {
            text: String::from(""),
            styles: Attrs::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub struct RichTextBlock {
    id: Option<String>,
    #[serde(flatten)]
    pub node: RichTextBlockNode,
    //Any blocks nested inside the block. The nested blocks are also represented using Block objects.
    pub children: Vec<RichTextBlock>,
    //The block's rich text content, usually represented as an array of InlineContent objects. This does not include content from any nested blocks
    #[serde(deserialize_with = "deserialize_content")]
    pub content: Vec<RichTextInline>,
    #[serde(flatten)]
    pub other: Option<HashMap<String, Value>>,
}

impl specta::Type for RichTextBlock {
    fn inline(type_map: &mut specta::TypeMap, generics: specta::Generics) -> specta::DataType {
        specta::DataType::Any
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub struct RichTextInline {
    #[serde(flatten)]
    pub node: RichTextInlineNode,
    #[serde(flatten)]
    pub other: Option<HashMap<String, Value>>,
}

impl specta::Type for RichTextInline {
    fn inline(type_map: &mut specta::TypeMap, generics: specta::Generics) -> specta::DataType {
        specta::DataType::Any
    }
}

impl Render for RichTextInline {
    fn render(&self) -> Markup {
        html! {
            @match &self.node {
                RichTextInlineNode::Link { content, href } => {
                    a href=(href) {
                        @for c in content {
                            (c)
                        }
                     }

                }
                RichTextInlineNode::Text { text, styles } => {
                    text {  (text.as_str())}
                }
                RichTextInlineNode::TableContent { rows } => {
                    @for row in rows {
                        tr {
                            @for col in &row.cells {
                                @for d in col {
                                    td { (d)}
                                }
                            }
                        }
                    }

                }
            }

        }
    }
}

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, AsRefStr)]
// #[serde(tag = "type")]
// pub enum RichTextInlineInner {
//     #[strum(serialize = "a")]
//     Link(Attrs),
//     #[strum(serialize = "text")]
//     Text(Attrs),
// }

impl Render for RichTextBlock {
    fn render(&self) -> Markup {
        let tag = self.node.as_ref();

        let props = match &self.node {
            RichTextBlockNode::Doc(attrs) => attrs,
            RichTextBlockNode::Div(attrs) => attrs,
            RichTextBlockNode::Paragraph(attrs) => attrs,
            RichTextBlockNode::BulletListItem(attrs) => attrs,
            RichTextBlockNode::Heading(attrs) => attrs,
            RichTextBlockNode::NumberedListItem(attrs) => attrs,
            RichTextBlockNode::Table(attrs) => attrs,
        };

        let class = css_to_tailwind(props.map.clone());

        html! {
            (maud::PreEscaped(format!("<{}", tag)))
            (maud::PreEscaped(format!(" class={:?}", class)))
            (maud::PreEscaped(">"))
            @for node in &self.children {
                (node)
            }
            @for node in &self.content {
                (node)
            }
            (maud::PreEscaped(format!("</{}>", tag)))
        }
    }
}

impl Default for RichTextBlockNode {
    fn default() -> Self {
        Self::Div(Attrs::new())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum RichText {
    Html(String),
    Tiptap(RichTextBlock),
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use maud::{html, PreEscaped, Render};
    use serde_json::json;

    static JSON_DATA: &str = include_str!("./rich_text.json");

    #[test]
    fn test_json_content_rendering() -> Result<(), serde_json::Error> {
        let doc: RichTextBlock = serde_json::from_str(JSON_DATA)?;
        println!("{:#?}", doc);

        let serialized = serde_json::to_string_pretty(&doc).unwrap();
        println!("{}", serialized);

        Ok(())
    }

    #[test]
    fn render_json() -> Result<(), serde_json::Error> {
        let doc: RichTextBlock = serde_json::from_str(JSON_DATA)?;

        let rendered = doc.render().0.to_string();
        println!("{:?}", &rendered);

        Ok(())
    }
}
