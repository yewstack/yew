mod cmark;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use proc_macro::{TokenStream, TokenTree};

use self::cmark::parse_commonmark;

lazy_static::lazy_static! {
static ref GLOBAL_STYLE : Arc<Mutex<HashMap<String, String>>> = {
    Default::default()
};
}

pub fn mdx_style(input: TokenStream) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    for chunk in input.chunks(4) {
        let (from, to) = (chunk.get(0), chunk.get(2));
        match from.zip(to) {
            Some((TokenTree::Ident(from), TokenTree::Ident(to))) => {
                GLOBAL_STYLE
                    .lock()
                    .unwrap()
                    .insert(from.to_string(), to.to_string());
            }
            _ => {}
        }
    }
    // GLOBAL_STYLE
    //     .lock()
    //     .unwrap()
    //     .insert("h3".into(), "MyHeading3".into());
    quote::quote! {}.into()
}

pub fn mdx(input: TokenStream) -> TokenStream {
    let parsed = input
        .into_iter()
        .map(|token| match token {
            lit @ TokenTree::Literal(_) => {
                let mdx_str = lit.to_string();
                let mdx_str = mdx_str
                    .strip_prefix("r#\"")
                    .unwrap()
                    .strip_suffix("\"#")
                    .unwrap();
                parse_commonmark(&mdx_str)
            }
            _ => panic!("mdx! expected literal"),
        })
        .collect::<TokenStream>();

    parsed
}

pub fn include_mdx(input: TokenStream) -> TokenStream {
    let file_path: std::path::PathBuf = input
        .to_string()
        .trim_start_matches('"')
        .trim_end_matches('"')
        .parse()
        .unwrap();

    let full_path = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap()
        .parse::<std::path::PathBuf>()
        .unwrap()
        .join(file_path);
    let contents = std::fs::read_to_string(full_path)
        .unwrap()
        .replace("\r", "");

    parse_commonmark(&contents)
}
