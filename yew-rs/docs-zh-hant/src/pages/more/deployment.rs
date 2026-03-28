pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["當您準備將 Yew 應用程式部署到伺服器時，您有多種部署方案可以選擇。"],
        p![
            code("trunk build --release"),
            " 會以發布模式建立您的應用程式。設定您的HTTP 伺服器，以便在存取您的網站時提供",
            code("index.html"),
            "，並且對於靜態路徑（例如",
            code("index_<hash>.js"),
            " 和",
            code("index_bg_<hash>.wasm"),
            "）的請求，應該從trunk產生的dist 目錄中提供相應的內容。",
        ],
        admonition!(
            AdmonitionType::Important,
            Some("有關 `trunk serve --release`"),
            p![
                "不要在生產環境中使用 ",
                code("trunk serve --release"),
                " 來提供您的應用程式。\n它只應該用於在開發過程中測試發布版本建置。",
            ],
        ),
        h2!["伺服器配置"],
        h3!["將 ", code("index.html"), " 當作回退提供"],
        p![
            "如果應用程式使用了 ",
            doc_link!(crate::pages::concepts::router, "Yew 路由"),
            "，您必須設定伺服器在請求不存在的檔案時傳回 ",
            code("index.html"),
            "。",
        ],
        p![
            "具有 Yew 路由的應用程式被建構為 ",
            link!(
                "https://developer.mozilla.org/en-US/docs/Glossary/SPA",
                "單頁應用程式 (SPA)",
            ),
            "。當使用者從正在執行的用戶端導覽到 URL 時，路由器會解釋 URL 並路由到該頁面。",
        ],
        p!["但是在刷新頁面或在網址列中輸入 URL \
            時，這些操作都是由瀏覽器本身處理的，而不是由正在執行的應用程式處理。\
            瀏覽器直接向伺服器請求該 URL，繞過了路由器。錯誤配置的伺服器會回傳 404 - 未找到 \
            狀態。"],
        p![
            "透過返回 ",
            code("index.html"),
            "，應用程式會像通常一樣加載，就好像請求是 ",
            code("/"),
            "，直到路由器注意到路由是 ",
            code("/show/42"),
            " 並顯示相應的內容。",
        ],
        h3!["為 Web Assembly 資源配置正確的 MIME 類型。"],
        p![
            "WASM 檔案必須使用 ",
            code("application/wasm"),
            " MIME 類型設定 ",
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type",
                "Content-Type 頭",
            ),
            "。",
        ],
        p![
            "大多數伺服器和託管服務預設已經這樣做。如果您的伺服器沒有這樣做，請查閱其文件。\
             在大多數 Web 瀏覽器中，錯誤的 MIME 類型會導致類似以下的錯誤："
        ],
        code_block(
            "ignore",
            r#"`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:
 TypeError: WebAssembly: Response has unsupported MIME type 'text/plain' expected 'application/wasm'"#,
        ),
        h2!["為相對路徑構建"],
        p![
            "預設情況下，trunk 會假定您的網站在 ",
            code("/"),
            " 處提供，並相應地建立網站。可以透過在 ",
            code("index.html"),
            " 檔案中加入 ",
            code("<base data-trunk-public-url />"),
            " 來覆寫此行為。 Trunk 會重寫此標籤以包含傳遞給 ",
            code("--public-url"),
            " 的值。 Yew 路由會自動偵測 ",
            code("<base />"),
            " 的存在並適當處理。",
        ],
        h2!["使用環境變數自訂行為"],
        p![
            "通常使用環境變數來自訂建構環境。由於應用程式在瀏覽器中運行，\
             我們無法在運行時讀取環境變數。 ",
            link!(
                "https://doc.rust-lang.org/std/macro.env.html",
                code("std::env!"),
            ),
            " 巨集可以在編譯時取得環境變數的值。",
        ],
    ])
    .with_description("部署 Yew 應用程式")
}

crate::doc_page!("部署", "/zh-Hant/docs/more/deployment", page_content());
