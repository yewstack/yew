use crate::typed_vdom::AttributePropDefinition;
use syn::parse_quote;

pub fn global_attributes() -> [AttributePropDefinition; 17] {
    [
        AttributePropDefinition::new(
            parse_quote! { autocapitalize },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { contextmenu },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { contenteditable },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { slot },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { spellcheck },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { class },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { title },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { itemprop },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { accesskey },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { lang },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { id },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { translate },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { draggable },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { style },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { dir },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { tabindex },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { hidden },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
    ]
}
