use super::CHILDREN_LABEL;
use crate::html_tree::HtmlDashedName;
use proc_macro2::{Spacing, TokenTree};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    ops::{Deref, DerefMut},
};
use syn::{
    braced,
    parse::{Parse, ParseBuffer, ParseStream},
    token::Brace,
    Block, Expr, ExprBlock, ExprPath, ExprRange, Stmt, Token,
};

pub struct Prop {
    pub label: HtmlDashedName,
    /// Punctuation between `label` and `value`.
    pub value: Expr,
}
impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            Self::parse_shorthand_prop_assignment(input)
        } else {
            Self::parse_prop_assignment(input)
        }
    }
}

/// Helpers for parsing props
impl Prop {
    /// Parse a prop using the shorthand syntax `{value}`, short for `value={value}`
    /// This only allows for labels with no hyphens, as it would otherwise create
    /// an ambiguity in the syntax
    fn parse_shorthand_prop_assignment(input: ParseStream) -> syn::Result<Self> {
        let value;
        let _brace = braced!(value in input);
        let expr = value.parse::<Expr>()?;
        let label = if let Expr::Path(ExprPath {
            ref attrs,
            qself: None,
            ref path,
        }) = expr
        {
            if let (Some(ident), true) = (path.get_ident(), attrs.is_empty()) {
                syn::Result::Ok(HtmlDashedName::from(ident.clone()))
            } else {
                Err(syn::Error::new_spanned(
                    path,
                    "only simple identifiers are allowed in the shorthand property syntax",
                ))
            }
        } else {
            return Err(syn::Error::new_spanned(
                expr,
                "missing label for property value. If trying to use the shorthand property syntax, only identifiers may be used",
            ));
        }?;

        Ok(Self { label, value: expr })
    }

    /// Parse a prop of the form `label={value}`
    fn parse_prop_assignment(input: ParseStream) -> syn::Result<Self> {
        let label = input.parse::<HtmlDashedName>()?;
        let equals = input.parse::<Token![=]>().map_err(|_| {
            syn::Error::new_spanned(
                &label,
                format!("`{}` doesn't have a value. (hint: set the value to `true` or `false` for boolean attributes)", label),
            )
        })?;
        if input.is_empty() {
            return Err(syn::Error::new_spanned(
                equals,
                "expected an expression following this equals sign",
            ));
        }

        let value = parse_prop_value(input)?;
        Ok(Self { label, value })
    }
}

fn parse_prop_value(input: &ParseBuffer) -> syn::Result<Expr> {
    if input.peek(Brace) {
        strip_braces(input.parse()?)
    } else {
        let expr = if let Some(ExprRange {
            from: Some(from), ..
        }) = range_expression_peek(input)
        {
            // If a range expression is seen, treat the left-side expression as the value
            // and leave the right-side expression to be parsed as a base expression
            advance_until_next_dot2(input)?;
            *from
        } else {
            input.parse()?
        };

        match &expr {
            Expr::Lit(_) => Ok(expr),
            _ => {
                Err(syn::Error::new_spanned(
                    &expr,
                    "the property value must be either a literal or enclosed in braces. Consider adding braces around your expression.",
                ))
            }
        }
    }
}

fn strip_braces(block: ExprBlock) -> syn::Result<Expr> {
    match block {
        ExprBlock {
            block: Block { mut stmts, .. },
            ..
        } if stmts.len() == 1 => {
            let stmt = stmts.remove(0);
            match stmt {
                Stmt::Expr(expr) => Ok(expr),
                Stmt::Semi(_expr, semi) => Err(syn::Error::new_spanned(
                        semi,
                        "only an expression may be assigned as a property. Consider removing this semicolon",
                )),
                _ =>             Err(syn::Error::new_spanned(
                        stmt,
                        "only an expression may be assigned as a property",
                ))
            }
        }
        block => Ok(Expr::Block(block)),
    }
}

// Without advancing cursor, returns the range expression at the current cursor position if any
fn range_expression_peek(input: &ParseBuffer) -> Option<ExprRange> {
    match input.fork().parse::<Expr>().ok()? {
        Expr::Range(range) => Some(range),
        _ => None,
    }
}

fn advance_until_next_dot2(input: &ParseBuffer) -> syn::Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        let mut first_dot = None;
        while let Some((tt, next)) = rest.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == '.' => {
                    if let Some(first_dot) = first_dot {
                        return Ok(((), first_dot));
                    } else {
                        // Only consider dot as potential first if there is no spacing after it
                        first_dot = if punct.spacing() == Spacing::Joint {
                            Some(rest)
                        } else {
                            None
                        };
                    }
                }
                _ => {
                    first_dot = None;
                }
            }
            rest = next;
        }
        Err(cursor.error("no `..` found in expression"))
    })
}

/// List of props sorted in alphabetical order*.
///
/// \*The "children" prop always comes last to match the behaviour of the `Properties` derive macro.
///
/// The list may contain multiple props with the same label.
/// Use `check_no_duplicates` to ensure that there are no duplicates.
pub struct SortedPropList(Vec<Prop>);
impl SortedPropList {
    /// Create a new `SortedPropList` from a vector of props.
    /// The given `props` doesn't need to be sorted.
    pub fn new(mut props: Vec<Prop>) -> Self {
        props.sort_by(|a, b| Self::cmp_label(&a.label.to_string(), &b.label.to_string()));
        Self(props)
    }

    fn cmp_label(a: &str, b: &str) -> Ordering {
        if a == b {
            Ordering::Equal
        } else if a == CHILDREN_LABEL {
            Ordering::Greater
        } else if b == CHILDREN_LABEL {
            Ordering::Less
        } else {
            a.cmp(b)
        }
    }

    fn position(&self, key: &str) -> Option<usize> {
        self.0
            .binary_search_by(|prop| Self::cmp_label(prop.label.to_string().as_str(), key))
            .ok()
    }

    /// Get the first prop with the given key.
    pub fn get_by_label(&self, key: &str) -> Option<&Prop> {
        self.position(key).and_then(|i| self.0.get(i))
    }

    /// Pop the first prop with the given key.
    pub fn pop(&mut self, key: &str) -> Option<Prop> {
        self.position(key).map(|i| self.0.remove(i))
    }

    /// Pop the prop with the given key and error if there are multiple ones.
    pub fn pop_unique(&mut self, key: &str) -> syn::Result<Option<Prop>> {
        let prop = self.pop(key);
        if prop.is_some() {
            if let Some(other_prop) = self.get_by_label(key) {
                return Err(syn::Error::new_spanned(
                    &other_prop.label,
                    format!("`{}` can only be specified once", key),
                ));
            }
        }

        Ok(prop)
    }

    /// Turn the props into a vector of `Prop`.
    pub fn into_vec(self) -> Vec<Prop> {
        self.0
    }

    /// Iterate over all duplicate props in order of appearance.
    fn iter_duplicates(&self) -> impl Iterator<Item = &Prop> {
        self.0.windows(2).filter_map(|pair| {
            let (a, b) = (&pair[0], &pair[1]);

            if a.label == b.label {
                Some(b)
            } else {
                None
            }
        })
    }

    /// Remove and return all props for which `filter` returns `true`.
    pub fn drain_filter(&mut self, filter: impl FnMut(&Prop) -> bool) -> Self {
        let (drained, others) = self.0.drain(..).partition(filter);
        self.0 = others;
        Self(drained)
    }

    /// Run the given function for all props and aggregate the errors.
    /// If there's at least one error, the result will be `Result::Err`.
    pub fn check_all(&self, f: impl FnMut(&Prop) -> syn::Result<()>) -> syn::Result<()> {
        crate::join_errors(self.0.iter().map(f).filter_map(Result::err))
    }

    /// Return an error for all duplicate props.
    pub fn check_no_duplicates(&self) -> syn::Result<()> {
        crate::join_errors(self.iter_duplicates().map(|prop| {
            syn::Error::new_spanned(
                &prop.label,
                format!(
                    "`{}` can only be specified once but is given here again",
                    prop.label
                ),
            )
        }))
    }
}
impl Parse for SortedPropList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut props: Vec<Prop> = Vec::new();
        // Stop parsing props if a base expression preceded by `..` is reached
        while !input.is_empty() && !input.peek(Token![..]) {
            props.push(input.parse()?);
        }

        Ok(Self::new(props))
    }
}
impl Deref for SortedPropList {
    type Target = [Prop];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct SpecialProps {
    pub node_ref: Option<Prop>,
    pub key: Option<Prop>,
}
impl SpecialProps {
    const REF_LABEL: &'static str = "ref";
    const KEY_LABEL: &'static str = "key";

    fn pop_from(props: &mut SortedPropList) -> syn::Result<Self> {
        let node_ref = props.pop_unique(Self::REF_LABEL)?;
        let key = props.pop_unique(Self::KEY_LABEL)?;
        Ok(Self { node_ref, key })
    }

    fn iter(&self) -> impl Iterator<Item = &Prop> {
        self.node_ref.as_ref().into_iter().chain(self.key.as_ref())
    }

    /// Run the given function for all props and aggregate the errors.
    /// If there's at least one error, the result will be `Result::Err`.
    pub fn check_all(&self, f: impl FnMut(&Prop) -> syn::Result<()>) -> syn::Result<()> {
        crate::join_errors(self.iter().map(f).filter_map(Result::err))
    }
}

pub struct Props {
    pub special: SpecialProps,
    pub prop_list: SortedPropList,
}
impl Parse for Props {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Self::try_from(input.parse::<SortedPropList>()?)
    }
}
impl Deref for Props {
    type Target = SortedPropList;

    fn deref(&self) -> &Self::Target {
        &self.prop_list
    }
}
impl DerefMut for Props {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.prop_list
    }
}

impl TryFrom<SortedPropList> for Props {
    type Error = syn::Error;

    fn try_from(mut prop_list: SortedPropList) -> Result<Self, Self::Error> {
        let special = SpecialProps::pop_from(&mut prop_list)?;
        Ok(Self { special, prop_list })
    }
}
