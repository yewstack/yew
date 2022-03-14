use web_sys::{Element, Node};

/// Insert a concrete [Node] into the DOM
pub(super) fn insert_node(node: &Node, parent: &Element, next_sibling: Option<&Node>) {
    match next_sibling {
        Some(next_sibling) => parent
            .insert_before(node, Some(next_sibling))
            .expect("failed to insert tag before next sibling"),
        None => parent.append_child(node).expect("failed to append child"),
    };
}

#[cfg(all(test, feature = "wasm_test", verbose_tests))]
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        ::wasm_bindgen_test::console_log!(concat!("\t  ", $fmt), $($arg),*);
    };
}
#[cfg(not(all(test, feature = "wasm_test", verbose_tests)))]
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
    use super::*;

    use std::borrow::Cow;

    use wasm_bindgen::JsCast;
    use web_sys::Element;

    pub(in crate::dom_bundle) fn node_type_str(node: &Node) -> Cow<'static, str> {
        match node.node_type() {
            Node::ELEMENT_NODE => {
                let tag = node
                    .dyn_into::<Element>()
                    .map(|m| m.tag_name().to_lowercase())
                    .unwrap_or_else(|_| "unknown".to_owned());

                format!("{} element node", tag).into()
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
