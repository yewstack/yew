# Lists

## Fragments

`html!` 巨集裡必須只有一個根結點。為了可以繞過這個限制，將兩個以上的結點，用空的標籤包裹起來，是合法的：

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

Yew 支援兩種不同的方式，從 iterator 建構 html：

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

