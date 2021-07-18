---
description: 元件，以及生命周期鉤子
---

# 元件

## 什麼是元件？

元件是 Yew 的基石。他們管理自己的狀態，可以渲染自己成為 DOM。元件可以透過實作，描述元件生命周期的 `Component` trait 來建立。

## 生命周期

:::note
`歡迎來貢獻我們的文件：` [Add a diagram of the component lifecycle](https://github.com/yewstack/docs/issues/22)
:::

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

更多使用細節，請參考 [`html!` 教學](html.md)。

### Rendered

`rendered()` 生命周期的方法會，在 `view()` 處理完並且 Yew 渲染完你的元件之後，與瀏覽器刷新頁面之前，被呼叫。一個元件可能希望實作這個方法，去執行只能在元件被渲染完元素才能做的事情。 你可以透過 `first_render` 變數來確認這個元件是不是第一次被渲染。

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

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<InputElement>() {
                input.focus();
            }
        }
    }
}
```

:::note
注意，這個生命周期方法，不是一定要被實作，預設的行為是不做任何事情。
:::

### Update

元件是可動態更新且可以註冊接收非同步的訊息。 `update()` 生命周期方法會被每個訊息呼叫。他基於訊息是什麼，來允許元件更新自己，且會決定是否需要重新渲染。 訊息可以被 HTML 元素的監聽器觸發，或被子元件、Agents、Services 或 Futures 傳送。 

`update()` 應用範例：

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
                   true // 重新渲染
               } else {
                   false
               }
           }
       }
    }
}
```

### Change

元件可能會被他的父元件重新渲染。當他被父元件重新渲染時，他會收到新的屬性，然後決定要不要再渲染一次。 這設計是讓父元件透過便於跟子元件溝通。

一個簡單的實作方式像：

```rust
impl Component for MyComponent {
    // ...

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
```

### Destroy

當元件從 DOM 上被解除掛載，Yew 會呼叫 `destroy()` 生命周期方法以提供任何需要清理的操作。這個方法是不一定要被實作的，預設不會做設任何事。

## 相關的型別

`Component` trait 有兩個相關的型別：`Message` 與 `Properties`。

```rust
impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}
```

`Message` 負責各式各樣的訊息，他可能被元件處理去觸發各種影響。舉例來說，你可能有一個 `Click` 的訊息，他會觸發 API 請求，或是切換 UI 元件的樣貌。下面是一個常見的實作，在你的元件模組中，創建一個叫作 `Msg` 的 enum，然後把他當作元件裡的 Message 型別。通常 message 會縮寫成 msg。

```rust
enum Msg {
    Click,
}
```

`Properties` 代表要從父員件傳遞到子元件的資訊。這個型別必須實作 `Properties` trait （通常會 deriving 他）並且可以決定某個屬性是必要的屬性，或是可選的屬性。這個型別會在創建元件的時候，或是更新元件的時候被使用到。常見的實作會在你的元件模組中，建立一個叫作 `Props`  的 struct，然後把他當作元件的`Properties` 型別。通常 properties 或縮寫成 props。因為屬性是從父原件被傳下來的，所以應用程式中的根元件的 `Properties` 原則上都是 `()`。如果你希望你的根元件有特定的屬性，可以使用 `App::mount_with_props` 的方法。

