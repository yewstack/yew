---
description: 用來產生 HTML 與 SVG 的巨集
---

# 使用 html!

`html!` 巨集可以讓你用 HTML 與 SVG 寫元件。如果你寫過 React 的 JSX（一種 JavaScript 的擴展，可以讓你在 JavaScript 中寫 HTML），應該會覺得這兩者十分相似。

**重要提示**

1. 在 `html!` 裡，只能有一個根結點（但你可以用 [Fragment 或是 Iterators](https://yew.rs/docs/concepts/html/lists) 來繞過這個限制。）
2. 空的 `html! {}` 是合法的，且他不會渲染任何東西在畫面上
3. 字串必須被雙引號與大括號包裹住：`html! { "Hello, World" }`
