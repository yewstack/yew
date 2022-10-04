use boolinator::Boolinator;
use proc_macro2::{Delimiter, Span, TokenStream};
use proc_macro_error::emit_warning;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Block, Expr, Ident, Lit, LitStr, Token};

use super::{HtmlChildrenTree, HtmlDashedName, TagTokens};
use crate::props::{ClassesForm, ElementProps, Prop, PropDirective};
use crate::stringify::{Stringify, Value};
use crate::{non_capitalized_ascii, Peek, PeekValue};

pub struct HtmlElement {
    pub name: TagName,
    pub props: ElementProps,
    pub children: HtmlChildrenTree,
}

impl PeekValue<()> for HtmlElement {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlElementOpen::peek(cursor)
            .or_else(|| HtmlElementClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HtmlElementClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlElementClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close.to_spanned(),
                    "this closing tag has no corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlElementOpen>()?;
        // Return early if it's a self-closing tag
        if open.is_self_closing() {
            return Ok(HtmlElement {
                name: open.name,
                props: open.props,
                children: HtmlChildrenTree::new(),
            });
        }

        if let TagName::Lit(name) = &open.name {
            // Void elements should not have children.
            // See https://html.spec.whatwg.org/multipage/syntax.html#void-elements
            //
            // For dynamic tags this is done at runtime!
            match name.to_ascii_lowercase_string().as_str() {
                "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                | "meta" | "param" | "source" | "track" | "wbr" => {
                    return Err(syn::Error::new_spanned(
                        open.to_spanned(),
                        format!(
                            "the tag `<{}>` is a void element and cannot have children (hint: \
                             rewrite this as `<{0}/>`)",
                            name
                        ),
                    ));
                }
                _ => {}
            }
        }

        let open_key = open.name.get_key();
        let mut children = HtmlChildrenTree::new();
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open.to_spanned(),
                    "this opening tag has no corresponding closing tag",
                ));
            }
            if let Some(close_key) = HtmlElementClose::peek(input.cursor()) {
                if open_key == close_key {
                    break;
                }
            }

            children.parse_child(input)?;
        }

        input.parse::<HtmlElementClose>()?;

        Ok(Self {
            name: open.name,
            props: open.props,
            children,
        })
    }
}

impl ToTokens for HtmlElement {
    #[allow(clippy::cognitive_complexity)]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            props,
            children,
        } = self;

        let ElementProps {
            classes,
            attributes,
            booleans,
            value,
            checked,
            listeners,
            special,
        } = &props;

        // attributes with special treatment

        let node_ref = special.wrap_node_ref_attr();
        let key = special.wrap_key_attr();
        let value = value
            .as_ref()
            .map(|prop| wrap_attr_value(prop.value.optimize_literals()))
            .unwrap_or(quote! { ::std::option::Option::None });
        let checked = checked
            .as_ref()
            .map(|attr| {
                let value = &attr.value;
                quote! { ::std::option::Option::Some( #value ) }
            })
            .unwrap_or(quote! { ::std::option::Option::None });

        // other attributes

        let attributes = {
            let normal_attrs = attributes.iter().map(
                |Prop {
                     label,
                     value,
                     directive,
                     ..
                 }| {
                    (
                        label.to_lit_str(),
                        value.optimize_literals_tagged(),
                        *directive,
                    )
                },
            );
            let boolean_attrs = booleans.iter().filter_map(
                |Prop {
                     label,
                     value,
                     directive,
                     ..
                 }| {
                    let key = label.to_lit_str();
                    Some((
                        key.clone(),
                        match value {
                            Expr::Lit(e) => match &e.lit {
                                Lit::Bool(b) => Value::Static(if b.value {
                                    quote! { #key }
                                } else {
                                    return None;
                                }),
                                _ => Value::Dynamic(quote_spanned! {value.span()=> {
                                    ::yew::utils::__ensure_type::<::std::primitive::bool>(#value);
                                    #key
                                }}),
                            },
                            expr => Value::Dynamic(
                                quote_spanned! {expr.span().resolved_at(Span::call_site())=>
                                    if #expr {
                                        ::std::option::Option::Some(
                                            ::yew::virtual_dom::AttrValue::Static(#key)
                                        )
                                    } else {
                                        ::std::option::Option::None
                                    }
                                },
                            ),
                        },
                        *directive,
                    ))
                },
            );
            let class_attr = classes.as_ref().and_then(|classes| match classes {
                ClassesForm::Tuple(classes) => {
                    let span = classes.span();
                    let classes: Vec<_> = classes.elems.iter().collect();
                    let n = classes.len();

                    let deprecation_warning = quote_spanned! {span=>
                        #[deprecated(
                            note = "the use of `(...)` with the attribute `class` is deprecated and will be removed in version 0.19. Use the `classes!` macro instead."
                        )]
                        fn deprecated_use_of_class() {}

                        if false {
                            deprecated_use_of_class();
                        };
                    };

                    Some((
                        LitStr::new("class", span),
                        Value::Dynamic(quote! {
                            {
                                #deprecation_warning

                                let mut __yew_classes = ::yew::html::Classes::with_capacity(#n);
                                #(__yew_classes.push(#classes);)*
                                __yew_classes
                            }
                        }),
                        None,
                    ))
                }
                ClassesForm::Single(classes) => {
                    match classes.try_into_lit() {
                        Some(lit) => {
                            if lit.value().is_empty() {
                                None
                            } else {
                                Some((
                                    LitStr::new("class", lit.span()),
                                    Value::Static(quote! { #lit }),
                                    None,
                                ))
                            }
                        }
                        None => {
                            Some((
                                LitStr::new("class", classes.span()),
                                Value::Dynamic(quote! {
                                    ::std::convert::Into::<::yew::html::Classes>::into(#classes)
                                }),
                                None,
                            ))
                        }
                    }
                }
            });

            fn apply_as(directive: Option<&PropDirective>) -> TokenStream {
                match directive {
                    Some(PropDirective::ApplyAsProperty(token)) => {
                        quote_spanned!(token.span()=> ::yew::virtual_dom::ApplyAttributeAs::Property)
                    }
                    None => quote!(::yew::virtual_dom::ApplyAttributeAs::Attribute),
                }
            }

            /// Try to turn attribute list into a `::yew::virtual_dom::Attributes::Static`
            fn try_into_static(
                src: &[(LitStr, Value, Option<PropDirective>)],
            ) -> Option<TokenStream> {
                let mut kv = Vec::with_capacity(src.len());
                for (k, v, directive) in src.iter() {
                    let v = match v {
                        Value::Static(v) => quote! { #v },
                        Value::Dynamic(_) => return None,
                    };
                    let apply_as = apply_as(directive.as_ref());
                    kv.push(quote! { ( #k, #v, #apply_as ) });
                }

                Some(quote! { ::yew::virtual_dom::Attributes::Static(&[#(#kv),*]) })
            }

            let attrs = normal_attrs
                .chain(boolean_attrs)
                .chain(class_attr)
                .collect::<Vec<(LitStr, Value, Option<PropDirective>)>>();
            try_into_static(&attrs).unwrap_or_else(|| {
                let keys = attrs.iter().map(|(k, ..)| quote! { #k });
                let values = attrs.iter().map(|(_, v, directive)| {
                    let apply_as = apply_as(directive.as_ref());
                    let value = wrap_attr_value(v);
                    quote! { ::std::option::Option::map(#value, |it| (it, #apply_as)) }
                });
                quote! {
                    ::yew::virtual_dom::Attributes::Dynamic{
                        keys: &[#(#keys),*],
                        values: ::std::boxed::Box::new([#(#values),*]),
                    }
                }
            })
        };

        let listeners = if listeners.is_empty() {
            quote! { ::yew::virtual_dom::listeners::Listeners::None }
        } else {
            let listeners_it = listeners.iter().map(|Prop { label, value, .. }| {
                let name = &label.name;
                quote! {
                    ::yew::html::#name::Wrapper::__macro_new(#value)
                }
            });

            quote! {
                ::yew::virtual_dom::listeners::Listeners::Pending(
                    ::std::boxed::Box::new([#(#listeners_it),*])
                )
            }
        };

        // TODO: if none of the children have possibly None expressions or literals as keys, we can
        // compute `VList.fully_keyed` at compile time.
        let child_list = quote! {
            ::yew::virtual_dom::VList::with_children(
                #children,
                ::std::option::Option::None,
            )
        };

        tokens.extend(match &name {
            TagName::Lit(dashedname) => {
                let name_span = dashedname.span();
                let name = dashedname.to_ascii_lowercase_string();
                if name != dashedname.to_string() {
                    emit_warning!(
                        dashedname.span(),
                        format!(
                            "The tag '{0}' is not matching its normalized form '{1}'. If you want \
                             to keep this form, change this to a dynamic tag `@{{\"{0}\"}}`.",
                            dashedname,
                            name,
                        )
                    )
                }
                let node = match &*name {
                    "input" => {
                        quote! {
                            ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                                ::yew::virtual_dom::VTag::__new_input(
                                    #value,
                                    #checked,
                                    #node_ref,
                                    #key,
                                    #attributes,
                                    #listeners,
                                ),
                            )
                        }
                    }
                    "textarea" => {
                        quote! {
                            ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                                ::yew::virtual_dom::VTag::__new_textarea(
                                    #value,
                                    #node_ref,
                                    #key,
                                    #attributes,
                                    #listeners,
                                ),
                            )
                        }
                    }
                    _ => {
                        quote! {
                            ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                                ::yew::virtual_dom::VTag::__new_other(
                                    ::std::borrow::Cow::<'static, ::std::primitive::str>::Borrowed(#name),
                                    #node_ref,
                                    #key,
                                    #attributes,
                                    #listeners,
                                    #child_list,
                                ),
                            )
                        }
                    }
                };
                // the return value can be inlined without the braces when this is stable:
                // https://github.com/rust-lang/rust/issues/15701
                quote_spanned!{
                    name_span =>
                    {
                        #[allow(clippy::redundant_clone, unused_braces)]
                        let node = #node;
                        node
                    }
                }
            }
            TagName::Expr(name) => {
                let vtag = Ident::new("__yew_vtag", name.span());
                let expr = &name.expr;
                let vtag_name = Ident::new("__yew_vtag_name", expr.span());

                // handle special attribute value
                let handle_value_attr = props.value.as_ref().map(|prop| {
                    let v = prop.value.optimize_literals();
                    quote_spanned! {v.span()=> {
                        __yew_vtag.__macro_push_attr("value", #v);
                    }}
                });

                #[cfg(nightly_yew)]
                let invalid_void_tag_msg_start = {
                    let span = vtag.span().unwrap();
                    let source_file = span.source_file().path();
                    let source_file = source_file.display();
                    let start = span.start();
                    format!("[{}:{}:{}] ", source_file, start.line, start.column)
                };

                #[cfg(not(nightly_yew))]
                let invalid_void_tag_msg_start = "";

                // this way we get a nice error message (with the correct span) when the expression
                // doesn't return a valid value
                quote_spanned! {expr.span()=> {
                    #[allow(unused_braces)]
                    // e.g. html!{<@{"div"}/>} will set `#expr` to `{"div"}`
                    // (note the extra braces). Hence the need for the `allow`.
                    // Anyways to remove the braces?
                    let mut #vtag_name = ::std::convert::Into::<
                        ::std::borrow::Cow::<'static, ::std::primitive::str>
                    >::into(#expr);
                    ::std::debug_assert!(
                        #vtag_name.is_ascii(),
                        "a dynamic tag returned a tag name containing non ASCII characters: `{}`",
                        #vtag_name,
                    );

                    #[allow(clippy::redundant_clone, unused_braces, clippy::let_and_return)]
                    let mut #vtag = match () {
                        _ if "input".eq_ignore_ascii_case(::std::convert::AsRef::<::std::primitive::str>::as_ref(&#vtag_name)) => {
                            ::yew::virtual_dom::VTag::__new_textarea(
                                #value,
                                #node_ref,
                                #key,
                                #attributes,
                                #listeners,
                            )
                        }
                        _ if "textarea".eq_ignore_ascii_case(::std::convert::AsRef::<::std::primitive::str>::as_ref(&#vtag_name)) => {
                            ::yew::virtual_dom::VTag::__new_textarea(
                                #value,
                                #node_ref,
                                #key,
                                #attributes,
                                #listeners,
                            )
                        }
                        _ => {
                            let mut __yew_vtag = ::yew::virtual_dom::VTag::__new_other(
                                #vtag_name,
                                #node_ref,
                                #key,
                                #attributes,
                                #listeners,
                                #child_list,
                            );

                            #handle_value_attr

                            __yew_vtag
                        }
                    };

                    // These are the runtime-checks exclusive to dynamic tags.
                    // For literal tags this is already done at compile-time.
                    //
                    // check void element
                    if !#vtag.children().is_empty() {
                        ::std::debug_assert!(
                            !::std::matches!(#vtag.tag().to_ascii_lowercase().as_str(),
                                "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input"
                                    | "link" | "meta" | "param" | "source" | "track" | "wbr"
                            ),
                            concat!(#invalid_void_tag_msg_start, "a dynamic tag tried to create a `<{0}>` tag with children. `<{0}>` is a void element which can't have any children."),
                            #vtag.tag(),
                        );
                    }

                    ::std::convert::Into::<::yew::virtual_dom::VNode>::into(#vtag)
                }}
            }
        });
    }
}

fn wrap_attr_value<T: ToTokens>(value: T) -> TokenStream {
    quote_spanned! {value.span()=>
        ::yew::html::IntoPropValue::<
            ::std::option::Option<
                ::yew::virtual_dom::AttrValue
            >
        >
        ::into_prop_value(#value)
    }
}

pub struct DynamicName {
    at: Token![@],
    expr: Option<Block>,
}

impl Peek<'_, ()> for DynamicName {
    fn peek(cursor: Cursor) -> Option<((), Cursor)> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '@').as_option()?;

        // move cursor past block if there is one
        let cursor = cursor
            .group(Delimiter::Brace)
            .map(|(_, _, cursor)| cursor)
            .unwrap_or(cursor);

        Some(((), cursor))
    }
}

impl Parse for DynamicName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let at = input.parse()?;
        // the expression block is optional, closing tags don't have it.
        let expr = if input.cursor().group(Delimiter::Brace).is_some() {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { at, expr })
    }
}

impl ToTokens for DynamicName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { at, expr } = self;
        tokens.extend(quote! {#at #expr});
    }
}

#[derive(PartialEq)]
enum TagKey {
    Lit(HtmlDashedName),
    Expr,
}

pub enum TagName {
    Lit(HtmlDashedName),
    Expr(DynamicName),
}

impl TagName {
    fn get_key(&self) -> TagKey {
        match self {
            TagName::Lit(name) => TagKey::Lit(name.clone()),
            TagName::Expr(_) => TagKey::Expr,
        }
    }
}

impl Peek<'_, TagKey> for TagName {
    fn peek(cursor: Cursor) -> Option<(TagKey, Cursor)> {
        if let Some((_, cursor)) = DynamicName::peek(cursor) {
            Some((TagKey::Expr, cursor))
        } else {
            HtmlDashedName::peek(cursor).map(|(name, cursor)| (TagKey::Lit(name), cursor))
        }
    }
}

impl Parse for TagName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if DynamicName::peek(input.cursor()).is_some() {
            DynamicName::parse(input).map(Self::Expr)
        } else {
            HtmlDashedName::parse(input).map(Self::Lit)
        }
    }
}

impl ToTokens for TagName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TagName::Lit(name) => name.to_tokens(tokens),
            TagName::Expr(name) => name.to_tokens(tokens),
        }
    }
}

struct HtmlElementOpen {
    tag: TagTokens,
    name: TagName,
    props: ElementProps,
}
impl HtmlElementOpen {
    fn is_self_closing(&self) -> bool {
        self.tag.div.is_some()
    }

    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<TagKey> for HtmlElementOpen {
    fn peek(cursor: Cursor) -> Option<TagKey> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (tag_key, cursor) = TagName::peek(cursor)?;
        if let TagKey::Lit(name) = &tag_key {
            // Avoid parsing `<key=[...]>` as an element. It needs to be parsed as an `HtmlList`.
            if name.to_string() == "key" {
                let (punct, _) = cursor.punct()?;
                // ... unless it isn't followed by a '='. `<key></key>` is a valid element!
                (punct.as_char() != '=').as_option()?;
            } else {
                non_capitalized_ascii(&name.to_string()).as_option()?;
            }
        }

        Some(tag_key)
    }
}

impl Parse for HtmlElementOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_start_content(input, |input, tag| {
            let name = input.parse::<TagName>()?;
            let mut props = input.parse::<ElementProps>()?;

            match &name {
                TagName::Lit(name) => {
                    // Don't treat value as special for non input / textarea fields
                    // For dynamic tags this is done at runtime!
                    match name.to_ascii_lowercase_string().as_str() {
                        "input" | "textarea" => {}
                        _ => {
                            if let Some(attr) = props.value.take() {
                                props.attributes.push(attr);
                            }
                        }
                    }
                }
                TagName::Expr(name) => {
                    if name.expr.is_none() {
                        return Err(syn::Error::new_spanned(
                            name,
                            "this dynamic tag is missing an expression block defining its value",
                        ));
                    }
                }
            }

            Ok(Self { tag, name, props })
        })
    }
}

struct HtmlElementClose {
    tag: TagTokens,
    _name: TagName,
}
impl HtmlElementClose {
    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<TagKey> for HtmlElementClose {
    fn peek(cursor: Cursor) -> Option<TagKey> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (tag_key, cursor) = TagName::peek(cursor)?;
        if let TagKey::Lit(name) = &tag_key {
            non_capitalized_ascii(&name.to_string()).as_option()?;
        }

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(tag_key)
    }
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_end_content(input, |input, tag| {
            let name = input.parse()?;

            if let TagName::Expr(name) = &name {
                if let Some(expr) = &name.expr {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "dynamic closing tags must not have a body (hint: replace it with just \
                         `</@>`)",
                    ));
                }
            }

            Ok(Self { tag, _name: name })
        })
    }
}
