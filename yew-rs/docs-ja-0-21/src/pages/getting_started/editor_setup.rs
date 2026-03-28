crate::doc_page!(
    "Editor Setup",
    "/ja/docs/getting-started/editor-setup",
    Content::new(vec![
        admonition![
            AdmonitionType::Warning,
            Some("Contribute"),
            p![
                "Using a different editor? Feel free to add instructions for your editor of \
                 choice."
            ],
        ],
        h2!["Add a template for creating components"],
        h3!["JetBrains IDEs"],
        ol![
            li!["Navigate to File | Settings | Editor | Live Templates."],
            li!["Select Rust and click on the + icon to add a new Live Template."],
            li!["Give it a name and description of your preference."],
            li!["Paste the following snippet(s) into the Template Text section."],
            li!["Change the applicability on the lower right, select Rust > Item > Module"],
        ],
        p!["For function components, use the following template.",],
        ul![li![
            "(Optional) Click on Edit Variable and give ",
            code("tag"),
            " a reasonable default value like \"div\", with double quotes.",
        ],],
        code_block(
            "rust",
            r#"#[derive(PartialEq, Properties)]
pub struct $Name$Props {
}

#[function_component]
pub fn $Name$(props: &$Name$Props) -> Html {
    html! {
        <$tag$>$END$</$tag$>
    }
}"#
        ),
        p!["For struct components, you can use the following more complicated template."],
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
}"#
        ),
        h3!["VS Code"],
        ol![
            li!["Navigate to File > Preferences > User Snippets."],
            li!["Select Rust as the language."],
            li!["Add the following snippet in the snippet JSON file:"],
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
            "#[function_component]",
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
}"##
        ),
        h2!["Support for the html! Macro"],
        h3!["JetBrains IDEs"],
        p!["Contribution Welcome!"],
        h3!["VS Code"],
        h4!["Rust-Yew extension"],
        p![
            "This is a ",
            bold!["work in progress"],
            ", and ",
            bold!["community maintained"],
            " project! ",
            link!(
                "https://github.com/TechTheAwesome/code-yew-server",
                "Please see details and direct related bug reports / issues / questions over to \
                 the extension's repository",
            ),
        ],
        p![
            "Rust-Yew extension is ",
            link!(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                "available on VSC Marketplace",
            ),
            ", providing syntax highlight, renames, hover, and more.",
        ],
        p![
            "Emmet support should work out of the box, if not please fall back to editing the ",
            code("settings.json"),
            " file:",
        ],
        code_block(
            "json",
            r#""emmet.includeLanguages": {
    "rust": "html",
}"#
        ),
        h3!["Neovim"],
        h4!["Lazyvim"],
        p![
            "Below configuration works with ",
            link!("https://www.lazyvim.org", "LazyVim"),
            " configuration and lazy.vim plugin, create a file in ",
            code("lua/plugins/nvim-lspconfig.lua"),
            " (or update your ",
            code("lspconfig"),
            ") with:",
        ],
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
}"#
        ),
    ])
    .with_description("Setting your code editor")
);
