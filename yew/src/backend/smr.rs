use super::DomBackend;

/// Represents a single rendering backend
#[derive(Debug)]
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

    fn element_remove_child(element: &Self::Element, child: &Self::Element) -> Result<Node, ()> {
        todo!()
    }

    fn cast_node_ref<INTO>(node_ref: &crate::NodeRef) -> Option<INTO> {
        todo!()
    }
}

/// Represents a mocked, mostly nonfunctional Event Listener
#[derive(Debug)]
pub struct EventListener;

/// Represents a mocked, mostly nonfunctional Input Event
#[derive(Debug)]
pub struct InputEvent {
    
}

/// Represents a mocked, mostly nonfunctional version of a File List
/// See: https://developer.mozilla.org/en-US/docs/Web/API/FileList
#[derive(Debug)]
pub struct FileList {}

/// Represents an Input element
#[derive(Debug)]
pub struct InputElement {
}

impl InputElement {
    pub fn set_type(&self, value: &str) {
        todo!()
    }
}

/// Represents a Select element
#[derive(Debug)]
pub struct SelectElement {}

/// Represents a TextArea element
#[derive(Debug)]
pub struct TextAreaElement {}

impl TextAreaElement {
    pub fn set_value(&self, value: &str) {
        todo!()
    }
}

impl InputElement {
    pub fn set_value(&self, value: &str) {
        todo!()
    }
}

/// Represents a Button element
#[derive(Debug)]
pub struct ButtonElement {}

/// Represents a Text node
#[derive(Debug, Clone)]
pub struct Text {}

impl Text {
    pub fn set_node_value(&self, value: Option<&str>) {

    }
}

/// Represents a generic Element
#[derive(Debug, Clone)]
pub struct Element;

impl Element {
    pub fn as_node(&self) -> Node {
        Node {}
    }
    pub fn last_child(&self) -> Option<Element> {
        todo!("Enable last child in SSR dom");
    }
    pub fn remove_child(&self, child: &Node) -> Result<Node, ()> {
        todo!("Enable child removal in SSR dom");
    }
    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), ()> {
        todo!("Enable attributes in SSR dom");
    }
    pub fn dyn_ref<T>(&self) -> Option<&T> {
        todo!("Support dyn ref casting in SSR dom");
    }

    pub fn namespace_uri(&self) -> Option<String> {
        todo!()
    }
}

impl From<&Element> for &Node {
    fn from(element: &Element) -> Self {
        todo!("Implement &element -> &node conversion")
    }
}

impl From<&Text> for &Node {
    fn from(text: &Text) -> Self {
        todo!("Implement &Text -> &node conversion")
    }
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        todo!("Implement element -> node conversion")
    }
}

impl From<Node> for Element {
    fn from(node: Node) -> Self {
        todo!("Implement node -> element conversion")
    }
}

/// Represents a generic Node
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Node;
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

impl From<Text> for Node {
    fn from(_: Text) -> Self {
        todo!()
    }
}

/// Represents a Window
#[derive(Debug)]
pub struct Window;

impl Window {
    pub fn location(&self) -> Option<String> {
        Some(format!("blah"))
    }
}

pub fn get_window() -> Window {
    Window
}

/// Represents a Document
#[derive(Debug)]
pub struct Document;
impl Document {
    pub fn location(&self) -> Option<String> {
        Some(format!("blah"))
    }

    pub fn query_selector(&self, selectors: &str) -> Option<Option<Element>> {
        Some(Some(Element))
    }
    pub fn create_element(&self, qualified_name: &str) -> Option<Element> {
        todo!("Create element");
    }
    pub fn create_element_ns(&self, namespace: &str, qualified_name: &str) -> Option<Element> {
        todo!("Create element");
    }
    pub fn create_text_node(&self, id: &str) -> Text {
        todo!("Create element");
    }
}

pub fn get_document() -> Document {
    Document
}

pub fn get_origin() -> String {
    format!("blah")
}

pub fn get_host() -> String {
    format!("blah")
}