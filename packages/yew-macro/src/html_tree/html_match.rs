use proc_macro2::{Delimiter, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Local, Pat, Stmt, Token, braced, token};

use super::HtmlChildrenTree;
use super::html_block::html_macro_call_span;
use super::html_node::HtmlNode;
use crate::PeekValue;

pub struct HtmlMatch {
    match_token: Token![match],
    expr: Box<Expr>,
    _brace: token::Brace,
    arms: Vec<HtmlMatchArm>,
}

struct HtmlMatchArm {
    pat: Pat,
    guard: Option<(Token![if], Box<Expr>)>,
    fat_arrow_token: Token![=>],
    body: HtmlMatchArmBody,
    comma: Option<Token![,]>,
}

enum HtmlMatchArmBody {
    Braced {
        brace: token::Brace,
        let_stmts: Vec<Local>,
        children: HtmlChildrenTree,
        deprecations: TokenStream,
    },
    Unbraced {
        tree: Box<super::HtmlTree>,
        deprecations: TokenStream,
    },
}

impl PeekValue<()> for HtmlMatch {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "match").then_some(())
    }
}

impl Parse for HtmlMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let match_token = input.parse::<Token![match]>()?;
        let expr = Box::new(input.call(Expr::parse_without_eager_brace)?);

        if let Expr::Block(syn::ExprBlock { block, .. }) = &*expr {
            if block.stmts.is_empty() {
                return Err(syn::Error::new(
                    expr.span(),
                    "missing expression for `match`",
                ));
            }
        }

        if input.is_empty() {
            return Err(syn::Error::new(
                expr.span(),
                "this `match` expression has a scrutinee, but no body",
            ));
        }

        let content;
        let brace = braced!(content in input);

        let mut arms = Vec::new();
        while !content.is_empty() {
            arms.push(content.parse::<HtmlMatchArm>()?);
        }

        if arms.is_empty() {
            return Err(syn::Error::new(
                brace.span.span(),
                "`match` expression must have at least one arm",
            ));
        }

        Ok(HtmlMatch {
            match_token,
            expr,
            _brace: brace,
            arms,
        })
    }
}

impl Parse for HtmlMatchArm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat = Pat::parse_multi_with_leading_vert(input)?;

        let guard = if input.peek(Token![if]) {
            let if_token: Token![if] = input.parse()?;
            let guard_expr = input.call(Expr::parse_without_eager_brace)?;
            Some((if_token, Box::new(guard_expr)))
        } else {
            None
        };

        let fat_arrow_token: Token![=>] = input.parse()?;

        let mut body = if input.cursor().group(Delimiter::Brace).is_some() {
            let content;
            let brace = braced!(content in input);
            let mut let_stmts = Vec::new();
            while content.peek(Token![let]) {
                let stmt: Stmt = content.parse()?;
                match stmt {
                    Stmt::Local(local) => let_stmts.push(local),
                    _ => unreachable!("peeked Token![let] but parsed non-local statement"),
                }
            }
            let children = HtmlChildrenTree::parse_delimited_with_nodes(&content)?;
            let deprecations = super::check_unnecessary_fragment(&children);
            HtmlMatchArmBody::Braced {
                brace,
                let_stmts,
                children,
                deprecations,
            }
        } else {
            HtmlMatchArmBody::Unbraced {
                tree: Box::new(super::HtmlTree::parse_or_node(input)?),
                deprecations: TokenStream::new(),
            }
        };

        let arm_deprecations = check_arm_html_macro_call(&body);
        if !arm_deprecations.is_empty() {
            match &mut body {
                HtmlMatchArmBody::Braced { deprecations, .. } => {
                    deprecations.extend(arm_deprecations)
                }
                HtmlMatchArmBody::Unbraced { deprecations, .. } => {
                    deprecations.extend(arm_deprecations);
                }
            }
        }

        let comma: Option<Token![,]> = input.parse()?;

        Ok(HtmlMatchArm {
            pat,
            guard,
            fat_arrow_token,
            body,
            comma,
        })
    }
}

impl ToTokens for HtmlMatchArmBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Braced {
                brace,
                let_stmts,
                children,
                deprecations,
            } => {
                tokens.extend(quote_spanned! {brace.span.span()=>
                    {
                        #deprecations
                        #(#let_stmts)*
                        ::yew::virtual_dom::VNode::VList(::std::rc::Rc::new(
                            ::yew::virtual_dom::VList::with_children(
                                #children, ::std::option::Option::None
                            )
                        ))
                    }
                });
            }
            Self::Unbraced { tree, deprecations } => {
                tokens.extend(quote_spanned! {tree.span()=>
                    {
                        #deprecations
                        ::std::convert::Into::<::yew::virtual_dom::VNode>::into(#tree)
                    }
                });
            }
        }
    }
}

impl ToTokens for HtmlMatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            match_token,
            expr,
            arms,
            ..
        } = self;

        let arms_tokens = arms.iter().map(|arm| {
            let HtmlMatchArm {
                pat,
                guard,
                fat_arrow_token,
                body,
                comma,
            } = arm;
            let guard_tokens = guard
                .as_ref()
                .map(|(if_token, guard_expr)| quote! { #if_token #guard_expr });
            quote! {
                #pat #guard_tokens #fat_arrow_token #body #comma
            }
        });

        tokens.extend(quote_spanned! {match_token.span()=>
            match #expr {
                #(#arms_tokens)*
            }
        });
    }
}

fn check_arm_html_macro_call(body: &HtmlMatchArmBody) -> TokenStream {
    let trees: Box<dyn Iterator<Item = &super::HtmlTree> + '_> = match body {
        HtmlMatchArmBody::Braced { children, .. } => Box::new(children.0.iter()),
        HtmlMatchArmBody::Unbraced { tree, .. } => Box::new(std::iter::once(tree.as_ref())),
    };
    for tree in trees {
        if let super::HtmlTree::Node(node) = tree {
            if let HtmlNode::Expression(expr) = node.as_ref() {
                if let Some(span) = html_macro_call_span(expr) {
                    return super::deprecated_call(
                        span,
                        "`html!` is not needed in `match` arms. Use bare elements directly",
                    );
                }
            }
        }
    }
    TokenStream::new()
}
