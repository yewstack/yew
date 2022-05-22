//! This module contains the implementation of a virtual text node `VText`.

use std::cmp::PartialEq;

use super::AttrValue;

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
#[derive(Clone)]
pub struct VText {
    /// Contains a text of the node.
    pub text: AttrValue,
}

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

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::borrow::Cow;

    use futures::channel::mpsc::UnboundedSender;

    use super::*;
    use crate::html::AnyScope;

    impl VText {
        pub(crate) async fn render_to_string(
            &self,
            w: &mut String,
            _parent_scope: &AnyScope,
            _hydratable: bool,
        ) {
            html_escape::encode_text_to_string(&self.text, w);
        }

        pub(crate) async fn render_into_stream<'a>(
            &'a self,
            tx: &'a mut UnboundedSender<Cow<'static, str>>,
            _parent_scope: &'a AnyScope,
            _hydratable: bool,
        ) {
            let mut s = String::with_capacity(self.text.len());
            html_escape::encode_text_to_string(&self.text, &mut s);

            let _ = tx.unbounded_send(s.into());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
