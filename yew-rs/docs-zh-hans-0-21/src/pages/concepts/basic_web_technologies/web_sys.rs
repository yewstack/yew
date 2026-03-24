crate::doc_page!(
    "web-sys",
    "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
    Content::new(vec![
        p![text("The "), code("web-sys"), text(" crate")],
        h2![text("Features in web-sys")],
        p![text("The "), code("web-sys"), code("web-sys"), text(" and exposes some types in its public API. You will often need to add "), code("web-sys"), text(" as a dependency yourself.")],
        h2![text("Inheritance in web-sys")],
        p![text("In the "), link!("/docs/0.21/concepts/basic-web-technologies/wasm-bindgen#simulating-inheritance", text("Simulating inheritance section")), code("web-sys"), text(" as understanding what methods are available on a type means understanding its inheritance.")],
        p![text("This section is going to look at a specific element and list out its inheritance using Rust by calling "), link!("https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref", code("Deref::deref")), text(" until the value is "), link!("/docs/0.21/concepts/basic-web-technologies/wasm-bindgen#jsvalue", code("JsValue")), text(":")],
        p![link!("https://wasm-bindgen.github.io/wasm-bindgen/web-sys/inheritance.html", text("Inheritance in web-sys in The wasm-bindgen Guide")), text(".")],
        h2![text("The Node in NodeRef")],
        p![text("Yew uses a "), link!("/docs/0.21/concepts/function-components/node-refs", code("NodeRef")), text(" to provide a way for keeping a reference to a "), code("Node"), text(" made by the "), link!("/docs/0.21/concepts/html", code("html!")), text(" macro. The "), code("Node"), text(" part of "), code("NodeRef"), text(" is referring to "), link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Node.html", code("web_sys::Node")), text(". The "), code("NodeRef::get"), text(" method will return a "), code("Option<Node>"), link!("/docs/0.21/concepts/basic-web-technologies/wasm-bindgen#jscast", code("JsCast")), text(" on the "), code("Node"), text(" value, if present, but Yew provides the "), code("NodeRef::cast"), code("wasm-bindgen"), text(" dependency for the "), code("JsCast"), text(" trait.")],
        p![text("The two code blocks below do essentially the same thing, the first is using "), code("NodeRef::cast"), text(" and the second is using "), link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into", code("JsCast::dyn_into")), text(" on the "), code("web_sys::Node"), text(" returned from "), code("NodeRef::get"), text(".")],
        h3![text("Using NodeRef::cast")],
        h3![text("Using NodeRef::get")],
        h2![text("JavaScript example to Rust")],
        p![code("web-sys"), text(" in Rust.")],
        h3![text("JavaScript example")],
        h3![text("web-sys example")],
        p![text("Using "), code("web-sys"), text(" alone the above JavaScript example could be implemented like this:")],
        p![code("JsCast"), text(" to cast into different types so that you can call its specific methods.")],
        h3![text("Yew example")],
        p![text("In Yew you will mostly be creating "), link!("/docs/0.21/concepts/function-components/callbacks", code("Callback")), text("s to use in the "), link!("/docs/0.21/concepts/html", code("html!"))],
        h2![text("External libraries")],
        p![code("web-sys"), code("web-sys"), text(" to provide more idiomatic Rust APIs.")],
        p![link!("/community/external-libs", text("External libraries page"))]
    ])
);
