pub mod vnode;
pub mod vtag;
pub mod vtext;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use stdweb::web::{Element, EventListenerHandle};

pub use self::vnode::VNode;
pub use self::vtag::VTag;
pub use self::vtext::VText;

pub trait Listener<MSG> {
    fn kind(&self) -> &'static str;
    fn attach(&mut self, element: &Element, messages: Messages<MSG>) -> EventListenerHandle;
}

impl<MSG> fmt::Debug for Listener<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

pub type Messages<MSG> = Rc<RefCell<Vec<MSG>>>;
type Listeners<MSG> = Vec<Box<Listener<MSG>>>;
type Attributes = HashMap<String, String>;
type Classes = HashSet<String>;

enum Patch<ID, T> {
    Add(ID, T),
    Replace(ID, T),
    Remove(ID),
}

