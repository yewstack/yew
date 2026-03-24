pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition!(
            AdmonitionType::Important,
            Some("改善ドキュメント"),
            p![text(
                "異なるエディタを使用していますか？おすすめがあれば、\
                 選択したエディタの説明を自由に追加してください。",
            )],
        ),
        h2![text("コンポーネント作成のためのテンプレートを追加")],
        h3![text("JetBrains IDEs")],
        ol![
            li![text(
                "ナビゲーションバーから順に File | Settings | Editor | Live Templates \
                 をクリックします。",
            )],
            li![text(
                "Rust を選択し、+ アイコンをクリックして新しい Live Template を追加します。",
            )],
            li![text("必要に応じて名前と説明を入力します。")],
            li![text(
                "以下のコードスニペットをテンプレートテキスト部分に貼り付けます。",
            )],
            li![text(
                "右下の適用範囲を変更し、Rust > Item > Module を選択します。",
            )],
        ],
        p![text(
            "関数型コンポーネントの場合、以下のテンプレートを使用します。",
        )],
        ul![li![
            text("(オプション) 変数を編集し、"),
            code("tag"),
            text(" に適切なデフォルト値（例：\"div\"）を設定します。"),
        ]],
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
        p![text(
            "構造体コンポーネントの場合、以下のより複雑なテンプレートを使用できます。",
        )],
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
        h3![text("VS Code")],
        ol![
            li![text(
                "ナビゲーションバーから順に File > Preferences > User Snippets をクリックします。",
            )],
            li![text("設定言語として Rust を選択します。")],
            li![text("以下の JSON ファイルにコードスニペットを追加します。",)],
        ],
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
        h2![code("html!"), text(" マクロのサポート")],
        h3![text("JetBrains IDEs")],
        p![text("Contribution Welcome!")],
        h3![text("VS Code")],
        h4![text("Rust-Yew 拡張機能")],
        blockquote![p![
            text("これは"),
            bold![text("進行中の")],
            text("、"),
            bold![text("コミュニティが維持している")],
            text("プロジェクトです！"),
            link!(
                "https://github.com/TechTheAwesome/code-yew-server",
                text(
                    "詳細を確認し、関連するバグ報告/問題/\
                     質問を直接拡張機能のリポジトリに送信してください",
                ),
            ),
        ]],
        p![
            text("Rust-Yew 拡張機能は "),
            link!(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                text("VSC Marketplace で見つけることができます"),
            ),
            text("、シンタックスハイライト、リネーム、ホバーなどの機能を提供します。"),
        ],
        p![
            text("Emmet サポートは直接使用できるはずですが、できない場合は "),
            code("settings.json"),
            text(" ファイルを編集してください："),
        ],
        code_block(
            "json",
            r#""emmet.includeLanguages": {
    "rust": "html",
}"#,
        ),
        h3![text("Neovim")],
        h4![text("Lazyvim")],
        blockquote![p![
            text("以下の設定は "),
            link!("https://www.lazyvim.org", text("LazyVim")),
            text(" 設定および lazy.vim プラグインに適用されます。"),
            code("lua/plugins/nvim-lspconfig.lua"),
            text(" にファイルを作成するか、既存の "),
            code("lspconfig"),
            text(" を更新してください："),
        ]],
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
