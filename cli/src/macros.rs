

#[macro_export]
macro_rules! rich_text {
    ($file_path:expr) => {{
        let file_content = include_str!($file_path);
        let file_path = std::path::Path::new($file_path);

        if file_path
            .extension()
            .map(|ext| ext == "json")
            .unwrap_or(false)
        {
            let branch_node: RichTextBlock =
                serde_json::from_str(file_content).expect("Invalid JSON content in file");
            RichText::Tiptap(branch_node)
        } else {
            RichText::Html(file_content.to_string())
        }
    }};
}

#[macro_export]
macro_rules! rich_text_block {
    ($file_path:expr) => {{
        let file_content = include_str!($file_path);
        let file_path = std::path::Path::new($file_path);

        if file_path
            .extension()
            .map(|ext| ext == "json")
            .unwrap_or(false)
        {
            let branch_node: blocks::block::rich_text::RichTextBlock =
                serde_json::from_str(file_content).expect("Invalid JSON content in file");
            Block::RichTextBlock(blocks::block::rich_text::RichTextProps {
                text: blocks::block::rich_text::RichText::Tiptap(branch_node),
            })
        } else {
            Block::RichTextBlock(blocks::block::rich_text::RichTextProps {
                text: blocks::block::rich_text::RichText::Html(file_content.to_string()),
            })
        }
    }};
}
