---
title: "#[function_component]"
description: "#[function_component]属性"
---

`#[function_component(_)]`将一个普通的 Rust 函数变成一个函数式组件。具有该属性的函数必须返回`Html`并且可以为组件应接受的 props 类型采用单个参数。参数类型需要是对实现`Properties`和`PartialEq`的类型的引用（例如`props: &MyProps` ）。如果函数没有任何参数，则生成的组件不接受任何 props 。

该属性不会将您的原函数替换为组件。您需要提供一个名称作为属性的输入，该属性值将成为组件的标识符。例如您有一个名为`chat_container`的函数并添加了属性`#[function_component(ChatContainer)]`，那么您就可以像这样使用该组件：

```rust
html! { <ChatContainer /> }
```

## 示例

<!--DOCUSAURUS_CODE_TABS-->

<!--With props-->

```rust
#[derive(Properties, Clone, PartialEq)]
pub struct RenderedAtProps {
    pub time: String,
}

#[function_component(RenderedAt)]
pub fn rendered_at(props: &RenderedAtProps) -> Html {
    html! {
        <p>
            <b>{ "Rendered at: " }</b>
            { props.time.clone() }
        </p>
    }
}
```

<!--Without props-->

```rust
#[function_component(App)]
fn app() -> Html {
    let (counter, set_counter) = use_state(|| 0);

    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

    html! {
        <div>
            <button onclick=onclick>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { counter }
            </p>
        </div>
    }
}
```

<!--END_DOCUSAURUS_CODE_TABS-->
