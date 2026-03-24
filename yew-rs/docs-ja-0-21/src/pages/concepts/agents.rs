crate::doc_page!("Agents", "/ja/docs/concepts/agents",
    Content::new(vec![
        p![
            text("エージェントは Angular の"),
            link!("https://angular.io/guide/architecture-services", text("サービス")),
            text("に似ており(ただし依存性インジェクションはありません)、"),
            link!("https://en.wikipedia.org/wiki/Actor_model", text("アクターモデル")),
            text("を提供します。\
                  エージェントはコンポーネント階層のどこに位置するかに関わらず、コンポーネント間でメッセージをルーティングしたり、\
                  共有状態を作成したり、UI をレンダリングするメインスレッドから計算量の多いタスクをオフロードするために使用することができます。\
                  また、Yew アプリケーションがタブをまたいで通信できるようにするためのエージェントのサポートも(将来的には)計画されています。"),
        ],
        p![
            text("エージェントが並行に動くように Yew は"),
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers",
                text("web-workers")
            ),
            text("を使用しています。"),
        ],
        h2![text("ライフサイクル")],
        img("https://user-images.githubusercontent.com/42674621/79125224-b6481d80-7d95-11ea-8e6a-ab9b52d1d8ac.png", "エージェントのライフサイクル"),
        h2![text("エージェントの種類")],
        h3![text("Reaches")],
        ul![
            li_blocks![
                p![
                    text("Context - Context エージェントのインスタンスは、常に最大 1 つ存在します。\
                          Bridges は、UI スレッド上で既にスポーンされたエージェントをスポーンするか、接続します。\
                          これは、コンポーネントまたは他のエージェント間の状態を調整するために使用することができます。\
                          このエージェントに Bridges が接続されていない場合、このエージェントは消滅します。"),
                ],
            ],
            li_blocks![
                p![
                    text("Job - 新しいブリッジごとに UI スレッド上で新しいエージェントをスポーンします。\
                          これは、ブラウザと通信する共有されているが独立した動作をコンポーネントの外に移動させるのに適しています。\
                          (TODO 確認) タスクが完了すると、エージェントは消えます。"),
                ],
            ],
            li![text("Public - Context と同じですが、独自の web worker で動作します。")],
            li![text("Private - Job と同じですが、独自の web worker で動作します。")],
            li![text("Global (WIP)")],
        ],
        h2![text("エージェントとコンポーネントのやり取り")],
        h3![text("Bridges")],
        p![
            text("Bridge は、エージェントとコンポーネント間の双方向通信を可能にします。\
                  また、Bridge はエージェント同士の通信を可能にします。"),
        ],
        h3![text("Dispatchers")],
        p![
            text("Dispatcher は、コンポーネントとエージェント間の一方向通信を可能にします。\
                  Dispatcher は、コンポーネントがエージェントにメッセージを送信することを可能にします。"),
        ],
        h2![text("オーバーヘッド")],
        p![
            text("独自の独立した web worker(プライベートとパブリック)にあるエージェントは、\
                  送受信するメッセージにシリアライズするオーバーヘッドが発生します。\
                  他のスレッドとの通信には"),
            link!("https://github.com/servo/bincode", text("bincode")),
            text("を使用するので、関数を呼び出すよりもコストはかなり高くなります。\
                  計算コストがメッセージの受け渡しコストを上回る場合を除き、ロジックを UI スレッドエージェント(Job または Context)に格納する必要があります。"),
        ],
        h2![text("参考資料")],
        ul![
            li![
                link!(
                    "https://github.com/yewstack/yew/tree/master/examples/web_worker_fib",
                    text("web_worker_fib")
                ),
                text("の例でコンポーネントがどのようにエージェントと通信させているかがわかります。"),
            ],
        ],
    ])
);
