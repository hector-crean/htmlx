use blocks::block::{
    rich_text::{RichText, RichTextProps},
    Block,
};

#[macro_export]
macro_rules! rich_text_block {
    ($file_path:expr) => {
        Block::RichTextBlock(RichTextProps {
            text: RichText::Html(include_str!($file_path).to_string()),
        })
    };
}

#[macro_export]
macro_rules! rich_text {
    ($file_path:expr) => {
        RichText::Html(include_str!($file_path).to_string())
    };
}
