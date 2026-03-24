crate::doc_page!("wasm-bindgen", "/ja/docs/concepts/basic-web-technologies/wasm-bindgen",
    Content::new(vec![
        p![
            link![
                "https://github.com/rustwasm/wasm-bindgen",
                code("wasm-bindgen")
            ],
            text(" is a library and tool to facilitate high-level interactions between Wasm modules \
                  and JavaScript; it is built with Rust by "),
            link![
                "https://rustwasm.github.io/",
                text("The Rust and WebAssembly Working Group")
            ],
            text("."),
        ],
        p![
            text("Yew uses "),
            code("wasm-bindgen"),
            text(" to interact with the browser through a number of crates:"),
        ],
        ul![
            li![link!["https://crates.io/crates/js-sys", code("js-sys")]],
            li![link!["https://crates.io/crates/wasm-bindgen", code("wasm-bindgen")]],
            li![link!["https://crates.io/crates/wasm-bindgen-futures", code("wasm-bindgen-futures")]],
            li![link!["https://crates.io/crates/web-sys", code("web-sys")]],
        ],
        p![
            text("This section will explore some of these crates at a high level, to make it easier to understand \
                  and use "),
            code("wasm-bindgen"),
            text(" APIs with Yew. For a more in-depth guide to "),
            code("wasm-bindgen"),
            text(" and its associated crates then check out "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                text("The wasm-bindgen Guide")
            ],
            text("."),
        ],
        p![
            text("For documentation on the above crates check out "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                text("wasm-bindgen docs.rs")
            ],
            text("."),
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                text("Use the "),
                code("wasm-bindgen"),
                text(" doc.rs search to find browser APIs and JavaScript types that have been imported \
                      over using "),
                code("wasm-bindgen"),
                text("."),
            ]
        ],
        h2![link![
            "https://crates.io/crates/wasm-bindgen",
            text("wasm-bindgen")
        ]],
        p![
            text("This crate provides many of the building blocks for the rest of the crates above. In this section we \
                  are only going to cover two main areas of the "),
            code("wasm-bindgen"),
            text(" crate and that is the macro and some types/traits you will see pop up again and again."),
        ],
        h3![text("#[wasm_bindgen] macro")],
        p![
            text("The "),
            code("#[wasm_bindgen]"),
            text(" macro provides an interface between Rust and JavaScript, providing a system \
                  for translating between the two. Using this macro is more advanced, and you should not need to reach \
                  for it unless you are trying to use an external JavaScript library. The "),
            code("js-sys"),
            text(" and "),
            code("web-sys"),
            text(" crates expose "),
            code("wasm-bindgen"),
            text(" definitions for built-in JavaScript types and browser APIs."),
        ],
        p![
            text("Let's go over a simple example of using the "),
            code("#[wasm-bindgen]"),
            text(" macro to import some specific flavours of the "),
            link![
                "https://developer.mozilla.org/en-US/docs/Web/API/Console/log",
                code("console.log")
            ],
            text(" function."),
        ],
        code_block("rust", r#"use wasm_bindgen::prelude::*;

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!
#[wasm_bindgen]
extern "C" {

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// using the imported functions!
log("Hello from Rust!");
log_u32(42);
log_many("Logging", "many values!");"#),
        p![
            italic![
                text("This example was adapted from "),
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html",
                    text("1.2 Using console.log of The wasm-bindgen Guide")
                ],
                text("."),
            ],
        ],
        h3![text("Simulating inheritance")],
        p![
            text("Inheritance between JavaScript classes is a core feature of the JavaScript language and the DOM \
                  (Document Object Model) is designed around it. When types are imported using "),
            code("wasm-bindgen"),
            text(" you can also add attributes that describe their inheritance."),
        ],
        p![
            text("In Rust, this inheritance is represented using the "),
            link!["https://doc.rust-lang.org/std/ops/trait.Deref.html", code("Deref")],
            text(" and "),
            link!["https://doc.rust-lang.org/std/convert/trait.AsRef.html", code("AsRef")],
            text(" traits. An example of this might help; so say you have three types "),
            code("A"),
            text(", "),
            code("B"),
            text(", and "),
            code("C"),
            text(" where "),
            code("C"),
            text(" extends "),
            code("B"),
            text(" which in turn extends "),
            code("A"),
            text("."),
        ],
        p![
            text("When importing these types the "),
            code("#[wasm-bindgen]"),
            text(" macro will implement the "),
            code("Deref"),
            text(" and "),
            code("AsRef"),
            text(" traits in the following way:"),
        ],
        ul![
            li![code("C"), text(" can "), code("Deref"), text(" to "), code("B")],
            li![code("B"), text(" can "), code("Deref"), text(" to "), code("A")],
            li![code("C"), text(" can be "), code("AsRef"), text(" to "), code("B")],
            li![text("Both "), code("C"), text(" & "), code("B"), text(" can be "), code("AsRef"), text(" to "), code("A")],
        ],
        p![
            text("These implementations allow you to call a method from "),
            code("A"),
            text(" on an instance of "),
            code("C"),
            text(" and to use "),
            code("C"),
            text(" as if it was "),
            code("&B"),
            text(" or "),
            code("&A"),
            text("."),
        ],
        p![
            text("It is important to note that every single type imported using "),
            code("#[wasm-bindgen]"),
            text(" has the same root type, you can think of it as the "),
            code("A"),
            text(" in the example above, this type is "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                code("JsValue")
            ],
            text(" which has its section below."),
        ],
        p![
            italic![
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html",
                    text("extends section in The wasm-bindgen Guide")
                ],
            ],
        ],
        h3![link![
            "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
            text("JsValue")
        ]],
        p![
            text("This is a representation of an object owned by JavaScript, this is a root catch-all type for "),
            code("wasm-bindgen"),
            text(". Because JavaScript does not have a strong type system, any type that comes from "),
            code("wasm-bindgen"),
            text(" is a "),
            code("JsValue"),
            text(". Functions in JavaScript do not define the type of any variables they take in or return; \
                  variables can be any valid JavaScript value, hence "),
            code("JsValue"),
            text(". If you are working with imported functions or types that accept a "),
            code("JsValue"),
            text(", then any imported value is "),
            italic![text("technically")],
            text(" valid."),
        ],
        p![
            text("Even though "),
            code("JsValue"),
            text(" may be accepted by a JS function, that function may still only "),
            italic![text("actually")],
            text(" accept certain types. Passing an incorrect "),
            code("JsValue"),
            text(" can lead to an exception which triggers a panic - so when using raw "),
            code("wasm-bindgen"),
            text(" APIs, check your JavaScript's documentation for types of inputs that will cause an exception (and a panic)."),
        ],
        p![
            italic![
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                    text("JsValue documentation")
                ],
                text("."),
            ],
        ],
        h3![link![
            "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
            text("JsCast")
        ]],
        p![
            text("Rust has a strong type system and JavaScript does not. For Rust to maintain these \
                  strong types but still be convenient, the WebAssembly group came up with a pretty neat trait "),
            code("JsCast"),
            text(". Its job is to help you move from one JavaScript \"type\" to another, which sounds vague, but it means \
                  that if you have one type which you know is another, then you can use the functions of "),
            code("JsCast"),
            text(" to jump from one type to the other. It is a nice trait to get to know when working with "),
            code("web-sys"),
            text(", "),
            code("wasm_bindgen"),
            text(", "),
            code("js-sys"),
            text(" - you will notice lots of types will implement "),
            code("JsCast"),
            text(" from those crates."),
        ],
        p![
            code("JsCast"),
            text(" provides both checked and unchecked methods of casting - so if at runtime you are \
                  unsure what type a certain object is, you can try to cast it, which returns possible failure types like "),
            link!["https://doc.rust-lang.org/std/option/enum.Option.html", code("Option")],
            text(" and "),
            link!["https://doc.rust-lang.org/std/result/enum.Result.html", code("Result")],
            text("."),
        ],
        p![
            text("A common example of this in "),
            link!["/ja/docs/concepts/basic-web-technologies/web-sys", code("web-sys")],
            text(" is when you are trying to get the target of an event. You might know what the target element is, but the "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                code("web_sys::Event")
            ],
            text(" API will always return an "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                code("Option<web_sys::EventTarget>")
            ],
            text(". You will need to cast it to the element type so you can call its methods."),
        ],
        code_block("rust",
"// need to import the trait.
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement, HtmlSelectElement};

fn handle_event(event: Event) {
    let target: EventTarget = event
        .target()
        .expect(\"I'm sure this event has a target!\");

    // maybe the target is a select element?
    if let Some(select_element) = target.dyn_ref::<HtmlSelectElement>() {
        // do something amazing here
        return;
    }

    // if it wasn't a select element then I KNOW it's a input element!
    let input_element: HtmlInputElement = target.unchecked_into();
}"),
        p![
            text("The "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_ref",
                code("dyn_ref")
            ],
            text(" method is a checked cast that returns an "),
            code("Option<&T>"),
            text(", which means the original type can be used again if the cast failed and thus returned "),
            code("None"),
            text(". The "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                code("dyn_into")
            ],
            text(" method will consume "),
            code("self"),
            text(", as per convention for "),
            code("into"),
            text(" methods in Rust, and the type returned is "),
            code("Result<T, Self>"),
            text(". If the casting fails, the original "),
            code("Self"),
            text(" value is returned in "),
            code("Err"),
            text(". You can try again or do something else with the original type."),
        ],
        p![
            italic![
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                    text("JsCast documentation")
                ],
                text("."),
            ],
        ],
        h3![link![
            "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html",
            text("Closure")
        ]],
        p![
            text("The "),
            code("Closure"),
            text(" type provides a way to transfer Rust closures to JavaScript. The closures passed to \
                  JavaScript must have a "),
            code("'static"),
            text(" lifetime for soundness reasons."),
        ],
        p![
            text("This type is a \"handle\" in the sense that whenever it is dropped, it will invalidate the JS \
                  closure that it refers to. Any usage of the closure in JS after the "),
            code("Closure"),
            text(" has been dropped will raise an exception."),
        ],
        p![
            code("Closure"),
            text(" is often used when you are working with a "),
            code("js-sys"),
            text(" or "),
            code("web-sys"),
            text(" API that accepts a type "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Function.html",
                code("&js_sys::Function")
            ],
            text(". An example of using a "),
            code("Closure"),
            text(" in Yew can be found in the "),
            link!["/ja/docs/concepts/html/events#using-closure-verbose", text("Using Closure section")],
            text(" on the "),
            link!["/ja/docs/concepts/html/events", text("Events")],
            text(" page."),
        ],
        p![
            italic![
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html",
                    text("Closure documentation")
                ],
                text("."),
            ],
        ],
        h2![link![
            "https://crates.io/crates/js-sys",
            text("js-sys")
        ]],
        p![
            text("The "),
            code("js-sys"),
            text(" crate provides bindings/imports of JavaScript's standard, built-in objects, including \
                  their methods and properties."),
        ],
        p![
            text("This does not include any web APIs; that's what "),
            link!["/ja/docs/concepts/basic-web-technologies/web-sys", code("web-sys")],
            text(" is for!"),
        ],
        p![
            italic![
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/index.html",
                    text("js-sys documentation")
                ],
                text("."),
            ],
        ],
        h2![link![
            "https://crates.io/crates/wasm-bindgen-futures",
            text("wasm-bindgen-futures")
        ]],
        p![
            text("The "),
            code("wasm-bindgen-futures"),
            text(" crate provides a bridge for working with JavaScript Promise types as a Rust "),
            link!["https://doc.rust-lang.org/stable/std/future/trait.Future.html", code("Future")],
            text(", and contains utilities to turn a Rust Future into a JavaScript Promise. This can be useful \
                  when working with asynchronous or otherwise blocking work in Rust (wasm), and provides the ability \
                  to interoperate with JavaScript events and JavaScript I/O primitives."),
        ],
        p![text("There are three main interfaces in this crate currently:")],
        ol![
            li_blocks![
                p![
                    link![
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/struct.JsFuture.html",
                        code("JsFuture")
                    ],
                    text(" - A type that is constructed with a "),
                    link![
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Promise.html",
                        code("Promise")
                    ],
                    text(" and can then be used as a "),
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    text(". This Future will resolve to "),
                    code("Ok"),
                    text(" if the Promise is resolved and "),
                    code("Err"),
                    text(" if the Promise is rejected, containing the resolved or rejected value from the Promise respectively."),
                ],
            ],
            li_blocks![
                p![
                    link![
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.future_to_promise.html",
                        code("future_to_promise")
                    ],
                    text(" - Converts a Rust "),
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    text(" into a JavaScript "),
                    code("Promise"),
                    text(". The future's result will translate to either a resolved or rejected Promise in JavaScript."),
                ],
            ],
            li_blocks![
                p![
                    link![
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                        code("spawn_local")
                    ],
                    text(" - Spawns a "),
                    code("Future<Output = ()>"),
                    text(" on the current thread. This is the best way to run a Future in Rust without sending it to JavaScript."),
                ],
            ],
        ],
        p![
            italic![
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/index.html",
                    text("wasm-bindgen-futures documentation")
                ],
                text("."),
            ],
        ],
        h3![link![
            "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
            text("spawn_local")
        ]],
        p![
            code("spawn_local"),
            text(" is going to be the most commonly used part of the "),
            code("wasm-bindgen-futures"),
            text(" crate in Yew as this helps when using libraries that have async APIs."),
        ],
        code_block("rust",
"use web_sys::console;
use wasm_bindgen_futures::spawn_local;

async fn my_async_fn() -> String { String::from(\"Hello\") }

spawn_local(async {
    let mut string = my_async_fn().await;
    string.push_str(\", world!\");
    // console log \"Hello, world!\"
    console::log_1(&string.into());
});"),
        p![
            text("Yew has also added support for futures in certain APIs, most notably you can create a "),
            code("callback_future"),
            text(" which accepts an "),
            code("async"),
            text(" block - this uses "),
            code("spawn_local"),
            text(" internally."),
        ],
        p![
            italic![
                link![
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                    text("spawn_local documentation")
                ],
                text("."),
            ],
        ],
    ])
);
