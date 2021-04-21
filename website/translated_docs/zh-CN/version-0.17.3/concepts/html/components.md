---
title: 组件
description: 使用具有层次结构的组件来创建复杂的布局
id: version-0.17.3-components
original_id: components
---

## 基础

任何实现了 `Component` trait 的类型都可被用在 `html!` 宏中：

```rust
html!{
    <>
        // 没有属性
        <MyComponent />

        // 具有属性
        <MyComponent prop1="lorem" prop2="ipsum" />

        // 同时提供全套的 props
        <MyComponent with props />
    </>
}
```

## 嵌套

如果组件的 `Properties` 中有 `children` 字段，则可以被传递子组件。

```rust
html! {
    <Container>
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}
```

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

:::note
要派生`Properties`的类型也必须实现`Clone` 。这同样可以通过使用`#[derive(Properties, Clone)]`或手动为您的类型实现`Clone` 
:::

## 拥有 Props 的嵌套子组件

如果包含组件标注了 children 的类型，则可以访问和更改嵌套组件的属性。在下面的示例中，`List` 组件可以包含 `ListItem` 组件。有关此模式的真实示例，请查看 `yew-router` 的源码。有关更高级的示例，请在 yew 主仓库中查看 `nested-list` 示例代码。

```rust
html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
}
```

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
