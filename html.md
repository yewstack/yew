---
description: html! macro
---

# html!

## HTML in Rust

The `html!` macro allows you to write HTML in Rust, with a few extensions and modifications. It is comparable to the JSX syntax used prominently in React. 

\(Note:`Html<COMP>` is an alias to `VNode<COMP>)`

### Fragments

At the top-most level, the `html!` macro expects a single virtual dom node, which would preclude you from having something like:

```rust
// INVALID
html! {
    <div></div>
    <p></p>
}
```

Fragments, which use `<></>` syntax, are used to denote a list of items as the top level of a `html!` macro.

```rust
// VALID
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```

### Tag structure

Tags deviate from normal HTML slightly. Opening tags must either have a corresponding closing tag, or be terminated with a `/>`.

This will prevent you from copying and pasting some vanilla HTML into your Yew project. 

```rust
html! {
    // INVALID
    <input id="my_input" >
}
html! {
    // VALID
    <input id="my_input" />
}
```

### Expressions

You can insert expressions in your HTML using `{}` blocks, as long as they resolve to `Html<_>.`

```rust
let show_link = true;
html! {
     <div>
          {
               if show_link {
                    html! {
                         <a href="https://example.com">{"Link"}</a>
                    }
               } else {
                    html! {
                         "No link today"
                    }
               }
          }
     </div>
}
```

It often makes sense to extract these expressions into functions or closures in order to keep the code from drifting rightward:

```rust
let show_link = true;
let maybe_display_link = move || -> Html<SomeComponent> {
     if show_link {
          html! {
               <a href="https://example.com">{"Link"}</a>
          }
     } else {
          html! {
               "No link today"
          }
     }
}
html! {
     <div>{maybe_display_link()}</div>
}
```

### Text

If these expressions resolve to types that implement `Display`,  they will be converted to strings and inserted into the DOM as a text node. 

All display text must be enclosed by `{}` blocks because text is handled like an expression. This is the largest deviation from normal HTML syntax that Yew makes.

```rust
let text = "lorem ipsum";
html!{
    <>
        <div>{text}</div>
        <div>{"dolor sit"}</div>
    </>
}
```

### Callbacks

Closures declared _within_ a `html!` macro are automatically converted to `Callbacks`. Callbacks will return messages to the component. They are used for receiving messages from child components, and for handling events from HTML elements like `input`s and `button`s.

```rust
pub enum Msg {
    ButtonClicked
}
html!{
    <button onclick=|_event: ClickEvent| Msg::ButtonClicked ></button>
}
```

If the message you want your callback to return _wraps_ the argument in the closure in a tuple-variant, you can use the function tuple syntax instead, but only for `Component`s, and not for plain elements.

```rust
pub enum Msg {
    ButtonClicked(ClickEvent)
}
html! {
    <ButtonComponent callback=Msg::ButtonClicked />
}
```

This extends to the case if the argument is the same as the message you want to capture:

```rust
html! {
    <ButtonComponent callback=From::from></button>
}
// or
html! {
    <ButtonComponent callback=std::convert::identity />
}
```



### 

### Components

See the following section on ways to use components in the `html!` macro.



