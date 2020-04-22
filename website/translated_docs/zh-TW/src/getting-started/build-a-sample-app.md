# 第一個簡單的 App

首先，先建立一個新的 binary 專案：

```bash
cargo new --bin yew-app && cd yew-app
```

在依賴庫裡加入 `yew` （最新的版號，請參考[這裡](https://docs.rs/yew)）

{% code title="Cargo.toml" %}
```text
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[dependencies]
yew = { version = "0.14.3", features = ["std_web"] }
```
{% endcode %}

將下面的模板複製進你的 `src/main.rs` 檔案：

{% code title="src/main.rs" %}
```rust
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

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
}
```
{% endcode %}

模板會建置名叫 `Model` 的根元件 `Component`，Model 會顯示一個按鈕，當你按下按鈕時， `Model` 會更新自己的狀態。需要特別注意的是，在 `main()` 裡的 `App::<Model>::new().mount_to_body()`，他會啟動你的 app 並且掛載 `Model` 裡的 HTML 到 `<body>` 標籤中。如果你想要在啟動應用程式時，帶入動態的屬性，你可以改用 `App::<Model>::new().mount_to_body_with_props(..)`。

## 執行你的 App！

建議使用 [`cargo-web`](https://github.com/koute/cargo-web)，可以最快執行專案。請用指令 `cargo intall cargo-web` 將工具安裝起來，再使用下面的指令執行專案：

```bash
cargo web start
```

`cargo-web` 會自動將 `wasm32-unknown-unknown` 加入你專案的編譯目標平台中，協助編譯你的 app，最後預設在 [http會自己更新畫面://\[::1\]:8000](http://[::1]:8000) 啟動。詳細參數請參考 `cargo web start --help`。

