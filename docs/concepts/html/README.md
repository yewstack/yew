---
description: The procedural macro for generating HTML and SVG
---

# Using html!

The `html!` macro allows you to write HTML and SVG code declaratively. It is similar to JSX \(a Javascript extension which allows you to write HTML-like code inside of Javascript\).

**Important notes**

1. The `html!` macro only accepts one root html node \(you can counteract this by 

   [using fragments or iterators](lists.md)\)

2. An empty `html! {}` invocation is valid and will not render anything
3. Literals must always be quoted and wrapped in braces: `html! { "Hello, World" }`

{% hint style="info" %}
The `html!` macro can reach easily the default recursion limit of the compiler. It is advised to bump its value if you encounter compilation errors. Use an attribute like `#![recursion_limit="1024"]` in the crate root \(i.e. either `lib.rs` or `main.rs`\) to overcome the problem. See the [official documentation](https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute) and [this Stack Overflow question](https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it) for details.
{% endhint %}

{% page-ref page="lists.md" %}

{% page-ref page="elements.md" %}

{% page-ref page="literals-and-expressions.md" %}

{% page-ref page="components.md" %}

