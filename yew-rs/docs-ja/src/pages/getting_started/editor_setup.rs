pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition(
            AdmonitionType::Important,
            Some("改善ドキュメント"),
            vec![p(vec![text(
                "異なるエディタを使用していますか？おすすめがあれば、\
                 選択したエディタの説明を自由に追加してください。",
            )])],
        ),
        h2(vec![text("コンポーネント作成のためのテンプレートを追加")]),
        h3(vec![text("JetBrains IDEs")]),
        ol(vec![
            li(vec![text(
                "ナビゲーションバーから順に File | Settings | Editor | Live Templates \
                 をクリックします。",
            )]),
            li(vec![text(
                "Rust を選択し、+ アイコンをクリックして新しい Live Template を追加します。",
            )]),
            li(vec![text("必要に応じて名前と説明を入力します。")]),
            li(vec![text(
                "以下のコードスニペットをテンプレートテキスト部分に貼り付けます。",
            )]),
            li(vec![text(
                "右下の適用範囲を変更し、Rust > Item > Module を選択します。",
            )]),
        ]),
        p(vec![text(
            "関数型コンポーネントの場合、以下のテンプレートを使用します。",
        )]),
        ul(vec![li(vec![
            text("(オプション) 変数を編集し、"),
            code("tag"),
            text(" に適切なデフォルト値（例：\"div\"）を設定します。"),
        ])]),
        code_block(
            "rust",
            r#"#[derive(PartialEq, Properties)]
pub struct $Name$Props {
}

#[component]
pub fn $Name$(props: &$Name$Props) -> Html {
    html! {
        <$tag$>$END$</$tag$>
    }
}"#,
        ),
        p(vec![text(
            "構造体コンポーネントの場合、以下のより複雑なテンプレートを使用できます。",
        )]),
        code_block(
            "rust",
            r#"struct $NAME$;

enum $NAME$Msg {
}

impl Component for $NAME$ {
    type Message = $NAME$Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            $HTML$
        }
    }
}"#,
        ),
        h3(vec![text("VS Code")]),
        ol(vec![
            li(vec![text(
                "ナビゲーションバーから順に File > Preferences > User Snippets をクリックします。",
            )]),
            li(vec![text("設定言語として Rust を選択します。")]),
            li(vec![text(
                "以下の JSON ファイルにコードスニペットを追加します。",
            )]),
        ]),
        code_block(
            "json",
            r##"{
    "New Yew function component": {
        "prefix": "yewfc",
        "body": [
            "#[derive(PartialEq, Properties)]",
            "pub struct ${1:ComponentName}Props {}",
            "",
            "#[component]",
            "pub fn $1(props: &${1}Props) -> Html {",
            "    let ${1}Props {} = props;",
            "    html! {",
            "        <${2:div}>$0</${2}>",
            "    }",
            "}"
        ],
        "description": "Create a minimal Yew function component"
    },
    "New Yew struct component": {
        "prefix": "yewsc",
        "body": [
            "pub struct ${1:ComponentName};",
            "",
            "pub enum ${1}Msg {",
            "}",
            "",
            "impl Component for ${1} {",
            "    type Message = ${1}Msg;",
            "    type Properties = ();",
            "",
            "    fn create(ctx: &Context<Self>) -> Self {",
            "        Self",
            "    }",
            "",
            "    fn view(&self, ctx: &Context<Self>) -> Html {",
            "        html! {",
            "            $0",
            "        }",
            "    }",
            "}"
        ],
        "description": "Create a new Yew component with a message enum"
    }
}"##,
        ),
        h2(vec![code("html!"), text(" マクロのサポート")]),
        h3(vec![text("JetBrains IDEs")]),
        p(vec![text("Contribution Welcome!")]),
        h3(vec![text("VS Code")]),
        h4(vec![text("Rust-Yew 拡張機能")]),
        blockquote(vec![p(vec![
            text("これは"),
            bold(vec![text("進行中の")]),
            text("、"),
            bold(vec![text("コミュニティが維持している")]),
            text("プロジェクトです！"),
            link(
                "https://github.com/TechTheAwesome/code-yew-server",
                vec![text(
                    "詳細を確認し、関連するバグ報告/問題/\
                     質問を直接拡張機能のリポジトリに送信してください",
                )],
            ),
        ])]),
        p(vec![
            text("Rust-Yew 拡張機能は "),
            link(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                vec![text("VSC Marketplace で見つけることができます")],
            ),
            text("、シンタックスハイライト、リネーム、ホバーなどの機能を提供します。"),
        ]),
        p(vec![
            text("Emmet サポートは直接使用できるはずですが、できない場合は "),
            code("settings.json"),
            text(" ファイルを編集してください："),
        ]),
        code_block(
            "json",
            r#""emmet.includeLanguages": {
    "rust": "html",
}"#,
        ),
        h3(vec![text("Neovim")]),
        h4(vec![text("Lazyvim")]),
        blockquote(vec![p(vec![
            text("以下の設定は "),
            link("https://www.lazyvim.org", vec![text("LazyVim")]),
            text(" 設定および lazy.vim プラグインに適用されます。"),
            code("lua/plugins/nvim-lspconfig.lua"),
            text(" にファイルを作成するか、既存の "),
            code("lspconfig"),
            text(" を更新してください："),
        ])]),
        code_block(
            "json",
            r#"return {
  {
    "neovim/nvim-lspconfig",
    init_options = {
      userLanguages = {
        eelixir = "html-eex",
        eruby = "erb",
        rust = "html",
      },
    },
  },
}"#,
        ),
    ])
}

crate::doc_page!(
    "エディタの設定",
    "/ja/docs/getting-started/editor-setup",
    page_content()
);
