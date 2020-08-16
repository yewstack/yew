# Lists

## Fragments

`html!` 巨集裡必須只有一個根結點。為了可以繞過這個限制，將兩個以上的結點，用空的標籤包裹起來，是合法的：

<!--DOCUSAURUS_CODE_TABS-->
<!--Valid-->
```rust
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```

<!--Invalid-->
```rust
/* error: only one root html element allowed */

html! {
    <div></div>
    <p></p>
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

## Iterators

Yew 支援兩種不同的方式，從 iterator 建構 html：

<!--DOCUSAURUS_CODE_TABS-->
<!--Syntax Type 1-->
```rust
html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}
```

<!--Syntax Type 2-->
```rust
html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

