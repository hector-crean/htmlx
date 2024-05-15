use blocks::block::rich_text::{RichText, RichTextProps};
use blocks::block::Block;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn rich_text(input: TokenStream) -> TokenStream {
    // Parse the input token stream
    let input = parse_macro_input!(input as LitStr);
    let file_path = input.value();

    // Read the file content
    let content = std::fs::read_to_string(file_path).expect("Unable to read file");

    // Generate the output tokens
    let expanded = quote! {
        RichText::Html(#content.to_string())
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn rich_text_block(input: TokenStream) -> TokenStream {
    // Parse the input token stream
    let input = parse_macro_input!(input as LitStr);
    let file_path = input.value();

    // Read the file content
    let content = std::fs::read_to_string(file_path).expect("Unable to read file");

    // Generate the output tokens
    let expanded = quote! {
        Block::RichTextBlock(RichTextProps {
            text: RichText::Html(#content.to_string())
        })

    };

    TokenStream::from(expanded)
}
