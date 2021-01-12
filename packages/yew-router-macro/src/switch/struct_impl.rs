pub use self::{build_route_section::BuildRouteSection, from_route_part::FromRoutePart};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

mod build_route_section;
mod from_route_part;

pub struct StructInner<'a> {
    pub from_route_part: FromRoutePart<'a>,
    pub build_route_section: BuildRouteSection<'a>,
}

impl<'a> ToTokens for StructInner<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StructInner {
            from_route_part,
            build_route_section,
        } = self;
        tokens.extend(quote! {
             #from_route_part
             #build_route_section
        })
    }
}
