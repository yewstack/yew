pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition!(
            AdmonitionType::Important,
            Some("改进文档"),
            p!["有在使用不同的编辑器？如有推荐，请随意添加您选择的编辑器的说明。"],
        ),
        h2!["为创建组件添加模板"],
        h3!["JetBrains IDEs"],
        ol![
            li!["从导航栏依次点击 File | Settings | Editor | Live Templates."],
            li!["选择 Rust 并点击 + 图标添加新的 Live Template。"],
            li!["根据需要给它一个的名称和描述。"],
            li!["将以下代码片段粘贴到模板文本部分。"],
            li!["在右下角更改适用性，选择 Rust > Item > Module"],
        ],
        p!["对于函数式组件，使用以下模板。"],
        ul![li![
            "(可选) 点击编辑变量，并给 ",
            code("tag"),
            " 一个合理的默认值，例如 \"div\"，用双引号。",
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
        p!["对于结构体组件，可以使用以下更复杂的模板。"],
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
        h3!["VS Code"],
        ol![
            li!["从导航栏依次点击 File > Preferences > User Snippets."],
            li!["选择 Rust 作为设置语言。"],
            li!["在 JSON 文件中添加以下代码片段："],
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
        h2!["支持 ", code("html!"), " 宏"],
        h3!["JetBrains IDEs"],
        p!["Contribution Welcome!"],
        h3!["VS Code"],
        h4!["Rust-Yew 扩展"],
        blockquote![p![
            "这是一个",
            bold!["正在进行中"],
            "的，",
            bold!["由社区维护"],
            "的项目！",
            link!(
                "https://github.com/TechTheAwesome/code-yew-server",
                "请查看详细信息，并将相关的 bug 报告/问题/疑问直接发送到扩展的存储库",
            ),
        ]],
        p![
            "Rust-Yew 扩展 ",
            link!(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                "可以在 VSC Marketplace 上找到",
            ),
            "，提供语法高亮、重命名、悬停等功能。",
        ],
        p![
            "Emmet 支持应该可以直接使用，如果不能，请回退到编辑 ",
            code("settings.json"),
            " 文件：",
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
            "下面的配置适用于 ",
            link!("https://www.lazyvim.org", "LazyVim"),
            " 配置和 lazy.vim 插件，请在 ",
            code("lua/plugins/nvim-lspconfig.lua"),
            " 中创建一个文件（或更新您的 ",
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
}

crate::doc_page!(
    "设置编辑器",
    "/zh-Hans/docs/getting-started/editor-setup",
    page_content()
);
