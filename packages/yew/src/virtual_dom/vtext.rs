//! This module contains the implementation of a virtual text node `VText`.

use std::cmp::PartialEq;

use super::AttrValue;
use crate::html::ImplicitClone;

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
#[derive(Clone)]
pub struct VText {
    /// Contains a text of the node.
    pub text: AttrValue,
}

impl ImplicitClone for VText {}

impl VText {
    /// Creates new virtual text node with a content.
    pub fn new(text: impl Into<AttrValue>) -> Self {
        VText { text: text.into() }
    }
}

impl std::fmt::Debug for VText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VText {{ text: \"{}\" }}", self.text)
    }
}

impl PartialEq for VText {
    fn eq(&self, other: &VText) -> bool {
        self.text == other.text
    }
}

impl<T: ToString> From<T> for VText {
    fn from(value: T) -> Self {
        VText::new(value.to_string())
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {

    use std::fmt::Write;

    use super::*;
    use crate::feat_ssr::VTagKind;
    use crate::html::AnyScope;
    use crate::platform::fmt::BufWriter;

    impl VText {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            _parent_scope: &AnyScope,
            _hydratable: bool,
            parent_vtag_kind: VTagKind,
        ) {
            _ = w.write_str(&match parent_vtag_kind {
                VTagKind::Style => html_escape::encode_style(&self.text),
                VTagKind::Script => html_escape::encode_script(&self.text),
                VTagKind::Other => html_escape::encode_text(&self.text),
            })
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "ssr")]
#[cfg(test)]
mod ssr_tests {
    use tokio::test;

    use crate::prelude::*;
    use crate::ServerRenderer;

    #[test]
    async fn test_simple_str() {
        #[function_component]
        fn Comp() -> Html {
            html! { "abc" }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(s, r#"abc"#);
    }
}
