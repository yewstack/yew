use html::{Tags, Node, Listener};

#[macro_export]
macro_rules! html_impl {
    // Start of openging tag
    ($stack:ident (< $starttag:ident $($tail:tt)*)) => {
        let node = $crate::html::Node::new(stringify!($starttag));
        $stack.push(node);
        html_impl! { $stack ($($tail)*) }
    };
    // PATTERN: class="",
    ($stack:ident (class = $class:expr, $($tail:tt)*)) => {
        $crate::macros::attach_class(&mut $stack, $class);
        html_impl! { $stack ($($tail)*) }
    };
    // PATTERN: (action)=expression,
    ($stack:ident (($action:ident) = $handler:expr, $($tail:tt)*)) => {
        // Catch value to a separate variable for clear error messages
        let handler = $handler;
        let listener = $crate::html::$action(handler);
        $crate::macros::attach_listener(&mut $stack, listener);
        html_impl! { $stack ($($tail)*) }
    };
    // PATTERN: attribute=value,
    ($stack:ident ($attr:ident = $val:expr, $($tail:tt)*)) => {
        // TODO Use ToString implementors
        html_impl! { $stack ($($tail)*) }
    };
    // PATTERN: { for expression }
    ($stack:ident ({ for $eval:expr } $($tail:tt)*)) => {
        let nodes = $eval;
        for node in nodes.map($crate::html::Node::from) {
            $crate::macros::add_child(&mut $stack, node);
        }
        html_impl! { $stack ($($tail)*) }
    };
    // PATTERN: { expression }
    ($stack:ident ({ $eval:expr } $($tail:tt)*)) => {
        let node = $crate::html::Node::from($eval);
        $crate::macros::add_child(&mut $stack, node);
        html_impl! { $stack ($($tail)*) }
    };
    // End of openging tag
    ($stack:ident (> $($tail:tt)*)) => {
        html_impl! { $stack ($($tail)*) }
    };
    // Self-closing of tag
    ($stack:ident (/ > $($tail:tt)*)) => {
        $crate::macros::child_to_parent(&mut $stack, None);
        html_impl! { $stack ($($tail)*) }
    };
    // Traditional tag closing
    ($stack:ident (< / $endtag:ident > $($tail:tt)*)) => {
        let endtag = stringify!($endtag);
        $crate::macros::child_to_parent(&mut $stack, Some(endtag));
        html_impl! { $stack ($($tail)*) }
    };
    // "End of paring" rule
    ($stack:ident ()) => {
        $stack.pop().expect("at least one element have to be in html!")
    };
}

// This entrypoint and implementation had separated to prevent infinite recursion.
#[macro_export]
macro_rules! html {
    ($($tail:tt)*) => {
        let mut stack = Vec::new();
        html_impl! { stack ($($tail)*) }
    };
}

#[doc(hidden)]
pub fn attach_class<MSG>(stack: &mut Tags<MSG>, class: &'static str) {
    if let Some(node) = stack.last_mut() {
        node.add_classes(class);
    } else {
        panic!("no tag to attach class: {}", class);
    }
}

#[doc(hidden)]
pub fn attach_listener<MSG>(stack: &mut Tags<MSG>, listener: Box<Listener<MSG>>) {
    if let Some(node) = stack.last_mut() {
        node.add_listener(listener);
    } else {
        panic!("no tag to attach listener: {:?}", listener);
    }
}

#[doc(hidden)]
pub fn add_child<MSG>(stack: &mut Tags<MSG>, node: Node<MSG>) {
    if let Some(parent) = stack.last_mut() {
        parent.add_child(node);
    } else {
        panic!("no nodes in stack to add child: {:?}", node);
    }
}

#[doc(hidden)]
pub fn child_to_parent<MSG>(stack: &mut Tags<MSG>, endtag: Option<&'static str>) {
    if let Some(node) = stack.pop() {
        if let Some(starttag) = node.tag() {
            if let Some(endtag) = endtag {
                if starttag != endtag {
                    panic!("wrong closing tag: <{}> -> </{}>", starttag, endtag);
                }
            }
        } else {
            panic!("trying to close untagged node with: {:?}", endtag);
        }
        if !stack.is_empty() {
            stack.last_mut().unwrap().add_child(node);
        } else {
            // Keep the last node in the stack
            stack.push(node);
        }
    } else {
        panic!("redundant closing tag: {:?}", endtag);
    }
}
