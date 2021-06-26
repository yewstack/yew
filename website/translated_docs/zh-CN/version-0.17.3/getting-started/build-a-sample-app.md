---
title: 第一个简单的 App
id: version-0.17.3-build-a-sample-app
original_id: build-a-sample-app
---

首先创建一个新的 Rust 库（**重要：**通过传入<code>--lib</code> 来创建一个*库*，而不是一个<em>二进制文件）：</em>

```bash
cargo new --lib yew-app && cd yew-app
```

将`yew`和`wasm-bindgen`添加到您的依赖项中（最新版本[请参阅此处](https://docs.rs/yew)）

```toml
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
yew = "0.17"
wasm-bindgen = "0.2.67"
```

将以下模板复制到您的`src/lib.rs`文件中：

```rust
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // 如果新传入的属性(Properties)与之前获取到的不同，就应该返回 "true"。
        // 当前组件没有属性，所以只需要返回 "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick={self.link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
```

这份代码通过一个 `Model` 构建了你的根 `组件`，它会显示一个按钮，当你点击按钮时，`Model` 将会更新自己的状态。特别注意 `main()` 中的 `App::<Model>::new().mount_to_body()`，它会启动你的应用并将其挂载到页面的 `<body>` 标签中。如果你想使用任何动态属性来启动应用程序，则可以使用 `App::<Model>::new().mount_to_body_with_props(..)`。

最后，在你的应用中新建一个`static`文件夹，然后将一个`index.html`文件添加到这个文件夹内：

```bash
mkdir static
```

```markup
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew Sample App</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
```

## 运行你的应用程序!

最好是使用[`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/)来启动和运行应用。如果还未安装`wasm-pack`，请先执行 `cargo install wasm-pack` 安装，接下来就可以用以下指令来构建和启动开发服务器：

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static
```

`wasm-pack` 会在 `./static`目录下生成一个包，其中包含应用程序已编译的 WebAssembly ，以及一个 JavaScript 包装器，它将加载应用程序的 WebAssembly 二进制文件并运行它。

然后，使用您喜欢的 Web 服务器为`./static`下的文件提供服务。例如：

```bash
cargo +nightly install miniserve
miniserve ./static --index index.html
```
