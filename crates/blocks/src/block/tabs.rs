use std::borrow::Cow;

use crate::block::{Block, BlocksProps, RichTextProps};
use askama::Template;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
pub struct Tab {
    pub name: String,
    pub tab_content: BlocksProps,
}

#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[template(path = "tabs.html", ext = "html", escape = "none")]
pub struct TabsProps {
    pub id: uuid::Uuid,
    pub tabs: Vec<Tab>,
}
