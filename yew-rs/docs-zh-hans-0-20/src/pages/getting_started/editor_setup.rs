crate::doc_page!(
    "Editor Setup",
    "/zh-Hans/docs/getting-started/editor-setup",
    Content::new(vec![
        admonition(
            AdmonitionType::Warning,
            Some("Contribute"),
            vec![p(vec![text(
                "Using a different editor? Feel free to add instructions for your editor of \
                 choice."
            )])],
        ),
        h2(vec![text("Add a template for creating components")]),
        h3(vec![text("JetBrains IDEs")]),
        ol(vec![
            li(vec![text(
                "Navigate to File | Settings | Editor | Live Templates."
            )]),
            li(vec![text(
                "Select Rust and click on the + icon to add a new Live Template."
            )]),
            li(vec![text(
                "Give it a name and description of your preference."
            )]),
            li(vec![text(
                "Paste the following snippet(s) into the Template Text section."
            )]),
            li(vec![text(
                "Change the applicability on the lower right, select Rust > Item > Module"
            )]),
        ]),
        p(vec![text(
            "For function components, use the following template."
        )]),
        ul(vec![li(vec![
            text("(Optional) Click on Edit Variable and give "),
            code("tag"),
            text(" a reasonable default value like \"div\", with double quotes."),
        ]),]),
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
        p(vec![text(
            "For struct components, you can use the following more complicated template."
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
}"#
        ),
        h3(vec![text("VS Code")]),
        ol(vec![
            li(vec![text(
                "Navigate to File > Preferences > User Snippets."
            )]),
            li(vec![text("Select Rust as the language.")]),
            li(vec![text(
                "Add the following snippet in the snippet JSON file:"
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
        h2(vec![text("Support for the html! Macro")]),
        h3(vec![text("JetBrains IDEs")]),
        p(vec![text("Contribution Welcome!")]),
        h3(vec![text("VS Code")]),
        h4(vec![text("Rust-Yew extension")]),
        p(vec![
            text("This is a "),
            bold(vec![text("work in progress")]),
            text(", and "),
            bold(vec![text("community maintained")]),
            text(" project! "),
            link(
                "https://github.com/TechTheAwesome/code-yew-server",
                vec![text(
                    "Please see details and direct related bug reports / issues / questions over \
                     to the extension's repository."
                )],
            ),
        ]),
        p(vec![
            text("Rust-Yew extension is "),
            link(
                "https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew",
                vec![text("available on VSC Marketplace")],
            ),
            text(", providing syntax highlight, renames, hover, and more."),
        ]),
        p(vec![
            text(
                "Emmet support should work out of the box, if not please fall back to editing the "
            ),
            code("settings.json"),
            text(" file:"),
        ]),
        code_block(
            "json",
            r#""emmet.includeLanguages": {
    "rust": "html",
}"#
        ),
    ])
);
