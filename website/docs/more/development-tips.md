---
title: "Tips for developing Yew applications"
---

:::important contribute
This document only contains information for adding supporting in Jetbrains IDEs and VS Code.
Feel free to contribute to add instructions for your editor of choice. 
:::

## Add a template for creating components

### Jetbrains IDEs

1. Navigate to File | Settings | Editor | Live Templates.
2. Select Rust and click on + icon to add a new Live Template.
3. Give it a name and description of your preference.
4. Paste the following snippet in Template Text section:

```rust
use yew::prelude::*;

struct $NAME$ {
    link: ComponentLink<Self>
}

enum Msg {
}

impl Component for $NAME$ {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            link 
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            $HTML$
        }
    }
}
```

### VS Code

1. Navigate to File > Preferences > User Snippets.
2. Select Rust as the language.
3. Add the following snippet in the snippet JSON file:

```json
{
	"Create new Yew component": {
		"prefix": "YOUR PREFIX OF CHOICE",
		"body": [
			"use yew::prelude::*;",
			"",
			"pub struct ${1} {",
			"    link: ComponentLink<Self>,",
			"}",
			"",
			"pub enum Msg {",
			"}",
			"",
			"impl Component for ${1} {",
			"    type Message = Msg;",
			"    type Properties = ();",
			"",
			"    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {",
			"        Self {",
			"            link,",
			"        }",
			"    }",
			"",
			"    fn update(&mut self, msg: Self::Message) -> ShouldRender {",
			"        match msg {}",
			"    }",
			"",
			"    fn change(&mut self, _props: Self::Properties) -> ShouldRender {",
			"        false",
			"    }",
			"",
			"    fn view(&self) -> Html {",
			"        html! {",
			"            ${0}",
			"        }",
			"    }",
			"}"
		],
		"description": "Create a new Yew component without properties but with a message enum"
	}
}

```

## Enable HTML intellisense for `html!` 

### Jetbrains IDEs

There is currently no way to extend the support for proc macros in `intellij-rust`. See [intellij-rust/intellij-rust#6367](https://github.com/intellij-rust/intellij-rust/issues/6367) and [intellij-rust/intellij-rust#1786](https://github.com/intellij-rust/intellij-rust/issues/1786).

### VS Code

There's no support for specialized syntax of `html!` but you can use the default HTML IntelliSense by adding the following snippet in your VS Code's `settings.json` file:

```json
"emmet.includeLanguages": {
    "rust": "html",
}
```
