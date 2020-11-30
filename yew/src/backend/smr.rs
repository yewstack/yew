// use super::DomBackend;
// use std::cell::Cell;
// use std::rc::Rc;

// // pub CURRENT_NODE_ID: u32 = 1;

// enum NodeData {
//     Element(ElementData),
//     Text(TextData),
// }

// struct ElementData {}

// struct TextData {}

// /// Represents a single rendering backend
// #[derive(Debug)]
// pub struct Renderer {}

// impl DomBackend for Renderer {
//     type Element = Element;
//     type Node = Node;
//     type Document = Document;
//     type Window = Window;

//     fn element_as_node(element: &Self::Element) -> Self::Node {
//         todo!()
//     }

//     fn element_last_child(element: &Self::Element) -> Option<Self::Element> {
//         todo!()
//     }

//     fn element_remove_child(element: &Self::Element, child: &Self::Element) -> Result<Node, ()> {
//         todo!()
//     }

//     fn cast_node_ref<INTO>(node_ref: &crate::NodeRef) -> Option<INTO> {
//         todo!()
//     }

//     fn get_document() -> Self::Document {
//         todo!()
//     }

//     fn get_window() -> Self::Window {
//         todo!()
//     }

//     fn get_origin() -> Result<String, anyhow::Error> {
//         todo!()
//     }

//     fn get_host() -> Result<String, anyhow::Error> {
//         todo!()
//     }
// }

// /// Represents a mocked, mostly nonfunctional Event Listener
// #[derive(Debug)]
// pub struct EventListener;

// /// Represents a mocked, mostly nonfunctional Input Event
// #[derive(Debug)]
// pub struct InputEvent {}

// /// Represents a mocked, mostly nonfunctional version of a File List
// /// See: https://developer.mozilla.org/en-US/docs/Web/API/FileList
// #[derive(Debug)]
// pub struct FileList {}

// /// Represents an Input element
// #[derive(Debug)]
// pub struct InputElement {}

// impl InputElement {
//     pub fn set_type(&self, value: &str) {
//         todo!()
//     }
// }

// /// Represents a Select element
// #[derive(Debug)]
// pub struct SelectElement {}

// /// Represents a TextArea element
// #[derive(Debug)]
// pub struct TextAreaElement {}

// impl TextAreaElement {
//     pub fn set_value(&self, value: &str) {
//         todo!()
//     }
// }

// impl InputElement {
//     pub fn set_value(&self, value: &str) {
//         todo!()
//     }
// }

// /// Represents a Button element
// #[derive(Debug)]
// pub struct ButtonElement {}

// /// Represents a Text node
// #[derive(Debug, Clone)]
// pub struct Text {}

// impl Element {}

// #[derive(Debug, Clone)]
// pub struct EventListener;

// #[derive(Debug, Clone)]
// pub struct Document;
// impl Text {
//     pub fn set_node_value(&self, value: Option<&str>) {}
// }

// /// Represents a generic Element
// #[derive(Debug, Clone)]
// pub struct Element {
//     pub node_data: Rc<Cell<NodeData>>,
// }

// impl Element {
//     pub fn remove_attribute(&self, name: &str) -> Result<(), ()> {
//         todo!("remove attributes")
//     }
//     pub fn as_node(&self) -> Node {
//         Node {}
//     }
//     pub fn last_child(&self) -> Option<Element> {
//         todo!("Enable last child in SSR dom");
//     }
//     pub fn remove_child(&self, child: &Node) -> Result<Node, ()> {
//         todo!("Enable child removal in SSR dom");
//     }
//     pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), ()> {
//         todo!("Enable attributes in SSR dom");
//     }
//     pub fn dyn_ref<T>(&self) -> Option<&T> {
//         todo!("Support dyn ref casting in SSR dom");
//     }

//     pub fn namespace_uri(&self) -> Option<String> {
//         todo!()
//     }
// }

// impl From<&Element> for &Node {
//     fn from(element: &Element) -> Self {
//         todo!("Implement &element -> &node conversion")
//     }
// }

// impl From<&Text> for &Node {
//     fn from(text: &Text) -> Self {
//         todo!("Implement &Text -> &node conversion")
//     }
// }

// impl From<Element> for Node {
//     fn from(element: Element) -> Self {
//         todo!("Implement element -> node conversion")
//     }
// }

// impl From<Node> for Element {
//     fn from(node: Node) -> Self {
//         let node_data = node.node_data;
//         match (*node_data).get() {
//             NodeData::Element(_) => {}
//             _ => {
//                 panic!("Cannot convert a non-element Node to an Element");
//             }
//         }
//         Element { node_data }
//     }
// }

// /// Represents a generic Node
// #[derive(PartialEq, Debug, Default, Clone)]
// pub struct Node;

// #[derive(PartialEq, Debug, Default, Clone)]
// pub struct TextNode;

// #[derive(Debug, Clone)]
// pub struct Node {
//     pub node_data: Rc<Cell<NodeData>>,
// }

// impl Node {
//     pub fn first_child(&self) -> Option<Node> {
//         todo!("Implement first child")
//     }
// }

// impl From<Option<Element>> for Node {
//     fn from(_: Option<Element>) -> Self {
//         todo!()
//     }
// }

// impl From<Text> for Node {
//     fn from(_: Text) -> Self {
//         todo!()
//     }
// }

// /// Represents a Window
// #[derive(Debug)]
// pub struct Window;

// impl Window {
//     pub fn location(&self) -> Option<String> {
//         Some(format!("blah"))
//     }
// }

// pub fn get_window() -> Window {
//     Window
// }

// /// Represents a Document
// #[derive(Debug)]
// pub struct Document;
// impl Document {
//     pub fn location(&self) -> Option<String> {
//         Some(format!("blah"))
//     }

//     pub fn query_selector(&self, id: &str) -> Option<Option<Element>> {
//         Some(Some(Element))
//     }
//     pub fn create_element(&self, id: &str) -> Option<Element> {
//         todo!("Create element");
//     }
//     pub fn create_text_node(&self, id: &str) -> Option<Element> {
//         todo!("Create element");
//     }
// }

// impl Node {
//     pub fn first_child(&self) -> Option<Node> {
//         todo!("Implement first child")
//     }
// }

// impl From<Option<Element>> for Node {
//     fn from(_: Option<Element>) -> Self {
//         todo!()
//     }
// }

// impl Element {
//     pub fn as_node(&self) -> Node {
//         Node {}
//     }
//     pub fn last_child(&self) -> Option<Element> {
//         todo!("Enable last child in SSR dom");
//         Some(Element)
//     }
//     pub fn remove_child(&self, child: &Element) -> Option<()> {
//         todo!("Enable child removal in SSR dom");
//         Some(())
//     }

//     pub fn query_selector(&self, selectors: &str) -> Option<Option<Element>> {
//         todo!("query selector")
//     }
//     pub fn create_element(&self, qualified_name: &str) -> Option<Element> {
//         todo!("Create element");
//     }
//     pub fn create_element_ns(
//         &self,
//         namespace: Option<&str>,
//         qualified_name: &str,
//     ) -> Option<Element> {
//         todo!("Create element");
//     }
//     pub fn create_text_node(&self, id: &str) -> Text {
//         todo!("Create element");
//     }
// }

// pub fn get_document() -> Document {
//     Document
// }

// pub fn get_origin() -> String {
//     format!("blah")
// }

// pub fn get_host() -> String {
//     format!("blah")
// }

// mod stringify_smr {

//     //! This module contains Yew's implementation of Sans-Mount Rendering (SMR), to support
//     //! future feature work such as Static Site Generation and Server-Side Rendering (SSR).
//     //! This functionality allows Yew Components to be rendered to a string without needing
//     //! to be mounted onto a DOM node first.
//     //!
//     //! *This module is only available if the `static_render` feature is enabled.*

//     use crate::virtual_dom::{VComp, VList, VNode, VTag, VText};
//     use htmlescape;
//     use std::convert::TryFrom;
//     use std::fmt::{self, Display, Formatter};
//     use thiserror::Error as ThisError;
//     /// Represents a block of HTML string content generated via Sans-Mount Rendering
//     #[derive(Debug, PartialEq, Eq, Clone)]
//     pub struct HtmlString(String);

//     impl HtmlString {
//         fn new(html: String) -> Self {
//             Self(html)
//         }
//     }

//     impl Display for HtmlString {
//         fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//             write!(f, "{}", self.0)
//         }
//     }

//     /// Represents errors associated with conversion of Yew structures to HTML.
//     #[derive(Debug, ThisError)]
//     pub enum HtmlRenderError {
//         /// Malformed/unserializable attribute name
//         #[error("cannot serialize invalid attribute name `{0}`")]
//         InvalidAttributeName(String),

//         /// Malformed/unserializable tag name
//         #[error("cannot serialize invalid tag name `{0}`")]
//         InvalidTagName(String),

//         /// Unsupported VRef serialization
//         #[error("cannot serialize VRef because that is unsupported")]
//         UnserializableVRef,
//     }

//     impl TryFrom<VComp> for HtmlString {
//         type Error = HtmlRenderError;

//         fn try_from(value: VComp) -> Result<HtmlString, HtmlRenderError> {
//             let html: String = match &value.scope {
//                 None => "".to_string(),
//                 Some(scope) => match scope.root_vnode() {
//                     None => "".to_string(),
//                     Some(root_vnode) => HtmlString::try_from(root_vnode.clone())?.to_string(),
//                 },
//             };
//             Ok(HtmlString::new(html))
//         }
//     }

//     /// HTML output for a VTag is not necessarily deterministic due to the
//     /// serialization of props which do not have a particular ordering.
//     impl TryFrom<VTag> for HtmlString {
//         type Error = HtmlRenderError;

//         fn try_from(value: VTag) -> Result<HtmlString, HtmlRenderError> {
//             let mut result = "".to_string();
//             let tag_name = htmlescape::encode_minimal(&value.tag).to_lowercase();

//             result.push_str(&format!("<{}", tag_name));

//             value.attributes.iter().for_each(|(key_unclean, value)| {
//                 let key = key_unclean.to_lowercase();

//                 // textareas' innerHTML properties are specified via the `value` prop which doesn't
//                 // exist in HTML, so we defer this prop's serialization until later in the process.
//                 if tag_name == "textarea" && key == "value" {
//                     return;
//                 }

//                 result.push_str(&format!(
//                     " {}=\"{}\"",
//                     htmlescape::encode_minimal(&key),
//                     htmlescape::encode_attribute(&value)
//                 ));
//             });

//             if value.checked {
//                 result.push_str(&" checked")
//             }

//             if tag_name == "input" {
//                 if let Some(kind) = &value.kind {
//                     result.push_str(&format!(
//                         " type=\"{}\"",
//                         htmlescape::encode_attribute(&kind)
//                     ));
//                 }
//             }

//             let children_html = match tag_name.as_ref() {
//                 "textarea" => {
//                     let vtext = VText::new(value.value.clone().unwrap_or_else(String::new));
//                     HtmlString::try_from(vtext)
//                 }
//                 _ => HtmlString::try_from(value.children),
//             }?
//             .to_string();

//             if children_html == "" {
//                 result.push_str(&" />");
//             } else {
//                 result.push_str(&">");
//                 result.push_str(&children_html);
//                 result.push_str(&format!("</{}>", tag_name));
//             }

//             result.shrink_to_fit();
//             Ok(HtmlString::new(result))
//         }
//     }

//     impl TryFrom<VText> for HtmlString {
//         type Error = HtmlRenderError;

//         fn try_from(value: VText) -> Result<HtmlString, HtmlRenderError> {
//             Ok(HtmlString::new(htmlescape::encode_minimal(&value.text)))
//         }
//     }

//     impl TryFrom<VList> for HtmlString {
//         type Error = HtmlRenderError;

//         fn try_from(value: VList) -> Result<HtmlString, HtmlRenderError> {
//             let mut result = "".to_string();
//             for child in value.children {
//                 let html = HtmlString::try_from(child)?.to_string();
//                 result.push_str(&html);
//             }

//             result.shrink_to_fit();
//             Ok(HtmlString::new(result))
//         }
//     }

//     impl TryFrom<VNode> for HtmlString {
//         type Error = HtmlRenderError;

//         fn try_from(value: VNode) -> Result<HtmlString, HtmlRenderError> {
//             Ok(match value {
//                 VNode::VTag(vtag) => HtmlString::try_from(*vtag)?,
//                 VNode::VText(vtext) => HtmlString::try_from(vtext)?,
//                 VNode::VComp(vcomp) => HtmlString::try_from(vcomp)?,
//                 VNode::VList(vlist) => HtmlString::try_from(vlist)?,
//                 VNode::VRef(_) => Err(HtmlRenderError::UnserializableVRef)?,
//             })
//         }
//     }

//     #[cfg(test)]
//     mod test_vtext {
//         use super::HtmlString;
//         use crate::html;
//         use std::convert::TryFrom;

//         #[cfg(feature = "wasm_test")]
//         use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

//         #[cfg(feature = "wasm_test")]
//         wasm_bindgen_test_configure!(run_in_browser);

//         #[test]
//         fn text_as_root_smr() {
//             let a = html! {
//                 "Text Node As Root"
//             };

//             let b = html! {
//                 { "Text Node As Root" }
//             };

//             assert_eq!(
//                 HtmlString::try_from(a.clone()).expect("HTML stringify error"),
//                 HtmlString::try_from(b.clone()).expect("HTML stringify error")
//             );
//             assert!(
//                 HtmlString::try_from(b)
//                     .expect("HTML stringify error")
//                     .to_string()
//                     == "Text Node As Root"
//             );
//         }

//         #[test]
//         fn special_chars_smr() {
//             let a = html! {
//                 "some special-chars\"> here!"
//             };

//             let b = html! {
//                 { "some special-chars\"> here!" }
//             };

//             assert_eq!(
//                 HtmlString::try_from(a.clone()).expect("HTML stringify error"),
//                 HtmlString::try_from(b.clone()).expect("HTML stringify error")
//             );
//             assert_eq!(
//                 HtmlString::try_from(b.clone())
//                     .expect("HTML stringify error")
//                     .to_string(),
//                 "some special-chars&quot;&gt; here!"
//             );
//         }
//     }

//     #[cfg(test)]
//     mod tests_vtag {
//         use super::*;
//         use crate::html;
//         use crate::html::NodeRef;
//         use std::convert::TryFrom;

//         #[cfg(feature = "wasm_test")]
//         use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

//         #[cfg(feature = "wasm_test")]
//         wasm_bindgen_test_configure!(run_in_browser);

//         #[test]
//         fn it_stringifies_simple() {
//             let p = html! {
//                 <p></p>
//             };

//             if let VNode::VTag(p) = p {
//                 let p_html = HtmlString::try_from(*p)
//                     .expect("HTML stringify error")
//                     .to_string();

//                 assert_eq!(p_html, "<p />");
//             } else {
//                 assert!(false);
//             }
//         }

//         #[test]
//         fn it_stringifies_complex() {
//             let other_sym = "bar";
//             let div = html! {
//                 <div class=("foo", other_sym)>
//                     { "quux" }
//                 </div>
//             };
//             let p = html! {
//                 <p aria-controls="it-works">
//                     { "test" }
//                     {div}
//                 </p>
//             };

//             if let VNode::VTag(p) = p {
//                 let p_html = HtmlString::try_from(*p)
//                     .expect("HTML stringify error")
//                     .to_string();

//                 assert_eq!(
//                 p_html,
//                 "<p aria-controls=\"it&#x2D;works\">test<div class=\"foo&#x20;bar\">quux</div></p>"
//             );
//             } else {
//                 assert!(false);
//             }
//         }

//         #[test]
//         fn it_stringifies_attrs() {
//             let div = html! {
//                 <div a="b" b="a" />
//             };

//             if let VNode::VTag(div) = div {
//                 let div_html = HtmlString::try_from(*div)
//                     .expect("HTML stringify error")
//                     .to_string();
//                 let order_1 = "<div a=\"b\" b=\"a\" />";
//                 let order_2 = "<div b=\"a\" a=\"b\" />";
//                 assert!(div_html == order_1 || div_html == order_2);
//             } else {
//                 assert!(false);
//             }
//         }

//         #[test]
//         fn it_does_not_stringify_special_attrs() {
//             let node_ref = NodeRef::default();

//             let div = html! {
//                 <div ref=node_ref />
//             };

//             if let VNode::VTag(div) = div {
//                 let div_html = HtmlString::try_from(*div)
//                     .expect("HTML stringify error")
//                     .to_string();
//                 assert_eq!(div_html, "<div />");
//             } else {
//                 assert!(false);
//             }
//         }
//     }

//     mod test_mounting {
//         use crate::html;

//         struct TestApplication {}
//         impl crate::Component for TestApplication {
//             type Message = ();

//             type Properties = ();

//             fn create(props: (), link: crate::ComponentLink<Self>) -> Self {
//                 Self {}
//             }

//             fn update(&mut self, msg: ()) -> bool {
//                 true
//             }

//             fn change(&mut self, _props: ()) -> bool {
//                 true
//             }

//             fn view(&self) -> crate::Html {
//                 html! {
//                     <div>
//                         <h1>{"This is rendered without a dom!"}</h1>
//                     </div>
//                 }
//             }
//         }
//     }
// }
