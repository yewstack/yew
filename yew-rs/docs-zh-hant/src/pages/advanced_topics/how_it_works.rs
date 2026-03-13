pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h1(vec![text("底層庫的內部細節")]),
        h2(vec![code("html!"), text(" 巨集的內部")]),
        p(vec![
            code("html!"),
            text(
                " macro 會將使用類似 HTML 的自訂語法所編寫的程式碼轉換為有效的 Rust \
                 程式碼。使用這個巨集對於開發 Yew \
                 應用程式並不是必需的，但是是建議的。這個巨集產生的程式碼使用了 Yew 的公共函式庫 \
                 API，如果你願意的話，可以直接使用。請注意，有些方法是有意未記錄的，\
                 以避免意外的誤用。隨著 ",
            ),
            code("yew-macro"),
            text(
                " 的每次更新，產生的程式碼將會更加高效，並且可以處理任何破壞性的更改，而不需要對 ",
            ),
            code("html!"),
            text(" 語法進行很多（如果有的話）修改。"),
        ]),
        p(vec![
            text("由於 "),
            code("html!"),
            text(
                " 巨集允許您以聲明式的風格編寫程式碼，因此您的 UI 佈局程式碼將與為頁面產生的 HTML \
                 非常相似。隨著您的應用程式變得更加互動式，您的程式碼庫變得更大，\
                 這種方式變得越來越有用。與手動編寫所有操作 DOM \
                 的程式碼相比，巨集會為您處理好這一切。",
            ),
        ]),
        p(vec![
            text("使用 "),
            code("html!"),
            text(
                " 巨集可能會讓人感到非常神奇，但它並沒有什麼可隱藏的。\
                 如果您對它的工作原理感到好奇，可以嘗試展開您程式中的 ",
            ),
            code("html!"),
            text(" 巨集呼叫。有一個有用的指令叫做 "),
            code("cargo expand"),
            text("，它允許您查看 Rust 巨集的展開。 "),
            code("cargo expand"),
            text(" 並不是預設隨 "),
            code("cargo"),
            text(" 一起提供的，所以如果您還沒有安裝它，您需要使用 "),
            code("cargo install cargo-expand"),
            text(" 來安裝它。 "),
            link(
                "https://rust-analyzer.github.io/",
                vec![text("Rust-Analyzer")],
            ),
            text(" 也提供了一個"),
            link(
                "https://rust-analyzer.github.io/manual.html #expand-macro-recursively",
                vec![text("從IDE 中取得巨集輸出的機制")],
            ),
            text("。"),
        ]),
        p(vec![
            code("html!"),
            text(
                " 巨集的輸出通常非常簡潔！這是一個功能：\
                 機器產生的程式碼有時可能會與應用程式中的其他程式碼衝突。為了防止問題，",
            ),
            code("proc_macro"),
            text(" 遵循了「衛生」規則。一些例子包括："),
        ]),
        ol(vec![
            li(vec![
                text("為了確保正確引用 Yew 套件，巨集產生的程式碼中使用 "),
                code("::yew::<module>"),
                text("，而不是直接使用 "),
                code("yew::<module>"),
                text("。這也是為什麼呼叫 "),
                code("::alloc::vec::Vec::new()"),
                text(" 而不是直接呼叫 "),
                code("Vec::new()"),
                text("。"),
            ]),
            li(vec![
                text("由於可能有 trait 方法名稱衝突，使用 "),
                code("<Type as Trait>"),
                text(" 來確保我們使用的是正確的 trait 成員。"),
            ]),
        ]),
        h2(vec![text("什麼是虛擬 DOM？")]),
        p(vec![text(
            "DOM（\"文件物件模型\"）是由瀏覽器管理的 HTML 內容的表示。 \"虛擬\" DOM 只是 DOM \
             的一個記憶體中的副本。管理虛擬 DOM \
             會導致更高的記憶體開銷，但可以透過避免或延遲使用瀏覽器 API 來實現批次和更快的讀取。",
        )]),
        p(vec![text(
            "在記憶體中擁有 DOM 的副本對於促進使用聲明式 UI \
             的函式庫是有幫助的。與需要特定程式碼來描述如何根據使用者事件修改 DOM \
             不同，程式庫可以使用一種通用的方法來進行 DOM \"diffing\"。當 Yew \
             元件更新並希望更改其呈現方式時，Yew 庫將建立虛擬 DOM \
             的第二個副本，並直接將其與鏡像當前螢幕上的內容的虛擬 DOM 進行比較。兩者之間的 \
             \"diff\"（差異）可以分解為增量更新，並與瀏覽器 API 一起應用。一旦更新應用，舊的虛擬 \
             DOM 副本將被丟棄，新的副本將被保存以供將來的差異檢查。",
        )]),
        p(vec![text(
            "這種 \"diff\" 演算法可以隨著時間的推移進行最佳化，以提高複雜應用程式的效能。由於 Yew \
             應用程式是透過 WebAssembly 運行的，我們相信 Yew \
             在未來採用更複雜的演算法方面具有競爭優勢。",
        )]),
        p(vec![text(
            "Yew 的虛擬 DOM 與瀏覽器 DOM 不完全一一對應。它還包括用於組織 DOM 元素的 \"列表\" 和 \
             \"元件\"。列表可以簡單地是元素的有序列表，但也可以更強大。透過為每個清單元素新增 \
             \"key\" 註解，應用程式開發人員可以幫助 Yew \
             進行額外的最佳化，以確保在清單變更時，計算差異更新所需的工作量最小。同樣，\
             元件提供了自訂邏輯，指示是否需要重新渲染，以幫助提高效能。",
        )]),
        h2(vec![text("Yew 調度器和元件範圍的事件循環")]),
        p(vec![italic(vec![
            text("貢獻文件 - 深入解釋 "),
            code("yew::scheduler"),
            text(" 和 "),
            code("yew::html::scope"),
            text(" 的工作原理"),
        ])]),
        h2(vec![text("進一步閱讀")]),
        ul(vec![
            li(vec![link(
                "https://doc.rust-lang.org/stable/book/ch19-06-macros.html",
                vec![text("Rust 手冊中關於巨集的更多資訊")],
            )]),
            li(vec![link(
                "https://github.com/dtolnay/cargo-expand",
                vec![code("cargo-expand"), text(" 的更多資訊")],
            )]),
            li(vec![link(
                "https://docs.rs/yew/*/yew/virtual_dom/index.html",
                vec![code("yew::virtual_dom"), text(" 的 API 文件")],
            )]),
        ]),
    ])
}

crate::doc_page!(
    "工作原理",
    "/zh-Hant/docs/advanced-topics/how-it-works",
    page_content()
);
