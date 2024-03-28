#![allow(unused_imports)]
#![allow(non_snake_case)]

mod router;

use anyhow::Result;
use router::{switch, Route};
use yew::prelude::*;
use yew::LocalServerRenderer;

#[function_component]
fn Content() -> Html {
    use yew_router::prelude::*;

    html! {
        <>
            <h1>{"Yew WASI SSR demo"}</h1>
            <Switch<Route> render={switch} />
        </>
    }
}

#[function_component]
fn App() -> Html {
    use yew_router::history::{AnyHistory, History, MemoryHistory};
    use yew_router::prelude::*;

    let history = AnyHistory::from(MemoryHistory::new());
    history.push("/");

    html! {
        <div>
            <Router history={history}>
                <Content />
            </Router>
        </div>
    }
}

pub async fn render() -> Result<String> {
    let renderer = LocalServerRenderer::<App>::new();
    let html_raw = renderer.render().await;

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
