#[cfg(all(test, target_arch = "wasm32", verbose_tests))]
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        ::wasm_bindgen_test::console_log!(concat!("\t  ", $fmt), $($arg),*);
    };
}
#[cfg(not(all(test, target_arch = "wasm32", verbose_tests)))]
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        // Only type-check the format expression, do not run any side effects
        let _ = || { std::format_args!(concat!("\t  ", $fmt), $($arg),*); };
    };
}
/// Log an operation during tests for debugging purposes
/// Set RUSTFLAGS="--cfg verbose_tests" environment variable to activate.
pub(super) use test_log;

#[cfg(feature = "hydration")]
mod feat_hydration {
    use std::borrow::Cow;

    use wasm_bindgen::JsCast;
    use web_sys::{Element, Node};

    pub(in crate::dom_bundle) fn node_type_str(node: &Node) -> Cow<'static, str> {
        match node.node_type() {
            Node::ELEMENT_NODE => {
                let tag = node
                    .dyn_ref::<Element>()
                    .map(|m| m.tag_name().to_lowercase())
                    .unwrap_or_else(|| "unknown".to_owned());

                format!("{tag} element node").into()
            }
            Node::ATTRIBUTE_NODE => "attribute node".into(),
            Node::TEXT_NODE => "text node".into(),
            Node::CDATA_SECTION_NODE => "cdata section node".into(),
            Node::ENTITY_REFERENCE_NODE => "entity reference node".into(),
            Node::ENTITY_NODE => "entity node".into(),
            Node::PROCESSING_INSTRUCTION_NODE => "processing instruction node".into(),
            Node::COMMENT_NODE => "comment node".into(),
            Node::DOCUMENT_NODE => "document node".into(),
            Node::DOCUMENT_TYPE_NODE => "document type node".into(),
            Node::DOCUMENT_FRAGMENT_NODE => "document fragment node".into(),
            Node::NOTATION_NODE => "notation node".into(),
            _ => "unknown node".into(),
        }
    }
}

#[cfg(feature = "hydration")]
pub(super) use feat_hydration::*;
#[cfg(test)]
// this is needed because clippy doesn't like the import not being used
#[allow(unused_imports)]
pub(super) use tests::*;

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use gloo::utils::document;
    use web_sys::Element;

    use crate::dom_bundle::{BSubtree, DomSlot};
    use crate::html::AnyScope;
    use crate::virtual_dom::vtag::SVG_NAMESPACE;

    pub fn setup_parent() -> (BSubtree, AnyScope, Element) {
        let scope = AnyScope::test();
        let parent = document().create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);

        document().body().unwrap().append_child(&parent).unwrap();

        (root, scope, parent)
    }

    pub fn setup_parent_svg() -> (BSubtree, AnyScope, Element) {
        let scope = AnyScope::test();
        let parent = document()
            .create_element_ns(Some(SVG_NAMESPACE), "svg")
            .unwrap();
        let root = BSubtree::create_root(&parent);

        document().body().unwrap().append_child(&parent).unwrap();

        (root, scope, parent)
    }

    pub const SIBLING_CONTENT: &str = "END";

    pub(crate) fn setup_parent_and_sibling() -> (BSubtree, AnyScope, Element, DomSlot) {
        let scope = AnyScope::test();
        let parent = document().create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);

        document().body().unwrap().append_child(&parent).unwrap();

        let end = document().create_text_node(SIBLING_CONTENT);
        parent.append_child(&end).unwrap();
        let sibling = DomSlot::at(end.into());

        (root, scope, parent, sibling)
    }
}
