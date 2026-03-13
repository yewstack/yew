crate::doc_page!(
    "",
    "/zh-Hans/docs/advanced-topics/optimizations",
    Content::new(vec![
        h1(vec![text("性能优化与最佳实践")]),
        h2(vec![text("neq_assign")]),
        p(vec![
            text("当组件从它的父组件接收 props 时，"),
            code("change"),
            text(" 方法将被调用。除了允许你更新组件的状态，还允许你返回一个布尔类型的值 "),
            code("ShouldRender"),
            text(" 来指示组件是否应该响应 props 的更改而重新渲染自身。"),
        ]),
        p(vec![
            text(
                "重新渲染的开销很大，你应该尽量避免。一个通用的法则是，你只应该在 props \
                 实际更改时重新渲染。以下代码块展示了此法则，如果 props 和先前的 props \
                 不同，则返回 "
            ),
            code("true"),
            text("："),
        ]),
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
lto = true"#
        ),
        p(vec![
            text("但是我们可以更进一步！对于任何实现了 "),
            code("PartialEq"),
            text(
                " 的项，可以使用一个 trait 和一个 blanket implementation \
                 将这六行样板代码减少到一行。"
            ),
        ]),
        code_block(
            "rust",
            r#"[unstable]
# Requires the rust-src component. `rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#
        ),
        p(vec![
            text("该 trait 称为 "),
            code("NeqAssign"),
            text(" 是因为如果目标值和新值不相等，它将赋为新值。"),
        ]),
        p(vec![text("这比简单的实现还要短：")]),
        code_block("rust", r#"wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"#),
        p(vec![
            text("你不仅限在 "),
            code("change"),
            text(" 函数中使用它。通常，在 "),
            code("update"),
            text(" 函数中执行此操作也是有意义的，尽管性能提升在那里不太明显。"),
        ]),
        h2(vec![text("RC")]),
        p(vec![
            text(
                "为了避免在重新渲染时为了创建 props \
                 而克隆大块数据，我们可以使用智能指针来只克隆指针。如果在 props 和子组件中使用 "
            ),
            code("Rc<_>"),
            text(
                " 而不是普通未装箱的值，则可以延迟克隆直到需要修改子组件中的数据为止，\
                 在该组件中可以使用 "
            ),
            code("Rc::make_mut"),
            text(
                " 来对要更改数据进行克隆和获取可变引用。通过在要修改前不进行克隆，\
                 子组件可以在几乎没有性能成本的情况下拒绝与它们在 "
            ),
            code("Component::change"),
            text(
                " 中拥有状态的 props 相同的 props，这与数据本身需要先复制到父级 props \
                 结构体中，然后在子级中进行比较和拒绝的情况相反。"
            ),
        ]),
        p(vec![
            text("对于不是 "),
            code("Copy"),
            text(
                " 类型的数据，这种优化是最有用的。如果你能轻松地拷贝数据，\
                 那么将其放入智能指针中可能是不值得的。对于可以包含大量数据的结构，例如 "
            ),
            code("Vec"),
            text("，"),
            code("HashMap"),
            text(" 和 "),
            code("String"),
            text("，这种优化应该是值得的。"),
        ]),
        p(vec![
            text(
                "如果子组件从不更新组件的值，则这种优化效果最好，如果父组件很少更新组件的值，\
                 则效果更好。这使得 "
            ),
            code("Rc<_>s"),
            text(" 是包装纯组件属性值的不错选择。"),
        ]),
        h2(vec![text("视图函数")]),
        p(vec![
            text("出于代码可读性的原因，将 "),
            code("html!"),
            text(
                " 各个部分的代码迁移到他们自己的函数中通常是有意义的，这样就可以避免在深层嵌套的 \
                 HTML 中出现代码块向右偏移。"
            ),
        ]),
        h2(vec![text("纯组件 / 函数式组件")]),
        p(vec![
            text(
                "纯组件是不会修改它们状态的组件，它们仅展示内容和向普通可变组件传递消息。\
                 它们与视图函数不同之处在于他们可以使用组件语法（"
            ),
            code("<SomePureComponent />"),
            text("）而不是表达式语法（"),
            code("{{some_view_function()}}"),
            text("）来在 "),
            code("html!"),
            text(" 宏中使用，并且根据它们的实现，它们可以被记忆化 - 使用前面提到的 "),
            code("neq_assign"),
            text(" 逻辑来防止因为相同的 props 而重新渲染。"),
        ]),
        p(vec![text(
            "Yew 没有原生支持纯组件或者函数式组件，但是可以通过外部库获取它们。"
        )]),
        p(vec![text(
            "函数式组件尚不存在，但是从理论上来讲，可以通过使用 proc 宏和标注函数生成纯组件。"
        )]),
        h2(vec![text("Keyed DOM nodes when they arrive")]),
        h2(vec![text("使用 Cargo Workspaces 进行编译速度优化")]),
        p(vec![
            text("可以说，使用 Yew 的最大缺点是编译时间长。编译时间似乎与 "),
            code("html!"),
            text(
                " 宏块中的代码量相关。对于较小的项目，这通常不是什么大问题，但是对于跨多个页面的 \
                 web 应用程序，将代码拆分为多个 crates \
                 以最大程度地减少编译器要做的工作通常是有意义的。"
            ),
        ]),
        p(vec![
            text(
                "你应该尝试让主 crate 处理路由和页面选择，将所有公用的代码移动到另一个 \
                 crate，然后为每一个页面创建一个不同的 \
                 crate，其中每个页面可能是一个不同的组件，或者只是一个产生 "
            ),
            code("Html"),
            text(
                " 的大函数。在最好的情况下，你将从重新构建所有代码到只重新构建主 crate \
                 和一个页面的 crate。在最糟糕的情况下，当你在\"公共\" crate \
                 中编辑内容时，你将回到起点：编译所有依赖此公用 crate \
                 的代码，这可能就是除此之外的所有代码。"
            ),
        ]),
        p(vec![text(
            "如果你的主 crate \
             过于庞大，或者你想在深层嵌套的页面（例如，在另一个页面顶部渲染的页面）中快速迭代，\
             则可以使用一个示例 crate 创建一个更简单的主页面实现并在之上渲染你正在开发的组件。"
        )]),
    ])
);
