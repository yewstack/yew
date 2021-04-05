---
title: Refs
description: 界限之外的 DOM 访问
id: version-0.17.3-refs
original_id: refs
---

`ref` 关键词可被用在任何 HTML 元素或组件内部以获得与之相关联的 DOM `元素` 。这样就能在 `view` 生命周期方法之外对 DOM 进行更改。

用这个可以方便地获取 canvas 元素或者滚动到页面的不同部分。

语法如下：

```rust
// 在 create 中
self.node_ref = NodeRef::default();

// 在 view 中
html! {
    <div ref=self.node_ref.clone()></div>
}

// 在 update 中
let has_attributes = self.node_ref.cast::<Element>().unwrap().has_attributes();
```
