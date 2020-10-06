use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{ParseStream, Parser},
    Token,
};

/// Helper type for parsing HTML tags.
/// The struct only stores the associated tokens similar to how delimiters work in `syn`.
pub struct TagTokens {
    pub lt: Token![<],
    pub div: Option<Token![/]>,
    pub gt: Token![>],
}
impl TagTokens {
    /// Parse the content of a start tag.
    /// The given parse function is called with a `ParseStream`
    /// containing only the contents of the tag and surrounding `TagTokens`.
    pub fn parse_start_content<T>(
        input: ParseStream,
        parse: impl FnOnce(ParseStream, Self) -> syn::Result<T>,
    ) -> syn::Result<T> {
        Self::parse_content(Self::parse_start(input)?, parse)
    }

    /// Same as `parse_start_content` but for end tags.
    pub fn parse_end_content<T>(
        input: ParseStream,
        parse: impl FnOnce(ParseStream, Self) -> syn::Result<T>,
    ) -> syn::Result<T> {
        Self::parse_content(Self::parse_end(input)?, parse)
    }

    fn parse_content<T>(
        (tag, content): (Self, TokenStream),
        parse: impl FnOnce(ParseStream, Self) -> syn::Result<T>,
    ) -> syn::Result<T> {
        let content_parser = |input: ParseStream| parse(input, tag);
        content_parser.parse2(content)
    }

    /// Parse a start tag
    fn parse_start(input: ParseStream) -> syn::Result<(Self, TokenStream)> {
        let lt = input.parse()?;
        let (content, div, gt) = Self::parse_until_end(input)?;

        Ok((Self { lt, div, gt }, content))
    }

    /// Parse an end tag.
    /// `div` will always be `Some` for end tags.
    fn parse_end(input: ParseStream) -> syn::Result<(Self, TokenStream)> {
        let lt = input.parse()?;
        let div = Some(input.parse()?);

        let (content, end_div, gt) = Self::parse_until_end(input)?;
        if end_div.is_some() {
            return Err(syn::Error::new_spanned(
                end_div,
                "unexpected `/` in this end tag",
            ));
        }

        Ok((Self { lt, div, gt }, content))
    }

    fn parse_until_end(
        input: ParseStream,
    ) -> syn::Result<(TokenStream, Option<Token![/]>, Token![>])> {
        let mut inner_trees = Vec::new();
        let mut angle_count: usize = 1;
        let mut div: Option<Token![/]> = None;
        let gt: Token![>];

        loop {
            let next = input.parse()?;
            if let TokenTree::Punct(punct) = &next {
                match punct.as_char() {
                    '/' => {
                        if angle_count == 1 && input.peek(Token![>]) {
                            div = Some(syn::token::Div {
                                spans: [punct.span()],
                            });
                            gt = input.parse()?;
                            break;
                        }
                    }
                    '>' => {
                        angle_count = angle_count.checked_sub(1).ok_or_else(|| {
                            syn::Error::new_spanned(
                                punct,
                                "this tag close has no corresponding tag open",
                            )
                        })?;
                        if angle_count == 0 {
                            gt = syn::token::Gt {
                                spans: [punct.span()],
                            };
                            break;
                        }
                    }
                    '<' => angle_count += 1,
                    _ => {}
                };
            }

            inner_trees.push(next);
        }

        Ok((inner_trees.into_iter().collect(), div, gt))
    }

    /// Generate tokens which can be used in `syn::Error::new_spanned` to span the entire tag.
    /// This is to work around limitation of being unable to manually join spans which exists in stable Rust.
    pub fn to_spanned(&self) -> impl ToTokens {
        let Self { lt, gt, .. } = self;
        quote! {#lt#gt}
    }
}
