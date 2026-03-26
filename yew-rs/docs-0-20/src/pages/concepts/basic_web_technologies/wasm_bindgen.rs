crate::doc_page!("wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen",
    Content::new(vec![
        p![
            link!("https://github.com/rustwasm/wasm-bindgen", code("wasm-bindgen")),
            " is a library and tool to facilitate high-level interactions between Wasm modules \
              and JavaScript; it is built with Rust by ",
            link!("https://rustwasm.github.io/", "The Rust and WebAssembly Working Group"),
            ".",
        ],
        p![
            "Yew uses ",
            code("wasm-bindgen"),
            " to interact with the browser through a number of crates:",
        ],
        ul![
            li![link!("https://crates.io/crates/js-sys", code("js-sys"))],
            li![link!("https://crates.io/crates/wasm-bindgen", code("wasm-bindgen"))],
            li![link!("https://crates.io/crates/wasm-bindgen-futures", code("wasm-bindgen-futures"))],
            li![link!("https://crates.io/crates/web-sys", code("web-sys"))],
        ],
        p![
            "This section will explore some of these crates in a high level in order to make it easier to understand \
              and use ",
            code("wasm-bindgen"),
            " APIs with Yew. For a more in-depth guide to ",
            code("wasm-bindgen"),
            " and its associated crates then check out ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/", "The wasm-bindgen Guide"),
            ".",
        ],
        p![
            "For documentation on the above crates check out ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html", "wasm-bindgen docs.rs"),
            ".",
        ],
        admonition![AdmonitionType::Tip, None,
            p![
                "Use the ",
                code("wasm-bindgen"),
                " doc.rs search to find browser APIs and JavaScript types that have been imported \
                  over using ",
                code("wasm-bindgen"),
                ".",
            ],
        ],
        h2![link!("https://crates.io/crates/wasm-bindgen", "wasm-bindgen")],
        p![
            "This crate provides many of the building blocks for the rest of the crates above. In this section we \
              are only going to cover two main areas of the ",
            code("wasm-bindgen"),
            " crate and that is the macro and some types / traits you will see pop up again and again.",
        ],
        h3!["#[wasm_bindgen] macro"],
        p![
            "The ",
            code("#[wasm_bindgen]"),
            " macro provides an interface between Rust and JavaScript, providing a system \
              for translating between the two. Using this macro is more advanced, and you shouldn't need to reach \
              for it unless you are trying to use an external JavaScript library. The ",
            code("js-sys"),
            " and ",
            code("web-sys"),
            " crates expose ",
            code("wasm-bindgen"),
            " definitions for built-in Javascript types and browser APIs.",
        ],
        p![
            "Let's go over a simple example of using the ",
            code("#[wasm-bindgen]"),
            " macro to import some specific flavours of the ",
            link!("https://developer.mozilla.org/en-US/docs/Web/API/Console/log", code("console.log")),
            " function.",
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
                "This example was adapted from ",
                link!("https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html", "1.2 Using console.log of The wasm-bindgen Guide"),
                ".",
            ],
        ],
        h3!["Simulating inheritance"],
        p![
            "Inheritance between JavaScript classes is a core feature of the Javascript language, and the DOM \
              (Document Object Model) is designed around it. When types are imported using ",
            code("wasm-bindgen"),
            " you can also add attributes that describe their inheritance.",
        ],
        p![
            "In Rust this inheritance is represented using the ",
            link!("https://doc.rust-lang.org/std/ops/trait.Deref.html", code("Deref")),
            " and ",
            link!("https://doc.rust-lang.org/std/convert/trait.AsRef.html", code("AsRef")),
            " traits. An example of this might help; so say you have three types ",
            code("A"),
            ", ",
            code("B"),
            ", and ",
            code("C"),
            " where ",
            code("C"),
            " extends ",
            code("B"),
            " which in turn extends ",
            code("A"),
            ".",
        ],
        p![
            "When importing these types the ",
            code("#[wasm-bindgen]"),
            " macro will implement the ",
            code("Deref"),
            " and ",
            code("AsRef"),
            " traits in the following way:",
        ],
        ul![
            li![code("C"), " can ", code("Deref"), " to ", code("B")],
            li![code("B"), " can ", code("Deref"), " to ", code("A")],
            li![code("C"), " can be ", code("AsRef"), " to ", code("B")],
            li!["Both ", code("C"), " & ", code("B"), " can be ", code("AsRef"), " to ", code("A")],
        ],
        p![
            "These implementations allow you to call a method from ",
            code("A"),
            " on an instance of ",
            code("C"),
            " and to use ",
            code("C"),
            " as if it was ",
            code("&B"),
            " or ",
            code("&A"),
            ".",
        ],
        p![
            "Its important to note that every single type imported using ",
            code("#[wasm-bindgen]"),
            " has the same root type, you can think of it as the ",
            code("A"),
            " in the example above, this type is ",
            link!("#jsvalue", code("JsValue")),
            " which has its own section below.",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html", "extends section in The wasm-bindgen Guide"),
            ],
        ],
        h3_id!("jsvalue",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html", "JsValue"),
        ),
        p![
            "This is a representation of an object owned by JavaScript, this is a root catch-all type for ",
            code("wasm-bindgen"),
            ". Any type that comes from ",
            code("wasm-bindgen"),
            " is a ",
            code("JsValue"),
            " and this is because JavaScript doesn't have a strong type system so any function that accepts a variable ",
            code("x"),
            " doesn't define its type so ",
            code("x"),
            " can be a valid JavaScript value; hence ",
            code("JsValue"),
            ". If you are working with imported functions or types that accept a ",
            code("JsValue"),
            ", then any imported value is ",
            italic!["technically"],
            " valid.",
        ],
        p![
            code("JsValue"),
            " can be accepted by a function but that function may still only accept certain types and this \
              can lead to panics - so when using raw ",
            code("wasm-bindgen"),
            " APIs check the documentation of the JavaScript \
              being imported as to whether an exception (panic) will be raised if that value is not a certain type.",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html", "JsValue documentation"),
                ".",
            ],
        ],
        h3_id!("JsCast",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", "JsCast"),
        ),
        p![
            "Rust has a strong type system and JavaScript...doesn't. In order for Rust to maintain these \
              strong types but still be convenient the WebAssembly group came up with a pretty neat trait ",
            code("JsCast"),
            ". Its job is to help you move from one JavaScript \"type\" to another, which sounds vague, but it means \
              that if you have one type which you know is really another then you can use the functions of ",
            code("JsCast"),
            " to jump from one type to the other. It's a nice trait to get to know when working with ",
            code("web-sys"),
            ", ",
            code("wasm_bindgen"),
            ", ",
            code("js-sys"),
            " - you'll notice lots of types will implement ",
            code("JsCast"),
            " from those crates.",
        ],
        p![
            code("JsCast"),
            " provides both checked and unchecked methods of casting - so that at runtime if you are \
              unsure what type a certain object is you can try to cast it which returns possible failure types like ",
            link!("https://doc.rust-lang.org/std/option/enum.Option.html", code("Option")),
            " and ",
            link!("https://doc.rust-lang.org/std/result/enum.Result.html", code("Result")),
            ".",
        ],
        p![
            "A common example of this in ",
            link!("/docs/concepts/basic-web-technologies/web-sys", code("web-sys")),
            " is when you are trying to get the target of an event, you might know what the target element is but the ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html", code("web_sys::Event")),
            " API will always return an ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target", code("Option<web_sys::EventTarget>")),
            " so you will need to cast it to the element type. so you can call its methods.",
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
            "The ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_ref", code("dyn_ref")),
            " method is a checked cast that returns an ",
            code("Option<&T>"),
            " which means the original type can be used again if the cast failed and thus returned ",
            code("None"),
            ". The ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into", code("dyn_into")),
            " method will consume ",
            code("self"),
            ", as per convention for into methods in Rust, and the type returned is ",
            code("Result<T, Self>"),
            ". If the casting fails, the original ",
            code("Self"),
            " value is returned in ",
            code("Err"),
            ". You can try again or do something else with the original type.",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", "JsCast documentation"),
                ".",
            ],
        ],
        h3![
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html", "Closure"),
        ],
        p![
            "The ",
            code("Closure"),
            " type provides a way to transfer Rust closures to JavaScript, the closures passed to \
              JavaScript must have a ",
            code("'static"),
            " lifetime for soundness reasons.",
        ],
        p![
            "This type is a \"handle\" in the sense that whenever it is dropped it will invalidate the JS \
              closure that it refers to. Any usage of the closure in JS after the ",
            code("Closure"),
            " has been dropped will raise an exception.",
        ],
        p![
            code("Closure"),
            " is often used when you are working with a ",
            code("js-sys"),
            " or ",
            code("web-sys"),
            " API that accepts a type ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Function.html", code("&js_sys::Function")),
            ". An example of using a ",
            code("Closure"),
            " in Yew can be found in the ",
            link!("/docs/concepts/html/events#using-closure-verbose", "Using Closure section"),
            " on the ",
            link!("/docs/concepts/html/events", "Events"),
            " page.",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html", "Closure documentation"),
                ".",
            ],
        ],
        h2![link!("https://crates.io/crates/js-sys", "js-sys")],
        p![
            "The ",
            code("js-sys"),
            " crate provides bindings / imports of JavaScript's standard, built-in objects, including \
              their methods and properties.",
        ],
        p![
            "This does not include any web APIs as this is what ",
            link!("/docs/concepts/basic-web-technologies/web-sys", code("web-sys")),
            " is for!",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/index.html", "js-sys documentation"),
                ".",
            ],
        ],
        h2![link!("https://crates.io/crates/wasm-bindgen-futures", "wasm-bindgen-futures")],
        p![
            "The ",
            code("wasm-bindgen-futures"),
            " crate provides a bridge for working with JavaScript Promise types as a Rust ",
            link!("https://doc.rust-lang.org/stable/std/future/trait.Future.html", code("Future")),
            ", and contains utilities to turn a Rust Future into a JavaScript Promise. This can be useful \
              when working with asynchronous or otherwise blocking work in Rust (wasm), and provides the ability \
              to interoperate with JavaScript events and JavaScript I/O primitives.",
        ],
        p!["There are three main interfaces in this crate currently:"],
        ol![
            li![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/struct.JsFuture.html", code("JsFuture")),
                " - A type that is constructed with a ",
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Promise.html", code("Promise")),
                " and can then be used as a ",
                code("Future<Output=Result<JsValue, JsValue>>"),
                ". This Rust future will resolve or reject with the value coming out of the ",
                code("Promise"),
                ".",
            ],
            li![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.future_to_promise.html", code("future_to_promise")),
                " - Converts a Rust ",
                code("Future<Output=Result<JsValue, JsValue>>"),
                " into a JavaScript ",
                code("Promise"),
                ". The future's result will translate to either a resolved or rejected Promise in JavaScript.",
            ],
            li![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html", code("spawn_local")),
                " - Spawns a ",
                code("Future<Output = ()>"),
                " on the current thread. This is the best way to run a Future in Rust without sending it to JavaScript.",
            ],
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/index.html", "wasm-bindgen-futures documentation"),
                ".",
            ],
        ],
        h3![
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html", "spawn_local"),
        ],
        p![
            code("spawn_local"),
            " is going to be the most commonly used part of the ",
            code("wasm-bindgen-futures"),
            " crate in Yew as this helps when using libraries that have async APIs.",
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
            "Yew has also added support for futures in certain APIs, most notably you can create a ",
            code("callback_future"),
            " which accepts an ",
            code("async"),
            " block - this uses ",
            code("spawn_local"),
            " internally.",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html", "spawn_local documentation"),
                ".",
            ],
        ],
    ])
);
