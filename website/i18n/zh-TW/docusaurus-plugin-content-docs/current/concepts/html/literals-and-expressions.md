# Literals & Expressions

## Literals

如果表達式中的型別有實作 `Display` ，他們會被轉換成字串，並在 DOM 中作為 [Text](https://developer.mozilla.org/en-US/docs/Web/API/Text) （文字）結點。

所有的文字都必須用 `{}` 括起來，因為文字是被當作表達式處理。這是 HTML 語法與 Yew  的語法中，最大的不同。

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

只要可以回傳 `Html`，你都可以在你的 HTML 中用 `{}` 插入表達式。

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

通常把這些表達式與包裝成方法或閉包會比較好，可以提升可讀性：

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

