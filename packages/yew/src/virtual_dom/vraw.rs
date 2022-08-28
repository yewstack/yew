use crate::AttrValue;

/// A raw HTML string to be used in VDOM.
#[derive(Clone, Debug)]
pub struct VRaw {
    pub html: AttrValue,
}

impl From<AttrValue> for VRaw {
    fn from(html: AttrValue) -> Self {
        Self { html }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::borrow::Cow;

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
            w.write(Cow::Borrowed(self.html.as_ref()))
        }
    }
}
