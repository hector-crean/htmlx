use maud::{html, PreEscaped};
use maud::{Markup, Render};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{format, Write};
use strum::Display;

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, Default, PartialEq)]
pub struct BranchNode {
    #[serde(rename = "type")]
    pub ty: BranchTag,
    pub attrs: Option<Attrs>,
    #[serde(rename = "content")]
    pub children: Option<Vec<TreeNode>>,
}

impl BranchNode {
    pub fn new() -> Self {
        Self {
            ty: BranchTag::Div,
            attrs: None,
            children: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Default)]
pub struct LeafNode {
    #[serde(rename = "type")]
    pub ty: LeafTag,
    pub text: Option<String>,
    pub attrs: Option<Attrs>,
    pub marks: Option<Vec<Mark>>,
}

impl LeafNode {
    pub fn new() -> Self {
        Self {
            ty: LeafTag::Text,
            text: None,
            attrs: None,
            marks: None,
        }
    }
}

#[derive(Serialize, Deserialize, specta::Type, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TreeNode {
    Leaf(LeafNode),
    Branch(BranchNode),
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::Branch(BranchNode::default())
    }
}

impl Render for LeafNode {
    fn render(&self) -> Markup {
        let tag = self.ty.tag();

        let attrs = if let Some(attrs) = &self.attrs {
            attrs
                .map
                .iter()
                .map(|(key, value)| {
                    if let Some(value) = value.as_str() {
                        let attr_str = format!("{:?}={:?}", key.to_string(), value.to_string());
                        maud::PreEscaped(attr_str)
                    } else {
                        maud::PreEscaped("".to_string())
                    }
                })
                .collect::<Vec<maud::PreEscaped<String>>>()
        } else {
            vec![]
        };

        html! {
            (maud::PreEscaped(format!("<{}", tag)))
            @for attr in &attrs {
                (attr)
            }
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

        let attrs = if let Some(attrs) = &self.attrs {
            attrs
                .map
                .iter()
                .map(|(key, value)| {
                    if let Some(value) = value.as_str() {
                        let attr_str = format!("{:?}={:?}", key.to_string(), value.to_string());
                        maud::PreEscaped(attr_str)
                    } else {
                        maud::PreEscaped("".to_string())
                    }
                })
                .collect::<Vec<maud::PreEscaped<String>>>()
        } else {
            vec![]
        };

        html! {
            (maud::PreEscaped(format!("<{}", tag)))
            @for attr in &attrs {
                (attr)
            }
            (maud::PreEscaped(">"))
            @match &self.children {
                Some(blocks) => {
                    @for block in blocks {
                       @match block {
                        TreeNode::Leaf(leaf) => (leaf),
                        TreeNode::Branch(branch) => (branch)
                       }
                    }
                },
                None => {}
            }
            (maud::PreEscaped(format!("</{}>", tag)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maud::{html, PreEscaped, Render};

    static JSON_DATA: &str = r#"
    {
      "type": "doc",
      "content": [
        {
          "type": "paragraph",
          "content": [
            {
              "type": "text",
              "text": "It’s 19871. You can’t t"
            },
            {
              "type": "text",
              "marks": [
                {
                  "type": "bold"
                }
              ],
              "text": "urn on a radio, or go "
            },
            {
              "type": "text",
              "text": "to a mall without hearing Olivia Newton-John’s hit song, Physical."
            }
          ]
        },
        {
          "type": "paragraph",
          "content": [
            {
              "type": "text",
              "text": "Hello,"
            }
          ]
        },
        {
          "type": "paragraph"
        },
        {
          "type": "paragraph",
          "content": [
            {
              "type": "text",
              "text": "how good is this"
            }
          ]
        }
      ]
    }
    "#;

    #[test]
    fn test_json_content_rendering() {
        let doc: BranchNode = serde_json::from_str(JSON_DATA).unwrap();
        println!("{:#?}", doc);

        let serialized = serde_json::to_string_pretty(&doc).unwrap();
        println!("{}", serialized);

        // assert_eq!(rendered, expected_output);
    }

    #[test]
    fn render_json() {
        let doc: BranchNode = serde_json::from_str(JSON_DATA).unwrap();
        println!("{:#?}", doc);

        let rendered = doc.render().0.to_string();

        println!("{:?}", &rendered);
    }
}
