---
title: 列表
id: version-0.17.3-lists
original_id: lists
---

## Fragments

`html!` 宏总是要求一个单一的根节点。为了绕开这个限制，可以把内容包裹在一个空标签内：

<!--DOCUSAURUS_CODE_TABS-->

<!--Valid-->

```rust
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```

<!--Invalid-->

```rust
/* 错误：只允许一个 html 根元素 */

html! {
    <div></div>
    <p></p>
}
```

<!--END_DOCUSAURUS_CODE_TABS-->

## 迭代器

Yew 支持两种从迭代器构建 html 的语法：

<!--DOCUSAURUS_CODE_TABS-->

<!--Syntax Type 1-->

```rust
html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}
```

<!--Syntax Type 2-->

```rust
html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}
```

<!--END_DOCUSAURUS_CODE_TABS-->
