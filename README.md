# Introduction

## What is Yew?

**Yew** is a Rust framework for creating multi-threaded frontend web apps with WebAssembly.

#### Why Rust and WebAssembly?

\*\*\*\*[**Rust**](https://www.rust-lang.org/) is blazing fast and super reliable with its rich type system and ownership model. It can have a tough learning curve but worth the effort. Rust has been voted the most loved programming language [multiple](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) [years](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) in a row in Stack Overflow Developer Surveys. 

\*\*\*\*[**WebAssembly**](https://webassembly.org/) _\(Wasm\)_ is a portable low-level language that Rust can compile into which aims to run at native speeds in the browser and is interoperable with JavaScript and supported in all major browsers. 

### Modern Web Framework

Yew is a component-based framework that makes it easy to create complex interactive UIs. Developers who have experience with frameworks like React and Elm should feel quite at home when using Yew. Creating HTML in Yew even looks a lot like React's JSX with a few minor exceptions. Here's a quick look:

```rust
fn view(&self) -> Html<Self> {
  html! {
    <section class="todoapp">
      <header class="header">
        <h1>{ "todos" }</h1>
      </header>
      <section class="main">
        <input type="checkbox"
            checked=self.all_completed()
            onclick=|_| Msg::ToggleAll />
        { self.view_todos() }
      </section>
    </section>
  }
}
```

### Performance and Concurrency

First and foremost, it should be clear that using Wasm is not a silver bullet for improving the performance of a web app. As of right now, using DOM APIs from WebAssembly is still slower than calling them directly from JavaScript. This is a temporary hurdle which the [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) proposal aims to resolve. If you would like to learn more, check out this [excellent article](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) from Mozilla.

In the meantime, Yew will boost the performance of your app by minimizing the number of expensive DOM API calls and making it simple to leverage workers to offload processing from the main browser thread. For more ideas on how WebAssembly can help out your app, check out this list of [Use Cases](https://webassembly.org/docs/use-cases/).

### Type Safety and Reliability 

Rust helps developers write safer code with its rich type system and ownership model. Say goodbye to hard to track down race condition bugs in JavaScript! In fact, with Rust, most of your bugs will be caught by the compiler before your app even runs, often with a very helpful error message explaining what went wrong. Rust also encourages proper error handling and in the uncommon case that your app panics, you can even get full stack-traces for your Rust code in the browser console.

### JavaScript Interop

Yew is built on top of great web tooling created by the Rust community. It's easy to call JavaScript code from Rust and vice-versa, enabling you to try out Yew for a small part of your web app without any headaches. It's even possible to use your favorite NPM packages within Yew! 

### 

