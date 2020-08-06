---
description: 外帶 DOM 的存取
---

# Refs

## Refs

`ref` 關鍵字可以被使用在任何 HTML 的元素或是元件，用來得到那個物件附加的 DOM `Element`。這個可以在 view 生命周期方法之外，改變 DOM。

對於要存取 canvas 元素，或滾動到頁面不同的區塊，很有幫助。

語法可以這樣使用：

```rust
// 建立
self.node_ref = NodeRef::default();

// 在 view 裡
html! {
    <div ref=self.node_ref.clone()></div>
}

// 更新
let has_attributes = self.node_ref.cast::<Element>().unwrap().has_attributes();
```
