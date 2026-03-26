crate::doc_page!(
    "優化與最佳實例",
    "/zh-Hant/docs/advanced-topics/optimizations",
    Content::new(vec![
        h2!["neq_assign"],
        p![
            "當元件從父元件接收到屬性時， ",
            code("change"),
            " 的方法就會被呼叫。除了讓你更新元件的狀態，也讓你回傳，\
                 決定元件是否要在屬性改變時，重新渲染自己的布林值 ",
            code("ShouldRender"),
            "。",
        ],
        p!["重新渲染是很浪費效能的，儘可能避免這麼做。一般來說，\
             只有在屬性真的改變時，才重新渲染。下面的程式碼是體現這個原則的例子，\
             當屬性改變時，才回傳 ", code("true"), "："],
        code_block(
            "rust",
            r#"[profile.release]
# less code to include into binary
panic = 'abort'
# optimization over all codebase ( better optimization, slower build )
codegen-units = 1
# optimization for size ( more aggressive )
opt-level = 'z'
# optimization for size
# opt-level = 's'
# link time optimization using using whole-program analysis
lto = true"#,
        ),
        p![
            "但我們可以走的更遠！這六行的模板，使用一個 trait 和一個 實作了 ",
            code("PartialEq"),
            " 的 blanket implementation ，可以被縮短至一行。請參考",
            link![
                "https://docs.rs/yewtil/*/yewtil/trait.NeqAssign.html",
                "這裡",
            ],
            "， ",
            code("yewtil"),
            " 的 crate 裡的 ",
            code("NeqAssign"),
            " trait。",
        ],
        h2!["RC"],
        p![
            "為了避免重新渲染時，複製大量的資料來建立屬性，\
                 我們可以使用智慧指針來讓程式只複製指針。如果你使用 ",
            code("Rc<_>"),
            " 來封裝你的屬性，而不是未封裝的值，你可以使用 ",
            code("Rc::make_mut"),
            "，去複製與存取你想要改變的資料的可變參考，\
                 這做到了延遲複製，直到你需要更動子元件的資料。\
                 透過避免複製直到有值改變，子元件可以在 ",
            code("Component::change"),
            " 拒絕與他狀態中的屬性相同值的屬性，而且這樣不會有任何效能成本。\
                 另外，這個方式，資料必須在與子元件比較與被拒絕之前，\
                 被複製進父元件的屬性中。",
        ],
        p![
            "這個優化最那些無法 ",
            code("Copy"),
            " 的資料型別最有用。如果你可以輕易複製你的資料，\
                 那把資料放進智慧指針裡面似乎就沒有這麼值得。對於那些包含很多像是 ",
            code("Vec"),
            " 、 ",
            code("HashMap"),
            " 與 ",
            code("String"),
            " 的結構，這個優化對他們會更值得。",
        ],
        p![
            "如果子元件幾乎不會更新值，那這個優化效果會很好，\
                 甚至如果父元件也很少更新，那效果會更好。上面的情況，使在純元件中使用 ",
            code("Rc<_>s"),
            " 是一個封裝屬性值很好的選擇。",
        ],
        h2!["View 方法"],
        p![
            "出於程式碼的可讀性，通常會寫方法包裝複雜的 ",
            code("html!"),
            "，這樣你可以避免巢狀的 HTML 造成過多的向右縮排。",
        ],
        h2!["純元件/函數式元件"],
        p![
            "純元件 是一種不會改變自己狀態的元件，他們只單純顯示內容或是向普通可變的元件傳送訊息。\
                 他們和 view 方法不同的地方在於們可以在 ",
            code("html!"),
            " 巨集中使用，語法會像（",
            code("<SomePureComponent />"),
            "），而不是表達式語法（",
            code("{some_view_function()}"),
            "），而且根據他們的實作方式，他們可以被 memoized，這樣可以套用前面所述的 ",
            code("neq_assign"),
            " 的邏輯避免重新渲染。",
        ],
        p!["Yew 本身不支援純元件或是函數式元件，但是你可以透過 external crates 使用。"],
        p!["函數式元件還不存在，但是理論上純元件可以透過巨集與宣告方法產生。"],
        h2!["Keyed DOM nodes when they arrive"],
        h2!["使用 Cargo Workspaces 加速編譯"],
        p![
            "Yew 最大的缺點就是花太多時間在編譯上了。編譯時間似乎和 ",
            code("html!"),
            " 巨集中的程式碼質量相同。 \
                 對於小專案來說，這應該不是什麼大問題，\
                 但是對於有很多頁面的大型網頁應用程式來說，\
                 就必須要將程式碼封裝成很多 crates 以減少編譯所花的時間。",
        ],
        p!["你應該將路由與頁面區塊封裝成一個 main crate，\
             然後將共用的程式碼與元件封裝成另一個 crate，\
             將每個頁面會用到的不同的元件，各自封裝到不同的 crate 中，\
             或是只產生 ", code("Html"), " 的大方法中。最好的狀況，你只需要重新編譯你 main crate \
             與修改的頁面的 crate 的程式碼；而最壞的情況，你編輯了共用的 crate，\
             你就必須重新編譯所有依賴這個共用 crate 的程式碼。"],
        p!["如果你的 main crate 太過龐大，或是你希望快速迭代深層巢狀的頁面\
             （一個頁面渲染另一個頁面的頂層），\
             你可以使用範例的 crate ，在一個簡單的主頁面上編輯你未完成的元件。"],
        h2!["編譯大小的優化"],
        ul![
            li!["優化 Rust 的程式碼"],
            li![
                code("cargo.toml"),
                " （定義釋出的設定檔）",
            ],
            li![
                "使用 ",
                code("wasm-opt"),
                " 優化 wasm 程式碼",
            ],
        ],
        p![
            "更多關於程式碼大小的資訊，請參考： ",
            link![
                "https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size",
                "rustwasm book",
            ],
        ],
        h3!["Cargo.toml"],
        p![
            "你可以設定你的發行版本更小的檔案大小，透過設定 ",
            code("Cargo.toml"),
            " 的 ",
            code("[profile.release]"),
            " 。",
        ],
        p![link![
            "https://doc.rust-lang.org/cargo/reference/profiles.html",
            "Rust profiles documentation",
        ]],
        code_block(
            "rust",
            r#"[unstable]
# Requires the rust-src component. `rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#,
        ),
        h3!["wasm-opt"],
        p![
            "更多優化 ",
            code("wasm"),
            " 程式碼大小的方法。",
        ],
        p![
            "wasm-opt 資訊： ",
            link![
                "https://github.com/WebAssembly/binaryen",
                "binaryen project",
            ],
        ],
        p![
            "Rust Wasm 中有一個關於減少 Wasm 二進位檔大小的章節：",
            link![
                "https://rustwasm.github.io/book/game-of-life/code-size.html",
                "Shrinking .wasm size",
            ],
        ],
        ul![
            li![
                "使用",
                code("wasm-pack"),
                " 預設在發行版本編譯時優化 ",
                code("wasm"),
                " 程式碼",
            ],
            li![
                "直接在 wasm 檔案上使用 ",
                code("wasm-opt"),
                " 。",
            ],
        ],
        code_block("rust", r#"wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"#),
        h4!["編譯 yew/examples/ 中 最小的例子"],
        p![
            "注意： ",
            code("wasm-pack"),
            " 包含對 Rust 與 wasm 程式碼的優化。而",
            code("wasm-bindgen"),
            " 只是一個單純的例子，沒有對 ",
            code("Rust"),
            " 做任何優化。",
        ],
        table(
            vec![vec!["used tool".into()], vec!["size".into()]],
            vec![
                vec![vec!["wasm-bindgen".into()], vec!["158KB".into()]],
                vec![
                    vec!["wasm-bindgen + wasm-opt -Os".into()],
                    vec!["116KB".into()],
                ],
                vec![vec!["wasm-pack".into()], vec!["99KB".into()]],
            ],
        ),
    ])
);
