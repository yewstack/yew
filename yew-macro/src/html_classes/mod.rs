use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
// TODO: should probably move those classes to module html_classes
use crate::html_tree::html_tag::tag_attributes::{ClassesForm, TagAttributes};
use crate::stringify::Stringify;

/// Same as HtmlRoot but always returns a VNode.
pub struct HtmlClasses(ClassesForm);
impl Parse for HtmlClasses {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(HtmlClasses(TagAttributes::map_classes(input.parse()?)))
    }
}
impl ToTokens for HtmlClasses {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_tokens = match &self.0 {
            ClassesForm::Tuple(classes) => {
                let n = classes.len();
                Some(quote! {
                    let mut __yew_classes = ::yew::virtual_dom::Classes::with_capacity(#n);
                    #(__yew_classes.push(#classes);)*
                    __yew_classes
                })
            }
            ClassesForm::Single(classes) => match classes.try_into_lit() {
                Some(lit) => {
                    if lit.value().is_empty() {
                        None
                    } else {
                        let sr = lit.stringify();
                        Some(quote! {
                            #sr
                        })
                    }
                }
                None => {
                    Some(quote! {
                        ::std::convert::Into::<::yew::virtual_dom::Classes>::into(#classes)
                    })
                }
            },
        };

        if let Some(new_tokens) = new_tokens {
            tokens.extend(quote! {{
                #[allow(clippy::useless_conversion, unused_braces)]
                #new_tokens
            }});
        }
    }
}
