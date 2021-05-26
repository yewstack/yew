---
description: HTML 與 SVG 元件都支援
---

# Elements

## 標籤結構

 元件標籤都必須要是自封閉的標籤 `<... />` 或是跟開啟標籤對應的關閉標籤。

<!--DOCUSAURUS_CODE_TABS-->
<!--Open - Close-->
```rust
html! {
  <div id="my_div"></div>
}
```

<!--INVALID-->
```rust
html! {
  <div id="my_div"> // <- 缺少關閉標籤
}
```

<!--Self-Closing-->
```rust
html! {
  <input id="my_input" />
}
```

<!--INVALID-->
```rust
html! {
  <input id="my_input"> // <- 缺少自封閉標籤語法
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

:::note
為了方便起見，通常需要關閉標籤的元件，也都可以用自封閉標籤表示。例如，寫 `html! { <div class="placeholder" /> }` 是合法的。
:::

## 子結點

輕鬆寫出複雜巢狀的 HTML 與 SVG 架構：

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

你很多方便的選項可以寫元件裡的 class：

<!--DOCUSAURUS_CODE_TABS-->
<!--Literal-->
```rust
html! {
  <div class="container"></div>
}
```

<!--Multiple-->
```rust
html! {
  <div class="container center-align"></div>
}
```

<!--Interpolated-->
```rust
html! {
  <div class=format!("{}-container", size)></div>
}
```

<!--Expression-->
```rust
html! {
  <div class=self.classes()></div>
}
```

<!--Tuple-->
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

## 監聽

監聽器的屬性必須要傳入一個 `Callback` ，他封裝了閉包。callback 的內容取決於，當觸發監聽事件時，你希望應用程式有什麼反應：

<!--DOCUSAURUS_CODE_TABS-->
<!--Component Handler-->
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
                // 處理點擊事件
            }
        }
    }

    fn view(&self) -> Html {
        // 從一個元件連結中，建立一個 callback 並在元件中處理他
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--Agent Handler-->
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
        // 從一個 worker 中建立一個 callback，並在其他的 context 中處理他
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--Other Cases-->
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
        // 建立一個臨時的 callback
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

