crate::doc_page!(
    "優化 & 最佳實踐",
    "/zh-Hant/docs/advanced-topics/optimizations",
    Content::new(vec![
        h2![text("使用智慧指針")],
        p![bold![
            text("注意：如果您對本節中使用的某些術語感到困惑，Rust 手冊中有一個有用的[關於智慧型指針的章節](https://doc.rust-lang.org/book/ch15-00- smart-pointers.html)。 "),
        ]],
        p![
            text("為了避免在重新渲染時克隆大量資料以創建 props，我們可以使用智慧指針，只克隆對資料的引用而不是資料本身。如果您在props 和子組件中傳遞與相關數據的引用而不是實際數據，您可以避免在需要修改數據的子組件中克隆任何數據，您可以使用"),
            code("Rc::make_mut"),
            text(" 來克隆並獲得要更改的數據的可變引用。"),
        ],
        p![
            text("這在 "),
            code("Component::changed"),
            text(" 中帶來了更多好處，可以確定 prop 變更是否需要元件重新渲染。這是因為可以比較指標位址（即資料儲存在機器記憶體中的位置）而不是資料的值；如果兩個指標指向相同的數據，則它們指向的資料的值必須相同。請注意，反之可能不成立！即使兩個指標位址不同，底層資料仍可能相同 - 在這種情況下，您應該比較底層資料。"),
        ],
        p![
            text("要進行此比較，您需要使用 "),
            code("Rc::ptr_eq"),
            text(" 而不僅使用 "),
            code("PartialEq"),
            text("（在使用相等運算子 "),
            code("=="),
            text(" 比較資料時自動使用）。 Rust 文件有關於 "),
            code("Rc::ptr_eq"),
            text(" 的"),
            link!(
                "https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq",
                text("更多細節"),
            ),
            text("。"),
        ],
        p![
            text("這種最佳化對於不實作 "),
            code("Copy"),
            text(" 的資料類型最有用。如果您可以廉價地複製數據，則沒有必要將其放在智慧指標後面。對於可能是資料密集的結構，如 "),
            code("Vec"),
            text("、"),
            code("HashMap"),
            text(" 和 "),
            code("String"),
            text("，使用智慧指標可能會帶來效能改進。"),
        ],
        p![
            text("如果數值從不被子元件更新，則此最佳化效果最佳，如果父元件很少更新，則效果更佳。這使得 "),
            code("Rc<_>"),
            text(" 是在純元件中包裝屬性值的一個不錯的選擇。"),
        ],
        p![
            text("但是，必須注意，除非您需要在子元件中自己克隆數據，否則這種最佳化不僅是無用的，而且還增加了不必要的引用計數成本。 Yew 中的 props 已經是引用計數的，內部不會發生資料克隆。"),
        ],
        h2![text("渲染函數")],
        p![
            text("出於程式碼可讀性的原因，將 "),
            code("html!"),
            text(" 的部分重複程式碼遷移到專門分割出來的函數中通常是有意義的。這不僅使您的程式碼更易讀，減少了程式碼縮進，而且還鼓勵良好的設計模式——特別是圍繞構建可組合應用程序，這些函數可以在多個地方調用，從而減少程式碼量。"),
        ],
        h2![text("純組件")],
        p![
            text("純組件是不會改變其狀態的元件，只顯示內容並將訊息傳播到普通的可變組件。它們與視圖函數的不同之處在於，它們可以在"),
            code("html!"),
            text(" 巨集中使用元件語法（"),
            code("<SomePureComponent />"),
            text("）而不是表達式語法（"),
            code("{{some_view_function()}}"),
            text("），並且根據其實現，它們可以被記憶化（這意味著一旦調用函數，其值就會被\"保存\"，因此如果多次使用相同的參數調用它，則不必重新計算其值，只需從第一個函數調用返回保存的值）- 防止相同的props 重新渲染。 Yew 在內部比較 props，因此僅在 props 更改時重新渲染 UI。"),
        ],
        h2![text("使用工作區減少編譯時間")],
        p![
            text("Yew 的最大缺點是編譯所需的時間很長。編譯專案所需的時間似乎與傳遞給 "),
            code("html!"),
            text(" 巨集的程式碼數量有關。對於較小的項目，這似乎不是什麼問題，但對於較大的應用程序，將程式碼拆分到多個 crate 中以最小化編譯器為應用程式所做的工作量是有意義的。"),
        ],
        p![
            text("一種可能的方法是使您的主 crate 處理路由/頁面選擇，然後為每個頁面建立一個不同的 crate，其中每個頁面可以是不同的元件或只是產生 "),
            code("Html"),
            text(" 的大函數。儲存在包含應用程式不同部分的 crate 之間的程式碼可以儲存在專案依賴的單獨 crate 中。在最理想的情況下，您從每次編譯時重新建置所有程式碼到僅重新建置主 crate 和一個頁面 crate。在最壞的情況下，如果您在「common」 crate 中編輯了某些內容，您將回到起點：編譯依賴於該常用共享 crate 的所有程式碼，這可能是其他所有內容。"),
        ],
        p![
            text("如果您的主crate 太重，或者您想快速迭代一個深度巢狀的頁面（例如。在另一個頁面上渲染的頁面），您可以使用範例crate 建立主頁面的簡化實現，並額外渲染您正在處理的組件。"),
        ],
        h2![text("減少二進位檔案大小")],
        ul![
            li![text("優化 Rust 程式碼")],
            li![
                code("cargo.toml"),
                text("（定義發布設定檔）"),
            ],
            li![
                text("使用 "),
                code("wasm-opt"),
                text(" 最佳化 wasm 程式碼"),
            ],
        ],
        p![bold![
            text("注意：有關減小二進位檔案大小的更多信息，請參閱[Rust Wasm 手冊](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code -size)。 "),
        ]],
        h3![text("Cargo.toml")],
        p![
            text("可以使用 "),
            code("Cargo.toml"),
            text(" 中 "),
            code("[profile.release]"),
            text(" 部分中的可用設定來配置發佈建置為更小。"),
        ],
        code_block_title(
            "toml,",
            "Cargo.toml",
            r#"[profile.release]
# 讓二進位檔案尺寸更小些
panic = 'abort'
# 優化整個程式碼庫（優化更好，但建置速度也會更慢）
codegen-units = 1
# 優化尺寸（更激進的做法）
opt-level = 'z'
# 優化尺寸
# opt-level = 's'
# 使用程式整體分析時進行連結時優化
lto = true"#,
        ),
        h3![text("開發版 Cargo 配置")],
        p![
            text("您還可以從 Rust 和 cargo 的實驗性開發版功能中獲得額外的好處。若要使用 "),
            code("trunk"),
            text(" 的開發版工具鏈，請設定 "),
            code("RUSTUP_TOOLCHAIN=\"nightly\""),
            text(" 環境變數。然後，您可以在 "),
            code(".cargo/config.toml"),
            text(" 中配置不穩定的 rustc 功能。請參考"),
            text("[不穩定功能]的文檔，特別是關於"),
            text("["),
            code("build-std"),
            text("]和"),
            text("["),
            code("build-std-features"),
            text("]的部分，以了解配置。"),
        ],
        code_block_title(
            "toml,",
            ".cargo/config.toml",
            r#"[unstable]
# 需要 rust-src 組件。`rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#,
        ),
        p![
            text("[不穩定特性列表]: https://doc.rust-lang.org/cargo/reference/unstable.html "),
            text("["),
            code("build-std"),
            text("]: https://doc.rust-lang.org/cargo/reference/unstable.html#build-std "),
            text("["),
            code("build-std-features"),
            text("]: https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features"),
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("開發版 Rust 編譯器可能包含錯誤，例如"),
                link!(
                    "https://github.com/yewstack/yew/issues/2696",
                    text("這個例子"),
                ),
                text("，需要偶爾注意和調整。請謹慎使用這些實驗性選項。"),
            ],
        ],
        h3![text("wasm-opt")],
        p![
            text("此外，可以最佳化 "),
            code("wasm"),
            text(" 程式碼的大小。"),
        ],
        p![
            text("Rust Wasm 手冊中有關於減少 Wasm 二進位檔案大小的部分："),
            link!(
                "https://rustwasm.github.io/book/game-of-life/code-size.html",
                text("縮小 .wasm 大小"),
            ),
        ],
        ul![
            li![
                text("使用 "),
                code("wasm-pack"),
                text("，預設會最佳化發佈建置中的 "),
                code("wasm"),
                text(" 程式碼"),
            ],
            li![
                text("直接在 "),
                code("wasm"),
                text(" 檔案上使用 "),
                code("wasm-opt"),
            ],
        ],
        code_block("text", r#"wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"#),
        h4![text(
            "在 yew/examples/ 中 'minimal' 範例的建置大小",
        )],
        p![
            text("注意："),
            code("wasm-pack"),
            text(" 結合了 Rust 和 Wasm 程式碼的最佳化。在此範例中，"),
            code("wasm-bindgen"),
            text(" 未經任何 Rust 大小最佳化。"),
        ],
        p![text(
            "| 工具鏈                      | 大小  | | :-------------------------- | :---- | | wasm-bindgen                | 158KB | | wasm-bindgen + wasm-opt -Os | 116KB | | wasm-pack                   | 99 KB |",
        )],
        h2![text("進一步閱讀")],
        ul![
            li![link!(
                "https://doc.rust-lang.org/book/ch15-00-smart-pointers.html",
                text("Rust 手冊中關於智慧型指標的章節"),
            )],
            li![link!(
                "https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size",
                text("Rust Wasm 手冊中關於減小二進位檔案大小的資訊"),
            )],
            li![link!(
                "https://doc.rust-lang.org/cargo/reference/profiles.html",
                text("Rust 設定檔的文件"),
            )],
            li![link!(
                "https://github.com/WebAssembly/binaryen",
                text("binaryen 專案"),
            )],
        ],
    ])
);
