---
description: HTML 和 SVG 元素均受支持
---

# 元素

## 标签结构

元素标签必须是自闭合的 `<... />`，或是每个标签都有一个对应的闭合标签。

<!--DOCUSAURUS_CODE_TABS-->
<!--标签 - 闭合标签-->
```rust
html! {
  <div id="my_div"></div>
}
```

<!--无效-->
```rust
html! {
  <div id="my_div"> // <- 缺少闭合标签
}
```

<!--自闭合-->
```rust
html! {
  <input id="my_input" />
}
```

<!--无效-->
```rust
html! {
  <input id="my_input"> // <- 没有自闭合
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

:::note
为方便起见，一些 _通常_ 需要闭合标签的元素是被**允许**自闭合的。例如，`html! { <div class="placeholder" /> }` 这样写是有效的。
:::

## Children

轻松创建复杂的嵌套 HTML 和 SVG 布局：

<!--DOCUSAURUS_CODE_TABS-->
<!--HTML-->
```rust
html! {
    <div>
        <div data-key="abc"></div>
        <div class="parent">
            <span class="child" value="anything"></span>
            <label for="first-name">{ "First Name" }</label>
            <input type="text" id="first-name" value="placeholder" />
            <input type="checkbox" checked=true />
            <textarea value="write a story" />
            <select name="status">
                <option selected=true disabled=false value="">{ "Selected" }</option>
                <option selected=false disabled=true value="">{ "Unselected" }</option>
            </select>
        </div>
    </div>
}
```

<!--SVG-->
```rust
html! {
    <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
        <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
        <g filter="url(#filter0_d)">
            <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
            <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
        </g>
        <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
        <defs>
            <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                <feGaussianBlur stdDeviation="2"/>
                <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
            </filter>
        </defs>
    </svg>
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

## Classes

有许多方便的选项可用于元素指定 classes：

<!--DOCUSAURUS_CODE_TABS-->
<!--常量-->
```rust
html! {
  <div class="container"></div>
}
```

<!--多个属性-->
```rust
html! {
  <div class="container center-align"></div>
}
```

<!--插值-->
```rust
html! {
  <div class=format!("{}-container", size)></div>
}
```

<!--表达式-->
```rust
html! {
  <div class=self.classes()></div>
}
```

<!--元组-->
```rust
html! {
  <div class=("class-1", "class-2")></div>
}
```

<!--Vector-->
```rust
html! {
  <div class=vec!["class-1", "class-2"]></div>
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

## 监听器

监听器属性需要传递一个由闭包包裹的 `Callback`。创建回调的方式取决于你希望你的应用程序如何响应监听器事件：

<!--DOCUSAURUS_CODE_TABS-->
<!--Component 处理器-->
```rust
struct MyComponent {
    link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                // 处理 Click
            }
        }
    }

    fn view(&self) -> Html {
        // 从组件 link 中创建回调来在组件中处理它
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--Agent 处理器-->
```rust
struct MyComponent {
    worker: Dispatcher<MyWorker>,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent {
            worker: MyWorker::dispatcher()
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // 从 worker 中创建回调来在另一个上下文中处理它
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--其他情况-->
```rust
struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // 创建一个短暂的回调
        let click_callback = Callback::from(|| {
            ConsoleService::new().log("clicked!");
        });

        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

