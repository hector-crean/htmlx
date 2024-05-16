use std::borrow::Cow;

use askama::Template;

use crate::block::Block;


#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[template(path = "page.html")]
pub struct Page {
    pub id: uuid::Uuid,
    pub blocks: Vec<Block>
}
