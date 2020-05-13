---
description: 元件，以及生命周期鉤子
---

# 元件

## 什麼是元件？

元件是 Yew 的基石。他們管理自己的狀態，可以渲染自己成為 DOM。元件可以透過實作，描述元件生命周期的 `Component` trait 來建立。

## 生命周期

{% hint style="info" %}
`歡迎來貢獻我們的文件：` [Add a diagram of the component lifecycle](https://github.com/yewstack/docs/issues/22)
{% endhint %}

## 生命周期的方法

### Create

當一個元件被建立，他會接收從父元件，也就是 `ComponentLink` ，傳下來的屬性。 這些屬性用來初始化元件的狀態，此外，「link」可以用來註冊回調函式或傳訊息給元件。

通常，你的元件 struct 會儲存 props 與 link，就像下面的例子：

```rust
pub struct MyComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for MyComponent {
    type Properties = Props;
    // ...

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent { props, link }
    }

    // ...
}
```

### View

元件會在 `view()` 方法中宣告佈局。Yew 提供 `html!` 巨集來宣告 HTML 合 SVG 的結點，包含他們的監聽事件與子結點。這個巨集扮演像是 React 的 JSX 的角色，但是是使用 Rust 的表達式，而不是 JavaScript 的。

```rust
impl Component for MyComponent {
    // ...

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Click);
        html! {
            <button onclick=onclick>{ self.props.button_text }</button>
        }
    }
}
```

更多使用細節，請參考 `html!` 教學：

{% page-ref page="../html/" %}

### Mounted

`mounted()` 生命周期的方法，會在 `view()` 方法結束後被呼叫，此時，Yew 已經掛載你的元件到 DOM 上，但是在瀏覽器還未刷新頁面。 實作這個方法，可以執行，希望在元件渲染完的元素之後，才能做的動作。如果結束 `mounted()` 的改變後，你希望重新渲染你的元件，請回傳 `true`。

```rust
use stdweb::web::html_element::InputElement;
use stdweb::web::IHtmlElement;
use yew::prelude::*;

pub struct MyComponent {
    node_ref: NodeRef,
}

impl Component for MyComponent {
    // ...

    fn view(&self) -> Html {
        html! {
            <input ref=self.node_ref.clone() type="text" />
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(input) = self.node_ref.try_into::<InputElement>() {
            input.focus();
        }
        false
    }
}
```

{% hint style="info" %}
注意，這個生命周期方法，不是一定要被實作，預設的行為是不做任何事情。
{% endhint %}

### Update

Components are dynamic and can register to receive asynchronous messages. The `update()` lifecycle method is called for each message. This allows the component to update itself based on what the message was, and determine if it needs to re-render itself. Messages can be triggered by HTML elements listeners or be sent by child components, Agents, Services, or Futures.

Here's an example of what `update()` could look like:

```rust
pub enum Msg {
    SetInputEnabled(bool)
}

impl Component for MyComponent {
    type Message = Msg;

    // ...

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
       match msg {
           Msg::SetInputEnabled(enabled) => {
               if self.input_enabled != enabled {
                   self.input_enabled = enabled;
                   true // Re-render
               } else {
                   false
               }
           }
       }
    }
}
```

### Change

Components may be re-rendered by their parents. When this happens, they could receive new properties and choose to re-render. This design facilitates parent to child component communication through changed properties. You don't have to implement `change()` but you probably want to if you want to update a component via props after it has been created.

A naive implementation would look like:

```rust
impl Component for MyComponent {
    // ...

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
       self.props = props;
       true // This will always re-render when new props are provided.
    }
}
```

### Destroy

After Components are unmounted from the DOM, Yew calls the `destroy()` lifecycle method to support any necessary clean up operations. This method is optional and does nothing by default.

## Associated Types

The `Component` trait has two associated types: `Message` and `Properties`.

```rust
impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}
```

`Message` represents a variety of messages that can be processed by the component to trigger some side effect. For example, you may have a `Click` message which triggers an API request or toggles the appearance of a UI component. It is common practice to create an enum called `Msg` in your component's module and use that as the message type in the component. It is common to shorten "message" to "msg".

```rust
enum Msg {
    Click,
}
```

`Properties` represents the information passed to a component from its parent. This type must implements the `Properties` trait \(usually by deriving it\) and can specify whether certain properties are required or optional. This type is used when creating and updating a component. It is common practice to create a struct called `Props` in your component's module and use that as the component's `Properties` type. It is common to shorten "properties" to "props". Since props are handed down from parent components, the root component of your application typically has a `Properties` type of `()`. If you wish to specify properties for your root component, use the `App::mount_with_props` method.

