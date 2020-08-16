---
description: 用于生成 HTML 和 SVG 的宏程序
---

# 使用 html! 宏

`html!` 宏允许你为组件编写声明式的 HTML 和 SVG。如果你使用过 React 的 JSX，将会感觉到非常熟悉。

**重要提示**

1. `html!` 宏调用中只能有一个根节点
2. 空的 `html! {}` 宏调用是有效的但不会渲染任何内容
3. 常量必须始终被引号括起来并被包含在大括号里：`html! { "Hello, World" }`
