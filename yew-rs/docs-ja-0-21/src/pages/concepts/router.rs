crate::doc_page!(
    "Router",
    "/ja/docs/concepts/router",
    Content::new(vec![
        p![link!(
            "https://crates.io/crates/yew-router",
            "crates.io にあるルータ"
        ),],
        p!["シングルページアプリケーション(SPA)におけるルータは URL \
            よってページを出し分けます。\
            リンクがクリックされたときに異なるリソースを要求するというデフォルトの動作の代わりに、\
            ルータはアプリケーション内の有効なルートを指すように URL \
            をローカルに設定します。ルータはこの変更を検出してから、\
            何をレンダリングするかを決定します。"],
        h2!["コアとなる要素"],
        h3![code("Route")],
        p![
            "URL 内のドメインの後のすべてを表す文字列と、オプションで history API \
             に保存されている状態を含みます。"
        ],
        h3![code("RouteService")],
        p!["ブラウザとやりとりしてルーティングを決めます。"],
        h3![code("RouteAgent")],
        p!["RouteService を所有し、ルートが変更された際の更新を調整するために使用します。"],
        h3![code("Switch")],
        p![
            code("Switch"),
            "トレイトは",
            code("Route"),
            "をトレイトの実装する側の間で変換するために用いられます。",
        ],
        h3![code("Router")],
        p![
            "Router コンポーネントは RouteAgent とやり取りし、エージェントがどうスイッチするか \
             Routes を自動的に解決します。これは、結果として得られるスイッチがどのように Html \
             に変換されるかを指定できるようにするため、props を介して公開されます。"
        ],
        h2!["ルータをどのように使うか"],
        p![
            "まず、アプリケーションのすべての状態を表す型を作成します。これは通常は列挙型ですが、\
             構造体もサポートされており、",
            code("Switch"),
            " を実装した他のアイテムを内部に入れ子にすることができることに注意してください。",
        ],
        p![
            "次に、",
            code("Switch"),
            "を型に継承させなければいけません。列挙型の場合は全ての variant は",
            code("#[to = \"/some/route\"]"),
            "とアノテーションされている必要があり、\
             代わり構造体を用いている場合は構造体宣言が外部から見えるようにしてなければいけません。\
             ",
        ],
        code_block(
            "rust",
            r#"#[derive(Switch)]
enum AppRoute {
  #[to="/login"]
  Login,
  #[to="/register"]
  Register,
  #[to="/delete_account"]
  Delete,
  #[to="/posts/{id}"]
  ViewPost(i32),
  #[to="/posts/view"]
  ViewPosts,
  #[to="/"]
  Home
}"#
        ),
        admonition![
            AdmonitionType::Warning,
            None,
            p![
                code("Switch"),
                "用の派生マクロによって生成された実装は、各 variant \
                 を最初から最後までの順にマッチさせようとするので、指定した",
                code("to"),
                "アノテーションのうち 2 \
                 つのルートにマッチする可能性がある場合は、最初のルートがマッチし、2 \
                 つ目のルートは試行されないことに注意してください。例えば、以下の",
                code("Switch"),
                "を定義した場合、マッチするルートは",
                code("AppRoute::Home"),
                "だけになります。",
            ],
            code_block(
                "rust",
                r#"#[derive(Switch)]
enum AppRoute {
  #[to="/"]
  Home,
  #[to="/login"]
  Login,
  #[to="/register"]
  Register,
  #[to="/delete_account"]
  Delete,
  #[to="/posts/{id}"]
  ViewPost(i32),
  #[to="/posts/view"]
  ViewPosts,
}"#
            ),
        ],
        p![
            "また、",
            code("#[to = \"\"]"),
            "アノテーションの中で",
            code("{}"),
            "のバリエーションを使ってセクションをキャプチャすることもできます。",
            code("{}"),
            "は、次の区切り文字(コンテキストに応じて \"/\", \"?\", \"&\", \"#\" のいずれか) \
             までのテキストをキャプチャします。",
            code("{*}"),
            "は、次の文字が一致するまでテキストをキャプチャすることを意味します。",
            code("{<number>}"),
            "は、指定した数の区切り文字が見つかるまでテキストをキャプチャすることを意味します(例: ",
            code("{2}"),
            "は区切り文字が 2 つ見つかるまでキャプチャします)。",
        ],
        p![
            "名前付きフィールドを持つ構造体や列挙型の場合は、\
             キャプチャグループ内で以下のようにフィールドの名前を指定する必要があります。",
            code("{user_name}"),
            " または ",
            code("{*:age}"),
            " のように、キャプチャグループ内でフィールドの名前を指定しなければなりません。",
        ],
        p![
            "Switch トレイトは文字列よりも構造化されたキャプチャグループで動作します。",
            code("Switch"),
            "を実装した任意の型を指定することができます。そのため、キャプチャグループが ",
            code("usize"),
            " であることを指定することができ、URL \
             のキャプチャ部分がそれに変換できない場合、variant はマッチしません。",
        ],
    ])
    .with_description("Yew's official router")
);
