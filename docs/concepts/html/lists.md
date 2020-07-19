---
id: lists
title: Lists
---

# Lists

## Fragments

The `html!` macro always requires a single root node. In order to get around this restriction, it's valid to wrap content in empty tags:

{% tabs %}
{% tab title="Valid" %}
```rust
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```
{% endtab %}

{% tab title="Invalid" %}
```rust
/* error: only one root html element allowed */

html! {
    <div></div>
    <p></p>
}
```
{% endtab %}
{% endtabs %}

## Iterators

Yew supports two different syntaxes for building html from an iterator:

{% tabs %}
{% tab title="Syntax Type 1" %}
```rust
html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}
```
{% endtab %}

{% tab title="Syntax Type 2" %}
```rust
html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}
```
{% endtab %}
{% endtabs %}

