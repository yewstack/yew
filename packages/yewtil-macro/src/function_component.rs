use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::token;
use syn::Token;
use syn::{braced, parenthesized};
use syn::{Block, Error, Field, Stmt, Type, VisPublic, Visibility};

pub fn function_component_handler(attr: TokenStream, item: TokenStream1) -> TokenStream1 {
    let component_name = attr.to_string();
    assert!(
        !component_name.is_empty(),
        "you must provide a component name. eg: function_component(MyComponent)"
    );
    let component_name = Ident::new(&component_name, Span::call_site());
    let function = parse_macro_input!(item as Function);
    TokenStream1::from(
        FunctionComponentInfo {
            component_name,
            function,
        }
        .to_token_stream(),
    )
}

pub struct FunctionComponentInfo {
    component_name: Ident,
    function: Function,
}

// TODO, support type parameters

pub struct Function {
    pub vis: Visibility,
    pub fn_token: Token![fn],
    pub name: Ident,
    pub paren_token: token::Paren,
    pub fields: Punctuated<Field, Token![,]>,
    pub returns_token: Token![->],
    pub return_ty: Ident,
    pub brace_token: token::Brace,
    pub body: Vec<Stmt>,
}

impl Parse for Function {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let vis = input.parse()?;
        let fn_token = input.parse()?;
        let name = input.parse()?;
        let content;
        let paren_token = parenthesized!(content in input);
        let returns_token = input.parse()?;
        let return_ty = input.parse()?;
        let content2;
        let brace_token = braced!(content2 in input);
        Ok(Function {
            vis,
            fn_token,
            name,
            paren_token,
            fields: content.parse_terminated(Field::parse_named)?,
            returns_token,
            return_ty,
            brace_token,
            body: content2.call(Block::parse_within)?,
        })
    }
}

impl ToTokens for Function {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Function {
            vis,
            fn_token,
            name,
            fields,
            returns_token,
            return_ty,
            body,
            ..
        } = self;
        let fields = fields
            .iter()
            .map(|field: &Field| {
                let mut new_field: Field = field.clone();
                new_field.attrs = vec![];
                new_field
            })
            .collect::<Punctuated<_, Token![,]>>();

        tokens.extend(quote! {
            #vis #fn_token #name(#fields) #returns_token #return_ty {
                #(#body)*
            }
        })
    }
}

impl ToTokens for FunctionComponentInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FunctionComponentInfo {
            component_name,
            function,
        } = self;
        // The function tokens must be re-generated in order to strip the attributes that are not allowed.
        let function_token_stream = function.to_token_stream();
        let Function {
            vis, name, fields, ..
        } = function;

        let impl_name = format!("FuncComp{}", component_name.to_string());
        let impl_name = Ident::new(&impl_name, Span::call_site());

        let alias = quote! {
            #vis type #component_name = ::yewtil::Pure<#impl_name>;
        };

        // Set the fields to be public and strips references as necessary.
        // This will preserve attributes like #[props(required)], which will appear in the generated struct below.
        let new_fields = fields
            .iter()
            .map(|field: &Field| {
                let mut new_field: Field = field.clone();
                let visibility = Visibility::Public(VisPublic {
                    pub_token: syn::token::Pub {
                        span: Span::call_site(),
                    },
                });
                // Strip references so the component can have a static lifetime.
                // TODO Handle 'static lifetimes gracefully here - allowing &'static strings instead of erroneously converting them to plain strs.
                let ty = match &field.ty {
                    Type::Reference(x) => {
                        let elem = x.elem.clone();
                        Type::Verbatim(quote! {
                            #elem
                        })
                    }
                    x => x.clone(),
                };
                new_field.vis = visibility;
                new_field.ty = ty;
                new_field
            })
            .collect::<Punctuated<_, Token![,]>>();

        let component_struct = quote! {
            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::yew::Properties)]
            #vis struct #impl_name {
                #new_fields
            }
        };

        let arguments = fields
            .iter()
            .zip(new_fields.iter())
            .map(|(field, new_field): (&Field, &Field)| {
                let field_name = field.ident.as_ref().expect("Field must have name");

                // If the fields differ, then a reference was removed from the function's field's type
                // to make it static.
                // Otherwise it is assumed that the type is not a reference on the function and it
                // implements clone, and that when calling the function, the type should be cloned again.
                if field.ty != new_field.ty {
                    quote! {
                        &self.#field_name
                    }
                } else {
                    quote! {
                        self.#field_name.clone()
                    }
                }
            })
            .collect::<Punctuated<_, Token![,]>>();

        let pure_component_impl = quote! {
            impl ::yewtil::PureComponent for #impl_name {
                fn render(&self) -> ::yew::Html {
                     #name(#arguments)
                }
            }
        };

        tokens.extend(quote! {
            #function_token_stream
            #alias
            #component_struct
            #pure_component_impl
        })
    }
}
