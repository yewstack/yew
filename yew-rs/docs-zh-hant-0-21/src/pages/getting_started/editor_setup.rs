crate::doc_page!(
    "設定編輯器",
    "/zh-Hant/docs/getting-started/editor-setup",
    Content::new(vec![
        admonition!(
            AdmonitionType::Important,
            Some("改進文檔"),
            p!["有在使用不同的編輯器？如有推薦，請隨意新增您選擇的編輯器的說明。"],
        ),
        h2!["為建立元件新增模板"],
        h3!["JetBrains IDEs"],
        ol![
            li!["從導覽列依序點擊 File | Settings | Editor | Live Templates."],
            li!["選擇 Rust 並點選 + 圖示新增新的 Live Template。"],
            li!["根據需要給它一個的名稱和描述。"],
            li!["將以下程式碼片段貼到範本文字部分。"],
            li!["在右下角更改適用性，選擇 Rust > Item > Module"],
        ],
        p!["對於函數式元件，使用以下模板。"],
        ul![li![
            "(可選) 點選編輯變量，並給 ",
            code("tag"),
            " 一個合理的預設值，例如 \"div\"，用雙引號。",
        ]],
        code_block_ignore(
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
        p!["對於結構體組件，可以使用以下更複雜的模板。"],
        code_block_ignore(
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
        h3!["VS Code"],
        ol![
            li!["從導覽列依序點選 File > Preferences > User Snippets."],
            li!["選擇 Rust 作為設定語言。"],
            li!["在 JSON 檔案中加入以下程式碼片段："],
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
        h2!["支援 ", code("html!"), " 宏"],
        h3!["JetBrains IDEs"],
        p!["Contribution Welcome!"],
        h3!["VS Code"],
        h4!["Rust-Yew 擴展"],
        blockquote![p![
            "這是一個",
            bold!["正在進行中"],
            "的，",
            bold!["由社區維護"],
            "的項目！ ",
            link!(
                "https://github.com/TechTheAwesome/code-yew-server",
                "請查看詳細信息，並將相關的 bug 報告/問題/疑問直接發送到擴展的存儲庫",
            ),
        ]],
        p![
            "Rust-Yew 擴充 ",
            link!(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                "可在 VSC Marketplace 上找到",
            ),
            "，提供語法高亮、重新命名、懸停等功能。",
        ],
        p![
            "Emmet 支援應該可以直接使用，如果不能，請回退到編輯 ",
            code("settings.json"),
            " 檔案：",
        ],
        code_block(
            "json",
            r#""emmet.includeLanguages": {
    "rust": "html",
}"#,
        ),
        h3!["Neovim"],
        h4!["Lazyvim"],
        blockquote![p![
            "下面的配置適用於",
            link!("https://www.lazyvim.org", "LazyVim"),
            " 配置和lazy.vim 插件，請在",
            code("lua/plugins/nvim-lspconfig.lua"),
            " 中建立一個檔案（或更新您的",
            code("lspconfig"),
            "）：",
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
    .with_description("Setting your code editor")
);
