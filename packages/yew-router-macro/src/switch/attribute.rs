use crate::switch::shadow::{ShadowCaptureVariant, ShadowMatcherToken};
use syn::{spanned::Spanned, Attribute, Lit, Meta, MetaNameValue};
use yew_router_route_parser::FieldNamingScheme;

pub enum AttrToken {
    ToOrAt(String),
    End,
    Rest(Option<String>),
}

impl AttrToken {
    pub fn convert_attributes_to_tokens(attributes: Vec<Attribute>) -> syn::Result<Vec<Self>> {
        fn get_meta_name_value_str(mnv: &MetaNameValue) -> syn::Result<String> {
            match &mnv.lit {
                Lit::Str(s) => Ok(s.value()),
                lit => Err(syn::Error::new_spanned(lit, "expected a string literal")),
            }
        }

        attributes
            .iter()
            .filter_map(|attr: &Attribute| attr.parse_meta().ok())
            .filter_map(|meta: Meta| {
                let meta_span = meta.span();
                match meta {
                    Meta::NameValue(mnv) => {
                        mnv.path
                            .get_ident()
                            .and_then(|ident| match ident.to_string().as_str() {
                                "to" | "at" => {
                                    Some(get_meta_name_value_str(&mnv).map(AttrToken::ToOrAt))
                                }
                                "rest" => Some(
                                    get_meta_name_value_str(&mnv).map(|s| AttrToken::Rest(Some(s))),
                                ),
                                _ => None,
                            })
                    }
                    Meta::Path(path) => {
                        path.get_ident()
                            .and_then(|ident| match ident.to_string().as_str() {
                                "end" => Some(Ok(AttrToken::End)),
                                "rest" => Some(Ok(AttrToken::Rest(None))),
                                _ => None,
                            })
                    }
                    Meta::List(list) => {
                        list.path
                            .get_ident()
                            .and_then(|ident| match ident.to_string().as_str() {
                                id @ "to" | id @ "at" | id @ "rest" => Some(Err(syn::Error::new(
                                    meta_span,
                                    &format!(
                                        "This syntax is not supported, did you mean `#[{} = ...]`?",
                                        id
                                    ),
                                ))),
                                _ => None,
                            })
                    }
                }
            })
            .collect()
    }

    /// The id is an unique identifier that allows otherwise unnamed captures to still be captured
    /// with unique names.
    pub fn into_shadow_matcher_tokens(
        self,
        id: usize,
        field_naming_scheme: FieldNamingScheme,
    ) -> Vec<ShadowMatcherToken> {
        match self {
            AttrToken::ToOrAt(matcher_string) => {
                yew_router_route_parser::parse_str_and_optimize_tokens(
                    &matcher_string,
                    field_naming_scheme,
                )
                .expect("Invalid Matcher") // This is the point where users should see an error message if their matcher string has some syntax error.
                .into_iter()
                .map(crate::switch::shadow::ShadowMatcherToken::from)
                .collect()
            }
            AttrToken::End => vec![ShadowMatcherToken::End],
            AttrToken::Rest(Some(capture_name)) => vec![ShadowMatcherToken::Capture(
                ShadowCaptureVariant::ManyNamed(capture_name),
            )],
            AttrToken::Rest(None) => vec![ShadowMatcherToken::Capture(
                ShadowCaptureVariant::ManyNamed(id.to_string()),
            )],
        }
    }
}
