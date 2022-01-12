//! This module contains the implementation of a virtual text node `VText`.

use super::AttrValue;
use std::cmp::PartialEq;

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
    use super::*;

    impl VText {
        pub(crate) async fn render_to_string(&self, w: &mut String) {
            html_escape::encode_text_to_string(&self.text, w);
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32"), feature = "ssr"))]
mod ssr_tests {
    use tokio::test;

    use super::*;

    #[test]
    async fn test_simple_str() {
        let vtext = VText::new("abc");

        let mut s = String::new();

        vtext.render_to_string(&mut s).await;

        assert_eq!("abc", s.as_str());
    }
}
