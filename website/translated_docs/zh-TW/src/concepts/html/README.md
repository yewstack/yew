---
description: The procedural macro for generating HTML and SVG
---

# 使用 html!

The `html!` macro allows you to write declarative HTML and SVG for your components. If you've used React's JSX it will feel quite familiar.

**Important notes**

1. There must be only one root node in an `html!` invocation
2. An empty `html! {}` invocation is valid and will not render anything
3. Literals must always be quoted and wrapped in braces: `html! { "Hello, World" }`

{% page-ref page="lists.md" %}

{% page-ref page="elements.md" %}

{% page-ref page="literals-and-expressions.md" %}

{% page-ref page="components.md" %}

