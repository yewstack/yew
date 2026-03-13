crate::doc_page!("最適化とベストプラクティス", "/ja/docs/advanced-topics/optimizations",
    Content::new(vec![
        h2(vec![text("効果的にスマートポインタを使う")]),
        p(vec![
            bold(vec![
                text("注意: このセクションで使われている用語がわからなければ Rust book は"),
                link("https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", vec![text("スマートポインタについての章")]),
                text("があり、非常に有用です。"),
            ]),
        ]),
        p(vec![
            text("再レンダリング時にpropsを作成するために大量のデータをクローンすることを避けるために、\
                  データそのものではなく、データへの参照のみをクローンするスマートポインタを使用できます。\
                  実際のデータではなく、propsと子コンポーネントで関連するデータへの参照を渡すことで、\
                  子コンポーネントでデータを変更する必要があるまでデータのクローンを避けることができます。\
                  その場合、"),
            code("Rc::make_mut"),
            text("を使用してクローンし、変更したいデータへの可変参照を取得できます。"),
        ]),
        p(vec![
            text("これにより、propの変更がコンポーネントの再レンダリングを必要とするかどうかを判断する際に、"),
            code("Component::changed"),
            text("でさらなる利点がもたらされます。これは、データの値を比較する代わりに、\
                  基礎となるポインタアドレス（つまり、データが格納されているマシンのメモリ上の位置）を\
                  比較できるためです。2つのポインタが同じデータを指している場合、それらが指すデータの値は\
                  同じでなければなりません。ただし、逆は必ずしも真ではないことに注意してください！\
                  2つのポインタアドレスが異なっていても、基礎となるデータは同じかもしれません。\
                  この場合、基礎となるデータを比較する必要があります。"),
        ]),
        p(vec![
            text("この比較を行うには、"),
            code("PartialEq"),
            text("（等値演算子"),
            code("=="),
            text("を使用してデータを比較する際に自動的に使用される）を\
                  使用するのではなく、"),
            code("Rc::ptr_eq"),
            text("を使用する必要があります。Rustのドキュメントには"),
            link("https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq", vec![text("Rc::ptr_eqについての詳細")]),
            text("があります。"),
        ]),
        p(vec![
            text("この最適化は、"),
            code("Copy"),
            text("を実装していないデータ型に最も有用です。データを安価にコピーできる場合、\
                  スマートポインタの背後に置く価値はありません。"),
            code("Vec"),
            text("、"),
            code("HashMap"),
            text("、"),
            code("String"),
            text("のようにデータが重くなる可能性のある\
                  構造体の場合、スマートポインタを使用するとパフォーマンスの改善が見込まれます。"),
        ]),
        p(vec![
            text("この最適化は、値が子によって決して更新されない場合に最も効果的であり、\
                  親によってもめったに更新されない場合はさらに効果的です。これにより、"),
            code("Rc<_>"),
            text("は\
                  ピュアコンポーネントのプロパティ値をラップするのに適した選択となります。"),
        ]),
        p(vec![
            text("ただし、子コンポーネントでデータを自分でクローンする必要がない限り、\
                  この最適化は無用であるだけでなく、参照カウントの不必要なコストも追加することに注意する必要があります。\
                  Yewのpropsはすでに参照カウントされており、内部的にデータのクローンは発生しません。"),
        ]),
        h2(vec![text("ビュー関数")]),
        p(vec![
            text("コードの可読性のために、"),
            code("html!"),
            text("のセクションを独自の関数に移行することはしばしば意味があります。\
                  これにより、インデントの量が減るためコードがより読みやすくなるだけでなく、\
                  良い設計パターンも促進されます。特に、これらの関数は複数の場所から呼び出すことができるため、\
                  記述する必要のあるコードの量が減り、構成可能なアプリケーションの構築に役立ちます。"),
        ]),
        h2(vec![text("ピュアコンポーネント")]),
        p(vec![
            text("ピュアコンポーネントは、状態を変更せず、コンテンツを表示し、\
                  通常の可変コンポーネントにメッセージを伝播するだけのコンポーネントです。\
                  ビュー関数とは異なり、式構文("),
            code("{some_view_function()}"),
            text(")ではなくコンポーネント構文("),
            code("<SomePureComponent />"),
            text(")を使用して"),
            code("html!"),
            text("マクロ内から使用でき、\
                  実装によってはメモ化できます（これは、関数が一度呼び出されるとその値が「保存」され、\
                  同じ引数で複数回呼び出された場合、値を再計算する必要がなく、\
                  最初の関数呼び出しから保存された値を返すことができることを意味します）。\
                  これにより、同一のpropsに対する再レンダリングを防ぎます。\
                  Yewは内部的にpropsを比較し、propsが変更された場合のみUIが再レンダリングされます。"),
        ]),
        h2(vec![text("ワークスペースを使用してコンパイル時間を短縮する")]),
        p(vec![
            text("間違いなく、Yewを使用する最大の欠点は、Yewアプリのコンパイルに長い時間がかかることです。\
                  プロジェクトのコンパイルにかかる時間は、"),
            code("html!"),
            text("マクロに渡されるコードの量に関連しているようです。\
                  これは小規模なプロジェクトではそれほど問題にならない傾向がありますが、大規模なアプリケーションでは、\
                  アプリケーションに加えられた各変更に対してコンパイラが行う必要がある作業量を最小限に抑えるために、\
                  コードを複数のクレートに分割することが理にかなっています。"),
        ]),
        p(vec![
            text("可能なアプローチの1つは、メインクレートにルーティング/ページ選択を処理させ、\
                  各ページに異なるクレートを作成することです。各ページは異なるコンポーネントか、"),
            code("Html"),
            text("を生成する大きな関数になる可能性があります。アプリケーションの異なる部分を含む\
                  クレート間で共有されるコードは、プロジェクトが依存する別のクレートに格納できます。\
                  最良のケースでは、各コンパイルですべてのコードを再ビルドすることから、\
                  メインクレートと1つのページクレートのみを再ビルドすることになります。\
                  最悪の場合、「共通」クレートで何かを編集すると、その共通に共有されるクレートに依存する\
                  すべてのコード（おそらく他のすべて）をコンパイルする元の状態に戻ってしまいます。"),
        ]),
        p(vec![
            text("メインクレートが重すぎる場合、または深くネストされたページ（例：別のページの上にレンダリングされるページ）で\
                  迅速に反復したい場合は、サンプルクレートを使用してメインページの簡略化された実装を作成し、\
                  作業中のコンポーネントを追加でレンダリングできます。"),
        ]),
        h2(vec![text("バイナリサイズを小さくする")]),
        ul(vec![
            li(vec![text("Rust のコードを最適化する")]),
            li(vec![
                code("cargo.toml"),
                text(" ( release profile を定義 )"),
            ]),
            li(vec![
                code("wasm-opt"),
                text("を用いて wasm のコードを最適化する"),
            ]),
        ]),
        p(vec![
            bold(vec![
                text("注意: バイナリサイズを小さくするのについては"),
                link("https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", vec![text("Rust Wasm Book")]),
                text("に詳しく書いてあります。"),
            ]),
        ]),
        h3(vec![text("Cargo.toml")]),
        p(vec![
            code("Cargo.toml"),
            text("で"),
            code("[profile.release]"),
            text("のセクションに設定を書き込むことでリリースビルドを小さくすることが可能です。"),
        ]),
        code_block_title("toml", "Cargo.toml", r#"[profile.release]
# バイナリに含むコードを少なくする
panic = 'abort'
# コードベース全体での最適化 ( 良い最適化だがビルドが遅くなる)
codegen-units = 1
# サイズの最適化( よりアグレッシブに )
opt-level = 'z'
# サイズの最適化
# opt-level = 's'
# プログラム全体の分析によるリンク時最適化
lto = true"#),
        h3(vec![text("Nightly Cargo設定")]),
        p(vec![
            text("rustとcargoの実験的なナイトリー機能から追加の利点を得ることもできます。"),
            code("trunk"),
            text("でナイトリーツールチェーンを使用するには、"),
            code("RUSTUP_TOOLCHAIN=\"nightly\""),
            text("環境変数を設定します。\
                  その後、"),
            code(".cargo/config.toml"),
            text("で不安定なrustc機能を設定できます。\
                  設定を理解するには、"),
            link("https://doc.rust-lang.org/cargo/reference/unstable.html", vec![text("unstable features")]),
            text("のドキュメント、特に"),
            link("https://doc.rust-lang.org/cargo/reference/unstable.html#build-std", vec![text("build-std")]),
            text("と"),
            link("https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features", vec![text("build-std-features")]),
            text("に関するセクションを参照してください。"),
        ]),
        code_block_title("toml", ".cargo/config.toml", r#"[unstable]
# rust-srcコンポーネントが必要です。`rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#),
        admonition(AdmonitionType::Warning, None, vec![
            p(vec![
                text("ナイトリーrustコンパイラには、"),
                link("https://github.com/yewstack/yew/issues/2696", vec![text("このような")]),
                text("バグが含まれている可能性があり、\
                      時折注意と調整が必要です。これらの実験的オプションは慎重に使用してください。"),
            ]),
        ]),
        h3(vec![text("wasm-opt")]),
        p(vec![text("更にwasmのコードのサイズを最適化することができます。")]),
        p(vec![
            text("The Rust Wasm Book には Wasm バイナリのサイズを小さくすることについてのセクションがあります: "),
            link("https://rustwasm.github.io/book/game-of-life/code-size.html", vec![text("Shrinking .wasm size")]),
        ]),
        ul(vec![
            li(vec![
                code("wasm-pack"),
                text("でデフォルトのwasmのコードをリリースビルド時に最適化する"),
            ]),
            li(vec![
                code("wasm-opt"),
                text("によって直接wasmファイルを最適化する"),
            ]),
        ]),
        code_block("text", "wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"),
        h4(vec![text("yew/examples/にある例を小さなサイズでビルドする")]),
        p(vec![
            text("注意: "),
            code("wasm-pack"),
            text("は Rust と Wasm のコードへの最適化を組み合わせます。"),
            code("wasm-bindgen"),
            text("はこの例では Rust のサイズ最適化を用いていません。"),
        ]),
        table(
            vec![vec![text("使用したツール")], vec![text("サイズ")]],
            vec![
                vec![vec![text("wasm-bindgen")], vec![text("158KB")]],
                vec![vec![text("wasm-bindgen + wasm-opt -Os")], vec![text("116KB")]],
                vec![vec![text("wasm-pack")], vec![text("99 KB")]],
            ],
        ),
        h2(vec![text("参考文献:")]),
        ul(vec![
            li(vec![
                link("https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", vec![text("The Rust Book のスマートポインタに関する章")]),
            ]),
            li(vec![
                link("https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", vec![text("the Rust Wasm Book でのバイナリサイズを小さくすることについて")]),
            ]),
            li(vec![
                link("https://doc.rust-lang.org/cargo/reference/profiles.html", vec![text("Rust profiles についてのドキュメント")]),
            ]),
            li(vec![
                link("https://github.com/WebAssembly/binaryen", vec![text("binaryen プロジェクト")]),
            ]),
        ]),
    ])
);
