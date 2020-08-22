---
title: Components
description: Create complex layouts with component hierarchies
---
## 基本

`Component`を実装しているあらゆる型は`html!`マクロの中で使えます:

```rust
html!{
    <>
        // No properties
        <MyComponent />

        // With Properties
        <MyComponent prop1="lorem" prop2="ipsum" />

        // With the whole set of props provided at once
        <MyComponent with props />
    </>
}
```

## ネスト

`children`フィールドが`Properties`の中にある場合はコンポーネントは子に渡されます。

```rust title="parent.rs"
html! {
    <Container>
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}
```

```rust title="container.rs"
pub struct Container(Props);

#[derive(Properties, Clone)]
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

:::note
`Properties`を継承した型は`Clone`を実装していなければいけません。
これは`#[derive(Properties, Clone)]`を使うか手で`Clone`を実装すれば良いです。
:::

## Propsとネストした子コンポーネント

ネストしたコンポーネントのプロパティは格納しているコンポーネントの型が子である場合はアクセス可能、または変更可能です。
以下の例では`List`コンポーネントは`ListItem`コンポーネントをラップできています。
実際の使用においてこのパターンの例については`yew-router`のソースコードを確認してみてください。
より進んだ例としてはYewのメインのリポジトリにある`nested-list`を確認してみてください。

```rust title="parent.rs"
html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
}
```

```rust title="list.rs"
pub struct List(Props);

#[derive(Properties, Clone)]
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

