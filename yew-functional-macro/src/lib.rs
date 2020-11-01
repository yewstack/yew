use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Block, FnArg, Ident, Item, Type};

struct FunctionalComponent {
    body: Block,
    props_type: Type,
}

impl Parse for FunctionalComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {
                let inputs = &func.sig.inputs;
                let props_type = match inputs.len() {
                    // If there is one and only one argument passed, use it for props
                    1 => match inputs.first().unwrap() {
                        FnArg::Typed(arg) => (*arg.ty).clone(),

                        _ => {
                            return Err(syn::Error::new_spanned(&func.sig, "invalid argument passed. (hint: first argument must not be a receiver. Functional components take one argument of props struct as value)"));
                        }
                    },
                    // If nothing is passed then use `()` for props
                    0 => Type::Tuple(syn::TypeTuple {
                        paren_token: Default::default(),
                        elems: Default::default(),
                    }),
                    // Error if more than 1 arguments are passed
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &func.sig,
                            "Functional components take only one argument of props struct as value",
                        ))
                    }
                };

                let body = *func.block;

                Ok(Self { body, props_type })
            }
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`functional_component` can only be applied to functions",
            )),
        }
    }
}

#[proc_macro_attribute]
pub fn functional_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name = Ident::new(
        &["Function", attr.to_string().as_str()].concat(),
        Span::call_site().into(),
    );
    let attr = proc_macro2::TokenStream::from(attr);

    let FunctionalComponent { body, props_type } = parse_macro_input!(item as FunctionalComponent);

    let quoted = quote! {
        pub struct #function_name;

        impl ::yew_functional::FunctionProvider for #function_name {
            type TProps = #props_type;

            fn run(props: &Self::TProps) -> Html {
                #body
            }
        }

        pub type #attr = ::yew_functional::FunctionComponent<#function_name>;
    };

    TokenStream::from(quoted)
}
