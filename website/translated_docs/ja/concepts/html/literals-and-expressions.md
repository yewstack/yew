---
title: Literals and Expressions
---
## リテラル

式が`Display`を実装した型を解決する場合、文字列に変換されてDOMに[Text](https://developer.mozilla.org/en-US/docs/Web/API/Text)ノードとして挿入されます。

テキストは式として処理されるため、全ての表示される内容は`{}`ブロックによって囲まれる必要があります。
これはYewのアプリと通常のHTMLの構文で最も異なる点です。

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

## 式

HTMLに`{}`ブロックを使って式を挿入することができます。

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

式を関数やクロージャに分離するのはコードの可読性の観点から有効なことがあります。

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
