# Introduction

## What is Yew?

**Yew** is a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly.

### Architecture

Yew's architecture is heavily inspired by Elm and React. The basic building block of a Yew app is called a `Component`. Each `Component` stores its own state, expresses how to render that state to HTML, and chooses how to respond to asynchronous updates. Yew maintains an internal "virtual DOM" in order to minimize the patches needed to update the browser page DOM for each re-render of a `Component`.

### Concurrency

Yew was built before the standardization of async-await and has promoted the use of the [actor model](https://en.wikipedia.org/wiki/Actor_model) of concurrency. This model will feel very natural if you choose to write the server side of your application with [actix-web](https://github.com/actix/actix-web). In Yew, an actor is called an `Agent`. Using agents, a Yew application can delegate tasks to worker threads using Web Workers and subscribe to async messages from those agents.

An alternative approach is using futures which are can be leveraged through the [wasm-bindgen-futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/) crate which bridges Rust futures to JS Promises. An example project using futures and async-await can be found [here](https://github.com/yewstack/yew/tree/master/examples/futures).

### Safety

Rust helps developers write safer code. For example, in JavaScript, an uncaught error can cause serious problems in your application. Rust encourages proper error handling and you can even get stacktraces for your Rust code with the [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) crate. Also, Yew apps can leverage Rust's static typing to ensure that your `Component` receives the correct properties for creation \(otherwise your app won't compile!\).

### JavaScript  

Yew is built on top of great web tooling like `wasm-bindgen` and `stdweb` and will be supporting `web-sys` and `js-sys` in the near future. These crates enable WebAssembly code to call into JavaScript and vice-versa. For some examples, visit [here](https://github.com/yewstack/yew/tree/master/examples/js_callback) to see how to leverage `stdweb` to write JavaScript code in your Rust app and [here](https://github.com/yewstack/yew/tree/master/examples/npm_and_rest) for how to interact with an NPM module.

### HTML

Yew includes a procedural macro for generating HTML. It closely resembles React's JSX with some exceptions _\(string literals and listener callbacks to name a few\)_. Here is a quick look at its usage:

```rust
html! {
    <section class="todoapp">
        <header class="header">
            <h1>{ "todos" }</h1>
            { self.view_input(&model) }
        </header>
        <section class="main">
            { self.view_entries(&model) }
        </section>
    </section>
}
```

Full guide [here](html.md)

### Why Rust?

Rust is a modern systems language pursuing safety, concurrency, and speed and has been voted the most loved programming languages [multiple](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) [years](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) in a row in Stack Overflow Developer Surveys. Rust can be compiled to WebAssembly using the `cargo` build system which can achieve near-native speeds in a browser.

Similar to how Node.js web applications can share JavaScript code between the server and client, Rust-based apps can reuse the same Rust code for the server and client, with the client code first needing to be compiled to WebAssembly for execution in a browser.

### Why WebAssembly?

First of all, using WASM is not going to be faster than a JavaScript app if you're primarily using DOM APIs. This will probably change in the near future, with the adoption of [Web IDL](https://heycam.github.io/webidl/). But for the time being, Wasm applications have to serialize commands from Wasm to JavaScript to interact with the DOM which will impact performance.

That being said, WebAssembly can be leveraged for data heavy and graphics intensive calculations in the background. When client UI performance is not too important \(internal tooling, for example\) using WebAssembly for the full web application can be acceptable.

