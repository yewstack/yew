pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["エージェント (Agents) は、タスクを Web Workers にオフロードする方法です。"],
        p![
            "エージェントが並行して動作できるようにするために、Yew は ",
            link!("https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers", "Web Workers"),
            " を使用します。",
        ],
        h2!["ライフサイクル"],
        themed_img("/img/agent-lifecycle-light.svg", "/img/agent-lifecycle-dark.svg", "agent lifecycle diagram"),
        h2!["エージェントの種類"],
        h3!["範囲"],
        ul![
            li!["公開 - 任意の時点で、公開エージェントのインスタンスは最大で1つだけです。ブリッジはWeb Worker内でエージェントを生成するか、既に生成されたエージェントに接続します。ブリッジがこのエージェントに接続されていない場合、エージェントは消滅します。"],
            li!["私有 - 新しいブリッジごとにWeb Worker内で新しいエージェントを生成します。これは、ブラウザと通信する共有だが独立した動作をコンポーネントから移動するのに適しています。接続されたブリッジが破棄されると、エージェントは消滅します。"],
            li!["グローバル (WIP)"],
        ],
        h2!["エージェントとコンポーネント間の通信"],
        h3!["通信ブリッジ (Bridges)"],
        p!["通信ブリッジ（ブリッジ）は、コンポーネントとエージェント間の通信チャネルです。これにより、コンポーネントはエージェントにメッセージを送信し、エージェントからのメッセージを受信できます。"],
        p![
            code("use_bridge"),
            " フックは、関数コンポーネント内でブリッジを作成する機能も提供します。",
        ],
        h3!["ディスパッチャー (Dispatchers)"],
        p!["ディスパッチャー（ディスパッチャー）は、コンポーネントとエージェント間の一方向通信を可能にし、コンポーネントがこの方法でエージェントにメッセージを送信します。"],
        h2!["オーバーヘッド"],
        p![
            "エージェントはWeb Workers（つまり、私有および公開）を使用します。メッセージの送受信時にシリアル化オーバーヘッドが発生します。エージェントは ",
            link!("https://github.com/bincode-org/bincode", "bincode"),
            " を使用して他のスレッドと通信するため、コストは関数を呼び出すだけの場合よりもはるかに高くなります。",
        ],
        h2!["さらなる読み物"],
        ul![
            li![
                link!("https://github.com/yewstack/yew/tree/master/examples/web_worker_fib", "web_worker_fib"),
                " の例は、コンポーネントがエージェントにメッセージを送信し、エージェントからのメッセージを受信する方法を示しています。",
            ],
        ],
    ])
}

crate::doc_page!(
    "エージェント (Agents)",
    "/ja/docs/concepts/agents",
    page_content()
);
