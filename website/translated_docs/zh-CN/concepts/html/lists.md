# 列表

## Fragments

`html!` 宏总是要求一个单一的根节点。为了绕开这个限制，把内容包裹在一个空标签内是有效的：

{% tabs %}
{% tab title="有效" %}
```rust
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```
{% endtab %}

{% tab title="无效" %}
```rust
/* 错误：只允许一个 html 根元素 */

html! {
    <div></div>
    <p></p>
}
```
{% endtab %}
{% endtabs %}

## 迭代器

Yew 支持两种从迭代器构建 html 的语法：

{% tabs %}
{% tab title="语法类型 1" %}
```rust
html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}
```
{% endtab %}

{% tab title="语法类型 2" %}
```rust
html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}
```
{% endtab %}
{% endtabs %}

