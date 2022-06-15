# 常量和表达式

## 常量

如果一个表达式的类型本身实现了 `Display` （一个标准库中的 Trait），他们将会被转化成字符串并且作为一个 [Text](https://developer.mozilla.org/en-US/docs/Web/API/Text) 节点插入 DOM 中。

所有的需要显示的文本必须被 `{}` 块包含，因为这些文本会被当做一个 Rust 表达式来处理。这一点上，Yew 中使用 HTML 的方式和正常 HTML 语法有巨大的区别。

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

## 表达式

你可以在 HTML 中使用 `{}` 块来插入 Rust 表达式，只要这些表达式最终可以被解析成 `Html`

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

通常我们会把这些表达式写进函数或者闭包中来增加可读性：

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
