use crate::switch::SwitchItem;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Field, Fields, Type};

pub struct FromRoutePart<'a> {
    pub switch_variants: &'a [SwitchItem],
    pub enum_ident: &'a Ident,
}

impl<'a> ToTokens for FromRoutePart<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_matchers = self.switch_variants.iter().map(|sv| {
            let SwitchItem {
                matcher,
                ident,
                fields,
            } = sv;
            let build_from_captures = build_variant_from_captures(&self.enum_ident, ident, fields);
            let matcher = super::super::build_matcher_from_tokens(&matcher);

            quote! {
                #matcher
                #build_from_captures
            }
        });

        tokens.extend(quote!{
            fn from_route_part<__T>(route: String, mut state: Option<__T>) -> (::std::option::Option<Self>, ::std::option::Option<__T>) {
                let route_string = route;
                #(#variant_matchers)*

                (::std::option::Option::None, state)
            }
        });
    }
}

/// Once the 'captures' exists, attempt to populate the fields from the list of captures.
fn build_variant_from_captures(
    enum_ident: &Ident,
    variant_ident: &Ident,
    fields: &Fields,
) -> TokenStream {
    match fields {
        Fields::Named(named_fields) => {
            let (field_declarations, fields): (Vec<_>, Vec<_>) = named_fields
                .named
                .iter()
                .filter_map(|field: &Field| {
                    let field_ty: &Type = &field.ty;
                    field.ident.as_ref().map(|i: &Ident| {
                        let key = i.to_string();
                        (i, key, field_ty)
                    })
                })
                .map(|(field_name, key, field_ty): (&Ident, String, &Type)| {
                    let field_decl = quote! {
                        let #field_name = {
                            let (v, s) = match captures.remove(#key) {
                                ::std::option::Option::Some(value) => {
                                    <#field_ty as ::yew_router::Switch>::from_route_part(
                                        value,
                                        state,
                                    )
                                }
                                ::std::option::Option::None => {
                                    (
                                        <#field_ty as ::yew_router::Switch>::key_not_available(),
                                        state,
                                    )
                                }
                            };
                            match v {
                                ::std::option::Option::Some(val) => {
                                    state = s; // Set state for the next var.
                                    val
                                },
                                ::std::option::Option::None => return (None, s) // Failed
                            }
                        };
                    };

                    (field_decl, field_name)
                })
                .unzip();

            quote! {
                let mut state = if let ::std::option::Option::Some(mut captures) = matcher
                    .capture_route_into_map(&route_string)
                    .ok()
                    .map(|x| x.1)
                {
                    let create_item = || {
                        #(#field_declarations)*

                        let val = ::std::option::Option::Some(
                            #enum_ident::#variant_ident {
                                #(#fields),*
                            }
                        );

                        (val, state)
                    };
                    let (val, state) = create_item();

                    if val.is_some() {
                        return (val, state);
                    }
                    state
                } else {
                    state
                };
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            let (field_declarations, fields): (Vec<_>, Vec<_>) = unnamed_fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, f)| {
                    let field_ty = &f.ty;
                    let field_var_name = Ident::new(&format!("field_{}", idx), Span::call_site());
                    let field_decl = quote! {
                        let #field_var_name = {
                            let (v, s) = match drain.next() {
                                ::std::option::Option::Some(value) => {
                                    <#field_ty as ::yew_router::Switch>::from_route_part(
                                        value,
                                        state,
                                    )
                                },
                                ::std::option::Option::None => {
                                    (
                                        <#field_ty as ::yew_router::Switch>::key_not_available(),
                                        state,
                                    )
                                }
                            };
                            match v {
                                ::std::option::Option::Some(val) => {
                                    state = s; // Set state for the next var.
                                    val
                                },
                                ::std::option::Option::None => return (None, s) // Failed
                            }
                        };
                    };

                    (field_decl, field_var_name)
                })
                .unzip();

            quote! {
                let mut state = if let ::std::option::Option::Some(mut captures) = matcher
                    .capture_route_into_vec(&route_string)
                    .ok()
                    .map(|x| x.1)
                {
                    let mut drain = captures.drain(..);
                    let create_item = || {
                        #(#field_declarations)*

                        (
                            ::std::option::Option::Some(
                                #enum_ident::#variant_ident(
                                    #(#fields),*
                                )
                            ),
                            state
                        )
                    };
                    let (val, state) = create_item();
                    if val.is_some() {
                        return (val, state);
                    }
                    state
                } else {
                    state
                };
            }
        }
        Fields::Unit => {
            quote! {
                let mut state = if let ::std::option::Option::Some(_captures) = matcher.capture_route_into_map(&route_string).ok().map(|x| x.1) {
                    return (::std::option::Option::Some(#enum_ident::#variant_ident), state);
                } else {
                    state
                };
            }
        }
    }
}
