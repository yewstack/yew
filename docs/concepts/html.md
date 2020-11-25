---
title: HTML
sidebar_label: Introduction
description: The procedural macro for generating HTML and SVG
---

The `html!` macro allows you to write HTML and SVG code declaratively. It is similar to JSX
\(an extension to JavaScript which allows you to write HTML-like code inside of JavaScript\).

**Important notes**

1. The `html!` macro only accepts one root html node \(you can counteract this by
   [using fragments or iterators](html/lists.md)\)
2. An empty `html! {}` invocation is valid and will not render anything
3. Literals must always be quoted and wrapped in braces: `html! { "Hello, World" }`

:::note
The `html!` macro can reach the default recursion limit of the compiler. If you encounter compilation errors, add an attribute like `#![recursion_limit="1024"]` in the crate root to overcome the problem.
:::

## Tag Structure

Tags are based on HTML tags. Components, Elements, and Lists are all based on this tag syntax.

Tags must either self-close `<... />` or have a corresponding end tag for each start tag.

<!--DOCUSAURUS_CODE_TABS-->
<!--Open - Close-->

```rust
html! {
  <div id="my_div"></div>
}
```

<!--Invalid-->

```rust
html! {
  <div id="my_div"> // <- MISSING CLOSE TAG
}
```

<!--Self-closing-->

```rust
html! {
  <input id="my_input" />
}
```

<!--Invalid-->

```rust
html! {
  <input id="my_input"> // <- MISSING SELF-CLOSE
}
```

<!--END_DOCUSAURUS_CODE_TABS-->

:::tip
For convenience, elements which _usually_ require a closing tag are **allowed** to self-close. For example, writing `html! { <div class="placeholder" /> }` is valid.
:::

## Children

Create complex nested HTML and SVG layouts with ease:

<!--DOCUSAURUS_CODE_TABS-->
<!--HTML-->

```rust
html! {
    <div>
        <div data-key="abc"></div>
        <div class="parent">
            <span class="child" value="anything"></span>
            <label for="first-name">{ "First Name" }</label>
            <input type="text" id="first-name" value="placeholder" />
            <input type="checkbox" checked=true />
            <textarea value="write a story" />
            <select name="status">
                <option selected=true disabled=false value="">{ "Selected" }</option>
                <option selected=false disabled=true value="">{ "Unselected" }</option>
            </select>
        </div>
    </div>
}
```

<!--SVG-->

```rust
html! {
    <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
        <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
        <g filter="url(#filter0_d)">
            <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
            <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
        </g>
        <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
        <defs>
            <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                <feGaussianBlur stdDeviation="2"/>
                <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
            </filter>
        </defs>
    </svg>
}
```

<!--END_DOCUSAURUS_CODE_TABS-->

## Special properties

There are special properties which don't directly influence the DOM but instead act as instructions to Yew's virtual DOM.
Currently, there are two such special props: `ref` and `key`.

`ref` allows you to access and manipulate the underlying DOM node directly. See [Refs](components/refs) for more details.

`key` on the other hand gives an element a unique identifier which Yew can use for optimization purposes.

:::important
The documentation for keys is yet to be written. See [#1263](https://github.com/yewstack/yew/issues/1263).

For now, use keys when you have a list where the order of elements changes. This includes inserting or removing elements from anywhere but the end of the list.
:::
