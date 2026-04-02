crate::doc_page!(
    "web-sys",
    "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
    Content::new(vec![
        p!["The ", code("web-sys"), " crate"],
        h2!["Features in web-sys"],
        p!["The ", code("web-sys"), code("web-sys"), " and exposes some types in its public API. You will often need to add ", code("web-sys"), " as a dependency yourself."],
        h2!["Inheritance in web-sys"],
        p!["In the ", doc_link!(crate::pages::concepts::basic_web_technologies::wasm_bindgen, #"simulating-inheritance", "Simulating inheritance section"), code("web-sys"), " as understanding what methods are available on a type means understanding its inheritance."],
        p!["This section is going to look at a specific element and list out its inheritance using Rust by calling ", link!("https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref", code("Deref::deref")), " until the value is ", doc_link!(crate::pages::concepts::basic_web_technologies::wasm_bindgen, #"jsvalue", code("JsValue")), ":"],
        p![link!("https://wasm-bindgen.github.io/wasm-bindgen/web-sys/inheritance.html", "Inheritance in web-sys in The wasm-bindgen Guide"), "."],
        h2!["The Node in NodeRef"],
        p!["Yew uses a ", doc_link!(crate::pages::concepts::function_components::node_refs, code("NodeRef")), " to provide a way for keeping a reference to a ", code("Node"), " made by the ", doc_link!(crate::pages::concepts::html::introduction, code("html!")), " macro. The ", code("Node"), " part of ", code("NodeRef"), " is referring to ", link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Node.html", code("web_sys::Node")), ". The ", code("NodeRef::get"), " method will return a ", code("Option<Node>"), doc_link!(crate::pages::concepts::basic_web_technologies::wasm_bindgen, #"jscast", code("JsCast")), " on the ", code("Node"), " value, if present, but Yew provides the ", code("NodeRef::cast"), code("wasm-bindgen"), " dependency for the ", code("JsCast"), " trait."],
        p!["The two code blocks below do essentially the same thing, the first is using ", code("NodeRef::cast"), " and the second is using ", link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into", code("JsCast::dyn_into")), " on the ", code("web_sys::Node"), " returned from ", code("NodeRef::get"), "."],
        h3!["Using NodeRef::cast"],
        h3!["Using NodeRef::get"],
        h2!["JavaScript example to Rust"],
        p![code("web-sys"), " in Rust."],
        h3!["JavaScript example"],
        h3!["web-sys example"],
        p!["Using ", code("web-sys"), " alone the above JavaScript example could be implemented like this:"],
        p![code("JsCast"), " to cast into different types so that you can call its specific methods."],
        h3!["Yew example"],
        p!["In Yew you will mostly be creating ", doc_link!(crate::pages::concepts::function_components::callbacks, code("Callback")), "s to use in the ", doc_link!(crate::pages::concepts::html::introduction, code("html!"))],
        h2!["External libraries"],
        p![code("web-sys"), code("web-sys"), " to provide more idiomatic Rust APIs."],
        p![link!("/community/external-libs", "External libraries page")]
    ])
    .with_description("The web-sys crate provides bindings for Web APIs.")
);
