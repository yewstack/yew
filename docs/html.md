---
description: The procedural macro for generating HTML
---

# html!

## HTML in Rust

The `html!` macro allows you to write HTML in Rust, with a few extensions and modifications. It is comparable to the JSX syntax used prominently in React. 

{% hint style="info" %}
Note that`Html<COMP>`is an alias to`VNode<COMP>`
{% endhint %}

### Fragments

The `html!` macro always requires a single root node. The following example will produce the following compiler message: `error: only one root html element allowed`

```rust
// INVALID
html! {
    <div></div>
    <p></p>
}
```

To work around this, Yew allows the use of fragments, which use `<></>` syntax, to denote a list of items as the top level of a `html!` macro.

```rust
// VALID
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```

### Tags

Tags are required to roughly follow the HTML standard syntax with some variations. For example, tags must either self-close...

```rust
html! {
    // INVALID (MISSING SELF-CLOSE)
    <input id="my_input">
}

html! {
    // VALID
    <input id="my_input" />
}
```

Or open tags must have a corresponding close tag

```rust
html! {
    // INVALID (MISSING CLOSE TAG)
    <div id="my_div">
}

html! {
    // VALID
    <div id="my_div"></div>
}
```

### Expressions

You can insert expressions in your HTML using `{}` blocks, as long as they resolve to `Html<_>`

```rust
html! {
  <div>
    {
      if show_link {
        html! {
          <a href="https://example.com">{"Link"}</a>
        }
      } else {
        html! {}
      }
    }
  </div>
}
```

It often makes sense to extract these expressions into functions or closures to optimize for readability:

```rust
let show_link = true;
let maybe_display_link = move || -> Html<SomeComponent> {
  if show_link {
    html! {
      <a href="https://example.com">{"Link"}</a>
    }
  } else {
    html! {}
  }
};

html! {
     <div>{maybe_display_link()}</div>
}
```

### Text Literals

If these expressions resolve to types that implement `Display`,  they will be converted to strings and inserted into the DOM as a [Text](https://developer.mozilla.org/en-US/docs/Web/API/Text) node. 

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
    <button onclick=|_| Msg::ButtonClicked>{ "Click Me!" }</button>
}
```

If the message you want your callback to return _wraps_ the argument in the closure in a tuple-variant, you can use the function tuple syntax instead, but only for `Component`s, and not for plain elements \([Issue](https://github.com/yewstack/yew/issues/733)\)

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

### Components

See the following section on ways to use components in the `html!` macro.



