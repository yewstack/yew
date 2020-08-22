---
description: 建立複雜元件層級與佈局
---

# Components

## 基本

任何實作 `Component` 的型別，都可以在 `html!` 的巨集中使用：

```rust
html!{
    <>
        // 沒有屬性
        <MyComponent />

        // 有屬性
        <MyComponent prop1="lorem" prop2="ipsum" />

        // 一次提供很多屬性
        <MyComponent with props />
    </>
}
```

## 巢狀

只要元件的 `Properties` 中有 `children`，就可以傳遞子結點給元件。

{% code title="parent.rs" %}
```rust
html! {
    <Container>
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}
```
{% endcode %}

{% code title="container.rs" %}
```rust
pub struct Container(Props);

#[derive(Properties)]
pub struct Props {
    pub children: Children,
}

impl Component for Container {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
       html! {
           <div id="container">
               { self.0.children.clone() }
           </div>
       }
    }
}
```
{% endcode %}

## 指定子結點的型別

如果指定了子結點的型別，就可以使用或改變巢狀元件的屬性。下面的範例就是， `List` 元件包裹 `ListItem` 元件。另一個真實的範例是 `yew-router` 的原始碼。還有一個更進階的範例，請參考 Yew GitHub repo 中的 `nested-list` 範例。

{% code title="parent.rs" %}
```rust
html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
}
```
{% endcode %}

{% code title="list.rs" %}
```rust
pub struct List(Props);

#[derive(Properties)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

impl Component for List {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
        html!{{
            for self.0.children.iter().map(|mut item| {
                item.props.value = format!("item-{}", item.props.value);
                item
            })
        }}
    }
}
```
{% endcode %}

