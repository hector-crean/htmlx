use blocks::block::rich_text::{RichText, RichTextProps};
use blocks::block::Block;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ExprArray, parse_quote};
// #[proc_macro]
// pub fn rich_text(input: TokenStream) -> TokenStream {
//     // Parse the input token stream
//     let input = parse_macro_input!(input as LitStr);
//     let file_path = input.value();

//     // Read the file content
//     let content = std::fs::read_to_string(file_path).expect("Unable to read file");

//     // Generate the output tokens
//     let expanded = quote! {
//         RichText::Html(#content.to_string())
//     };

//     TokenStream::from(expanded)
// }


// #[proc_macro]
// pub fn rich_text_block(input: TokenStream) -> TokenStream {
//     // Parse the input token stream
//     let input = parse_macro_input!(input as LitStr);
//     let file_path = input.value();
//     let path = Path::new(&file_path);

//     // Read the file content safely
//     let content = std::fs::read_to_string(path)
//         .unwrap_or_else(|err| panic!("Unable to read file at {:?}: {}", path, err));

//     // Generate the output tokens
//     let expanded = quote! {
//         Block::RichTextBlock(RichTextProps {
//             text: RichText::Html(#content.to_string())
//         })
//     };

//     TokenStream::from(expanded)
// }





#[proc_macro]
pub fn rich_text_block(input: TokenStream) -> TokenStream {
    // Parse the input as an array of expressions
    let input_array = parse_macro_input!(input as ExprArray);

    // Prepare to evaluate each expression to a path component at compile time
    let mut path_components = Vec::new();
    for expr in input_array.elems {
        let tokens: proc_macro2::TokenStream = expr.into_token_stream();
        path_components.push(tokens);
    }

    // Use the collected tokens to build the final path expression
    let path_expr: proc_macro2::TokenStream = parse_quote! {
        {
            let mut path_buf = std::path::PathBuf::new();
            #(
                path_buf.push(#path_components);
            )*
            path_buf
        }
    };

    // Read the file content and embed into the output
    let expanded = quote! {
        {
            let file_path = #path_expr.to_str().expect("Invalid file path");
            let content = std::fs::read_to_string(file_path)
                .unwrap_or_else(|_| panic!("Unable to read file at {:?}", file_path));
            Block::RichTextBlock(RichTextProps {
                text: RichText::Html(content)
            })
        }
    };

    TokenStream::from(expanded)
}
