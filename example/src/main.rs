use yew::{html, AttrValue, Component, Context, Html};

/// An HTML snippet that we want to embed in our app.
/// This simulates the use case from the issue where a developer has an HTML
/// string (e.g. loaded via `include_str!`) and wants to render it as real HTML.
const INNER_HTML_SNIPPET: &str = r#"
<h3>This is a heading inside inner HTML</h3>
<p>This paragraph contains <strong>bold</strong> and <em>italic</em> text.</p>
<ul>
  <li>List item 1</li>
  <li>List item 2</li>
  <li>List item 3</li>
</ul>
<p>This was rendered using <code>Html::from_html_unchecked()</code>.</p>
"#;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // The original issue: placing an HTML string inside html! {} renders it
        // as escaped plain text (showing raw angle-brackets to the user).
        let plain_text: Html = INNER_HTML_SNIPPET.into();

        // The fix: Html::from_html_unchecked sets innerHTML directly, so the
        // string is interpreted as real HTML markup.
        let inner_html: Html =
            Html::from_html_unchecked(AttrValue::from(INNER_HTML_SNIPPET));

        html! {
            <div>
                <h1>{ "Html::from_html_unchecked Demo" }</h1>
                <p>
                    { "This demo shows the difference between using a plain string \
                       inside html! (renders as escaped text) vs using \
                       Html::from_html_unchecked (renders as real HTML)." }
                </p>

                <div class="section">
                    <h2>{ "❌  Plain string interpolation (renders as escaped text)" }</h2>
                    <p class="label">{ r#"Code: html! { { INNER_HTML_SNIPPET } }"# }</p>
                    <div id="plain-text-output" class="box">{ plain_text }</div>
                </div>

                <div class="section">
                    <h2>{ "✅  Html::from_html_unchecked (renders as real HTML)" }</h2>
                    <p class="label">
                        { "Code: Html::from_html_unchecked(AttrValue::from(INNER_HTML_SNIPPET))" }
                    </p>
                    <div id="inner-html-output" class="box">{ inner_html }</div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
