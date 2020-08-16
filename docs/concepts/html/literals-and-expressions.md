---
title: Literals and Expressions
---
## Literals

If expressions resolve to types that implement `Display`, they will be converted to strings and inserted into the DOM as a [Text](https://developer.mozilla.org/en-US/docs/Web/API/Text) node.

All display text must be enclosed by `{}` blocks because text is handled as an expression. This is 
the largest deviation from normal HTML syntax that Yew makes.

```rust
let text = "lorem ipsum";
html!{
    <>
        <div>{text}</div>
        <div>{"dolor sit"}</div>
        <span>{42}</span>
    </>
}
```

## Expressions

You can insert expressions in your HTML using `{}` blocks, as long as they resolve to `Html`

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
let maybe_display_link = move || -> Html {
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
