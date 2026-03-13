crate::doc_page!(
    "web-sys",
    "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
    Content::new(vec![
        p(vec![text("The "), code("web-sys"), text(" crate")]),
        h2(vec![text("Features in web-sys")]),
        p(vec![text("The "), code("web-sys"), code("web-sys"), text(" and exposes some types in its public API. You will often need to add "), code("web-sys"), text(" as a dependency yourself.")]),
        h2(vec![text("Inheritance in web-sys")]),
        p(vec![text("In the "), link("/docs/0.21/concepts/basic-web-technologies/wasm-bindgen#simulating-inheritance", vec![text("Simulating inheritance section")]), code("web-sys"), text(" as understanding what methods are available on a type means understanding its inheritance.")]),
        p(vec![text("This section is going to look at a specific element and list out its inheritance using Rust by calling "), link("https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref", vec![code("Deref::deref")]), text(" until the value is "), link("/docs/0.21/concepts/basic-web-technologies/wasm-bindgen#jsvalue", vec![code("JsValue")]), text(":")]),
        p(vec![link("https://wasm-bindgen.github.io/wasm-bindgen/web-sys/inheritance.html", vec![text("Inheritance in web-sys in The wasm-bindgen Guide")]), text(".")]),
        h2(vec![text("The Node in NodeRef")]),
        p(vec![text("Yew uses a "), link("/docs/0.21/concepts/function-components/node-refs", vec![code("NodeRef")]), text(" to provide a way for keeping a reference to a "), code("Node"), text(" made by the "), link("/docs/0.21/concepts/html", vec![code("html!")]), text(" macro. The "), code("Node"), text(" part of "), code("NodeRef"), text(" is referring to "), link("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Node.html", vec![code("web_sys::Node")]), text(". The "), code("NodeRef::get"), text(" method will return a "), code("Option<Node>"), link("/docs/0.21/concepts/basic-web-technologies/wasm-bindgen#jscast", vec![code("JsCast")]), text(" on the "), code("Node"), text(" value, if present, but Yew provides the "), code("NodeRef::cast"), code("wasm-bindgen"), text(" dependency for the "), code("JsCast"), text(" trait.")]),
        p(vec![text("The two code blocks below do essentially the same thing, the first is using "), code("NodeRef::cast"), text(" and the second is using "), link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into", vec![code("JsCast::dyn_into")]), text(" on the "), code("web_sys::Node"), text(" returned from "), code("NodeRef::get"), text(".")]),
        h3(vec![text("Using NodeRef::cast")]),
        h3(vec![text("Using NodeRef::get")]),
        h2(vec![text("JavaScript example to Rust")]),
        p(vec![code("web-sys"), text(" in Rust.")]),
        h3(vec![text("JavaScript example")]),
        h3(vec![text("web-sys example")]),
        p(vec![text("Using "), code("web-sys"), text(" alone the above JavaScript example could be implemented like this:")]),
        p(vec![code("JsCast"), text(" to cast into different types so that you can call its specific methods.")]),
        h3(vec![text("Yew example")]),
        p(vec![text("In Yew you will mostly be creating "), link("/docs/0.21/concepts/function-components/callbacks", vec![code("Callback")]), text("s to use in the "), link("/docs/0.21/concepts/html", vec![code("html!")])]),
        h2(vec![text("External libraries")]),
        p(vec![code("web-sys"), code("web-sys"), text(" to provide more idiomatic Rust APIs.")]),
        p(vec![link("/community/external-libs", vec![text("External libraries page")])])
    ])
);
