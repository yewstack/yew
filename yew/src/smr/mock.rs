//! A collection of "virtual dom" methods that support stringification and SSR
//! Yew will chug along just fine, however the functionality of these components will be vastly neutered

#[derive(Clone)]
pub struct Element;

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

pub struct EventListener;

#[derive(Clone)]
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

pub struct Window;

impl Window {
    pub fn location(&self) -> Option<String> {
        Some(format!("blah"))
    }
}

pub fn get_window() -> Window {
    Window
}

pub struct Document;
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

pub fn get_document() -> Document {
    Document
}

pub fn get_origin() -> String {
    format!("blah")
}

pub fn get_host() -> String {
    format!("blah")
}
