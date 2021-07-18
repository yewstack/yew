---
title: Refs
description: 超出界限的 DOM 访问
---

`ref` 关键词可被用在任何 HTML 元素或组件内部以获得该项所附加到的 DOM 元素。这可被用于在 `view` 生命周期方法之外来对 DOM 进行更改。

这对于获取 canvas 元素或者滚动到页面的不同部分是有用的。

语法如下：

```rust
// 在 create 中
self.node_ref = NodeRef::default();

// 在 view 中
html! {
    <div ref={self.node_ref.clone()}></div>
}

// 在 update 中
let has_attributes = self.node_ref.cast::<Element>().unwrap().has_attributes();
```
