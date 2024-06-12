use blocks::block::rich_text::{RichText, RichTextBlock, RichTextProps};
use blocks::block::Block;
use proc_macro::TokenStream;
use quote::quote;
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn rich_text_block(input: TokenStream) -> TokenStream {
    // Parse the input as a literal string
    let input = parse_macro_input!(input as LitStr);
    let file_path = input.value();

    // Generate the output tokens
    let expanded = quote! {
        {
            let file_path = std::path::Path::new(#file_path);
            let content = std::fs::read_to_string(file_path)
                .unwrap_or_else(|_| panic!("Unable to read file at {:?}", file_path));

            let text = if file_path.extension().map(|s| s == "json").unwrap_or(false) {
                let rich_text_block: RichTextBlock = serde_json::from_str(&content)
                    .unwrap_or_else(|_| panic!("Invalid JSON content in file {:?}", file_path));
                RichText::Tiptap(rich_text_block)
            } else {
                RichText::Html(content)
            };

            Block::RichTextBlock(RichTextProps {
                text
            })
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn rich_text(input: TokenStream) -> TokenStream {
    // Parse the input as a literal string
    let input = parse_macro_input!(input as LitStr);
    let file_path = input.value();

    // Generate the output tokens
    let expanded = quote! {
        {
            let file_path = std::path::Path::new(#file_path);
            let content = std::fs::read_to_string(file_path)
                .unwrap_or_else(|_| panic!("Unable to read file at {:?}", file_path));

            let text = if file_path.extension().map(|s| s == "json").unwrap_or(false) {
                let rich_text_block: RichTextBlock = serde_json::from_str(&content)
                    .unwrap_or_else(|_| panic!("Invalid JSON content in file {:?}", file_path));
                RichText::Tiptap(rich_text_block)
            } else {
                RichText::Html(content)
            };

            text
        }
    };

    TokenStream::from(expanded)
}
