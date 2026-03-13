pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("スマートポインタの使用")]),
        p(vec![
            bold(vec![
                text("注意：このセクションで使用されている用語に混乱がある場合は、Rustのマニュアルにある"),
                link("https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", vec![text("スマートポインタに関する章")]),
                text("が役立ちます。"),
            ]),
        ]),
        p(vec![
            text("再レンダリング時に大量のデータをクローンしてpropsを作成するのを避けるために、スマートポインタを使用してデータ自体ではなくデータへの参照のみをクローンすることができます。propsや子コンポーネントに関連データの参照を渡すことで、データを変更する必要がある子コンポーネントでデータをクローンするのを避けることができます。"),
            code("Rc::make_mut"),
            text("を使用してデータをクローンし、変更するための可変参照を取得できます。"),
        ]),
        p(vec![
            text("これにより、"),
            code("Component::changed"),
            text("でのpropの変更がコンポーネントの再レンダリングを必要とするかどうかを判断する際にさらに利点があります。これは、データの値ではなくポインタのアドレス（つまり、データがマシンメモリに格納されている場所）を比較できるためです。2つのポインタが同じデータを指している場合、それらが指しているデータの値は同じでなければなりません。逆は必ずしも真ではないことに注意してください！2つのポインタアドレスが異なる場合でも、基になるデータは同じである可能性があります。この場合、基になるデータを比較する必要があります。"),
        ]),
        p(vec![
            text("この比較を行うには、"),
            code("Rc::ptr_eq"),
            text(" を使用する必要があります。"),
            code("PartialEq"),
            text("（データを比較する際に自動的に使用される等価演算子"),
            code("=="),
            text("）ではなく。Rustのドキュメントには、"),
            link("https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq", vec![text("Rc::ptr_eqに関する詳細")]),
            text("があります。"),
        ]),
        p(vec![
            text("この最適化は、"),
            code("Copy"),
            text("を実装していないデータ型に最も有用です。データを安価にコピーできる場合、それをスマートポインタの後ろに置く必要はありません。"),
            code("Vec"),
            text("、"),
            code("HashMap"),
            text("、"),
            code("String"),
            text("などのデータ集約型の構造体に対して、スマートポインタを使用することでパフォーマンスの向上が見込まれます。"),
        ]),
        p(vec![
            text("この最適化は、子コンポーネントが値を更新しない場合に最も効果的であり、親コンポーネントがほとんど更新されない場合にさらに効果的です。これにより、"),
            code("Rc<_>"),
            text("は純粋なコンポーネントでpropsの値をラップするのに適した選択肢となります。"),
        ]),
        p(vec![text("ただし、子コンポーネントでデータを自分でクローンする必要がない限り、この最適化は無駄であり、不要な参照カウントのコストを追加するだけです。Yewのpropsはすでに参照カウントされており、内部でデータのクローンは行われません。")]),
        h2(vec![text("レンダリング関数")]),
        p(vec![
            text("コードの可読性のために、"),
            code("html!"),
            text("の一部の繰り返しコードを専用の分割関数に移行することは通常意味があります。これにより、コードが読みやすくなり、インデントが減り、良いデザインパターンを奨励します。特に、複数の場所で呼び出すことができるこれらの関数を使用して、コード量を減らすことができます。"),
        ]),
        h2(vec![text("純粋なコンポーネント")]),
        p(vec![
            text("純粋なコンポーネントは、その状態を変更せず、コンテンツを表示し、メッセージを通常の可変コンポーネントに伝播するコンポーネントです。これらは、"),
            code("html!"),
            text("マクロ内でコンポーネント構文（"),
            code("<SomePureComponent />"),
            text("）を使用する点でビュー関数とは異なり、実装に応じてメモ化される可能性があります（これは、一度関数が呼び出されると、その値が「保存」されることを意味し、同じパラメータで複数回呼び出された場合、その値を再計算する必要がなく、最初の関数呼び出しから保存された値を返すだけです）。Yewは内部でpropsを比較するため、propsが変更された場合にのみUIを再レンダリングします。"),
        ]),
        h2(vec![text("ワークスペースを使用してコンパイル時間を短縮する")]),
        p(vec![
            text("Yewの最大の欠点は、コンパイルにかかる時間が長いことです。プロジェクトのコンパイルにかかる時間は、"),
            code("html!"),
            text("マクロに渡されるコードの量に関連しているようです。小規模なプロジェクトでは問題にならないようですが、大規模なアプリケーションでは、コンパイラがアプリケーションのために行う作業量を最小限に抑えるためにコードを複数のクレートに分割することが理にかなっています。"),
        ]),
        p(vec![
            text("1つの方法として、メインクレートがルーティング/ページ選択を処理し、各ページごとに異なるクレートを作成することが考えられます。各ページは異なるコンポーネントまたは"),
            code("Html"),
            text("を生成する大きな関数である可能性があります。アプリケーションの異なる部分を含むクレート間で共有されるコードは、プロジェクトが依存する別のクレートに格納できます。理想的には、すべてのコードを再コンパイルするのではなく、メインクレートと1つのページクレートのみを再コンパイルすることになります。最悪の場合、「共通」クレートで何かを編集した場合、すべての依存コードを再コンパイルする必要があり、元の状態に戻ります。"),
        ]),
        p(vec![text("メインクレートが重すぎる場合や、深くネストされたページ（例：別のページ上にレンダリングされるページ）を迅速に反復したい場合は、メインページの簡略化された実装を作成し、作業中のコンポーネントを追加でレンダリングするためにサンプルクレートを使用できます。")]),
        h2(vec![text("バイナリサイズの縮小")]),
        ul(vec![
            li(vec![text("Rustコードの最適化")]),
            li(vec![
                code("Cargo.toml"),
                text("（リリースプロファイルの定義）"),
            ]),
            li(vec![
                code("wasm-opt"),
                text(" を使用してwasmコードを最適化"),
            ]),
        ]),
        p(vec![
            bold(vec![
                text("注意：バイナリサイズの縮小に関する詳細は、"),
                link("https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", vec![text("Rust Wasmマニュアル")]),
                text("を参照してください。"),
            ]),
        ]),
        h3(vec![text("Cargo.toml")]),
        p(vec![
            text("リリースビルドをより小さくするために、"),
            code("Cargo.toml"),
            text("の"),
            code("[profile.release]"),
            text("セクションで利用可能な設定を使用して構成できます。"),
        ]),
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
        h3(vec![text("開発版 Cargo 設定")]),
        p(vec![
            text("Rust と cargo の実験的な開発版機能から追加の利点を得ることもできます。"),
            code("trunk"),
            text(" の開発版ツールチェーンを使用するには、"),
            code("RUSTUP_TOOLCHAIN=\"nightly\""),
            text(" 環境変数を設定します。その後、"),
            code(".cargo/config.toml"),
            text(" で不安定な rustc 機能を構成できます。"),
            link("https://doc.rust-lang.org/cargo/reference/unstable.html", vec![text("不安定な機能")]),
            text("のドキュメント、特に"),
            link("https://doc.rust-lang.org/cargo/reference/unstable.html#build-std", vec![text("build-std")]),
            text("および"),
            link("https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features", vec![text("build-std-features")]),
            text("に関する部分を参照して、設定方法を確認してください。"),
        ]),
        code_block_title("toml", ".cargo/config.toml", r#"[unstable]
# rust-srcコンポーネントが必要です。`rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#),
        admonition(AdmonitionType::Caution, None, vec![
            p(vec![
                text("開発版のRustコンパイラには、"),
                link("https://github.com/yewstack/yew/issues/2696", vec![text("この例")]),
                text("のようなバグが含まれている可能性があるため、定期的に監視し調整する必要があります。これらの実験的なオプションを使用する際は注意が必要です。"),
            ]),
        ]),
        h3(vec![text("wasm-opt")]),
        p(vec![text("さらに、"), code("wasm"), text(" コードのサイズを最適化することができます。")]),
        p(vec![
            text("Rust Wasm マニュアルには、Wasm バイナリファイルのサイズを縮小する方法に関するセクションがあります："),
            link("https://rustwasm.github.io/book/game-of-life/code-size.html", vec![text(".wasm サイズの縮小")]),
        ]),
        ul(vec![
            li(vec![
                code("wasm-pack"),
                text(" を使用すると、デフォルトでリリースビルドの wasm コードが最適化されます"),
            ]),
            li(vec![
                text("wasm ファイルに直接 "),
                code("wasm-opt"),
                text(" を使用する"),
            ]),
        ]),
        code_block("text", "wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"),
        h4(vec![text("yew/examples/ の 'minimal' サンプルのビルドサイズ")]),
        p(vec![
            text("注意："),
            code("wasm-pack"),
            text(" は Rust と Wasm コードの最適化を組み合わせています。"),
            code("wasm-bindgen"),
            text(" は、この例では Rust のサイズ最適化を行っていません。"),
        ]),
        table(
            vec![vec![text("ツールチェーン")], vec![text("サイズ")]],
            vec![
                vec![vec![text("wasm-bindgen")], vec![text("158KB")]],
                vec![vec![text("wasm-bindgen + wasm-opt -Os")], vec![text("116KB")]],
                vec![vec![text("wasm-pack")], vec![text("99 KB")]],
            ],
        ),
        h2(vec![text("さらに読む")]),
        ul(vec![
            li(vec![link("https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", vec![text("Rust マニュアルのスマート ポインターに関する章")])]),
            li(vec![link("https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", vec![text("Rust Wasm マニュアルのコードサイズの縮小に関する章")])]),
            li(vec![link("https://doc.rust-lang.org/cargo/reference/profiles.html", vec![text("Rust プロファイルに関するドキュメント")])]),
            li(vec![link("https://github.com/WebAssembly/binaryen", vec![text("binaryen プロジェクト")])]),
        ]),
    ])
}

crate::doc_page!(
    "最適化とベストプラクティス",
    "/ja/docs/advanced-topics/optimizations",
    page_content()
);
