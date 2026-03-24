crate::doc_page!("Optimizations & Best Practices", "/docs/advanced-topics/optimizations",
    Content::new(vec![
        h2![text("Using smart pointers effectively")],
        p![
            bold![
                text("Note: if you're unsure about some of the terms used in this section, the Rust Book has a useful "),
                link!("https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", text("chapter about smart pointers")),
                text("."),
            ],
        ],
        p![
            text("To avoid cloning large amounts of data to create props when re-rendering, we can use \
              smart pointers to only clone a reference to the data instead of the data itself. If you pass \
              references to the relevant data in your props and child components instead of the actual data you \
              can avoid cloning any data until you need to modify it in the child component, where you can \
              use "),
            code("Rc::make_mut"),
            text(" to clone and obtain a mutable reference to the data you want to alter."),
        ],
        p![
            text("This brings further benefits in "),
            code("Component::changed"),
            text(" when working out whether prop changes require \
              the component to re-render. This is because instead of comparing the value of the data the \
              underlying pointer addresses (i.e. the position in a machine's memory where the data is stored) can \
              instead be compared; if two pointers point to the same data then the value of the data they point to \
              must be the same. Note that the inverse might not be true! Even if two pointer addresses differ the \
              underlying data might still be the same - in this case you should compare the underlying data."),
        ],
        p![
            text("To do this comparison you'll need to use "),
            code("Rc::ptr_eq"),
            text(" instead of just using "),
            code("PartialEq"),
            text(" (which is automatically used when comparing data using the equality operator "),
            code("=="),
            text("). The Rust documentation has "),
            link!("https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq", text("more details about Rc::ptr_eq")),
            text("."),
        ],
        p![
            text("This optimization is most useful for data types that don't implement "),
            code("Copy"),
            text(". If you can copy your data cheaply, then it isn't worth putting it behind a smart pointer. For structures that \
              can be data-heavy like "),
            code("Vec"),
            text("s, "),
            code("HashMap"),
            text("s, and "),
            code("String"),
            text("s using smart pointers is likely to bring performance improvements."),
        ],
        p![
            text("This optimization works best if the values are never updated by the children, and even better if \
              they are rarely updated by parents. This makes "),
            code("Rc<_>"),
            text("s a good choice for wrapping property values in pure components."),
        ],
        p![
            text("However, it must be noted that unless you need to clone the data yourself in the child component, \
              this optimization is not only useless, but it also adds the unnecessary cost of reference counting. Props \
              in Yew are already reference counted and no data clones occur internally."),
        ],
        h2![text("View functions")],
        p![
            text("For code readability reasons, it often makes sense to migrate sections of "),
            code("html!"),
            text(" to their own functions. Not only does this make your code more readable because it reduces the amount of \
              indentation present, it also encourages good design patterns, particularly around building \
              composable applications because these functions can be called in multiple places which reduces the \
              amount of code that has to be written."),
        ],
        h2![text("Pure Components")],
        p![
            text("Pure components are components that don't mutate their state, only displaying content and \
              propagating messages up to normal, mutable components. They differ from view functions in that they \
              can be used from within the "),
            code("html!"),
            text(" macro using the component syntax ("),
            code("<SomePureComponent />"),
            text(") instead of expression syntax ("),
            code("{some_view_function()}"),
            text("), and that depending on their \
              implementation, they can be memoized (this means that once a function is called its value is \"saved\" \
              so that if it's called with the same arguments more than once it doesn't have to recompute its value \
              and can just return the saved value from the first function call) - preventing re-renders for \
              identical props. Yew compares the props internally and so the UI is only re-rendered if the props change."),
        ],
        h2![text("Reducing compile time using workspaces")],
        p![
            text("Arguably, the largest drawback to using Yew is the long time it takes to compile Yew apps. The time \
              taken to compile a project seems to be related to the quantity of code passed to the "),
            code("html!"),
            text(" macro. \
              This tends to not be much of an issue for smaller projects, but for larger applications, it makes \
              sense to split code across multiple crates to minimize the amount of work the compiler has to do for \
              each change made to the application."),
        ],
        p![
            text("One possible approach is to make your main crate handle routing/page selection, and then make a \
              different crate for each page, where each page could be a different component or just a big \
              function that produces "),
            code("Html"),
            text(". Code that is shared between the crates containing different parts of \
              the application could be stored in a separate crate which the project depends on. \
              In the best-case scenario, you go from rebuilding all of your code on each compile to rebuilding \
              only the main crate, and one of your page crates. In the worst case, where you edit something in the \
              \"common\" crate, you will be right back to where you started: compiling all code that depends on that \
              commonly shared crate, which is probably everything else."),
        ],
        p![
            text("If your main crate is too heavyweight, or you want to rapidly iterate on a deeply nested page (e.g. \
              a page that renders on top of another page), you can use an example crate to create a simplified \
              implementation of the main page and additionally render the component you are working on."),
        ],
        h2![text("Reducing binary sizes")],
        ul![
            li![text("optimize Rust code")],
            li![
                code("cargo.toml"),
                text(" ( defining release profile )"),
            ],
            li![
                text("optimize wasm code using "),
                code("wasm-opt"),
            ],
        ],
        p![
            bold![
                text("Note: more information about reducing binary sizes can be found in the "),
                link!("https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", text("Rust Wasm Book")),
                text("."),
            ],
        ],
        h3![text("Cargo.toml")],
        p![
            text("It is possible to configure release builds to be smaller using the available settings in the "),
            code("[profile.release]"),
            text(" section of your "),
            code("Cargo.toml"),
            text("."),
        ],
        code_block_title("toml", "Cargo.toml", r#"[profile.release]
# less code to include into binary
panic = 'abort'
# optimization over all codebase ( better optimization, slower build )
codegen-units = 1
# optimization for size ( more aggressive )
opt-level = 'z'
# optimization for size
# opt-level = 's'
# link time optimization using using whole-program analysis
lto = true"#),
        h3![text("Nightly Cargo configuration")],
        p![
            text("You can also gain additional benefits from experimental nightly features of rust and \
              cargo. To use the nightly toolchain with "),
            code("trunk"),
            text(", set the "),
            code("RUSTUP_TOOLCHAIN=\"nightly\""),
            text(" environment variable. Then, you can configure unstable rustc features in your "),
            code(".cargo/config.toml"),
            text(". Refer to the doc of "),
            link!("https://doc.rust-lang.org/cargo/reference/unstable.html", text("unstable features")),
            text(", specifically the section about "),
            link!("https://doc.rust-lang.org/cargo/reference/unstable.html#build-std", text("build-std")),
            text(" and "),
            link!("https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features", text("build-std-features")),
            text(", to understand the configuration."),
        ],
        code_block_title("toml", ".cargo/config.toml", r#"[unstable]
# Requires the rust-src component. `rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#),
        admonition![AdmonitionType::Caution, None,
            p![
                text("The nightly rust compiler can contain bugs, such as "),
                link!("https://github.com/yewstack/yew/issues/2696", text("this one")),
                text(", that require occasional attention and tweaking. Use these experimental options with care."),
            ],
        ],
        h3![text("wasm-opt")],
        p![
            text("Further, it is possible to optimize the size of "),
            code("wasm"),
            text(" code."),
        ],
        p![
            text("The Rust Wasm Book has a section about reducing the size of Wasm binaries: "),
            link!("https://rustwasm.github.io/book/game-of-life/code-size.html", text("Shrinking .wasm size")),
        ],
        ul![
            li![
                text("using "),
                code("wasm-pack"),
                text(" which by default optimizes "),
                code("wasm"),
                text(" code in release builds"),
            ],
            li![
                text("using "),
                code("wasm-opt"),
                text(" directly on "),
                code("wasm"),
                text(" files."),
            ],
        ],
        code_block("text", "wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"),
        h4![text("Build size of 'minimal' example in yew/examples/")],
        p![
            text("Note: "),
            code("wasm-pack"),
            text(" combines optimization for Rust and Wasm code. "),
            code("wasm-bindgen"),
            text(" is used in this example without any Rust size optimization."),
        ],
        table(
            vec![vec![text("used tool")], vec![text("size")]],
            vec![
                vec![vec![text("wasm-bindgen")], vec![text("158KB")]],
                vec![vec![text("wasm-bindgen + wasm-opt -Os")], vec![text("116KB")]],
                vec![vec![text("wasm-pack")], vec![text("99 KB")]],
            ],
        ),
        h2![text("Further reading:")],
        ul![
            li![
                link!("https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", text("The Rust Book's chapter on smart pointers")),
            ],
            li![
                link!("https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", text("Information from the Rust Wasm Book about reducing binary sizes")),
            ],
            li![
                link!("https://doc.rust-lang.org/cargo/reference/profiles.html", text("Documentation about Rust profiles")),
            ],
            li![
                link!("https://github.com/WebAssembly/binaryen", text("binaryen project")),
            ],
        ],
    ])
);
