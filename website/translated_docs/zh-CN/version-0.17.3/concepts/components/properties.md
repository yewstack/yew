---
title: 属性（Properties）
description: 父组件到子组件的通信
id: version-0.17.3-properties
original_id: properties
---

如“组件（Components）”页面所述，Properties 用于父级到子组件的通信。

## 派生宏

不要尝试自己去实现 `Properties`，而是通过使用 `#[derive(Properties)]` 来派生它。

:::note
要派生`Properties`的类型也必须实现`Clone` 。这同样可以通过使用`#[derive(Properties, Clone)]`或手动为您的类型实现`Clone` 
:::

### 必需属性

默认情况下，实现了 `Properties` 的结构体中的字段是必需的。当缺少了该字段并且在 `html!` 宏中创建了组件时，将返回编译错误。对于具有可选属性的字段，使用 `#[prop_or_default]` 来使用该类型的默认值。要指定一个值，请使用 `#[prop_or(value)]`，其中 value 是该属性的默认值，或者是使用 `#[prop_or_else(function)]` 其中的`function` 用来返回默认值。例如，要将一个布尔类型的默认值设置为 `true`，请使用属性 `#[prop_or(true)]`。可选属性通常使用 `Option`，其默认值为 `None`。

### PartialEq

如果可以的话，尽可能地为你的 props 上派生 `PartialEq` 。这里面使用了一个在**性能优化与最佳实践**部分说明了的技巧，可以更轻松地避免重新渲染。

## Properties 的内存/速度开销

你在`Component::view`中对组件的状态取了一个引用，并以此来创建 `Html`。但是 properties 是有所有权的值（owned values）。这意味着为了创造它们并且将它们传递给子组件，我们需要获取 `view` 函数里提供的引用的所有权。这是在将引用传递给组件时隐式克隆引用完成的，以获得构成其 props 的有所有权的值。

这意味着每个组件都有从其父级传递来的状态的独特副本，而且，每当你重新渲染一个组件时，该重新渲染组件的所有子组件的 props 都将被克隆。

这意味着如果你将 *大量* 数据作为 props（大小为 10 KB 的字符串）向下传递，则可能需要考虑将子组件转换为在父级运行返回 `Html` 的函数，因为这样就不会被强制克隆你的数据。

另外，如果你不需要修改作为 props 传递的大数据，而只需要显示它，则可以将其包装在 `Rc` 中，以便仅克隆一个引用计数的指针，而不是数据本身。

## 示例

```rust
use std::rc::Rc;
use yew::Properties;

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

impl Default for LinkColor {
    fn default() -> Self {
        // 除非另有说明，否则链接的颜色将为蓝色
        LinkColor::Blue
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// 链接必须有一个目标地址
    href: String,
    /// 如果链接文本很大，这将使得复制字符串开销更小
    /// 除非有性能问题，否则通常不建议这么做
    text: Rc<String>,
    /// 链接的颜色
    #[prop_or_default]
    color: LinkColor,
    /// 如果为 None，则 view 函数将不指定大小
    #[prop_or_default]
    size: Option<u32>,
    /// 当 view 函数没有指定 active，其默认为 true
    #[prop_or(true)]
    active: bool,
}
```
