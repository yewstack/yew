pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![text(
            "佔位標籤 (Suspense) 是一種在等待任務完成前暫停元件渲染的方式，同時顯示一個回退（佔位符）UI。",
        )]),
        p(vec![text(
            "它可以用於從伺服器取得數據，等待代理完成任務，或執行其他後台非同步任務。",
        )]),
        p(vec![text(
            "在佔位標籤出現之前，資料擷取通常發生在元件渲染之後（渲染時取得）或之前（取得後渲染）。",
        )]),
        h3(vec![text("邊渲染，邊下載")]),
        p(vec![text(
            "佔位標籤 (Suspense) 提供了一種新的方法，允許元件在渲染過程中啟動資料請求。當元件啟動資料請求時，渲染過程將被暫停，並顯示一個回退 UI，直到請求完成。",
        )]),
        p(vec![text(
            "建議使用鉤子 (Hook) 來使用佔位標籤。",
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"#,
        ),
        p(vec![
            text("在上面的範例中，"),
            code("use_user"),
            text(" 鉤子將在載入使用者資訊時暫停元件渲染，並在載入 "),
            code("user"),
            text(" 之前顯示 "),
            code("Loading..."),
            text(" 佔位符。"),
        ]),
        p(vec![
            text("要定義一個暫停元件渲染的鉤子，它需要傳回一個 "),
            code("SuspensionResult<T>"),
            text("。當元件需要暫停時，鉤子應該傳回一個 "),
            code("Err(Suspension)"),
            text("，使用者應該使用 "),
            code("?"),
            text(" 解包它，這樣它將被轉換為 "),
            code("Html"),
            text("。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

struct User {
    name: String,
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // 當使用者載入完成時，我們將其作為 Ok(user) 傳回。
        Some(m) => Ok(m),
        None => {
            // 當使用者仍在載入時，我們建立一個 `Suspension`
            // 並在資料載入完成時呼叫 `SuspensionHandle::resume`，
            // 元件將自動重新渲染。
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}"#,
        ),
        h4(vec![text("關於實作暫停鉤子 (Hook) 的注意事項")]),
        p(vec![
            link(
                "https://docs.rs/yew/latest/yew/suspense/struct.Suspension.html#method.new",
                vec![code("Suspension::new")],
            ),
            text(" 傳回 2 個值：暫停上下文本身和一個暫停句柄。後者負責在何時重新渲染暫停的元件，它提供了 2 種可互換的方法："),
        ]),
        ol(vec![
            li(vec![
                text("呼叫其 "),
                link(
                    "https://docs.rs/yew/latest/yew/suspense/struct.SuspensionHandle.html#method.resume",
                    vec![code("resume")],
                ),
                text(" 方法。"),
            ]),
            li(vec![text("丟棄句柄。")]),
        ]),
        admonition(
            AdmonitionType::Danger,
            None,
            vec![p(vec![
                text("暫停句柄必須儲存直到更新元件的時候，也就是使用新接收的資料；否則，暫停的元件將進入無限重新渲染循環，從而影響效能。\n在上面的範例中，暫停句柄會透過移至閉包中並傳遞給 "),
                code("on_load_user_complete"),
                text(" 來儲存。\n當虛擬使用者載入時，將呼叫閉包，從而呼叫 "),
                code("handle.resume()"),
                text(" 並重新渲染與暫停上下文相關的元件。"),
            ])],
        ),
        h1(vec![text("完整範例")]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Debug)]
struct User {
    name: String,
}

fn load_user() -> Option<User> {
    todo!()  // 略
}

fn on_load_user_complete<F: FnOnce()>(_fn: F) {
    todo!()  // 略
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // 如果用戶已加載，則將其作為 Ok(user) 返回。
        Some(m) => Ok(m),
        None => {
            // 當使用者仍在載入時，我們建立一個 `Suspension`
            // 並在資料載入完成時呼叫 `SuspensionHandle::resume`，
            // 元件將自動重新渲染。
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}

#[component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"#,
        ),
        h3(vec![text("在結構體組件中使用佔位標籤")]),
        p(vec![
            text("直接暫停結構體組件是不可能的。然而，您可以使用函數元件作為"),
            link("/zh-Hant/docs/advanced-topics/struct-components/hoc", vec![text("高階元件")]),
            text("來實現基於佔位標籤的資料取得。"),
        ]),
        p(vec![
            text("Yew 倉庫中的"),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/suspense/src/struct_consumer.rs",
                vec![text("佔位標籤範例")],
            ),
            text("示範如何使用這個元件。"),
        ]),
        h2(vec![text("相關範例")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/master/examples/suspense",
            vec![text("佔位標籤")],
        )])]),
    ])
}

crate::doc_page!(
    "佔位標籤 (Suspense)",
    "/zh-Hant/docs/concepts/suspense",
    page_content()
);
