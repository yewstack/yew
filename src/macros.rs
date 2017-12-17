use html::{Tags, Node};

#[macro_export]
macro_rules! html_impl {
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
    ($stack:ident ({ $eval:expr } $($tail:tt)*)) => {
        let node = $crate::html::Node::new_text($eval);
        $crate::macros::add_child(&mut $stack, node);
        html_impl! { $stack ($($tail)*) }
    };
    ($stack:ident (> $($tail:tt)*)) => {
        html_impl! { $stack ($($tail)*) }
    };
    ($stack:ident (< / $endtag:ident > $($tail:tt)*)) => {
        let endtag = stringify!($endtag);
        $crate::macros::child_to_parent(&mut $stack, endtag);
        html_impl! { $stack ($($tail)*) }
    };
    ($stack:ident ()) => {
        $stack.pop().unwrap()
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

#[doc(hidden)]
pub fn attach_class<MSG>(stack: &mut Tags<MSG>, class: &'static str) {
    if let Some(node) = stack.last_mut() {
        node.add_classes(class);
    } else {
        panic!("no tag to attach class: {}", class);
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
pub fn child_to_parent<MSG>(stack: &mut Tags<MSG>, endtag: &'static str) {
    if let Some(node) = stack.pop() {
        if let Some(starttag) = node.tag() {
            if starttag != endtag {
                panic!("wrong closing tag: <{}> -> </{}>", starttag, endtag);
            }
        } else {
            panic!("trying to close untagged node with: {}", endtag);
        }
        if !stack.is_empty() {
            stack.last_mut().unwrap().add_child(node);
        } else {
            // Keep the last node in the stack
            stack.push(node);
        }
    } else {
        panic!("redundant closing tag: {}", endtag);
    }
}
