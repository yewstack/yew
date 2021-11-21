---
title: Introduction
description: Components and their lifecycle hooks
---
## コンポーネントとは?

コンポーネントはYewを構成するブロックです。
コンポーネントは状態を管理し、自身をDOMへレンダリングすることができます。
コンポーネントはライフサイクルの機能がある`Component`トレイトを実装することによって作られます。

## ライフサイクル

:::important contribute
`Contribute to our docs:` [Add a diagram of the component lifecycle](https://github.com/yewstack/docs/issues/22)
:::

## ライフサイクルのメソッド

### Create

コンポーネントが作られると、`ComponentLink`と同様に親コンポーネントからプロパティを受け取ります。
プロパティはコンポーネントの状態を初期化するのに使われ、"link"はコールバックを登録したりコンポーネントにメッセージを送るのに使われます。

propsとlinkをコンポーネント構造体に格納するのが一般的です。
例えば:

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

コンポーネントは`view()`メソッドによってレイアウトを宣言します。
Yewは`html!`マクロによってHTMLとSVGノード、リスナー、子コンポーネントを宣言できます。
マクロはReactのJSXのような動きをしますが、JavaScriptの代わりにRustの式を用います。

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

使い方については[`html!`ガイド](html.md)をご確認ください。

### Rendered

`rendered()`コンポーネントのライフサイクルのメソッドは`view()`が処理されたてYewがコンポーネントをレンダリングした後、
ブラウザがページを更新する前に呼ばれます。
コンポーネントは、コンポーネントが要素をレンダリングした後にのみ実行できるアクションを実行するため、このメソッドを実装したい場合があります。
コンポーネントが初めてレンダリングされたかどうかは `first_render` パラメータで確認できます。

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
            if let Some(input) = self.node_ref.try_into::<InputElement>() {
                input.focus();
            }
        }
    }
}
```

:::tip note
ライフサイクルメソッドは実装の必要がなく、デフォルトでは何もしません。
:::

### Update

コンポーネントは動的で、非同期メッセージを受信するために登録することができます。
ライフサイクルメソッド `update()` はメッセージごとに呼び出されます。
これにより、コンポーネントはメッセージが何であったかに基づいて自身を更新し、自身を再レンダリングする必要があるかどうかを判断することができます。
メッセージは、HTML要素リスナーによってトリガーされたり、子コンポーネント、エージェント、サービス、またはFuturesによって送信されたりします。

`update()`がどのようなのかについての例は以下の通りです:

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

コンポーネントは親によって再レンダリングされることがあります。
このような場合、新しいプロパティを受け取り、再レンダリングを選択する可能性があります。
この設計では、プロパティを変更することで、親から子へのコンポーネントの通信が容易になります。

典型的な実装例は以下の通りです:

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

コンポーネントが DOM からアンマウントされた後、Yew は `destroy()` ライフサイクルメソッドを呼び出し、必要なクリーンアップ操作をサポートします。
このメソッドはオプションで、デフォルトでは何もしません。

## Associated Types

`Component`トレイトは2つの関連型があります: `Message`と`Properties`です。

```rust
impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}
```

`Message`はコンポーネントによって処理され、何らかの副作用を引き起こすことができるさまざまなメッセージを表します。
例えば、APIリクエストをトリガーしたり、UIコンポーネントの外観を切り替えたりする `Click` メッセージがあります。
コンポーネントのモジュールで `Msg` という名前の列挙型を作成し、それをコンポーネントのメッセージ型として使用するのが一般的です。
"message"を"msg"と省略するのも一般的です。

```rust
enum Msg {
    Click,
}
```

`Properties`は、親からコンポーネントに渡される情報を表します。
この型はProperties traitを実装していなければならず\(通常はこれを派生させることで\)、特定のプロパティが必須かオプションかを指定することができます。
この型は、コンポーネントの作成・更新時に使用されます。
コンポーネントのモジュール内に `Props` という構造体を作成し、それをコンポーネントの `Properties` 型として使用するのが一般的です。
”Properties”を"props"に短縮するのが一般的です。
Propsは親コンポーネントから継承されるので、アプリケーションのルートコンポーネントは通常`()`型の`Properties`を持ちます。
ルートコンポーネントのプロパティを指定したい場合は、`App::mount_with_props`メソッドを利用します。