pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition!(
            AdmonitionType::Important,
            Some("contribute"),
            p![text(
                "Using a different editor? Feel free to add instructions for your editor of \
                 choice.",
            )],
        ),
        h2![text("Add a template for creating components")],
        h3![text("JetBrains IDEs")],
        ol![
            li![text(
                "Navigate to File | Settings | Editor | Live Templates.",
            )],
            li![text(
                "Select Rust and click on the + icon to add a new Live Template.",
            )],
            li![text("Give it a name and description of your preference.",)],
            li![text(
                "Paste the following snippet(s) into the Template Text section.",
            )],
            li![text(
                "Change the applicability on the lower right, select Rust > Item > Module",
            )],
        ],
        p![text("For function components, use the following template.",)],
        ul![li![
            text("(Optional) Click on Edit Variable and give "),
            code("tag"),
            text(" a reasonable default value like \"div\", with double quotes."),
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
        p![text(
            "For struct components, you can use the following more complicated template.",
        )],
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
        h3![text("VS Code")],
        ol![
            li![text("Navigate to File > Preferences > User Snippets.",)],
            li![text("Select Rust as the language.")],
            li![text("Add the following snippet in the snippet JSON file:",)],
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
        h2![text("Support for the "), code("html!"), text(" macro"),],
        h3![text("JetBrains IDEs")],
        p![text("Contribution Welcome!")],
        h3![text("VS Code")],
        h4![text("The Rust-Yew extension")],
        blockquote![p![
            text("This is a "),
            bold![text("work in progress")],
            text(", and "),
            bold![text("community maintained")],
            text(" project! "),
            link!(
                "https://github.com/TechTheAwesome/code-yew-server",
                text(
                    "Please see details and direct related bug reports / issues / questions over \
                     to the extension's repository",
                ),
            ),
        ]],
        p![
            text("The Rust-Yew extension is "),
            link!(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                text("available on VSC Marketplace"),
            ),
            text(", providing syntax highlighting, renames, hover, and more."),
        ],
        p![
            text(
                "Emmet support should work out of the box; if not, please fall back to editing \
                 the ",
            ),
            code("settings.json"),
            text(" file:"),
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
            text("The below configuration works with "),
            link!("https://www.lazyvim.org", text("LazyVim")),
            text(" and the lazy.vim plugin. Create a file in "),
            code("lua/plugins/nvim-lspconfig.lua"),
            text(" (or update your "),
            code("lspconfig"),
            text(") with:"),
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
    "Editor setup",
    "/docs/getting-started/editor-setup",
    page_content()
);
