pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition(
            AdmonitionType::Important,
            Some("改進文檔"),
            vec![p(vec![text(
                "有在使用不同的編輯器？如有推薦，請隨意新增您選擇的編輯器的說明。",
            )])],
        ),
        h2(vec![text("為建立元件新增模板")]),
        h3(vec![text("JetBrains IDEs")]),
        ol(vec![
            li(vec![text(
                "從導覽列依序點擊 File | Settings | Editor | Live Templates.",
            )]),
            li(vec![text("選擇 Rust 並點選 + 圖示新增新的 Live Template.")]),
            li(vec![text("根據需要給它一個的名稱和描述。")]),
            li(vec![text("將以下程式碼片段貼到範本文字部分。")]),
            li(vec![text("在右下角更改適用性，選擇 Rust > Item > Module")]),
        ]),
        p(vec![text("對於函數式元件，使用以下模板。")]),
        ul(vec![li(vec![
            text("(可選) 點選編輯變量，並給 "),
            code("tag"),
            text(" 一個合理的預設值，例如 \"div\"，用雙引號。"),
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
        p(vec![text("對於結構體組件，可以使用以下更複雜的模板。")]),
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
                "從導覽列依序點選 File > Preferences > User Snippets.",
            )]),
            li(vec![text("選擇 Rust 作為設定語言。")]),
            li(vec![text("在 JSON 檔案中加入以下程式碼片段：")]),
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
        h2(vec![text("支援 "), code("html!"), text(" 宏")]),
        h3(vec![text("JetBrains IDEs")]),
        p(vec![text("Contribution Welcome!")]),
        h3(vec![text("VS Code")]),
        h4(vec![text("Rust-Yew 擴展")]),
        blockquote(vec![p(vec![
            text("這是一個"),
            bold(vec![text("正在進行中")]),
            text("的，"),
            bold(vec![text("由社區維護")]),
            text("的項目！ "),
            link(
                "https://github.com/TechTheAwesome/code-yew-server",
                vec![text(
                    "請查看詳細信息，並將相關的 bug 報告/問題/疑問直接發送到擴展的存儲庫",
                )],
            ),
        ])]),
        p(vec![
            text("Rust-Yew 擴充 "),
            link(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                vec![text("可在 VSC Marketplace 上找到")],
            ),
            text("，提供語法高亮、重新命名、懸停等功能。"),
        ]),
        p(vec![
            text("Emmet 支援應該可以直接使用，如果不能，請回退到編輯 "),
            code("settings.json"),
            text(" 檔案："),
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
            text("下面的配置適用於 "),
            link("https://www.lazyvim.org", vec![text("LazyVim")]),
            text(" 配置和 lazy.vim 插件，請在 "),
            code("lua/plugins/nvim-lspconfig.lua"),
            text(" 中建立一個檔案（或更新您的 "),
            code("lspconfig"),
            text("）："),
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
    "設定編輯器",
    "/zh-Hant/docs/getting-started/editor-setup",
    page_content()
);
