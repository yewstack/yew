pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["スマートポインタの使用"],
        p![
            bold![
                "注意：このセクションで使用されている用語に混乱がある場合は、Rustのマニュアルにある",
                link!["https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", "スマートポインタに関する章"],
                "が役立ちます。",
            ],
        ],
        p![
            "再レンダリング時に大量のデータをクローンしてpropsを作成するのを避けるために、スマートポインタを使用してデータ自体ではなくデータへの参照のみをクローンすることができます。propsや子コンポーネントに関連データの参照を渡すことで、データを変更する必要がある子コンポーネントでデータをクローンするのを避けることができます。",
            code("Rc::make_mut"),
            "を使用してデータをクローンし、変更するための可変参照を取得できます。",
        ],
        p![
            "これにより、",
            code("Component::changed"),
            "でのpropの変更がコンポーネントの再レンダリングを必要とするかどうかを判断する際にさらに利点があります。これは、データの値ではなくポインタのアドレス（つまり、データがマシンメモリに格納されている場所）を比較できるためです。2つのポインタが同じデータを指している場合、それらが指しているデータの値は同じでなければなりません。逆は必ずしも真ではないことに注意してください！2つのポインタアドレスが異なる場合でも、基になるデータは同じである可能性があります。この場合、基になるデータを比較する必要があります。",
        ],
        p![
            "この比較を行うには、",
            code("Rc::ptr_eq"),
            " を使用する必要があります。",
            code("PartialEq"),
            "（データを比較する際に自動的に使用される等価演算子",
            code("=="),
            "）ではなく。Rustのドキュメントには、",
            link!["https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq", "Rc::ptr_eqに関する詳細"],
            "があります。",
        ],
        p![
            "この最適化は、",
            code("Copy"),
            "を実装していないデータ型に最も有用です。データを安価にコピーできる場合、それをスマートポインタの後ろに置く必要はありません。",
            code("Vec"),
            "、",
            code("HashMap"),
            "、",
            code("String"),
            "などのデータ集約型の構造体に対して、スマートポインタを使用することでパフォーマンスの向上が見込まれます。",
        ],
        p![
            "この最適化は、子コンポーネントが値を更新しない場合に最も効果的であり、親コンポーネントがほとんど更新されない場合にさらに効果的です。これにより、",
            code("Rc<_>"),
            "は純粋なコンポーネントでpropsの値をラップするのに適した選択肢となります。",
        ],
        p!["ただし、子コンポーネントでデータを自分でクローンする必要がない限り、この最適化は無駄であり、不要な参照カウントのコストを追加するだけです。Yewのpropsはすでに参照カウントされており、内部でデータのクローンは行われません。"],
        h2!["レンダリング関数"],
        p![
            "コードの可読性のために、",
            code("html!"),
            "の一部の繰り返しコードを専用の分割関数に移行することは通常意味があります。これにより、コードが読みやすくなり、インデントが減り、良いデザインパターンを奨励します。特に、複数の場所で呼び出すことができるこれらの関数を使用して、コード量を減らすことができます。",
        ],
        h2!["純粋なコンポーネント"],
        p![
            "純粋なコンポーネントは、その状態を変更せず、コンテンツを表示し、メッセージを通常の可変コンポーネントに伝播するコンポーネントです。これらは、",
            code("html!"),
            "マクロ内でコンポーネント構文（",
            code("<SomePureComponent />"),
            "）を使用する点でビュー関数とは異なり、実装に応じてメモ化される可能性があります（これは、一度関数が呼び出されると、その値が「保存」されることを意味し、同じパラメータで複数回呼び出された場合、その値を再計算する必要がなく、最初の関数呼び出しから保存された値を返すだけです）。Yewは内部でpropsを比較するため、propsが変更された場合にのみUIを再レンダリングします。",
        ],
        h2!["ワークスペースを使用してコンパイル時間を短縮する"],
        p![
            "Yewの最大の欠点は、コンパイルにかかる時間が長いことです。プロジェクトのコンパイルにかかる時間は、",
            code("html!"),
            "マクロに渡されるコードの量に関連しているようです。小規模なプロジェクトでは問題にならないようですが、大規模なアプリケーションでは、コンパイラがアプリケーションのために行う作業量を最小限に抑えるためにコードを複数のクレートに分割することが理にかなっています。",
        ],
        p![
            "1つの方法として、メインクレートがルーティング/ページ選択を処理し、各ページごとに異なるクレートを作成することが考えられます。各ページは異なるコンポーネントまたは",
            code("Html"),
            "を生成する大きな関数である可能性があります。アプリケーションの異なる部分を含むクレート間で共有されるコードは、プロジェクトが依存する別のクレートに格納できます。理想的には、すべてのコードを再コンパイルするのではなく、メインクレートと1つのページクレートのみを再コンパイルすることになります。最悪の場合、「共通」クレートで何かを編集した場合、すべての依存コードを再コンパイルする必要があり、元の状態に戻ります。",
        ],
        p!["メインクレートが重すぎる場合や、深くネストされたページ（例：別のページ上にレンダリングされるページ）を迅速に反復したい場合は、メインページの簡略化された実装を作成し、作業中のコンポーネントを追加でレンダリングするためにサンプルクレートを使用できます。"],
        h2!["バイナリサイズの縮小"],
        ul![
            li!["Rustコードの最適化"],
            li![
                code("Cargo.toml"),
                "（リリースプロファイルの定義）",
            ],
            li![
                code("wasm-opt"),
                " を使用してwasmコードを最適化",
            ],
        ],
        p![
            bold![
                "注意：バイナリサイズの縮小に関する詳細は、",
                link!["https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", "Rust Wasmマニュアル"],
                "を参照してください。",
            ],
        ],
        h3!["Cargo.toml"],
        p![
            "リリースビルドをより小さくするために、",
            code("Cargo.toml"),
            "の",
            code("[profile.release]"),
            "セクションで利用可能な設定を使用して構成できます。",
        ],
        code_block_title("toml", "Cargo.toml", r#"[profile.release]
# バイナリサイズを小さくする
panic = 'abort'
# コード全体を最適化する（最適化は良くなるが、ビルド速度は遅くなる）
codegen-units = 1
# サイズを最適化する（より積極的なアプローチ）
opt-level = 'z'
# サイズを最適化する
# opt-level = 's'
# プログラム全体の解析を使用してリンク時に最適化
lto = true"#),
        h3!["開発版 Cargo 設定"],
        p![
            "Rust と cargo の実験的な開発版機能から追加の利点を得ることもできます。",
            code("trunk"),
            " の開発版ツールチェーンを使用するには、",
            code("RUSTUP_TOOLCHAIN=\"nightly\""),
            " 環境変数を設定します。その後、",
            code(".cargo/config.toml"),
            " で不安定な rustc 機能を構成できます。",
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html", "不安定な機能"],
            "のドキュメント、特に",
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html#build-std", "build-std"],
            "および",
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features", "build-std-features"],
            "に関する部分を参照して、設定方法を確認してください。",
        ],
        code_block_title("toml", ".cargo/config.toml", r#"[unstable]
# rust-srcコンポーネントが必要です。`rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#),
        admonition![AdmonitionType::Caution, None,
            p![
                "開発版のRustコンパイラには、",
                link!["https://github.com/yewstack/yew/issues/2696", "この例"],
                "のようなバグが含まれている可能性があるため、定期的に監視し調整する必要があります。これらの実験的なオプションを使用する際は注意が必要です。",
            ],
        ],
        h3!["wasm-opt"],
        p!["さらに、", code("wasm"), " コードのサイズを最適化することができます。"],
        p![
            "Rust Wasm マニュアルには、Wasm バイナリファイルのサイズを縮小する方法に関するセクションがあります：",
            link!["https://rustwasm.github.io/book/game-of-life/code-size.html", ".wasm サイズの縮小"],
        ],
        ul![
            li![
                code("wasm-pack"),
                " を使用すると、デフォルトでリリースビルドの wasm コードが最適化されます",
            ],
            li![
                "wasm ファイルに直接 ",
                code("wasm-opt"),
                " を使用する",
            ],
        ],
        code_block("text", "wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"),
        h4!["yew/examples/ の 'minimal' サンプルのビルドサイズ"],
        p![
            "注意：",
            code("wasm-pack"),
            " は Rust と Wasm コードの最適化を組み合わせています。",
            code("wasm-bindgen"),
            " は、この例では Rust のサイズ最適化を行っていません。",
        ],
        table(
            vec![vec!["ツールチェーン".into()], vec!["サイズ".into()]],
            vec![
                vec![vec!["wasm-bindgen".into()], vec!["158KB".into()]],
                vec![vec!["wasm-bindgen + wasm-opt -Os".into()], vec!["116KB".into()]],
                vec![vec!["wasm-pack".into()], vec!["99 KB".into()]],
            ],
        ),
        h2!["さらに読む"],
        ul![
            li![link!["https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", "Rust マニュアルのスマート ポインターに関する章"]],
            li![link!["https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", "Rust Wasm マニュアルのコードサイズの縮小に関する章"]],
            li![link!["https://doc.rust-lang.org/cargo/reference/profiles.html", "Rust プロファイルに関するドキュメント"]],
            li![link!["https://github.com/WebAssembly/binaryen", "binaryen プロジェクト"]],
        ],
    ])
    .with_description("Make your app faster")
}

crate::doc_page!(
    "最適化とベストプラクティス",
    "/ja/docs/advanced-topics/optimizations",
    page_content()
);
