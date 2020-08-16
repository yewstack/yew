---
title: Lists
---

## フラグメント

`html!`マクロは常にルートノードが1つであることを要求します。
この制限のために、空のタグを使って内容をラップすると良いでしょう。

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


## イテレータ

YewはイテレータからHTMLをビルドするのに2つの方法をサポートしています。

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
