use crate::html_tree::HtmlDashedName;
use quote::quote;
use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

pub struct Prop {
    pub label: HtmlDashedName,
    pub question_mark: Option<Token![?]>,
    pub equals: Token![=],
    pub value: Expr,
}
impl Prop {
    /// Checks if the prop uses the optional attribute syntax.
    /// If it does, an error is returned.
    pub fn ensure_not_optional(&self) -> syn::Result<()> {
        let Self {
            label,
            question_mark,
            equals,
            ..
        } = self;
        if question_mark.is_some() {
            let msg = format!(
                "`{}` does not support being used as an optional attribute",
                label
            );
            // include `?=` in the span
            Err(syn::Error::new_spanned(quote! {#label#equals}, msg))
        } else {
            Ok(())
        }
    }
}
impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let label = input.parse::<HtmlDashedName>()?;
        let question_mark = input.parse::<Token![?]>().ok();
        let equals = input.parse::<Token![=]>().map_err(|_| {
            syn::Error::new_spanned(
                &label,
                format!("`{}` doesn't have a value. Set the value to `true` or `false` for boolean attributes", label),
            )
        })?;
        if input.is_empty() {
            return Err(syn::Error::new_spanned(
                equals,
                "expected an expression following this equals sign",
            ));
        }
        let value = input.parse::<Expr>()?;
        Ok(Self {
            label,
            question_mark,
            equals,
            value,
        })
    }
}

/// List of props.
pub struct PropList(Vec<Prop>);
impl PropList {
    const CHILDREN_LABEL: &'static str = "children";

    fn cmp_label(a: &str, b: &str) -> Ordering {
        if a == b {
            Ordering::Equal
        } else if a == Self::CHILDREN_LABEL {
            Ordering::Greater
        } else if b == Self::CHILDREN_LABEL {
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

    pub fn pop(&mut self, key: &str) -> Option<Prop> {
        self.position(key).map(|i| self.0.remove(i))
    }

    /// Pop the prop with the given key and error if there are multiple ones.
    fn pop_unique(&mut self, key: &str) -> syn::Result<Option<Prop>> {
        let prop = self.pop(key);
        if prop.is_some() {
            if let Some(other_prop) = self.pop(key) {
                return Err(syn::Error::new_spanned(
                    other_prop.label,
                    format!("`{}` can only be set once", key),
                ));
            }
        }

        Ok(prop)
    }

    pub fn pop_unique_nonoptional(&mut self, key: &str) -> syn::Result<Option<Prop>> {
        match self.pop_unique(key) {
            Ok(Some(prop)) => {
                if prop.question_mark.is_some() {
                    let label = &prop.label;
                    Err(syn::Error::new_spanned(
                        label,
                        format!("`{}` can not be optional", label),
                    ))
                } else {
                    Ok(Some(prop))
                }
            }
            res => res,
        }
    }

    pub fn into_inner(self) -> Vec<Prop> {
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

    pub fn drain_filter(&mut self, filter: impl FnMut(&Prop) -> bool) -> PropList {
        let (drained, others) = self.0.drain(..).partition(filter);
        self.0 = others;
        PropList(drained)
    }

    /// Run the given function for all props and aggregate the errors.
    /// If there's at least one error, the result will be `Result::Err`.
    pub fn check_all(&self, f: impl FnMut(&Prop) -> syn::Result<()>) -> syn::Result<()> {
        crate::join_errors(self.0.iter().map(f).filter_map(Result::err))
    }

    pub fn error_if_duplicates(&self) -> syn::Result<()> {
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
impl Parse for PropList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut props: Vec<Prop> = Vec::new();

        while !input.is_empty() {
            props.push(input.parse()?);
        }

        props.sort_by(|a, b| Self::cmp_label(&a.label.to_string(), &b.label.to_string()));

        Ok(Self(props))
    }
}
impl Deref for PropList {
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

    fn pop_from(props: &mut PropList) -> syn::Result<Self> {
        let node_ref = props.pop_unique_nonoptional(Self::REF_LABEL)?;
        let key = props.pop_unique_nonoptional(Self::KEY_LABEL)?;
        Ok(Self { node_ref, key })
    }

    pub fn get_slot_mut(&mut self, key: &str) -> Option<&mut Option<Prop>> {
        match key {
            Self::REF_LABEL => Some(&mut self.node_ref),
            Self::KEY_LABEL => Some(&mut self.key),
            _ => None,
        }
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
    pub prop_list: PropList,
}
impl Parse for Props {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut prop_list = input.parse::<PropList>()?;
        let special = SpecialProps::pop_from(&mut prop_list)?;

        Ok(Self { special, prop_list })
    }
}
impl Deref for Props {
    type Target = PropList;

    fn deref(&self) -> &Self::Target {
        &self.prop_list
    }
}
impl DerefMut for Props {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.prop_list
    }
}
