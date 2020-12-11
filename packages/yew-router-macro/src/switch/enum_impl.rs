use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub use self::{build_route_section::BuildRouteSection, from_route_part::FromRoutePart};

mod build_route_section;
mod from_route_part;

pub struct EnumInner<'a> {
    pub from_route_part: FromRoutePart<'a>,
    pub build_route_section: BuildRouteSection<'a>,
}

impl<'a> ToTokens for EnumInner<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let EnumInner {
            from_route_part,
            build_route_section,
        } = self;
        tokens.extend(quote! {
            #from_route_part
            #build_route_section
        });
    }
}
