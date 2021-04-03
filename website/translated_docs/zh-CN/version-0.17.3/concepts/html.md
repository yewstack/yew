---
title: html! 宏
sidebar_label: 介绍
description: 用于生成 HTML 和 SVG 的宏程序
id: version-0.17.3-html
original_id: html
---

`html!` 宏允许你为组件编写声明式的 HTML 和 SVG。如果你使用过 React 的 JSX，将会感觉到非常熟悉。

**重要提示**

1. `html!` 宏调用中只能有一个根节点（你可以通过[使用片段（fragments）或迭代器](html/lists.md)来绕过这一点）
2. 空的 `html! {}` 宏调用是有效的但不会渲染任何内容
3. 常量必须始终被引号括起来并被包含在大括号里：`html! { "Hello, World" }`

::: note `html!`宏可以轻松达到编译器的默认递归限制。如果遇到编译错误，建议增大其值。在根 crate 使用这样的属性`#![recursion_limit="1024"]`（`lib.rs`或`main.rs` 也是同理）处理这个问题。

查看 [官方文档](https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute) 和 [这篇 Stack Overflow 问答](https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it) 来获取更多细节。 :::
