use crate::html_tree::HtmlDashedName;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

pub struct HtmlProp {
    pub label: HtmlDashedName,
    pub question_mark: Option<Token![?]>,
    pub value: Expr,
}
impl HtmlProp {
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
impl Parse for HtmlProp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let label = input.parse::<HtmlDashedName>()?;
        let question_mark = input.parse::<Token![?]>().ok();
        let equals = input.parse::<Token![=]>().map_err(|_| {
            syn::Error::new_spanned(
                &label,
                "this prop doesn't have a value. \
                          In case of boolean attributes, set the value to `true` or `false` \
                          to control whether or not it will be present",
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

pub struct HtmlPropList(Vec<HtmlProp>);
impl HtmlPropList {
    pub fn iter(&self) -> std::slice::Iter<HtmlProp> {
        self.0.iter()
    }
}
impl Parse for HtmlPropList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut props: Vec<HtmlProp> = Vec::new();

        while !input.is_empty() {
            props.push(input.parse()?);
        }

        props.sort_by(|a, b| a.label.to_string().cmp(&b.label.to_string()));

        Ok(Self(props))
    }
}
