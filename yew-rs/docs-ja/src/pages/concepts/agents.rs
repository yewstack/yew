pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![text("エージェント (Agents) は、タスクを Web Workers にオフロードする方法です。")]),
        p(vec![
            text("エージェントが並行して動作できるようにするために、Yew は "),
            link("https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers", vec![text("Web Workers")]),
            text(" を使用します。"),
        ]),
        h2(vec![text("ライフサイクル")]),
        themed_img("/img/agent-lifecycle-light.svg", "/img/agent-lifecycle-dark.svg", "agent lifecycle diagram"),
        h2(vec![text("エージェントの種類")]),
        h3(vec![text("範囲")]),
        ul(vec![
            li(vec![text("公開 - 任意の時点で、公開エージェントのインスタンスは最大で1つだけです。ブリッジはWeb Worker内でエージェントを生成するか、既に生成されたエージェントに接続します。ブリッジがこのエージェントに接続されていない場合、エージェントは消滅します。")]),
            li(vec![text("私有 - 新しいブリッジごとにWeb Worker内で新しいエージェントを生成します。これは、ブラウザと通信する共有だが独立した動作をコンポーネントから移動するのに適しています。接続されたブリッジが破棄されると、エージェントは消滅します。")]),
            li(vec![text("グローバル (WIP)")]),
        ]),
        h2(vec![text("エージェントとコンポーネント間の通信")]),
        h3(vec![text("通信ブリッジ (Bridges)")]),
        p(vec![text("通信ブリッジ（ブリッジ）は、コンポーネントとエージェント間の通信チャネルです。これにより、コンポーネントはエージェントにメッセージを送信し、エージェントからのメッセージを受信できます。")]),
        p(vec![
            code("use_bridge"),
            text(" フックは、関数コンポーネント内でブリッジを作成する機能も提供します。"),
        ]),
        h3(vec![text("ディスパッチャー (Dispatchers)")]),
        p(vec![text("ディスパッチャー（ディスパッチャー）は、コンポーネントとエージェント間の一方向通信を可能にし、コンポーネントがこの方法でエージェントにメッセージを送信します。")]),
        h2(vec![text("オーバーヘッド")]),
        p(vec![
            text("エージェントはWeb Workers（つまり、私有および公開）を使用します。メッセージの送受信時にシリアル化オーバーヘッドが発生します。エージェントは "),
            link("https://github.com/bincode-org/bincode", vec![text("bincode")]),
            text(" を使用して他のスレッドと通信するため、コストは関数を呼び出すだけの場合よりもはるかに高くなります。"),
        ]),
        h2(vec![text("さらなる読み物")]),
        ul(vec![
            li(vec![
                link("https://github.com/yewstack/yew/tree/master/examples/web_worker_fib", vec![text("web_worker_fib")]),
                text(" の例は、コンポーネントがエージェントにメッセージを送信し、エージェントからのメッセージを受信する方法を示しています。"),
            ]),
        ]),
    ])
}

crate::doc_page!(
    "エージェント (Agents)",
    "/ja/docs/concepts/agents",
    page_content()
);
