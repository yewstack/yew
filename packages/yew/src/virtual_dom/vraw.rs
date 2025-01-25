use crate::html::ImplicitClone;
use crate::AttrValue;

/// A raw HTML string to be used in VDOM.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VRaw {
    pub html: AttrValue,
}

impl ImplicitClone for VRaw {}

impl From<AttrValue> for VRaw {
    fn from(html: AttrValue) -> Self {
        Self { html }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::fmt::Write;

    use super::*;
    use crate::html::AnyScope;
    use crate::platform::fmt::BufWriter;
    use crate::virtual_dom::Collectable;

    impl VRaw {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            _parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            let collectable = Collectable::Raw;

            if hydratable {
                collectable.write_open_tag(w);
            }

            let _ = w.write_str(self.html.as_ref());

            if hydratable {
                collectable.write_close_tag(w);
            }
        }
    }
}
