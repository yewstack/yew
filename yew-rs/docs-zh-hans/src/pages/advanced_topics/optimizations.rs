pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("使用智能指针")],
        p![bold![
            text("注意：如果您对本节中使用的某些术语感到困惑，Rust 手册中有一个有用的"),
            link![
                "https://doc.rust-lang.org/book/ch15-00-smart-pointers.html",
                text("关于智能指针的章节"),
            ],
            text("。"),
        ]],
        p![
            text("为了避免在重新渲染时克隆大量数据以创建 props，我们可以使用智能指针，只克隆对数据的引用而不是数据本身。如果您在 props 和子组件中传递与相关数据的引用而不是实际数据，您可以避免在需要修改数据的子组件中克隆任何数据，您可以使用 "),
            code("Rc::make_mut"),
            text(" 来克隆并获得要更改的数据的可变引用。"),
        ],
        p![
            text("这在 "),
            code("Component::changed"),
            text(" 中带来了更多好处，可以确定 prop 更改是否需要组件重新渲染。这是因为可以比较指针地址（即数据存储在机器内存中的位置）而不是数据的值；如果两个指针指向相同的数据，则它们指向的数据的值必须相同。请注意，反之可能不成立！即使两个指针地址不同，底层数据仍可能相同 - 在这种情况下，您应该比较底层数据。"),
        ],
        p![
            text("要进行此比较，您需要使用 "),
            code("Rc::ptr_eq"),
            text(" 而不仅仅使用 "),
            code("PartialEq"),
            text("（在使用相等运算符 "),
            code("=="),
            text(" 比较数据时自动使用）。Rust 文档有关于 "),
            code("Rc::ptr_eq"),
            text(" 的"),
            link!["https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq", text("更多细节")],
            text("。"),
        ],
        p![
            text("这种优化对于不实现 "),
            code("Copy"),
            text(" 的数据类型最有用。如果您可以廉价地复制数据，则没有必要将其放在智能指针后面。对于可能是数据密集型的结构，如 "),
            code("Vec"),
            text("、"),
            code("HashMap"),
            text(" 和 "),
            code("String"),
            text("，使用智能指针可能会带来性能改进。"),
        ],
        p![
            text("如果值从不被子组件更新，则此优化效果最佳，如果父组件很少更新，则效果更佳。这使得 "),
            code("Rc<_>"),
            text(" 是在纯组件中包装属性值的一个不错的选择。"),
        ],
        p![
            text("但是，必须注意，除非您需要在子组件中自己克隆数据，否则这种优化不仅是无用的，而且还增加了不必要的引用计数成本。Yew 中的 props 已经是引用计数的，内部不会发生数据克隆。"),
        ],
        h2![text("渲染函数")],
        p![
            text("出于代码可读性的原因，将 "),
            code("html!"),
            text(" 的部分重复代码迁移到专门分割出来的函数中通常是有意义的。这不仅使您的代码更易读，减少了代码缩进，而且还鼓励良好的设计模式——特别是围绕构建可组合应用程序，这些函数可以在多个地方调用，从而减少代码量。"),
        ],
        h2![text("纯组件")],
        p![
            text("纯组件是不会改变其状态的组件，只显示内容并将消息传播到普通的可变组件。它们与视图函数的不同之处在于，它们可以在 "),
            code("html!"),
            text(" 宏中使用组件语法（"),
            code("<SomePureComponent />"),
            text("）而不是表达式语法（"),
            code("{some_view_function()}"),
            text("），并且根据其实现，它们可以被记忆化（这意味着一旦调用函数，其值就会被\"保存\"，因此如果多次使用相同的参数调用它，则不必重新计算其值，只需从第一个函数调用返回保存的值）- 防止相同的 props 重新渲染。Yew 在内部比较 props，因此仅在 props 更改时重新渲染 UI。"),
        ],
        h2![text("使用工作区减少编译时间")],
        p![
            text("Yew 的最大缺点是编译所需的时间很长。编译项目所需的时间似乎与传递给 "),
            code("html!"),
            text(" 宏的代码数量有关。对于较小的项目，这似乎不是什么问题，但对于较大的应用程序，将代码拆分到多个 crate 中以最小化编译器为应用程序所做的工作量是有意义的。"),
        ],
        p![
            text("一种可能的方法是使您的主 crate 处理路由/页面选择，然后为每个页面创建一个不同的 crate，其中每个页面可以是不同的组件或只是生成 "),
            code("Html"),
            text(" 的大函数。存储在包含应用程序不同部分的 crate 之间的代码可以存储在项目依赖的单独 crate 中。在最理想的情况下，您从在每次编译时重新构建所有代码到仅重新构建主 crate 和一个页面 crate。在最坏的情况下，如果您在\"common\" crate 中编辑了某些内容，您将回到起点：编译依赖于该常用共享 crate 的所有代码，这可能是其他所有内容。"),
        ],
        p![
            text("如果您的主 crate 太重，或者您想快速迭代一个深度嵌套的页面（例如。在另一个页面上渲染的页面），您可以使用示例 crate 创建主页面的简化实现，并额外渲染您正在处理的组件。"),
        ],
        h2![text("减小二进制文件大小")],
        ul![
            li![text("优化 Rust 代码")],
            li![code("cargo.toml"), text("（定义发布配置文件）")],
            li![text("使用 "), code("wasm-opt"), text(" 优化 wasm 代码")],
        ],
        p![bold![
            text("注意：有关减小二进制文件大小的更多信息，请参阅"),
            link![
                "https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size",
                text("Rust Wasm 手册"),
            ],
            text("。"),
        ]],
        h3![text("Cargo.toml")],
        p![
            text("可以使用 "),
            code("Cargo.toml"),
            text(" 中 "),
            code("[profile.release]"),
            text(" 部分中的可用设置来配置发布构建为更小。"),
        ],
        code_block_title("toml", "Cargo.toml", r#"[profile.release]
# 让二进制文件尺寸更小些
panic = 'abort'
# 优化整个代码库（优化更好，但构建速度也会更慢）
codegen-units = 1
# 优化尺寸（更激进的做法）
opt-level = 'z'
# 优化尺寸
# opt-level = 's'
# 使用程序整体分析时进行链接时优化
lto = true"#),
        h3![text("开发版 Cargo 配置")],
        p![
            text("您还可以从 Rust 和 cargo 的实验性开发版功能中获得额外的好处。要使用 "),
            code("trunk"),
            text(" 的开发版工具链，请设置 "),
            code("RUSTUP_TOOLCHAIN=\"nightly\""),
            text(" 环境变量。然后，您可以在 "),
            code(".cargo/config.toml"),
            text(" 中配置不稳定的 rustc 功能。请参考"),
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html", text("不稳定功能")],
            text("的文档，特别是关于"),
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html#build-std", code("build-std")],
            text("和"),
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features", code("build-std-features")],
            text("的部分，以了解配置。"),
        ],
        code_block_title("toml", ".cargo/config.toml", r#"[unstable]
# 需要 rust-src 组件。`rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#),
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("开发版 Rust 编译器可能包含错误，例如"),
                link!["https://github.com/yewstack/yew/issues/2696", text("这个例子")],
                text("，需要偶尔关注和调整。请谨慎使用这些实验性选项。"),
            ],
        ],
        h3![text("wasm-opt")],
        p![
            text("此外，可以优化 "),
            code("wasm"),
            text(" 代码的大小。"),
        ],
        p![
            text("Rust Wasm 手册中有关于减小 Wasm 二进制文件大小的部分："),
            link!["https://rustwasm.github.io/book/game-of-life/code-size.html", text("缩小 .wasm 大小")],
        ],
        ul![
            li![
                text("使用 "),
                code("wasm-pack"),
                text("，默认情况下会优化发布构建中的 "),
                code("wasm"),
                text(" 代码"),
            ],
            li![
                text("直接在 "),
                code("wasm"),
                text(" 文件上使用 "),
                code("wasm-opt"),
            ],
        ],
        code_block("text", r#"wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"#),
        h4![text("在 yew/examples/ 中 'minimal' 示例的构建大小")],
        p![
            text("注意："),
            code("wasm-pack"),
            text(" 结合了 Rust 和 Wasm 代码的优化。在此示例中，"),
            code("wasm-bindgen"),
            text(" 未经任何 Rust 大小优化。"),
        ],
        table(
            vec![vec![text("工具链")], vec![text("大小")]],
            vec![
                vec![vec![text("wasm-bindgen")], vec![text("158KB")]],
                vec![vec![text("wasm-bindgen + wasm-opt -Os")], vec![text("116KB")]],
                vec![vec![text("wasm-pack")], vec![text("99 KB")]],
            ],
        ),
        h2![text("进一步阅读")],
        ul![
            li![
                link!["https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", text("Rust 手册中关于智能指针的章节")],
            ],
            li![
                link!["https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", text("Rust Wasm 手册中关于减小二进制文件大小的信息")],
            ],
            li![
                link!["https://doc.rust-lang.org/cargo/reference/profiles.html", text("Rust 配置文件的文档")],
            ],
            li![
                link!["https://github.com/WebAssembly/binaryen", text("binaryen 项目")],
            ],
        ],
    ])
}

crate::doc_page!(
    "优化 & 最佳实践",
    "/zh-Hans/docs/advanced-topics/optimizations",
    page_content()
);
