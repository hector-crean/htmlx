


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[cfg_attr(feature = "typings", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct RichText {
    pub text: String,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[cfg_attr(feature = "typings", derive(specta::Type))]
#[serde(tag = "type", content = "data")]
pub enum Block {
    RichTextBlock(RichText),
}
