use crate::AttrValue;

/// A raw HTML string to be used in VDOM.
#[derive(Clone, Debug)]
pub struct VRaw {
    pub html: AttrValue,
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::borrow::Cow;

    use html_parser::Dom;

    use super::*;
    use crate::html::AnyScope;
    use crate::platform::io::BufWriter;

    impl VRaw {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            _parent_scope: &AnyScope,
            _hydratable: bool,
        ) {
            // this is needed to ensure the resulting HTML during CSR and SSR is the same
            let dom = Dom::parse(self.html.as_ref()).expect("invalid HTML was passed");
            if dom.children.len() > 1 {
                w.write(Cow::Owned(format!("<div>{}</div>", self.html)))
            } else {
                w.write(Cow::Borrowed(self.html.as_ref()))
            }
        }
    }
}
