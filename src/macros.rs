use html::Tags;

#[macro_export]
macro_rules! html {
    ($stack:ident (< $starttag:ident $($tail:tt)*)) => {
        let node = $crate::html::Node::new(stringify!($starttag));
        $stack.push(node);
        html! { $stack ($($tail)*) }
    };
    ($stack:ident (> $($tail:tt)*)) => {
        html! { $stack ($($tail)*) }
    };
    ($stack:ident (< / $endtag:ident > $($tail:tt)*)) => {
        let endtag = stringify!($endtag);
        $crate::macros::child_to_parent(&mut $stack, endtag);
        html! { $stack ($($tail)*) }
    };
    ($stack:ident ()) => {
        $stack.pop().unwrap()
    };
    ($($tail:tt)*) => {{
        let mut stack = Vec::new();
        html! { stack ($($tail)*) }
    }};
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
