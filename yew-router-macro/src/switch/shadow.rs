use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use yew_router_route_parser::{CaptureVariant, MatcherToken};

impl ToTokens for ShadowMatcherToken {
    fn to_tokens(&self, ts: &mut TokenStream) {
        use ShadowMatcherToken as SOT;
        let t: TokenStream = match self {
            SOT::Exact(s) => quote! {
                ::yew_router::matcher::MatcherToken::Exact(#s.to_string())
            },
            SOT::Capture(variant) => quote! {
                ::yew_router::matcher::MatcherToken::Capture(#variant)
            },
            SOT::End => quote! {
                ::yew_router::matcher::MatcherToken::End
            },
        };
        ts.extend(t)
    }
}

/// A shadow of the OptimizedToken type.
/// It should match it exactly so that this macro can expand to the original.
pub enum ShadowMatcherToken {
    Exact(String),
    Capture(ShadowCaptureVariant),
    End,
}

pub enum ShadowCaptureVariant {
    /// {}
    Unnamed,
    /// {*}
    ManyUnnamed,
    /// {5}
    NumberedUnnamed {
        /// Number of sections to match.
        sections: usize,
    },
    /// {name} - captures a section and adds it to the map with a given name
    Named(String),
    /// {*:name} - captures over many sections and adds it to the map with a given name.
    ManyNamed(String),
    /// {2:name} - captures a fixed number of sections with a given name.
    NumberedNamed { sections: usize, name: String },
}

impl ToTokens for ShadowCaptureVariant {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let t = match self {
            ShadowCaptureVariant::Named(name) => {
                quote! {::yew_router::matcher::CaptureVariant::Named(#name.to_string())}
            }
            ShadowCaptureVariant::ManyNamed(name) => {
                quote! {::yew_router::matcher::CaptureVariant::ManyNamed(#name.to_string())}
            }
            ShadowCaptureVariant::NumberedNamed { sections, name } => {
                quote! {::yew_router::matcher::CaptureVariant::NumberedNamed{sections: #sections, name: #name.to_string()}}
            }
            ShadowCaptureVariant::Unnamed => {
                quote! {::yew_router::matcher::CaptureVariant::Unnamed}
            }
            ShadowCaptureVariant::ManyUnnamed => {
                quote! {::yew_router::matcher::CaptureVariant::ManyUnnamed}
            }
            ShadowCaptureVariant::NumberedUnnamed { sections } => {
                quote! {::yew_router::matcher::CaptureVariant::NumberedUnnamed{sections: #sections}}
            }
        };
        ts.extend(t)
    }
}

impl From<MatcherToken> for ShadowMatcherToken {
    fn from(mt: MatcherToken) -> Self {
        use MatcherToken as MT;
        use ShadowMatcherToken as SOT;
        match mt {
            MT::Exact(s) => SOT::Exact(s),
            MT::Capture(capture) => SOT::Capture(capture.into()),
            MT::End => SOT::End,
        }
    }
}

impl From<CaptureVariant> for ShadowCaptureVariant {
    fn from(cv: CaptureVariant) -> Self {
        use ShadowCaptureVariant as SCV;
        match cv {
            CaptureVariant::Named(name) => SCV::Named(name),
            CaptureVariant::ManyNamed(name) => SCV::ManyNamed(name),
            CaptureVariant::NumberedNamed { sections, name } => {
                SCV::NumberedNamed { sections, name }
            }
            CaptureVariant::Unnamed => SCV::Unnamed,
            CaptureVariant::ManyUnnamed => SCV::ManyUnnamed,
            CaptureVariant::NumberedUnnamed { sections } => SCV::NumberedUnnamed { sections },
        }
    }
}
