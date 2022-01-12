use crate::typed_vdom::{build_fields, build_setters, kw};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Iter;
use syn::spanned::Spanned;
use syn::{Field, Fields, FieldsNamed, ItemStruct, Token, Type};

struct Properties {
    extends_ty: Option<Type>,
    item: ItemStruct,
}

impl ToTokens for Properties {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self.extends_ty {
            Some(_) => self.build_extended_props(),
            None => self.build_non_extended_props(),
        });
    }
}

impl Properties {
    fn builder_ident(&self) -> Ident {
        let ident = &self.item.ident;
        format_ident!("{}Builder", ident, span = ident.span())
    }

    fn build_extended_props(&self) -> TokenStream {
        let ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            ..
        } = &self.item;
        let fields = self.fields();
        let setters = self
            .fields()
            .map(|field| build_setters(field.ident.as_ref().unwrap(), &field.ty, false));
        let extends_ty = &self.extends_ty.as_ref().unwrap();
        let builder_ident = self.builder_ident();

        quote! {
            #(#attrs)*
            #vis #struct_token #ident #generics {
                #(#fields)*,
                __parent: #extends_ty
            }

            impl #ident {
                #(#setters)*
            }

            impl ::std::ops::Deref for #ident {
                type Target = #extends_ty;

                fn deref(&self) -> &Self::Target {
                    &self.__parent
                }
            }

            impl ::std::ops::DerefMut for #ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.__parent
                }
            }

            struct #builder_ident { props: #ident }

            impl #builder_ident {
                fn build(self) -> #ident {
                    self.props
                }
            }

            impl ::yew::Properties for #ident {
                type Builder = #builder_ident;

                fn builder() -> Self::Builder {
                    #builder_ident { props: Self::default() }
                }
            }


            impl ::std::ops::Deref for #builder_ident {
                type Target = #ident;

                fn deref(&self) -> &Self::Target {
                    &self.props
                }
            }

            impl ::std::ops::DerefMut for #builder_ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.props
                }
            }
        }
    }

    fn build_non_extended_props(&self) -> TokenStream {
        let item = &self.item;
        let ident = &item.ident;
        let builder_ident = self.builder_ident();
        let fields = self
            .fields()
            .map(|it| build_fields(it.ident.as_ref().unwrap(), &it.ty, true));
        let setters = self
            .fields()
            .map(|it| build_setters(it.ident.as_ref().unwrap(), &it.ty, true));
        let built_fields = self.fields().map(|field| {
            let ident = field.ident.as_ref().unwrap();
            quote! {
                // todo support unwrap_or{_else}
                #ident: ::std::option::Option::unwrap(self.#ident)
            }
        });

        quote! {
            #item

            impl ::yew::Properties for #ident {
                type Builder = #builder_ident;

                fn builder() -> Self::Builder {
                    ::std::default::Default::default()
                }
            }

            #[derive(::std::default::Default)]
            struct #builder_ident {
                #(#fields)*
            }

            impl #builder_ident {
                #(#setters)*

                fn build(self) -> #ident {
                    #ident {
                        #(#built_fields)*
                    }
                }
            }
        }
    }

    fn fields(&self) -> Iter<Field> {
        match &self.item.fields {
            Fields::Unit | Fields::Unnamed(_) => unreachable!(),
            Fields::Named(FieldsNamed { named, .. }) => named.iter(),
        }
    }
}

struct PropertiesAttrs {
    extend_ty: Option<Type>,
}

impl Parse for PropertiesAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let extend_ty = if input.parse::<kw::extends>().is_ok() {
            let _separator = input.parse::<Token![=]>()?;
            Some(input.parse::<Type>()?)
        } else {
            None
        };
        Ok(Self { extend_ty })
    }
}

pub fn parse_properties(attrs: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let item: ItemStruct = syn::parse2(item)?;
    if matches!(item.fields, Fields::Unit | Fields::Unnamed(_)) {
        return Err(syn::Error::new(
            item.fields.span(),
            "only structs with named fields may be used for props",
        ));
    };

    let attrs = syn::parse2::<PropertiesAttrs>(attrs)?;

    let properties = Properties {
        extends_ty: attrs.extend_ty,
        item,
    };
    Ok(properties.to_token_stream())
}
