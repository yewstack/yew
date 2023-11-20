use anyhow::Result;

use yew::{function_component, prelude::*, ServerRenderer};

#[function_component]
fn App() -> Html {
    html! {
        <div>
            {"Hello, World!"}
        </div>
    }
}

pub async fn render() -> Result<String> {
    let renderer = ServerRenderer::<App>::new();
    let html_raw = renderer.render_async().await;

    let mut body = String::new();
    body.push_str("<body>");
    body.push_str("<div id='app' style='width: 100vw; height: 100vh; position: fixed;'>");
    body.push_str(&html_raw);
    body.push_str("</div>");
    body.push_str("</body>");

    Ok(body)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let ret = render().await?;
    println!("{}", ret);

    Ok(())
}
