use std::fmt;
use stdweb::web::{INode, TextNode};

pub struct VText {
    pub text: String,
}

impl VText {
    pub fn new<T: ToString>(text: T) -> Self {
        VText { text: text.to_string() }
    }

    pub fn render(&mut self, subject: &TextNode, opposite: Option<Self>) {
        if let Some(opposite) = opposite {
            if self.text != opposite.text {
                subject.set_node_value(Some(&self.text));
            }
        } else {
            subject.set_node_value(Some(&self.text));
        }
    }
}

impl fmt::Debug for VText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

