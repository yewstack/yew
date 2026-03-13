crate::doc_page!(
    "服務端渲染",
    "/zh-Hant/docs/advanced-topics/server-side-rendering",
    Content::new(vec![
        h1(vec![text("服務端渲染 (Server-Side Rendering)")]),
        p(vec![
            text("預設情況下，Yew 元件在客戶端渲染。當使用者造訪一個網站時，伺服器會傳送一個骨架 HTML 文件，不包含任何實際內容，以及一個 WebAssembly 套件給瀏覽器。所有內容都由 WebAssembly 套件在客戶端渲染。這被稱為客戶端渲染。"),
        ]),
        p(vec![
            text("這種方法對於大多數網站來說都是有效的，但有一些注意事項："),
        ]),
        ol(vec![
            li(vec![
                text("使用者在整個 WebAssembly 套件下載並完成初始渲染之前將看不到任何內容。這可能會導致在緩慢的網路上用戶體驗不佳。"),
            ]),
            li(vec![
                text("有些搜尋引擎不支援動態渲染的網頁內容，而那些支援的搜尋引擎通常會將動態網站排名較低。"),
            ]),
        ]),
        p(vec![text("為了解決這些問題，我們可以在服務端渲染我們的網站。")]),
        h2(vec![text("工作原理")]),
        p(vec![
            text("Yew 提供了一個 "),
            code("ServerRenderer"),
            text(" 來在服務端渲染頁面。"),
        ]),
        p(vec![
            text("要在服務端渲染Yew 元件，您可以使用"),
            code("ServerRenderer::<App>::new()"),
            text(" 建立一個渲染器，並呼叫"),
            code("renderer.render().await"),
            text(" 將"),
            code("<App />"),
            text(" 渲染為一個"),
            code("String"),
            text("。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew::ServerRenderer;

#[function_component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

// 我們在使用 `flavor = "current_thread"` 以確保這個範例可以在 CI 中的 WASM 環境運作,
// 如果你希望使用多執行緒的話，可以使用預設的 `#[tokio::main]` 宏
#[tokio::main(flavor = "current_thread")]
async fn no_main() {
    let renderer = ServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    // 列印: <div>Hello, World!</div>
    println!("{}", rendered);
}"#,
        ),
        h2(vec![text("組件生命週期")]),
        p(vec![
            text("推薦的服務端渲染工作方式是使用函數元件。"),
        ]),
        p(vec![
            text("在元件成功第一次渲染為 "),
            code("Html"),
            text(" 之前，除了 "),
            code("use_effect"),
            text(" (和 "),
            code("use_effect_with"),
            text(") 之外的所有鉤子都會正常運作。"),
        ]),
        admonition(
            AdmonitionType::Caution,
            Some("瀏覽器介面不可用！"),
            vec![p(vec![
                text("瀏覽器相關的接口，如 "),
                code("web_sys"),
                text("，在元件在服務端渲染時是不可用的。如果您嘗試使用它們，您的應用程式將會崩潰。您應該將需要這部分邏輯隔離在 "),
                code("use_effect"),
                text(" 或 "),
                code("use_effect_with"),
                text(" 中，因為在服務端渲染時它們無法也不應執行。"),
            ])],
        ),
        admonition(
            AdmonitionType::Danger,
            Some("結構化組件"),
            vec![
                p(vec![
                    text("儘管可以在服務端渲染時使用結構化元件，但是在客戶端安全邏輯（如函數元件的"),
                    code("use_effect"),
                    text(" 鉤子）和生命週期事件之間沒有明確的邊界，並且生命週期事件的呼叫順序與客戶端不同。"),
                ]),
                p(vec![
                    text("此外，結構化元件將繼續接受訊息，直到所有子元件都被渲染並呼叫了 "),
                    code("destroy"),
                    text(" 方法。開發人員需要確保不會將可能傳遞給元件的訊息連結到呼叫瀏覽器介面的邏輯。"),
                ]),
                p(vec![
                    text("在設計支援服務端渲染的應用程式時，請盡量使用函數元件，除非您有充分的理由不這樣做。"),
                ]),
            ],
        ),
        h2(vec![text("服務端渲染期間的資料獲取")]),
        p(vec![
            text("資料取得是服務端渲染和水合（hydration）期間的困難之一。"),
        ]),
        p(vec![
            text("在傳統做法中，當一個元件渲染時，它會立即可用（輸出一個虛擬 DOM 以進行渲染）。當元件不需要取得任何資料時，這種方式是有效的。但是如果元件在渲染時想要取得一些資料會發生什麼事呢？"),
        ]),
        p(vec![
            text("過去，Yew 沒有機制來檢測組件是否仍在取得資料。資料擷取用戶端負責實作一個解決方案，以偵測在初始渲染期間請求了什麼，並在請求完成後觸發第二次渲染。伺服器會重複這個過程，直到在回傳回應之前沒有在渲染期間添加更多的掛起請求。"),
        ]),
        p(vec![
            text("這不僅浪費了CPU 資源，因為重複渲染元件，而且資料用戶端還需要提供一種方法，在水合過程中使在服務端獲取的資料可用，以確保初始渲染返回的虛擬DOM 與服務端渲染的DOM樹一致，這可能很難實現。"),
        ]),
        p(vec![
            text("Yew 採用了不同的方法，透過 "),
            code("<Suspense />"),
            text(" 來解決這個問題。"),
        ]),
        p(vec![
            code("<Suspense />"),
            text(" 是一個特殊的元件，當在客戶端使用時，它提供了一種在元件獲取資料（掛起）時顯示一個回退UI 的方法，並在資料獲取完成後恢復到正常UI。"),
        ]),
        p(vec![
            text("當應用程式在服務端渲染時，Yew 會等待元件不再掛起，然後將其序列化到字串緩衝區中。"),
        ]),
        p(vec![
            text("在水合過程中，"),
            code("<Suspense />"),
            text(" 組件中的元素保持未水合狀態，直到所有子組件不再掛起。"),
        ]),
        p(vec![
            text("透過這種方法，開發人員可以輕鬆建立一個準備好進行服務端渲染的、與客戶端無關的應用程序，並進行資料擷取。"),
        ]),
        h2(vec![text("SSR 水合（SSR Hydration）")]),
        p(vec![
            text("水合是將 Yew 應用程式連接到服務端產生的 HTML 檔案的過程。預設情況下，"),
            code("ServerRender"),
            text(" 會列印可水合的 HTML 字串，其中包含額外的資訊以便於水合。當呼叫 "),
            code("Renderer::hydrate"),
            text(" 方法時，Yew 不會從頭開始渲染，而是將應用程式產生的虛擬 DOM 與伺服器渲染器產生的 HTML 字串進行協調。"),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("要成功對由 "),
                code("ServerRenderer"),
                text(" 建立的 HTML 標記進行水合，用戶端必須產生一個虛擬 DOM 佈局，它與用於 SSR 的佈局完全匹配，包括不包含任何元素的元件。如果您有任何只在一個實作中有用的元件，您可能想要使用 "),
                code("PhantomComponent"),
                text(" 來填入額外元件的位置。"),
            ])],
        ),
        admonition(
            AdmonitionType::Warning,
            None,
            vec![p(vec![
                text("只有在瀏覽器初始渲染 SSR 輸出（靜態 HTML）後，真實 DOM 與預期 DOM 相符時，水合才能成功。如果您的 HTML 不符合規範，水合可能會失敗。瀏覽器可能會更改不正確的 HTML 的 DOM 結構，導致實際 DOM 與預期 DOM 不同。例如，"),
                link(
                    "https://github.com/yewstack/yew/issues/2684",
                    vec![text("如果您有一個沒有<tbody> 的<table>，瀏覽器可能會向DOM 添加一個<tbody>")],
                ),
            ])],
        ),
        h2(vec![text("水合期間的組件生命週期")]),
        p(vec![
            text("在水合期間，元件在創建後安排了 2 次連續的渲染。任何效果都是在第二次渲染完成後調用的。確保您的元件的渲染函數沒有副作用是很重要的。它不應該改變任何狀態或觸發額外的渲染。如果您的元件目前改變狀態或觸發額外的渲染，請將它們移到 "),
            code("use_effect"),
            text(" 鉤子中。"),
        ]),
        p(vec![
            text("在水合過程中，可以使用結構化元件進行服務端渲染，視圖函數將在渲染函數之前被調用多次。直到呼叫渲染函數之前，DOM 被認為是未連接的，您應該防止在呼叫 "),
            code("rendered()"),
            text(" 方法之前存取渲染節點。"),
        ]),
        h2(vec![text("範例")]),
        code_block_ignore(
            "rust",
            r#"use yew::prelude::*;
use yew::Renderer;

#[function_component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

fn main() {
    let renderer = Renderer::<App>::new();

    // 對 body 元素下的所有內容進行水合，並移除可能有的任何尾隨元素。
    renderer.hydrate();
}"#,
        ),
        p(vec![
            text("範例: "),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/simple_ssr",
                vec![text("simple_ssr")],
            ),
            text(" 範例: "),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/ssr_router",
                vec![text("ssr_router")],
            ),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("服務端渲染目前是實驗性的。如果您發現了一個 bug，"),
                link(
                    "https://github.com/yewstack/yew/issues/new?assignees=&labels=bug&template=bug_report.md&title=",
                    vec![text("請在 GitHub 回饋")],
                ),
                text("。"),
            ])],
        ),
    ])
);
