pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition(
            AdmonitionType::Important,
            Some("改进文档"),
            vec![p(vec![text(
                "有在使用不同的编辑器？如有推荐，请随意添加您选择的编辑器的说明。",
            )])],
        ),
        h2(vec![text("为创建组件添加模板")]),
        h3(vec![text("JetBrains IDEs")]),
        ol(vec![
            li(vec![text(
                "从导航栏依次点击 File | Settings | Editor | Live Templates.",
            )]),
            li(vec![text(
                "选择 Rust 并点击 + 图标添加新的 Live Template。",
            )]),
            li(vec![text("根据需要给它一个的名称和描述。")]),
            li(vec![text("将以下代码片段粘贴到模板文本部分。")]),
            li(vec![text("在右下角更改适用性，选择 Rust > Item > Module")]),
        ]),
        p(vec![text("对于函数式组件，使用以下模板。")]),
        ul(vec![li(vec![
            text("(可选) 点击编辑变量，并给 "),
            code("tag"),
            text(" 一个合理的默认值，例如 \"div\"，用双引号。"),
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
        p(vec![text("对于结构体组件，可以使用以下更复杂的模板。")]),
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
                "从导航栏依次点击 File > Preferences > User Snippets.",
            )]),
            li(vec![text("选择 Rust 作为设置语言。")]),
            li(vec![text("在 JSON 文件中添加以下代码片段：")]),
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
        h2(vec![text("支持 "), code("html!"), text(" 宏")]),
        h3(vec![text("JetBrains IDEs")]),
        p(vec![text("Contribution Welcome!")]),
        h3(vec![text("VS Code")]),
        h4(vec![text("Rust-Yew 扩展")]),
        blockquote(vec![p(vec![
            text("这是一个"),
            bold(vec![text("正在进行中")]),
            text("的，"),
            bold(vec![text("由社区维护")]),
            text("的项目！"),
            link(
                "https://github.com/TechTheAwesome/code-yew-server",
                vec![text(
                    "请查看详细信息，并将相关的 bug 报告/问题/疑问直接发送到扩展的存储库",
                )],
            ),
        ])]),
        p(vec![
            text("Rust-Yew 扩展 "),
            link(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                vec![text("可以在 VSC Marketplace 上找到")],
            ),
            text("，提供语法高亮、重命名、悬停等功能。"),
        ]),
        p(vec![
            text("Emmet 支持应该可以直接使用，如果不能，请回退到编辑 "),
            code("settings.json"),
            text(" 文件："),
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
            text("下面的配置适用于 "),
            link("https://www.lazyvim.org", vec![text("LazyVim")]),
            text(" 配置和 lazy.vim 插件，请在 "),
            code("lua/plugins/nvim-lspconfig.lua"),
            text(" 中创建一个文件（或更新您的 "),
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
    "设置编辑器",
    "/zh-Hans/docs/getting-started/editor-setup",
    page_content()
);
