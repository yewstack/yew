crate::doc_page!(
    "Router",
    "/ja/docs/concepts/router",
    Content::new(vec![
        p(vec![
            link("https://crates.io/crates/yew-router", vec![text("crates.io にあるルータ")]),
        ]),
        p(vec![text(
            "シングルページアプリケーション(SPA)におけるルータは URL よってページを出し分けます。\
             リンクがクリックされたときに異なるリソースを要求するというデフォルトの動作の代わりに、\
             ルータはアプリケーション内の有効なルートを指すように URL をローカルに設定します。\
             ルータはこの変更を検出してから、何をレンダリングするかを決定します。"
        )]),
        h2(vec![text("コアとなる要素")]),
        h3(vec![code("Route")]),
        p(vec![text(
            "URL 内のドメインの後のすべてを表す文字列と、オプションで history API に保存されている状態を含みます。"
        )]),
        h3(vec![code("RouteService")]),
        p(vec![text("ブラウザとやりとりしてルーティングを決めます。")]),
        h3(vec![code("RouteAgent")]),
        p(vec![text(
            "RouteService を所有し、ルートが変更された際の更新を調整するために使用します。"
        )]),
        h3(vec![code("Switch")]),
        p(vec![
            code("Switch"),
            text("トレイトは"),
            code("Route"),
            text("をトレイトの実装する側の間で変換するために用いられます。"),
        ]),
        h3(vec![code("Router")]),
        p(vec![text(
            "Router コンポーネントは RouteAgent とやり取りし、エージェントがどうスイッチするか Routes を自動的に解決します。\
             これは、結果として得られるスイッチがどのように Html に変換されるかを指定できるようにするため、props を介して公開されます。"
        )]),
        h2(vec![text("ルータをどのように使うか")]),
        p(vec![text(
            "まず、アプリケーションのすべての状態を表す型を作成します。\
             これは通常は列挙型ですが、構造体もサポートされており、"
        ),
            code("Switch"),
            text(" を実装した他のアイテムを内部に入れ子にすることができることに注意してください。"),
        ]),
        p(vec![
            text("次に、"),
            code("Switch"),
            text("を型に継承させなければいけません。\
                  列挙型の場合は全ての variant は"),
            code("#[to = \"/some/route\"]"),
            text("とアノテーションされている必要があり、代わり構造体を用いている場合は構造体宣言が外部から見えるようにしてなければいけません。"),
        ]),
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
        admonition(
            AdmonitionType::Warning,
            None,
            vec![
                p(vec![
                    code("Switch"),
                    text("用の派生マクロによって生成された実装は、各 variant を最初から最後までの順にマッチさせようとするので、指定した"),
                    code("to"),
                    text("アノテーションのうち 2 つのルートにマッチする可能性がある場合は、最初のルートがマッチし、\
                          2 つ目のルートは試行されないことに注意してください。例えば、以下の"),
                    code("Switch"),
                    text("を定義した場合、マッチするルートは"),
                    code("AppRoute::Home"),
                    text("だけになります。"),
                ]),
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
            ]
        ),
        p(vec![
            text("また、"),
            code("#[to = \"\"]"),
            text("アノテーションの中で"),
            code("{}"),
            text("のバリエーションを使ってセクションをキャプチャすることもできます。"),
            code("{}"),
            text("は、次の区切り文字(コンテキストに応じて \"/\", \"?\", \"&\", \"#\" のいずれか) までのテキストをキャプチャします。"),
            code("{*}"),
            text("は、次の文字が一致するまでテキストをキャプチャすることを意味します。"),
            code("{<number>}"),
            text("は、指定した数の区切り文字が見つかるまでテキストをキャプチャすることを意味します\
                  (例: "),
            code("{2}"),
            text("は区切り文字が 2 つ見つかるまでキャプチャします)。"),
        ]),
        p(vec![
            text("名前付きフィールドを持つ構造体や列挙型の場合は、キャプチャグループ内で以下のようにフィールドの名前を指定する必要があります。"),
            code("{user_name}"),
            text(" または "),
            code("{*:age}"),
            text(" のように、キャプチャグループ内でフィールドの名前を指定しなければなりません。"),
        ]),
        p(vec![
            text("Switch トレイトは文字列よりも構造化されたキャプチャグループで動作します。"),
            code("Switch"),
            text("を実装した任意の型を指定することができます。\
                  そのため、キャプチャグループが "),
            code("usize"),
            text(" であることを指定することができ、URL のキャプチャ部分がそれに変換できない場合、variant はマッチしません。"),
        ]),
    ])
);
