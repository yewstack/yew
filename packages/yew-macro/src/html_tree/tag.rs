use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::{ParseStream, Parser};
use syn::Token;

/// Check whether two spans are equal.
/// The implementation is really silly but I couldn't find another way to do it on stable.
/// This check isn't required to be fully accurate so it's not the end of the world if it breaks.
fn span_eq_hack(a: &Span, b: &Span) -> bool {
    format!("{:?}", a) == format!("{:?}", b)
}

/// Change all occurrences of span `from` to `to` in the given error.
fn error_replace_span(err: syn::Error, from: Span, to: impl ToTokens) -> syn::Error {
    let err_it = err.into_iter().map(|err| {
        if span_eq_hack(&err.span(), &from) {
            syn::Error::new_spanned(&to, err.to_string())
        } else {
            err
        }
    });

    // SAFETY: all errors have at least one message
    crate::join_errors(err_it).unwrap_err()
}

/// Helper type for parsing HTML tags.
/// The struct only stores the associated tokens, not the content of the tag.
/// This is meant to mirror the design of delimiters in `syn`.
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
        let scope_spanned = tag.to_spanned();
        let content_parser = |input: ParseStream| {
            parse(input, tag).map_err(|err| {
                // we can't modify the scope span used by `ParseStream`. It just uses the call site
                // by default. The scope span is used when an error can't be
                // attributed to a token tree (ex. when the input is empty).
                // We rewrite all spans to point at the tag which at least narrows down the correct
                // location. It's not ideal, but it'll have to do until `syn` gives
                // us more access.
                error_replace_span(err, Span::call_site(), &scope_spanned)
            })
        };
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
    /// This is to work around the limitation of being unable to manually join spans on stable.
    pub fn to_spanned(&self) -> impl ToTokens {
        let Self { lt, gt, .. } = self;
        quote! {#lt #gt}
    }
}
