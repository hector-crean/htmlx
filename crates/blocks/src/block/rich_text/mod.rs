pub mod css;

use css::css_to_tailwind;
use maud::{html, PreEscaped};
use maud::{Markup, Render};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{format, Write};
use strum::Display;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum RichText {
    Html(String),
    Tiptap(BranchNode),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum BranchTag {
    Doc,
    Div,
    Paragraph,
    Link,
    // Add other branch tags as needed
}

impl BranchTag {
    fn tag(&self) -> &str {
        match self {
            BranchTag::Div => "div",
            BranchTag::Paragraph => "p",
            _ => "div",
        }
    }
}

impl Default for BranchTag {
    fn default() -> Self {
        BranchTag::Div
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum LeafTag {
    Text,
    // Add other leaf tags as needed
}

impl Default for LeafTag {
    fn default() -> Self {
        LeafTag::Text
    }
}
impl LeafTag {
    fn tag(&self) -> &str {
        match self {
            LeafTag::Text => "text",
            _ => "span",
        }
    }
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Default)]
pub struct Mark {
    #[serde(rename = "type")]
    pub ty: String,
    pub attrs: Option<Attrs>,
}

impl Mark {
    pub fn new() -> Self {
        Self {
            ty: String::new(),
            attrs: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub struct BranchNode {
    id: String,
    #[serde(rename = "type")]
    pub ty: BranchTag,
    pub props: Attrs,
    #[serde(rename = "content")]
    pub children: Vec<TreeNode>,
    #[serde(rename = "children")]
    pub children2: Vec<TreeNode>,
    #[serde(flatten)]
    pub other: Option<HashMap<String, Value>>,
}

impl specta::Type for BranchNode {
    fn inline(type_map: &mut specta::TypeMap, generics: specta::Generics) -> specta::DataType {
        specta::DataType::Any
    }
}

impl BranchNode {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            ty: BranchTag::Div,
            props: Attrs::new(),
            children: vec![],
            children2: vec![],
            other: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct LeafNode {
    #[serde(rename = "type")]
    pub ty: LeafTag,
    pub text: Option<String>,
    #[serde(rename = "styles")]
    pub marks: Attrs,
    #[serde(flatten)]
    pub other: Option<HashMap<String, Value>>,
}

impl specta::Type for LeafNode {
    fn inline(type_map: &mut specta::TypeMap, generics: specta::Generics) -> specta::DataType {
        specta::DataType::Any
    }
}

impl LeafNode {
    pub fn new() -> Self {
        Self {
            ty: LeafTag::Text,
            text: None,
            other: None,
            marks: Attrs::new(),
        }
    }
}

#[derive(Serialize, Deserialize, specta::Type, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TreeNode {
    Leaf(LeafNode),
    Branch(BranchNode),
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
            @for node in &self.children {
                (node)
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
        let doc: Vec<TreeNode> = serde_json::from_str(JSON_DATA)?;
        println!("{:#?}", doc);

        let serialized = serde_json::to_string_pretty(&doc).unwrap();
        println!("{}", serialized);

        // assert_eq!(rendered, expected_output);
        Ok(())
    }

    #[test]
    fn render_json() -> Result<(), serde_json::Error> {
        let doc: Vec<BranchNode> = serde_json::from_str(JSON_DATA)?;
        println!("{:#?}", doc);

        for node in doc {
            let rendered = node.render().0.to_string();

            println!("{:?}", &rendered);
        }

        Ok(())
    }
}
