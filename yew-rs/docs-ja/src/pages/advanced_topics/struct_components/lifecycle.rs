pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            code("Component"),
            text(" トレイトには、実装する必要がある多くのメソッドがあります。Yew はコンポーネントのライフサイクルのさまざまな段階でこれらのメソッドを呼び出します。"),
        ]),
        h2(vec![text("ライフサイクル")]),
        admonition(AdmonitionType::Important, Some("ドキュメントの改善"), vec![
            p(vec![
                code("ドキュメントに貢献する："),
                text(" "),
                link("https://github.com/yewstack/yew/issues/1915", vec![text("カスタムライフサイクルを持つコンポーネントの例を追加")]),
            ]),
        ]),
        h2(vec![text("ライフサイクルメソッド")]),
        h3(vec![text("Create")]),
        p(vec![
            text("コンポーネントが作成されるとき、それは親コンポーネントからプロパティを受け取り、それらは "),
            code("create"),
            text(" メソッドに渡される "),
            code("Context<Self>"),
            text(" に保存されます。これらのプロパティはコンポーネントの状態を初期化するために使用でき、\"link\" はコールバックを登録したり、コンポーネントにメッセージを送信したりするために使用できます。"),
        ]),
        code_block("rust", r#"use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = Props;

    // highlight-start
    fn create(ctx: &Context<Self>) -> Self {
        MyComponent
    }
    // highlight-end

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // 具体的な実装
        }
    }
}"#),
        h3(vec![text("View")]),
        p(vec![
            code("view"),
            text(" メソッドは、コンポーネントがDOMにどのようにレンダリングされるべきかを記述することを可能にします。Rust関数を使用してHTMLに似たコードを書くことは非常に混乱する可能性があるため、Yewは"),
            code("html!"),
            text("マクロを提供しています。これにより、HTMLおよびSVGノードを宣言し（およびそれらに属性とイベントリスナーを追加し）、子コンポーネントを便利にレンダリングする方法が提供されます。このマクロは、ReactのJSXに似ています（プログラミング言語の違いを除いて）。一つの違いは、YewがSvelteのようなプロパティの簡略化された構文を提供している点です。ここでは、"),
            code("{onclick}"),
            text("とだけ書くことができ、"),
            code("onclick={onclick}"),
            text("と書く必要はありません。"),
        ]),
        code_block("rust", r#"use yew::{Component, Context, html, Html, Properties};

enum Msg {
    Click,
}

#[derive(PartialEq, Properties)]
struct Props {
    button_text: String,
}

struct MyComponent;

impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // highlight-start
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Click);
        html! {
            <button {onclick}>{ &ctx.props().button_text }</button>
        }
    }
    // highlight-end
}"#),
        p(vec![
            text("使用方法の詳細については、"),
            link("/ja/docs/concepts/html", vec![text("html! ガイド")]),
            text(" を参照してください。"),
        ]),
        h3(vec![text("Rendered")]),
        p(vec![
            code("rendered"),
            text(" コンポーネントライフサイクルメソッドは、"),
            code("view"),
            text(" が呼び出され、Yew がその結果を DOM にレンダリングした後、ブラウザがページを更新する前に呼び出されます。このメソッドは、コンポーネントが要素をレンダリングした後にのみ完了できる操作を実行したい場合に非常に便利です。また、"),
            code("first_render"),
            text(" という名前のパラメーターがあり、この関数が最初のレンダリング時に呼び出されたか、後続のレンダリング時に呼び出されたかを判断するために使用できます。"),
        ]),
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::{
    Component, Context, html, Html, NodeRef,
};

pub struct MyComponent {
    node_ref: NodeRef,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input ref={self.node_ref.clone()} type="text" />
        }
    }

    // highlight-start
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }
    // highlight-end
}"#),
        admonition(AdmonitionType::Tip, Some("note"), vec![
            p(vec![text("このライフサイクルメソッドは実装する必要はなく、デフォルトでは何も実行しません。")]),
        ]),
        h3(vec![text("Update")]),
        p(vec![
            text("コンポーネントとの通信は主にメッセージを通じて行われ、これらのメッセージは "),
            code("update"),
            text(" ライフサイクルメソッドによって処理されます。これにより、コンポーネントはメッセージに基づいて自身を更新し、再レンダリングが必要かどうかを判断できます。メッセージはイベントリスナー、子コンポーネント、エージェント、サービス、またはフューチャーによって送信されることがあります。"),
        ]),
        p(vec![
            text("以下は "),
            code("update"),
            text(" の実装例です："),
        ]),
        code_block("rust", r#"use yew::{Component, Context, html, Html};

// highlight-start
pub enum Msg {
    SetInputEnabled(bool)
}
// highlight-end

struct MyComponent {
    input_enabled: bool,
}

impl Component for MyComponent {
    // highlight-next-line
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_enabled: false,
        }
    }

    // highlight-start
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInputEnabled(enabled) => {
                if self.input_enabled != enabled {
                    self.input_enabled = enabled;
                    true // 再レンダリング
                } else {
                    false
                }
            }
        }
    }
    // highlight-end

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // 具体的な実装
        }
    }

}"#),
        h3(vec![text("Changed")]),
        p(vec![text("コンポーネントは親コンポーネントによって再レンダリングされることがあります。この場合、新しいプロパティを受け取り、再レンダリングが必要になることがあります。この設計により、プロパティの値を変更するだけで親子コンポーネント間の通信が促進されます。プロパティが変更されると、デフォルトの実装によりコンポーネントが再レンダリングされます。")]),
        h3(vec![text("Destroy")]),
        p(vec![
            text("コンポーネントがDOMからアンマウントされると、Yewは"),
            code("destroy"),
            text("ライフサイクルメソッドを呼び出します。コンポーネントが破棄される前にクリーンアップ操作を実行する必要がある場合に便利です。このメソッドはオプションであり、デフォルトでは何も実行しません。"),
        ]),
        h3(vec![text("無限ループ")]),
        p(vec![text("Yewのライフサイクルメソッドでは無限ループが発生する可能性がありますが、それは各レンダリング後に同じコンポーネントを更新し、その更新が再レンダリングを要求する場合にのみ発生します。")]),
        p(vec![text("以下は簡単な例です：")]),
        code_block("rust", r#"use yew::{Context, Component, Html};

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // どのメッセージでも常に再レンダリングを要求します
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // レンダリングする内容は重要ではありません
        Html::default()
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        // この新しいメッセージを使用してコンポーネントを更新するように要求します
        ctx.link().send_message(());
    }
}"#),
        p(vec![text("ここで何が起こっているのか見てみましょう：")]),
        ol(vec![
            li(vec![
                code("create"),
                text(" 関数を使用してコンポーネントを作成します。"),
            ]),
            li(vec![
                code("view"),
                text(" メソッドを呼び出して、Yew がブラウザの DOM にレンダリングする内容を知ることができます。"),
            ]),
            li(vec![
                code("rendered"),
                text(" メソッドを呼び出し、"),
                code("Context"),
                text(" リンクを使用して更新メッセージをスケジュールします。"),
            ]),
            li(vec![text("Yew がレンダリングフェーズを完了します。")]),
            li(vec![text("Yew はスケジュールされたイベントをチェックし、更新メッセージキューが空でないことを確認してメッセージを処理します。")]),
            li(vec![
                code("update"),
                text(" メソッドを呼び出し、変更が発生し、コンポーネントが再レンダリングする必要があることを示す "),
                code("true"),
                text(" を返します。"),
            ]),
            li(vec![text("ステップ2に戻ります。")]),
        ]),
        p(vec![
            code("rendered"),
            text(" メソッドで更新をスケジュールすることは依然として可能であり、これは通常便利ですが、その際にはこのループをどのように終了させるかを考慮してください。"),
        ]),
        h2(vec![text("関連タイプ")]),
        p(vec![
            code("Component"),
            text(" トレイトには、"),
            code("Message"),
            text(" と "),
            code("Properties"),
            text(" の2つの関連タイプがあります。"),
        ]),
        code_block("rust", r#"impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}"#),
        p(vec![
            code("Message"),
            text(" タイプは、イベントが発生した後にコンポーネントにメッセージを送信するために使用されます。例えば、ユーザーがボタンをクリックしたり、ページをスクロールしたりしたときに何かを実行したい場合があります。コンポーネントは通常、複数のイベントに応答する必要があるため、"),
            code("Message"),
            text(" タイプは通常、処理するイベントごとにバリアントを持つ列挙型です。"),
        ]),
        p(vec![
            text("コードベースを整理する際には、"),
            code("Message"),
            text(" タイプの定義をコンポーネントを定義する同じモジュールに含めるのが賢明です。メッセージタイプの命名に一貫した命名規則を採用することが役立つ場合があります。一つのオプション（唯一のオプションではありませんが）は、タイプを "),
            code("ComponentNameMsg"),
            text(" と命名することです。例えば、コンポーネントが "),
            code("Homepage"),
            text(" と名付けられている場合、タイプを "),
            code("HomepageMsg"),
            text(" と命名することができます。"),
        ]),
        code_block("rust", r#"enum Msg {
    Click,
    FormInput(String)
}"#),
        p(vec![
            code("Properties"),
            text(" は、親コンポーネントからコンポーネントに渡される情報を表します。この型は "),
            code("Properties"),
            text(" トレイトを実装する必要があり（通常はそれを派生させる）、特定のプロパティが必須かオプションかを指定できます。コンポーネントの作成および更新時にこの型が使用されます。コンポーネントのモジュール内で "),
            code("Props"),
            text(" という名前の構造体を作成し、それをコンポーネントの "),
            code("Properties"),
            text(" 型として使用するのが一般的な方法です。通常、\"properties\" は \"props\" と略されます。プロパティは親コンポーネントから渡されるため、アプリケーションのルートコンポーネントは通常、"),
            code("Properties"),
            text(" 型として "),
            code("()"),
            text(" を持ちます。ルートコンポーネントにプロパティを指定する場合は、"),
            code("App::mount_with_props"),
            text(" メソッドを使用します。"),
        ]),
        admonition(AdmonitionType::Info, None, vec![
            p(vec![link(
                "/ja/docs/advanced-topics/struct-components/properties",
                vec![text("プロパティに関する詳細はこちら")],
            )]),
        ]),
        h2(vec![text("ライフサイクルコンテキスト")]),
        p(vec![text("すべてのコンポーネントライフサイクルメソッドは、コンテキストオブジェクトを受け取ります。このオブジェクトは、コンポーネントのスコープへの参照を提供し、コンポーネントにメッセージを送信したり、コンポーネントに渡されたプロパティを取得したりすることができます。")]),
    ])
}

crate::doc_page!(
    "ライフサイクル",
    "/ja/docs/advanced-topics/struct-components/lifecycle",
    page_content()
);
