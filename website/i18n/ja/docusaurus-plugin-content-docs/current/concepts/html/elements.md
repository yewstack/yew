---
title: Elements
description: Both HTML and SVG elements are supported
---
## タグ構造

要素のタグは`<... />`のような自己完結タグか、開始タグに対応した終了タグを持っている必要があります。

<!--DOCUSAURUS_CODE_TABS-->
<!--Open - Close-->
```rust
html! {
  <div id="my_div"></div>
}
```
<!--Invalid-->
```rust
html! {
  <div id="my_div"> // <- MISSING CLOSE TAG
}
```

<!--Self-closing-->
```rust
html! {
  <input id="my_input" />
}
```

<!--Invalid-->
```rust
html! {
  <input id="my_input"> // <- MISSING SELF-CLOSE
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

:::note
便利さのために、_普通は_終了タグを必要とする要素は自己完結タグとすることが**できます**。
例えば`html! { <div class="placeholder" /> }`と書くのは有効です。
:::

## 子

複雑にネストしたHTMLやSVGのレイアウトを書くのには以下のようにするのが楽です:
**
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

## クラス

要素へのクラスを特定する便利なやり方はたくさんあります:

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

## リスナー

リスナー属性はクロージャのラッパーである`Callback`に渡される必要があります。
コールバックをどのように作るかはアプリをリスナーイベントにどう反応させたいかによります。

<!--DOCUSAURUS_CODE_TABS-->
<!--Component handler-->
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
                // Handle Click
            }
        }
    }

    fn view(&self) -> Html {
        // Create a callback from a component link to handle it in a component
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
        // Create a callback from a worker to handle it in another context
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
        // Create an ephemeral callback
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

## イベントの型

:::note
以下のテーブルでは`yew`を`web-sys`と使う場合 (デフォルトでは使うようになっている) `web-sys`のイベントの型が使われるべきです。
`yew-stdweb`クレートを使う場合は`stdweb`のイベントの型を使用してください。
詳細については[`web-sys`と`stdweb`をどちらを使うべきかについてのドキュメント](https://yew.rs/docs/getting-started/choose-web-library)をご確認ください。
:::

:::note
以下のテーブルにある全てのイベントの型は`yew::events`で再エクスポートされています。
All the event types mentioned in the following table are re-exported under `yew::events`. Using the types from
`yew::events` makes it easier to ensure version compatibility than if you were to manually include `web-sys` 
or `stdweb` as dependencies in your crate because you won't end up using a version which conflicts with
the version Yew specifies. 
:::

| イベント名  | `web_sys` イベント型 | `stdweb` イベント型 |
| ----------- | -------------------- |  ------------------ |
| `onabort` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [ResourceAbortEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceAbortEvent.html) |
| `onauxclick` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [AuxClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.AuxClickEvent.html) |
| `onblur` | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html) | [BlurEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.BlurEvent.html) |
| `oncancel` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `oncanplay` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `oncanplaythrough` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onchange` | [ChangeData](https://docs.rs/yew/latest/yew/events/enum.ChangeData.html) | [ChangeData](https://docs.rs/yew-stdweb/latest/yew_stdweb/events/enum.ChangeData.html) |
| `onclick` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [ClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ClickEvent.html) |
| `onclose` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `oncontextmenu` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [ContextMenuEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ContextMenuEvent.html) |
| `oncuechange` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `ondblclick` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [DoubleClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DoubleClickEvent.html) |
| `ondrag` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html) | [DragEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEvent.html) |
| `ondragend` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html) | [DragEndEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEndEvent.html) |
| `ondragenter` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html) | [DragEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEnterEvent.html) |
| `ondragexit` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html) | [DragExitEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragExitEvent.html) |
| `ondragleave` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.htmk) | [DragLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragLeaveEvent.html) |
| `ondragover` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html) | [DragOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragOverEvent.html) |
| `ondragstart` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html) | [DragStartEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragStartEvent.html) |
| `ondrop` | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html) | [DragDropEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragDropEvent.html) |
| `ondurationchange` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onemptied` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onended` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onerror` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [ResourceErrorEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceErrorEvent.html) |
| `onfocus` | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html) | [FocusEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.FocusEvent.html) |
| `onformdata` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `oninput` | [InputData](https://docs.rs/yew/latest/yew/events/struct.InputData.html) | [InputData](https://docs.rs/yew-stdweb/latest/yew_stdweb/events/struct.InputData.html) |
| `oninvalid` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onkeydown` | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html) | [KeyDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyDownEvent.html) |
| `onkeypress` | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html) | [KeyPressEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyPressEvent.html) |
| `onkeyup` | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html) | [KeyUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyUpEvent.html) |
| `onload` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [ResourceLoadEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceLoadEvent.html) |
| `onloadeddata` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onloadedmetadata` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onloadstart` | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html) | [LoadStartEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LoadStartEvent.html) |
| `onmousedown` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [MouseDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseDownEvent.html) |
| `onmouseenter` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [MouseEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseEnterEvent.html) |
| `onmouseleave` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [MouseLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseLeaveEvent.html) |
| `onmousemove` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [MouseMoveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseMoveEvent.html) |
| `onmouseout` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [MouseOutEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseOutEvent.html) |
| `onmouseover` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [MouseOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseOverEvent.html) |
| `onmouseup` | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html) | [MouseUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseUpEvent.html) |
| `onpause` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onplay` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onplaying` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onprogress` | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html) | [ProgressEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ProgressEvent.html) |
| `onratechange` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onreset` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onresize` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [ResizeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResizeEvent.html) |
| `onscroll` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [ScrollEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ScrollEvent.html) |
| `onsecuritypolicyviolation` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onseeked` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onseeking` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onselect` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onslotchange` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [SlotChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SlotChangeEvent.html) |
| `onstalled` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onsubmit` | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html) | [SubmitEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SubmitEvent.html) |
| `onsuspend` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `ontimeupdate` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `ontoggle` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onvolumechange` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onwaiting` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onwheel` | [WheelEvent](https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html) | [MouseWheelEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseWheelEvent.html) |
| `oncopy` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `oncut` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onpaste` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onanimationcancel` | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html) | サポート無し |
| `onanimationend` | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html) | サポート無し |
| `onanimationiteration` | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html) | サポート無し |
| `onanimationstart` | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html) | サポート無し |
| `ongotpointercapture` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [GotPointerCaptureEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.GotPointerCaptureEvent.html) |
| `onloadend` | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html) | [LoadEndEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LoadEndEvent.html) |
| `onlostpointercapture` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [LostPointerCaptureEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LostPointerCaptureEvent.html) |
| `onpointercancel` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerCancelEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerCancelEvent.html) |
| `onpointerdown` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerDownEvent.html) |
| `onpointerenter` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerEnterEvent.html) |
| `onpointerleave` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLeaveEvent.html) |
| `onpointerlockchange` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [PointerLockChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLockChangeEvent.html) |
| `onpointerlockerror` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [PointerLockErrorEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLockErrorEvent.html) |
| `onpointermove` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerMoveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerMoveEvent.html) |
| `onpointerout` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerOutEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerOutEvent.html) |
| `onpointerover` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerOverEvent.html) |
| `onpointerup` | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html) | [PointerUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerUpEvent.html) |
| `onselectionchange` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | [SelectionChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SelectionChangeEvent.html) |
| `onselectstart` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `onshow` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html) | サポート無し |
| `ontouchcancel` | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html) | [TouchCancel](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchCancel.html) |
| `ontouchend` | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html) | [TouchEnd](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchEnd.html) |
| `ontouchmove` | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html) | [TouchMove](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchMove.html) |
| `ontouchstart` | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html) | [TouchStart](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchStart.html) |
| `ontransitioncancel` | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | サポート無し |
| `ontransitionend` | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | サポート無し |
| `ontransitionrun` | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | サポート無し |
| `ontransitionstart` | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | サポート無し |
