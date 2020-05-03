---
description: The procedural macro for generating HTML and SVG
---

# Using html!

The `html!` macro allows you to write HTML and SVG code declaratively. It is similar to JSX (a Javascript extension which allows you to write HTML code inside of Javascript).

**Important notes**

1. The `html!` macro only accepts one root html node (you can counteract this by [using fragments or iterators](lists.md))
2. An empty `html! {}` invocation is valid and will not render anything
3. Literals must always be quoted and wrapped in braces: `html! { "Hello, World" }`

{% page-ref page="lists.md" %}

{% page-ref page="elements.md" %}

{% page-ref page="literals-and-expressions.md" %}

{% page-ref page="components.md" %}

