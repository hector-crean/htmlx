pub mod css;

use css::css_to_tailwind;
use maud::{html, PreEscaped};
use maud::{Markup, Render};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::{format, Write};
use strum::{AsRefStr, Display};
use uuid::Uuid;

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub struct RichTextNode {
    id: String,
    #[serde(flatten)]
    pub node: RichTextNodeInner,
    //Any blocks nested inside the block. The nested blocks are also represented using Block objects.
    pub children: Vec<RichTextNode>,
    //The block's rich text content, usually represented as an array of InlineContent objects. This does not include content from any nested blocks
    pub content: Vec<RichTextNode>,
    #[serde(flatten)]
    pub other: Option<HashMap<String, Value>>,
}

impl specta::Type for RichTextNode {
    fn inline(type_map: &mut specta::TypeMap, generics: specta::Generics) -> specta::DataType {
        specta::DataType::Any
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, AsRefStr)]
#[serde(tag = "type", content = "props")]
pub enum RichTextNodeInner {
    #[strum(serialize = "div")]
    Doc(Attrs),
    #[strum(serialize = "div")]
    Div(Attrs),
    #[strum(serialize = "p")]
    Paragraph(Attrs),
    #[strum(serialize = "a")]
    Link(Attrs),
    #[strum(serialize = "li")]
    BulletListItem(Attrs),
    #[strum(serialize = "span")]
    Heading(Attrs),
    #[strum(serialize = "li")]
    NumberedListItem(Attrs),
    #[strum(serialize = "text")]
    Text(Attrs),
}

impl Render for RichTextNodeInner {
    fn render(&self) -> Markup {
        let tag = self.as_ref();

        let props = match &self {
            RichTextNodeInner::BulletListItem(li) => li,
        };

        html! {
            (maud::PreEscaped(format!("<{}", tag)))
            // @for mark in self.marks {
            // }
            (maud::PreEscaped(format!(" class={:?}", class)))
            (maud::PreEscaped(">"))
            @match &self.text {
                Some(text) =>  {
                    (text)
                },
                None => {}
            }
            (maud::PreEscaped(format!("</{}>", tag)))
        }
    }
}

impl Default for RichTextNodeInner {
    fn default() -> Self {
        Self::Div(Attrs::new())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum RichText {
    Html(String),
    Tiptap(RichTextNode),
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

impl Render for TreeNode {
    fn render(&self) -> Markup {
        html! {
            @match self {
                TreeNode::Leaf(leaf) => (leaf),
                TreeNode::Branch(branch) => (branch)
               }
        }
    }
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::Branch(BranchNode::default())
    }
}

impl Render for LeafNode {
    fn render(&self) -> Markup {
        let tag = self.ty.tag();

        let class = css_to_tailwind(self.marks.map.clone());

        html! {
            (maud::PreEscaped(format!("<{}", tag)))
            // @for mark in self.marks {
            // }
            (maud::PreEscaped(format!(" class={:?}", class)))
            (maud::PreEscaped(">"))
            @match &self.text {
                Some(text) =>  {
                    (text)
                },
                None => {}
            }
            (maud::PreEscaped(format!("</{}>", tag)))
        }
    }
}

impl Render for BranchNode {
    fn render(&self) -> Markup {
        let tag = self.ty.tag();

        let class = css_to_tailwind(self.props.map.clone());

        html! {
            (maud::PreEscaped(format!("<{}", tag)))
            // @for prop in props {
            //     (prop)
            // }
            (maud::PreEscaped(format!(" class={:?}", class)))
            (maud::PreEscaped(">"))
            @for node in &self.content {
                (node)
              }
              @for nested_node in &self.children {
                (nested_node)
              }
            (maud::PreEscaped(format!("</{}>", tag)))
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
        let doc: TreeNode = serde_json::from_str(JSON_DATA)?;
        println!("{:#?}", doc);

        let serialized = serde_json::to_string_pretty(&doc).unwrap();
        println!("{}", serialized);

        Ok(())
    }

    #[test]
    fn render_json() -> Result<(), serde_json::Error> {
        let doc: BranchNode = serde_json::from_str(JSON_DATA)?;

        let rendered = doc.render().0.to_string();
        println!("{:?}", &rendered);

        Ok(())
    }
}
