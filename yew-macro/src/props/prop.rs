use crate::html_tree::HtmlDashedName;
use std::cmp::Ordering;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

pub struct Prop {
    pub label: HtmlDashedName,
    pub question_mark: Option<Token![?]>,
    pub value: Expr,
}
impl Prop {
    /// Checks if the prop uses the optional attribute syntax.
    /// If it does, an error is returned.
    pub fn ensure_not_optional(&self) -> syn::Result<()> {
        if self.question_mark.is_some() {
            let msg = format!(
                "the `{}` attribute does not support being used as an optional attribute",
                self.label
            );
            Err(syn::Error::new_spanned(&self.label, msg))
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
                "this prop doesn't have a value. \
                          Set the value to `true` or `false` for boolean attributes",
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
            value,
        })
    }
}

/// Always sorted by label. Duplicates are allowed.
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

    fn find(&self, key: &str) -> Option<usize> {
        self.0
            .binary_search_by(|prop| Self::cmp_label(prop.label.to_string().as_str(), key))
            .ok()
    }

    fn pop(&mut self, key: &str) -> Option<Prop> {
        self.find(key).map(|i| self.0.remove(i))
    }

    fn pop_unique(&mut self, key: &str) -> syn::Result<Option<Prop>> {
        let prop = self.pop(key);
        if prop.is_some() {
            if let Some(other_prop) = self.pop(key) {
                return Err(syn::Error::new_spanned(
                    other_prop.label,
                    &format!("`{}` can only be set once", key),
                ));
            }
        }

        Ok(prop)
    }

    pub fn into_inner(self) -> Vec<Prop> {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<Prop> {
        self.0.iter()
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

#[derive(Default)]
pub struct SpecialProps {
    pub node_ref: Option<Prop>,
    pub key: Option<Prop>,
}
impl SpecialProps {
    const REF_LABEL: &'static str = "ref";
    const KEY_LABEL: &'static str = "key";

    fn pop_from(props: &mut PropList) -> syn::Result<Self> {
        let node_ref = props.pop_unique(Self::REF_LABEL)?;
        let key = props.pop_unique(Self::KEY_LABEL)?;
        Ok(Self { node_ref, key })
    }

    pub fn get_slot_mut(&mut self, key: &str) -> Option<&mut Option<Prop>> {
        match key {
            Self::REF_LABEL => Some(&mut self.node_ref),
            Self::KEY_LABEL => Some(&mut self.key),
            _ => None,
        }
    }
}

pub struct Props {
    pub special: SpecialProps,
    pub props: PropList,
}
impl Parse for Props {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut props = input.parse::<PropList>()?;
        let special = SpecialProps::pop_from(&mut props)?;

        Ok(Self { special, props })
    }
}
