use yew::prelude::*;

#[cfg(feature = "ssr")]
async fn fetch_product() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    "Yew Framework T-Shirt".to_string()
}

#[component]
fn Product() -> HtmlResult {
    let name =
        use_prepared_state!((), async move |_| -> String { fetch_product().await })?.unwrap();

    Ok(html! {
        <div id="content" class="product">
            <h2>{name}</h2>
            <p>{"$29.99"}</p>
        </div>
    })
}

#[component]
pub fn App() -> Html {
    let fallback = html! {
        <div id="fallback" class="product skeleton">
            <h2>{"Loading product..."}</h2>
        </div>
    };

    html! {
        <div class="page">
            <header><h1>{"Yew Store"}</h1></header>
            <main>
                <Suspense {fallback}>
                    <Product />
                </Suspense>
            </main>
            <footer>{"© 2025 Yew Contributors"}</footer>
        </div>
    }
}
