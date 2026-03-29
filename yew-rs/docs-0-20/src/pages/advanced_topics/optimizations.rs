crate::doc_page!("Optimizations & Best Practices", "/docs/advanced-topics/optimizations",
    Content::new(vec![
        h2!["Using smart pointers effectively"],
        p![
            bold![
                "Note: if you're unsure about some of the terms used in this section, the Rust Book has a useful ",
                link!["https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", "chapter about smart pointers"],
                ".",
            ],
        ],
        p![
            "In an effort to avoid cloning large amounts of data to create props when re-rendering, we can use \
              smart pointers to only clone a reference to the data instead of the data itself. If you pass \
              references to the relevant data in your props and child components instead of the actual data you \
              can avoid cloning any data until you need to modify it in the child component, where you can \
              use ",
            code("Rc::make_mut"),
            " to clone and obtain a mutable reference to the data you want to alter.",
        ],
        p![
            "This brings further benefits in ",
            code("Component::changed"),
            " when working out whether prop changes require \
              the component to re-render. This is because instead of comparing the value of the data the \
              underlying pointer addresses (i.e. the position in a machine's memory where the data is stored) can \
              instead be compared; if two pointers point to the same data then the value of the data they point to \
              must be the same. Note that the inverse might not be true! Even if two pointer addresses differ the \
              underlying data might still be the same - in this case you should compare the underlying data.",
        ],
        p![
            "To do this comparison you'll need to use ",
            code("Rc::ptr_eq"),
            " instead of just using ",
            code("PartialEq"),
            " (which is automatically used when comparing data using the equality operator ",
            code("=="),
            "). The Rust documentation has ",
            link!["https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq", "more details about Rc::ptr_eq"],
            ".",
        ],
        p![
            "This optimization is most useful for data types that don't implement ",
            code("Copy"),
            ". If you can copy your data cheaply, then it isn't worth putting it behind a smart pointer. For structures that \
              can be data-heavy like ",
            code("Vec"),
            "s, ",
            code("HashMap"),
            "s, and ",
            code("String"),
            "s using smart pointers is likely to bring performance improvements.",
        ],
        p![
            "This optimization works best if the values are never updated by the children, and even better, if \
              they are rarely updated by parents. This makes ",
            code("Rc<_>"),
            "s a good choice for wrapping property values in for pure components.",
        ],
        p![
            "However, it must be noted that unless you need to clone the data yourself in the child component, \
              this optimization is not only useless, it also adds unnecessary cost of reference counting. Props \
              in Yew are already reference counted and no data clones occur internally.",
        ],
        h2!["View functions"],
        p![
            "For code readability reasons, it often makes sense to migrate sections of ",
            code("html!"),
            " to their own functions. Not only does this make your code more readable because it reduces the amount of \
              indentation present, it also encourages good design patterns - particularly around building \
              composable applications because these functions can be called in multiple places which reduces the \
              amount of code that has to be written.",
        ],
        h2!["Pure Components"],
        p![
            "Pure components are components that don't mutate their state, only displaying content and \
              propagating messages up to normal, mutable components. They differ from view functions in that they \
              can be used from within the ",
            code("html!"),
            " macro using the component syntax (",
            code("<SomePureComponent />"),
            ") instead of expression syntax (",
            code("{{some_view_function()}}"),
            "), and that depending on their \
              implementation, they can be memoized (this means that once a function is called its value is \"saved\" \
              so that if it's called with the same arguments more than once it doesn't have to recompute its value \
              and can just return the saved value from the first function call) - preventing re-renders for \
              identical props. Yew compares the props internally and so the UI is only re-rendered if the props change.",
        ],
        h2!["Reducing compile time using workspaces"],
        p![
            "Arguably, the largest drawback to using Yew is the long time it takes to compile Yew apps. The time \
              taken to compile a project seems to be related to the quantity of code passed to the ",
            code("html!"),
            " macro. This tends to not be much of an issue for smaller projects, but for larger applications it makes \
              sense to split code across multiple crates to minimize the amount of work the compiler has to do for \
              each change made to the application.",
        ],
        p![
            "One possible approach is to make your main crate handle routing/page selection, and then make a \
              different crate for each page, where each page could be a different component, or just a big \
              function that produces ",
            code("Html"),
            ". Code which is shared between the crates containing different parts of \
              the application could be stored in a separate crate which is depended on throughout the project. \
              In the best case scenario, you go from rebuilding all of your code on each compile to rebuilding \
              only the main crate, and one of your page crates. In the worst case, where you edit something in the \
              \"common\" crate, you will be right back to where you started: compiling all code that depends on that \
              commonly shared crate, which is probably everything else.",
        ],
        p![
            "If your main crate is too heavyweight, or you want to rapidly iterate on a deeply nested page (eg. \
              a page that renders on top of another page), you can use an example crate to create a simplified \
              implementation of the main page and render the component you are working on on top of that.",
        ],
        h2!["Reducing binary sizes"],
        ul![
            li!["optimize Rust code"],
            li![
                code("cargo.toml"),
                " ( defining release profile )",
            ],
            li![
                "optimize wasm code using ",
                code("wasm-opt"),
            ],
        ],
        p![
            bold![
                "Note: more information about reducing binary sizes can be found in the ",
                link!["https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", "Rust Wasm Book"],
                ".",
            ],
        ],
        h3!["Cargo.toml"],
        p![
            "It is possible to configure release builds to be smaller using the available settings in the ",
            code("[profile.release]"),
            " section of your ",
            code("Cargo.toml"),
            ".",
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
        h3!["Nightly Cargo configuration"],
        p![
            "You can also gain additional benefits from experimental nightly features of rust and \
              cargo. To use the nightly toolchain with ",
            code("trunk"),
            ", set the ",
            code("RUSTUP_TOOLCHAIN=\"nightly\""),
            " environment variable. Then, you can configure unstable rustc features in your ",
            code(".cargo/config.toml"),
            ". Refer to the doc of ",
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html", "unstable features"],
            ", specifically the section about ",
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html#build-std", "build-std"],
            " and ",
            link!["https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features", "build-std-features"],
            ", to understand the configuration.",
        ],
        code_block_title("toml", ".cargo/config.toml", r#"[unstable]
# Requires the rust-src component. `rustup +nightly component add rust-src`
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]"#),
        admonition![AdmonitionType::Warning, None,
            p![
                "The nightly rust compiler can contain bugs, such as ",
                link!["https://github.com/yewstack/yew/issues/2696", "this one"],
                ", that require occasional attention and tweaking. Use these experimental options with care.",
            ],
        ],
        h3!["wasm-opt"],
        p![
            "Further more it is possible to optimize size of ",
            code("wasm"),
            " code.",
        ],
        p![
            "The Rust Wasm Book has a section about reducing the size of Wasm binaries: ",
            link!["https://rustwasm.github.io/book/game-of-life/code-size.html", "Shrinking .wasm size"],
        ],
        ul![
            li![
                "using ",
                code("wasm-pack"),
                " which by default optimizes ",
                code("wasm"),
                " code in release builds",
            ],
            li![
                "using ",
                code("wasm-opt"),
                " directly on ",
                code("wasm"),
                " files.",
            ],
        ],
        code_block("text", r#"wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm"#),
        h4!["Build size of 'minimal' example in yew/examples/"],
        p![
            "Note: ",
            code("wasm-pack"),
            " combines optimization for Rust and Wasm code. ",
            code("wasm-bindgen"),
            " is used in this example without any Rust size optimization.",
        ],
        table(
            vec![
                vec!["used tool".into()],
                vec!["size".into()],
            ],
            vec![
                vec![
                    vec!["wasm-bindgen".into()],
                    vec!["158KB".into()],
                ],
                vec![
                    vec!["wasm-bindgen + wasm-opt -Os".into()],
                    vec!["116KB".into()],
                ],
                vec![
                    vec!["wasm-pack".into()],
                    vec!["99 KB".into()],
                ],
            ],
        ),
        h2!["Further reading:"],
        ul![
            li![
                link!["https://doc.rust-lang.org/book/ch15-00-smart-pointers.html", "The Rust Book's chapter on smart pointers"],
            ],
            li![
                link!["https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size", "Information from the Rust Wasm Book about reducing binary sizes"],
            ],
            li![
                link!["https://doc.rust-lang.org/cargo/reference/profiles.html", "Documentation about Rust profiles"],
            ],
            li![
                link!["https://github.com/WebAssembly/binaryen", "binaryen project"],
            ],
        ],
    ])
    .with_description("Make your app faster")
);
