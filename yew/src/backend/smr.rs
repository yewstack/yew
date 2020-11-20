use super::DomBackend;

pub struct Renderer {}

impl DomBackend for Renderer {
    type Element = Element;
    type Node = Node;
    type Document = Document;
    type Window = Window;

    fn element_as_node(element: &Self::Element) -> Self::Node {
        todo!()
    }

    fn element_last_child(element: &Self::Element) -> Option<Self::Element> {
        todo!()
    }

    fn element_remove_child(element: &Self::Element, child: &Self::Element) -> Option<()> {
        todo!()
    }

    fn cast_node_ref<INTO>(node_ref: &crate::NodeRef) -> Option<INTO> {
        todo!()
    }

    fn get_document() -> Self::Document {
        todo!()
    }

    fn get_window() -> Self::Window {
        todo!()
    }

    fn get_origin() -> Result<String, anyhow::Error> {
        todo!()
    }

    fn get_host() -> Result<String, anyhow::Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Element;

impl Element {}

#[derive(Debug, Clone)]
pub struct EventListener;

#[derive(Debug, Clone)]
pub struct Document;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Node;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct TextNode;

#[derive(Debug, Clone)]
pub struct Window;

impl Window {
    pub fn location(&self) -> Option<String> {
        Some(format!("blah"))
    }
}

impl Document {
    pub fn location(&self) -> Option<String> {
        Some(format!("blah"))
    }

    pub fn query_selector(&self, id: &str) -> Option<Option<Element>> {
        Some(Some(Element))
    }
    pub fn create_element(&self, id: &str) -> Option<Element> {
        todo!("Create element");
    }
    pub fn create_text_node(&self, id: &str) -> Option<Element> {
        todo!("Create element");
    }
}

impl Node {
    pub fn first_child(&self) -> Option<Node> {
        todo!("Implement first child")
    }
}

impl From<Option<Element>> for Node {
    fn from(_: Option<Element>) -> Self {
        todo!()
    }
}

impl Element {
    pub fn as_node(&self) -> Node {
        Node {}
    }
    pub fn last_child(&self) -> Option<Element> {
        todo!("Enable last child in SSR dom");
        Some(Element)
    }
    pub fn remove_child(&self, child: &Element) -> Option<()> {
        todo!("Enable child removal in SSR dom");
        Some(())
    }
}
