//! This module contains macros which implements `html!` macro
//! and JSX-like templates.

use virtual_dom::{VTag, VNode, Listener};

#[macro_export]
macro_rules! html_impl {
    // Start of component tag
    ($stack:ident (< $comp:ty : $($tail:tt)*)) => {
        let comp = $crate::virtual_dom::VComp::lazy::<$comp>();
        $stack.push(comp.into());
        html_impl! { @vcomp $stack ($($tail)*) }
    };
    // Self-closing of tag
    (@vcomp $stack:ident (/ > $($tail:tt)*)) => {
        $crate::macros::child_to_parent(&mut $stack, None);
        html_impl! { $stack ($($tail)*) }
    };
    // Start of opening tag
    ($stack:ident (< $starttag:ident $($tail:tt)*)) => {
        let vtag = $crate::virtual_dom::VTag::new(stringify!($starttag));
        $stack.push(vtag.into());
        html_impl! { @vtag $stack ($($tail)*) }
    };
    // PATTERN: class=("class-1", "class-2"),
    (@vtag $stack:ident (class = ($($class:expr),*), $($tail:tt)*)) => {
        $( $crate::macros::attach_class(&mut $stack, $class); )*
        html_impl! { @vtag $stack ($($tail)*) }
    };
    (@vtag $stack:ident (class = $class:expr, $($tail:tt)*)) => {
        $crate::macros::attach_class(&mut $stack, $class);
        html_impl! { @vtag $stack ($($tail)*) }
    };
    // PATTERN: value="",
    (@vtag $stack:ident (value = $value:expr, $($tail:tt)*)) => {
        $crate::macros::set_value(&mut $stack, $value);
        html_impl! { @vtag $stack ($($tail)*) }
    };
    // PATTERN: attribute=value, - workaround for `type` attribute
    // because `type` is a keyword in Rust
    (@vtag $stack:ident (type = $kind:expr, $($tail:tt)*)) => {
        $crate::macros::set_kind(&mut $stack, $kind);
        html_impl! { @vtag $stack ($($tail)*) }
    };
    (@vtag $stack:ident (checked = $kind:expr, $($tail:tt)*)) => {
        $crate::macros::set_checked(&mut $stack, $kind);
        html_impl! { @vtag $stack ($($tail)*) }
    };
    (@vtag $stack:ident (disabled = $kind:expr, $($tail:tt)*)) => {
        if $kind {
            $crate::macros::add_attribute(&mut $stack, "disabled", "true");
        }
        html_impl! { @vtag $stack ($($tail)*) }
    };
    // Events:
    (@vtag $stack:ident (onclick = $handler:expr, $($tail:tt)*)) => {
        html_impl! { @vtag $stack ((onclick) = $handler, $($tail)*) }
    };
    (@vtag $stack:ident (ondoubleclick = $handler:expr, $($tail:tt)*)) => {
        html_impl! { @vtag $stack ((ondoubleclick) = $handler, $($tail)*) }
    };
    (@vtag $stack:ident (onkeypress = $handler:expr, $($tail:tt)*)) => {
        html_impl! { @vtag $stack ((onkeypress) = $handler, $($tail)*) }
    };
    (@vtag $stack:ident (oninput = $handler:expr, $($tail:tt)*)) => {
        html_impl! { @vtag $stack ((oninput) = $handler, $($tail)*) }
    };
    // PATTERN: (action)=expression,
    (@vtag $stack:ident (($action:ident) = $handler:expr, $($tail:tt)*)) => {
        // Catch value to a separate variable for clear error messages
        let handler = $handler;
        let listener = $crate::html::$action::Wrapper::from(handler);
        $crate::macros::attach_listener(&mut $stack, Box::new(listener));
        html_impl! { @vtag $stack ($($tail)*) }
    };
    // Attributes:
    (@vtag $stack:ident (href = $href:expr, $($tail:tt)*)) => {
        let href: $crate::html::Href = $href.into();
        $crate::macros::add_attribute(&mut $stack, "href", href);
        html_impl! { @vtag $stack ($($tail)*) }
    };
    (@vtag $stack:ident ($attr:ident = $val:expr, $($tail:tt)*)) => {
        $crate::macros::add_attribute(&mut $stack, stringify!($attr), $val);
        html_impl! { @vtag $stack ($($tail)*) }
    };
    // End of openging tag
    (@vtag $stack:ident (> $($tail:tt)*)) => {
        html_impl! { $stack ($($tail)*) }
    };
    // Self-closing of tag
    (@vtag $stack:ident (/ > $($tail:tt)*)) => {
        $crate::macros::child_to_parent(&mut $stack, None);
        html_impl! { $stack ($($tail)*) }
    };
    // Traditional tag closing
    ($stack:ident (< / $endtag:ident > $($tail:tt)*)) => {
        let endtag = stringify!($endtag);
        $crate::macros::child_to_parent(&mut $stack, Some(endtag));
        html_impl! { $stack ($($tail)*) }
    };
    // PATTERN: { for expression }
    ($stack:ident ({ for $eval:expr } $($tail:tt)*)) => {
        let nodes = $eval;
        for node in nodes.map($crate::virtual_dom::VNode::from) {
            $crate::macros::add_child(&mut $stack, node);
        }
        html_impl! { $stack ($($tail)*) }
    };
    // PATTERN: { expression }
    ($stack:ident ({ $eval:expr } $($tail:tt)*)) => {
        let node = $crate::virtual_dom::VNode::from($eval);
        $crate::macros::add_child(&mut $stack, node);
        html_impl! { $stack ($($tail)*) }
    };
    // "End of paring" rule
    ($stack:ident ()) => {
        $crate::macros::unpack($stack)
    };
}

// This entrypoint and implementation had separated to prevent infinite recursion.
#[macro_export]
macro_rules! html {
    ($($tail:tt)*) => {{
        let mut stack = Vec::new();
        html_impl! { stack ($($tail)*) }
    }};
}

type Stack<MSG, CTX> = Vec<VNode<MSG, CTX>>;

#[doc(hidden)]
pub fn unpack<MSG, CTX>(mut stack: Stack<MSG, CTX>) -> VNode<MSG, CTX> {
    if stack.len() != 1 {
        panic!("exactly one element have to be in html!");
    }
    stack.pop().expect("no html elements in the stack")
}

#[doc(hidden)]
pub fn set_value<MSG, CTX, T: ToString>(stack: &mut Stack<MSG, CTX>, value: T) {
    if let Some(&mut VNode::VTag{ ref mut vtag, .. }) = stack.last_mut() {
        vtag.set_value(&value);
    } else {
        panic!("no tag to set value: {}", value.to_string());
    }
}

#[doc(hidden)]
pub fn set_kind<MSG, CTX, T: ToString>(stack: &mut Stack<MSG, CTX>, value: T) {
    if let Some(&mut VNode::VTag{ ref mut vtag, .. }) = stack.last_mut() {
        vtag.set_kind(value);
    } else {
        panic!("no tag to set type: {}", value.to_string());
    }
}

#[doc(hidden)]
pub fn set_checked<MSG, CTX>(stack: &mut Stack<MSG, CTX>, value: bool) {
    if let Some(&mut VNode::VTag{ ref mut vtag, .. }) = stack.last_mut() {
        vtag.set_checked(value);
    } else {
        panic!("no tag to set checked: {}", value);
    }
}

#[doc(hidden)]
pub fn add_attribute<MSG, CTX, T: ToString>(stack: &mut Stack<MSG, CTX>, name: &'static str, value: T) {
    if let Some(&mut VNode::VTag{ ref mut vtag, .. }) = stack.last_mut() {
        vtag.add_attribute(name, value);
    } else {
        panic!("no tag to set attribute: {}", name);
    }
}

#[doc(hidden)]
pub fn attach_class<MSG, CTX>(stack: &mut Stack<MSG, CTX>, class: &'static str) {
    if let Some(&mut VNode::VTag{ ref mut vtag, .. }) = stack.last_mut() {
        vtag.add_classes(class);
    } else {
        panic!("no tag to attach class: {}", class);
    }
}

#[doc(hidden)]
pub fn attach_listener<MSG, CTX>(stack: &mut Stack<MSG, CTX>, listener: Box<Listener<MSG>>) {
    if let Some(&mut VNode::VTag{ ref mut vtag, .. }) = stack.last_mut() {
        vtag.add_listener(listener);
    } else {
        panic!("no tag to attach listener: {:?}", listener);
    }
}

#[doc(hidden)]
pub fn add_child<MSG, CTX>(stack: &mut Stack<MSG, CTX>, child: VNode<MSG, CTX>) {
    if let Some(&mut VNode::VTag{ ref mut vtag, .. }) = stack.last_mut() {
        vtag.add_child(child);
    } else {
        panic!("no nodes in stack to add child: {:?}", child);
    }
}

#[doc(hidden)]
pub fn child_to_parent<MSG, CTX>(stack: &mut Stack<MSG, CTX>, endtag: Option<&'static str>) {
    if let Some(mut node) = stack.pop() {
        if let (&mut VNode::VTag { ref mut vtag, .. }, Some(endtag)) = (&mut node, endtag) {
            let starttag = vtag.tag();
            if starttag != endtag {
                panic!("wrong closing tag: <{}> -> </{}>", starttag, endtag);
            }
        }
        if !stack.is_empty() {
            if let Some(&mut VNode::VTag{ ref mut vtag, ..}) = stack.last_mut() {
                vtag.add_child(node);
            } else {
                panic!("can't add child to this type of node");
            }
        } else {
            // Keep the last node in the stack
            stack.push(node);
        }
    } else {
        panic!("redundant closing tag: {:?}", endtag);
    }
}

#[macro_export]
macro_rules! debug {
    ($($e:expr),*) => {
        if cfg!(debug) {
            println!($($e,)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($e:expr),*) => {
        eprintln!($($e,)*);
    };
}
